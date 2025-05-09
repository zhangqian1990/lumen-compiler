use std::path::Path;
use log::{debug, info};
use anyhow::{Result, anyhow};

// 引入其他模块
use lumen_core::IR;
use lumen_compiler::CompileOptions;
use lumen_parser::{ParseOptions, JsParser};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

/// Wasm选项
#[derive(Debug, Clone)]
pub struct WasmOptions {
    /// 是否生成内存高效的Wasm
    pub memory_efficient: bool,
    /// 是否优化Wasm大小
    pub optimize_size: bool,
    /// 是否启用SIMD
    pub enable_simd: bool,
    /// 是否启用多线程
    pub enable_threads: bool,
    /// 最大内存使用（MB）
    pub max_memory_mb: usize,
    /// 特定于目标的选项
    pub target_features: Vec<String>,
}

impl Default for WasmOptions {
    fn default() -> Self {
        Self {
            memory_efficient: true,
            optimize_size: true,
            enable_simd: false,
            enable_threads: false,
            max_memory_mb: 1024,
            target_features: Vec::new(),
        }
    }
}

/// Wasm转换选项
#[derive(Debug, Clone)]
pub struct WasmTransformOptions {
    /// 编译选项
    pub compile_options: CompileOptions,
    /// Wasm特定选项
    pub wasm_options: WasmOptions,
    /// 是否生成TypeScript类型定义
    pub generate_types: bool,
    /// 是否导出ES模块
    pub es_module: bool,
}

impl Default for WasmTransformOptions {
    fn default() -> Self {
        Self {
            compile_options: CompileOptions::default(),
            wasm_options: WasmOptions::default(),
            generate_types: true,
            es_module: true,
        }
    }
}

/// WASM输出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WasmOutputFormat {
    /// 原始Wasm二进制格式
    Binary,
    /// Wasm文本格式(WAT)
    Text,
    /// JavaScript包装器
    JavaScript,
    /// JavaScript包装器+Wasm二进制
    Bundle,
}

/// WASM转换结果
#[derive(Debug)]
pub struct WasmTransformResult {
    /// 主要输出内容
    pub output: Vec<u8>,
    /// 输出格式
    pub format: WasmOutputFormat,
    /// 生成的TypeScript类型定义（如果请求）
    pub typescript_types: Option<String>,
    /// 额外元数据
    pub metadata: serde_json::Value,
    /// 错误信息
    pub errors: Vec<String>,
    /// 警告信息
    pub warnings: Vec<String>,
    /// 处理时间（毫秒）
    pub time_ms: u64,
}

/// WASM转换器
pub struct WasmTransformer {
    options: WasmTransformOptions,
}

impl WasmTransformer {
    pub fn new(options: WasmTransformOptions) -> Self {
        Self { options }
    }
    
    pub fn transform_to_wasm(&self, _ir: &IR, format: WasmOutputFormat) -> Result<WasmTransformResult> {
        let start = std::time::Instant::now();
        debug!("开始转换IR到WebAssembly，格式: {:?}", format);
        
        // TODO: 实现IR到WASM的转换
        // 这里只是一个简单的示例
        
        // 生成模拟结果
        let output = match format {
            WasmOutputFormat::Binary => vec![0, 97, 115, 109, 1, 0, 0, 0], // WASM魔数 + 版本号
            WasmOutputFormat::Text => b"(module)".to_vec(),
            WasmOutputFormat::JavaScript => b"export function init() { /* ... */ }".to_vec(),
            WasmOutputFormat::Bundle => {
                let js = b"export function init(wasmBytes) { /* ... */ }";
                let wasm = vec![0, 97, 115, 109, 1, 0, 0, 0];
                
                // 组合JS和WASM
                [js, &b"\n// WASM_BINARY:"[..], &wasm[..]].concat()
            }
        };
        
        let ts_types = if self.options.generate_types {
            Some("export function init(): Promise<void>;".to_string())
        } else {
            None
        };
        
        let metadata = serde_json::json!({
            "version": "0.1.0",
            "format": format_to_string(format),
            "features": {
                "simd": self.options.wasm_options.enable_simd,
                "threads": self.options.wasm_options.enable_threads,
            }
        });
        
        let elapsed = start.elapsed();
        info!("WebAssembly转换完成，格式: {:?}, 大小: {} 字节, 耗时: {:?}", 
            format, output.len(), elapsed);
        
        Ok(WasmTransformResult {
            output,
            format,
            typescript_types: ts_types,
            metadata,
            errors: Vec::new(),
            warnings: Vec::new(),
            time_ms: elapsed.as_millis() as u64,
        })
    }
    
    pub fn transform_js(&self, source: &str, format: WasmOutputFormat) -> Result<WasmTransformResult> {
        debug!("解析并转换JavaScript到WebAssembly");
        
        // 解析JavaScript
        let parser = JsParser::new(self.options.compile_options.parse_options.clone());
        let ir = parser.parse_string(source)?;
        
        // 转换为WebAssembly
        self.transform_to_wasm(&ir, format)
    }
    
    pub fn transform_file<P: AsRef<Path>>(&self, input: P, format: WasmOutputFormat) -> Result<WasmTransformResult> {
        let input_path = input.as_ref();
        info!("转换文件到WebAssembly: {}", input_path.display());
        
        // 读取输入文件
        let source = std::fs::read_to_string(input_path)
            .map_err(|e| anyhow!("读取文件失败: {}", e))?;
        
        self.transform_js(&source, format)
    }
    
    pub fn write_output<P: AsRef<Path>>(&self, result: &WasmTransformResult, output_path: P) -> Result<()> {
        let output_path = output_path.as_ref();
        info!("写入WebAssembly输出: {}", output_path.display());
        
        std::fs::write(output_path, &result.output)
            .map_err(|e| anyhow!("写入输出文件失败: {}", e))?;
        
        // 如果生成了TypeScript类型定义，写入相应的.d.ts文件
        if let Some(ref ts_types) = result.typescript_types {
            let mut ts_path = output_path.to_path_buf();
            let file_stem = output_path.file_stem()
                .ok_or_else(|| anyhow!("无法获取文件名"))?
                .to_string_lossy();
            
            ts_path.set_file_name(format!("{}.d.ts", file_stem));
            
            std::fs::write(&ts_path, ts_types)
                .map_err(|e| anyhow!("写入TypeScript类型定义失败: {}", e))?;
            
            info!("TypeScript类型定义已写入: {}", ts_path.display());
        }
        
        Ok(())
    }
}

/// 将输出格式转换为字符串
fn format_to_string(format: WasmOutputFormat) -> &'static str {
    match format {
        WasmOutputFormat::Binary => "binary",
        WasmOutputFormat::Text => "text",
        WasmOutputFormat::JavaScript => "javascript",
        WasmOutputFormat::Bundle => "bundle",
    }
}

/// Wasm编译器 - 结合编译和Wasm转换
pub struct WasmCompiler {
    config: WasmConfig,
}

#[derive(Debug, Clone)]
pub struct WasmConfig {
    pub optimize_level: u8,
    pub enable_simd: bool,
    pub enable_threads: bool,
}

impl Default for WasmConfig {
    fn default() -> Self {
        Self {
            optimize_level: 1,
            enable_simd: false,
            enable_threads: false,
        }
    }
}

impl WasmCompiler {
    pub fn new() -> Self {
        Self {
            config: WasmConfig::default(),
        }
    }

    pub fn with_config(config: WasmConfig) -> Self {
        Self { config }
    }

    pub fn compile_to_wasm(&self, _ir: &IR) -> Result<Vec<u8>> {
        // 简单实现，返回一个最小的WASM模块
        // 实际实现应转换IR到WASM
        let min_wasm_module = vec![
            0x00, 0x61, 0x73, 0x6d, // magic: \0asm
            0x01, 0x00, 0x00, 0x00, // version: 1
        ];
        
        Ok(min_wasm_module)
    }
    
    // 添加从源字符串编译的方法
    pub fn compile_from_source(&self, source: &str) -> Result<Vec<u8>> {
        // 在实际实现中，应该先解析源码得到IR，然后编译到WASM
        // 这里简单返回一个最小的WASM模块
        info!("从源代码编译到WASM，长度: {} 字节", source.len());
        
        let min_wasm_module = vec![
            0x00, 0x61, 0x73, 0x6d, // magic: \0asm
            0x01, 0x00, 0x00, 0x00, // version: 1
        ];
        
        Ok(min_wasm_module)
    }
    
    // 添加编译文件的方法
    pub fn compile_file<P: AsRef<Path>>(&self, input_path: P, output_path: P) -> Result<()> {
        let input = input_path.as_ref();
        let output = output_path.as_ref();
        
        info!("编译文件到WASM: {} -> {}", input.display(), output.display());
        
        // 读取输入文件
        let source = std::fs::read_to_string(input)
            .map_err(|e| anyhow!("读取文件失败: {}", e))?;
        
        // 编译到WASM
        let wasm_data = self.compile_from_source(&source)?;
        
        // 写入输出文件
        std::fs::write(output, &wasm_data)
            .map_err(|e| anyhow!("写入输出文件失败: {}", e))?;
        
        Ok(())
    }
}

pub fn compile_to_wasm(ir: &IR) -> Result<Vec<u8>> {
    let compiler = WasmCompiler::new();
    compiler.compile_to_wasm(ir)
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub struct WasmCompilerJS {
    inner: WasmCompiler,
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
impl WasmCompilerJS {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            inner: WasmCompiler::new(),
        }
    }
    
    #[wasm_bindgen]
    pub fn compile(&self, source: &str) -> Result<JsValue, JsValue> {
        let result = self.inner.compile_to_wasm(source, WasmOutputFormat::Bundle)
            .map_err(|e| JsValue::from_str(&e))?;
            
        // 将结果转换为JS对象
        let output_array = js_sys::Uint8Array::new_with_length(result.output.len() as u32);
        output_array.copy_from(&result.output);
        
        let js_result = js_sys::Object::new();
        js_sys::Reflect::set(&js_result, &JsValue::from_str("output"), &output_array)
            .map_err(|_| JsValue::from_str("无法设置输出属性"))?;
            
        js_sys::Reflect::set(&js_result, &JsValue::from_str("format"), &JsValue::from_str(format_to_string(result.format)))
            .map_err(|_| JsValue::from_str("无法设置格式属性"))?;
            
        if let Some(ts_types) = result.typescript_types {
            js_sys::Reflect::set(&js_result, &JsValue::from_str("typescriptTypes"), &JsValue::from_str(&ts_types))
                .map_err(|_| JsValue::from_str("无法设置TypeScript类型属性"))?;
        }
        
        Ok(js_result.into())
    }
    
    #[wasm_bindgen]
    pub fn set_target(&mut self, target: &str) {
        self.inner = self.inner.clone().with_target(target);
    }
    
    #[wasm_bindgen]
    pub fn enable_simd(&mut self, enable: bool) {
        self.inner = self.inner.clone().with_simd(enable);
    }
    
    #[wasm_bindgen]
    pub fn enable_threads(&mut self, enable: bool) {
        self.inner = self.inner.clone().with_threads(enable);
    }
    
    #[wasm_bindgen]
    pub fn generate_typescript(&mut self, enable: bool) {
        self.inner = self.inner.clone().with_typescript(enable);
    }
}

// 便捷函数
/// 快速将JavaScript转换为Wasm
pub fn js_to_wasm(source: &str) -> Result<Vec<u8>> {
    let compiler = WasmCompiler::new();
    compiler.compile_from_source(source)
}

/// 快速编译文件到Wasm
pub fn compile_file_to_wasm<P: AsRef<Path>>(input: P, output: P) -> Result<()> {
    let compiler = WasmCompiler::new();
    compiler.compile_file(input, output)
} 
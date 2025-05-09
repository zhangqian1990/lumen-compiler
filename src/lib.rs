use std::path::Path;
use std::collections::HashMap;
use std::time::Instant;
use log::{debug, info, warn};

mod error;
mod config;
mod utils;
mod ffi;
mod gpu;
mod distributed;

pub use error::{Error, Result};
pub use config::CompileOptions;
pub use gpu::GpuOptimizer;
pub use distributed::DistributedCompiler;

/// Lumen编译器主入口
pub struct Lumen {
    config: config::Config,
    cache: Option<HashMap<String, Vec<u8>>>,
}

impl Lumen {
    /// 创建一个新的Lumen编译器实例
    pub fn new() -> Self {
        Self {
            config: config::Config::default(),
            cache: Some(HashMap::new()),
        }
    }
    
    /// 使用自定义配置创建实例
    pub fn with_config(config: config::Config) -> Self {
        Self {
            config,
            cache: Some(HashMap::new()),
        }
    }
    
    /// 编译单个字符串
    pub fn compile_str(&self, source: &str, filename: Option<&str>) -> Result<String> {
        let filename = filename.unwrap_or("input.js");
        debug!("编译字符串内容，模拟文件名: {}", filename);
        
        let start = Instant::now();
        
        // TODO: 实现实际的编译逻辑
        // 1. 调用解析器 (lumen-parser)
        // 2. 生成IR (lumen-core)
        // 3. 应用优化 (lumen-optimizers)
        // 4. 生成代码 (lumen-compiler)
        
        // 模拟一个编译结果
        let result = format!(
            "// 由Lumen编译 - https://github.com/lumen-compiler\n// 源文件: {}\n{}",
            filename,
            source.replace("var ", "let ") // 简单模拟一下转换
        );
        
        info!("编译完成，耗时: {:?}", start.elapsed());
        
        Ok(result)
    }
    
    /// 编译单个文件
    pub fn compile_file<P: AsRef<Path>>(&self, input: P, output: Option<P>) -> Result<()> {
        let input_path = input.as_ref();
        info!("编译文件: {}", input_path.display());
        
        let start = Instant::now();
        
        // 读取输入文件
        let source = std::fs::read_to_string(input_path)
            .map_err(|e| Error::IoError(e))?;
        
        // 编译内容
        let compiled = self.compile_str(&source, Some(input_path.to_str().unwrap_or("unknown")))?;
        
        // 确定输出路径
        let output_path = match output {
            Some(p) => p.as_ref().to_path_buf(),
            None => {
                let mut out_path = input_path.to_path_buf();
                if let Some(ext) = out_path.extension() {
                    let mut new_ext = ext.to_os_string();
                    new_ext.push(".compiled");
                    out_path.set_extension(new_ext);
                } else {
                    out_path.set_extension("compiled");
                }
                out_path
            }
        };
        
        // 写入输出文件
        std::fs::write(&output_path, compiled)
            .map_err(|e| Error::IoError(e))?;
            
        info!("编译完成，输出到: {}，耗时: {:?}", output_path.display(), start.elapsed());
        
        Ok(())
    }
    
    /// 批量编译文件
    pub fn compile_files<P: AsRef<Path>>(&self, inputs: Vec<P>, output_dir: Option<P>) -> Result<()> {
        let start = Instant::now();
        info!("批量编译 {} 个文件", inputs.len());
        
        let output_dir = output_dir.map(|p| p.as_ref().to_path_buf());
        
        // 如果指定了输出目录，确保它存在
        if let Some(dir) = &output_dir {
            if !dir.exists() {
                std::fs::create_dir_all(dir)
                    .map_err(|e| Error::IoError(e))?;
            }
        }
        
        // 使用标准迭代器而不是并行迭代器
        let results: Vec<Result<()>> = inputs.iter().map(|input| {
            let input_path = input.as_ref();
            let output_path = match &output_dir {
                Some(dir) => {
                    let file_name = input_path.file_name().unwrap_or_default();
                    let mut path = dir.join(file_name);
                    path.set_extension("js");
                    Some(path)
                },
                None => None,
            };
            
            self.compile_file(input_path, output_path.as_ref().map(|v| &**v))
        }).collect();
        
        // 检查结果
        let mut success_count = 0;
        let mut error_count = 0;
        
        for result in results {
            match result {
                Ok(_) => success_count += 1,
                Err(_) => error_count += 1,
            }
        }
        
        info!(
            "批量编译完成: 成功 {}, 失败 {}, 总耗时: {:?}", 
            success_count, error_count, Instant::now().duration_since(start)
        );
        
        if error_count > 0 {
            warn!("存在 {} 个文件编译失败", error_count);
            return Err(Error::BatchCompilationFailed(error_count));
        }
        
        Ok(())
    }
    
    /// 启用代码压缩
    pub fn with_minify(mut self, enable: bool) -> Self {
        self.config.minify = enable;
        self
    }
    
    /// 启用sourcemap生成
    pub fn with_sourcemap(mut self, enable: bool) -> Self {
        self.config.sourcemap = enable;
        self
    }
    
    /// 设置目标环境
    pub fn with_target(mut self, target: &str) -> Self {
        self.config.target = target.to_string();
        self
    }
    
    /// 启用分布式编译
    pub fn with_distributed(mut self, enable: bool) -> Self {
        self.config.distributed = enable;
        self
    }
    
    /// 启用GPU加速
    pub fn with_gpu(mut self, enable: bool) -> Self {
        self.config.gpu = enable;
        self
    }
    
    /// 清除缓存
    pub fn clear_cache(&mut self) {
        if let Some(cache) = &mut self.cache {
            cache.clear();
        }
    }
    
    /// 禁用缓存
    pub fn disable_cache(&mut self) {
        self.cache = None;
    }
}

// 添加混合语言编译器
pub struct Compiler {
    options: config::Config,
    use_cpp: bool,
}

impl Compiler {
    /// 创建一个新的编译器实例
    pub fn new() -> Self {
        Self {
            options: config::Config::default(),
            use_cpp: true, // 默认使用C++解析器
        }
    }
    
    /// 编译单个字符串
    pub fn compile_str(&self, source: &str, filename: Option<&str>) -> Result<CompileResult> {
        let filename = filename.unwrap_or("input.js");
        info!("编译字符串内容，文件名: {}", filename);
        
        let start = Instant::now();
        
        // 1. 解析阶段 - 使用C++或Rust解析器
        let ir_json = if self.use_cpp {
            self.parse_with_cpp(source, filename)?
        } else {
            self.parse_with_rust(source, filename)?
        };
        
        let parse_time = start.elapsed();
        debug!("解析阶段完成，耗时: {:?}", parse_time);
        
        // 2. 优化阶段
        let optimized_ir = self.optimize_ir(&ir_json)?;
        let optimize_time = start.elapsed() - parse_time;
        debug!("优化阶段完成，耗时: {:?}", optimize_time);
        
        // 3. 代码生成阶段
        let output = self.generate_code(&optimized_ir)?;
        let generate_time = start.elapsed() - optimize_time - parse_time;
        debug!("代码生成阶段完成，耗时: {:?}", generate_time);
        
        let elapsed = start.elapsed();
        info!("编译完成，总耗时: {:?}", elapsed);
        
        let output_size = output.len();
        let result = CompileResult {
            code: output.clone(),
            source_map: None, // TODO: 实现sourcemap生成
            time_ms: elapsed.as_millis() as u64,
            input_size: source.len(),
            output_size,
            compression_ratio: if source.len() > 0 {
                1.0 - (output.len() as f64 / source.len() as f64)
            } else {
                0.0
            },
        };
        
        Ok(result)
    }
    
    /// 使用C++解析器解析源代码
    fn parse_with_cpp(&self, source: &str, filename: &str) -> Result<String> {
        debug!("使用C++解析器解析文件: {}", filename);
        
        let is_ts = filename.ends_with(".ts") || filename.ends_with(".tsx");
        let is_jsx = filename.ends_with(".jsx") || filename.ends_with(".tsx");
        
        let result = if is_ts && is_jsx {
            ffi::cpp_bindings::CppParser::parse_tsx(source)
        } else if is_ts {
            ffi::cpp_bindings::CppParser::parse_ts(source)
        } else if is_jsx {
            ffi::cpp_bindings::CppParser::parse_jsx(source)
        } else {
            ffi::cpp_bindings::CppParser::parse_js(source)
        };
        
        result.map_err(|e| Error::ParseError(e))
    }
    
    /// 使用Rust解析器解析源代码
    fn parse_with_rust(&self, source: &str, filename: &str) -> Result<String> {
        debug!("使用Rust解析器解析代码");
        let parse_options = lumen_parser::ParseOptions::default();
        let parser = lumen_parser::JsParser::new(parse_options);
        
        let ir = parser.parse_string(source)
            .map_err(|e| Error::ParseError(e.to_string()))?;
            
        // 转换为JSON
        let json = serde_json::to_string(&ir)
            .map_err(|e| Error::InternalError(format!("IR转JSON失败: {}", e)))?;
            
        Ok(json)
    }
    
    /// 优化IR
    fn optimize_ir(&self, ir_json: &str) -> Result<String> {
        if !self.options.minify {
            return Ok(ir_json.to_string());
        }
        
        debug!("开始优化IR");
        
        let level = match self.options.minify {
            true => 3, // 激进优化
            false => 0, // 无优化
        };
        
        if self.use_cpp {
            // 使用C++优化器
            ffi::cpp_bindings::CppOptimizer::optimize(ir_json, level)
                .map_err(|e| Error::CompileError(format!("C++优化器错误: {}", e)))
        } else {
            // 使用Rust优化器 (简化实现)
            Ok(ir_json.to_string())
        }
    }
    
    /// 生成最终代码
    fn generate_code(&self, ir_json: &str) -> Result<String> {
        debug!("生成输出代码");
        
        // 解析IR JSON
        let ir: lumen_core::IR = serde_json::from_str(ir_json)
            .map_err(|e| Error::InternalError(format!("IR解析失败: {}", e)))?;
            
        // 应用代码生成选项
        let options = lumen_core::CodegenOptions {
            minify: self.options.minify,
            sourcemap: self.options.sourcemap,
            target: self.options.target.clone(),
            inline_sources: true,
            preserve_comments: false,
        };
        
        // 生成代码
        let source = ir_json;  // 使用源IR JSON
        let output = self.generate_output(&ir, &options, source)?;
        
        Ok(output)
    }
    
    /// 从IR生成最终输出代码
    fn generate_output(&self, ir: &lumen_core::IR, options: &lumen_core::CodegenOptions, source: &str) -> Result<String> {
        // 这里是代码生成的实际实现
        // 在真实应用中会调用代码生成器模块
        
        // 模拟代码生成过程
        let mut output = String::new();
        
        // 收集所有标识符节点
        let mut identifiers = Vec::new();
        ir.visit(|node| {
            if node.node_type == lumen_core::NodeType::Identifier {
                if let Some(name) = node.get_string_value("name") {
                    identifiers.push(name.to_string());
                }
            }
        });
        
        // 收集所有字符串字面量
        let mut strings = Vec::new();
        ir.visit(|node| {
            if node.node_type == lumen_core::NodeType::StringLiteral {
                if let Some(value) = node.get_string_value("value") {
                    strings.push(value.to_string());
                }
            }
        });
        
        // 生成简化的JavaScript代码
        output.push_str("// 生成的代码\n");
        
        for id in &identifiers {
            output.push_str(&format!("var {} = {};\n", id, id));
        }
        
        for s in &strings {
            output.push_str(&format!("console.log({});\n", s));
        }
        
        // 如果启用了压缩，应用简单的压缩
        if options.minify {
            output = output
                .replace("\n", "")
                .replace("  ", " ")
                .replace("; ", ";")
                .replace(" = ", "=");
        }
        
        // 处理sourcemap (这里只是简单模拟)
        if options.sourcemap {
            output.push_str("\n//# sourceMappingURL=output.js.map");
        }
        
        // 计算压缩率
        let compression_ratio = 1.0 - (output.len() as f64 / source.len() as f64);
        
        info!("代码生成完成: 压缩率={:.2}%", compression_ratio * 100.0);
        
        Ok(output)
    }
    
    /// 编译文件
    pub fn compile_file<P: AsRef<Path>>(&self, input: P, output: Option<P>) -> Result<CompileResult> {
        let input_path = input.as_ref();
        info!("编译文件: {}", input_path.display());
        
        // 读取输入文件
        let source = std::fs::read_to_string(input_path)
            .map_err(|e| Error::IoError(e))?;
        
        // 编译内容
        let result = self.compile_str(&source, Some(input_path.to_str().unwrap_or("unknown")))?;
        
        // 如果指定了输出路径，写入文件
        if let Some(output_path) = output {
            let output_path = output_path.as_ref();
            std::fs::write(output_path, &result.code)
                .map_err(|e| Error::IoError(e))?;
            info!("输出文件已写入: {}", output_path.display());
        }
        
        Ok(result)
    }
    
    /// 批量编译文件
    pub fn compile_files<P: AsRef<Path>>(&self, inputs: Vec<P>, output_dir: Option<P>) -> Result<Vec<CompileResult>> {
        let output_dir = output_dir.map(|p| p.as_ref().to_path_buf());
        
        // 如果指定了输出目录，确保它存在
        if let Some(dir) = &output_dir {
            if !dir.exists() {
                std::fs::create_dir_all(dir)
                    .map_err(|e| Error::IoError(e))?;
            }
        }
        
        // 使用标准迭代器而不是并行迭代器
        let results: Vec<Result<CompileResult>> = inputs.iter().map(|input| {
            let input_path = input.as_ref();
            let output_path = match &output_dir {
                Some(dir) => {
                    let file_name = input_path.file_name().unwrap_or_default();
                    let mut path = dir.join(file_name);
                    path.set_extension("js");
                    Some(path)
                },
                None => None,
            };
            
            self.compile_file(input_path, output_path.as_ref().map(|v| &**v))
        }).collect();
        
        // 处理结果
        let mut successful_results = Vec::new();
        let mut error_count = 0;
        
        for result in results {
            match result {
                Ok(res) => successful_results.push(res),
                Err(e) => {
                    error_count += 1;
                    warn!("编译文件失败: {}", e);
                }
            }
        }
        
        if error_count > 0 {
            warn!("{} 个文件编译失败", error_count);
        }
        
        Ok(successful_results)
    }
    
    /// 编译为WebAssembly
    pub fn compile_to_wasm(&self, source: &str, options_json: &str) -> Result<Vec<u8>> {
        info!("开始编译到WebAssembly");
        
        if self.use_cpp {
            // 使用C++解析和WebAssembly生成
            let ir_json = self.parse_with_cpp(source, "input.js")?;
            let wasm_result = ffi::cpp_bindings::CppCodeGenerator::generate_wasm(&ir_json, options_json)
                .map_err(|e| Error::CompileError(format!("C++ WebAssembly生成错误: {}", e)))?;
                
            // 假设返回的是Base64编码的WASM二进制数据
            let bytes = base64::decode(&wasm_result)
                .map_err(|e| Error::InternalError(format!("Base64解码错误: {}", e)))?;
                
            Ok(bytes)
        } else {
            // 使用Rust的WebAssembly生成 (简化实现)
            Err(Error::InternalError("Rust WebAssembly生成尚未实现".to_string()))
        }
    }
    
    /// 启用C++解析器
    pub fn with_cpp(mut self, enable: bool) -> Self {
        self.use_cpp = enable;
        self
    }
    
    // 设置选项方法
    pub fn with_minify(mut self, enable: bool) -> Self {
        self.options.minify = enable;
        self
    }
    
    pub fn with_sourcemap(mut self, enable: bool) -> Self {
        self.options.sourcemap = enable;
        self
    }
    
    pub fn with_target(mut self, target: &str) -> Self {
        self.options.target = target.to_string();
        self
    }
    
    pub fn with_distributed(mut self, enable: bool) -> Self {
        self.options.distributed = enable;
        self
    }
    
    pub fn with_gpu(mut self, enable: bool) -> Self {
        self.options.gpu = enable;
        self
    }
}

/// 编译结果
#[derive(Debug, Clone)]
pub struct CompileResult {
    /// 编译后的代码
    pub code: String,
    /// 源码映射
    pub source_map: Option<String>,
    /// 编译时间（毫秒）
    pub time_ms: u64,
    /// 输入大小（字节）
    pub input_size: usize,
    /// 输出大小（字节）
    pub output_size: usize,
    /// 压缩率
    pub compression_ratio: f64,
}

// WebAssembly编译器
pub struct WasmCompiler {
    compiler: Compiler,
}

impl WasmCompiler {
    pub fn new() -> Self {
        Self {
            compiler: Compiler::new(),
        }
    }
    
    pub fn compile_file_to_wasm<P: AsRef<Path>>(
        &self,
        input: P,
        output: P,
        format: &str
    ) -> Result<()> {
        let input_path = input.as_ref();
        let output_path = output.as_ref();
        
        info!("编译文件到WebAssembly: {} -> {}", input_path.display(), output_path.display());
        
        // 读取输入文件
        let source = std::fs::read_to_string(input_path)
            .map_err(|e| Error::IoError(e))?;
        
        // 设置Wasm选项
        let options_json = format!(
            "{{\"format\":\"{}\",\"simd\":{},\"threads\":{},\"typescript\":{}}}",
            format,
            self.compiler.options.gpu, // 使用GPU加速作为SIMD的指示
            self.compiler.options.distributed, // 使用分布式作为线程的指示
            input_path.to_str().unwrap_or("").ends_with(".ts")
        );
        
        // 编译到WebAssembly
        let wasm_bytes = self.compiler.compile_to_wasm(&source, &options_json)?;
        
        // 写入输出文件
        std::fs::write(output_path, wasm_bytes)
            .map_err(|e| Error::IoError(e))?;
            
        info!("WebAssembly编译完成: {}", output_path.display());
        
        Ok(())
    }
    
    // 设置选项方法
    pub fn with_target(mut self, target: &str) -> Self {
        self.compiler = self.compiler.with_target(target);
        self
    }
    
    pub fn with_simd(mut self, enable: bool) -> Self {
        self.compiler = self.compiler.with_gpu(enable);
        self
    }
    
    pub fn with_threads(mut self, enable: bool) -> Self {
        self.compiler = self.compiler.with_distributed(enable);
        self
    }
    
    pub fn with_typescript(mut self, enable: bool) -> Self {
        // 这里其实不需要特别处理，因为会根据文件扩展名自动判断
        self
    }
}

// 添加集成了所有优化特性的高性能编译器
pub struct LumenCompiler {
    compiler: Compiler,
    gpu_optimizer: Option<gpu::GpuOptimizer>,
    distributed_compiler: Option<distributed::DistributedCompiler>,
}

impl LumenCompiler {
    /// 创建新的高性能编译器实例
    pub fn new() -> Self {
        Self {
            compiler: Compiler::new(),
            gpu_optimizer: None,
            distributed_compiler: None,
        }
    }
    
    /// 启用GPU加速
    pub fn with_gpu(mut self, enable: bool) -> Self {
        if enable {
            self.gpu_optimizer = Some(gpu::GpuOptimizer::new());
            self.compiler = self.compiler.with_gpu(true);
        } else {
            self.gpu_optimizer = None;
            self.compiler = self.compiler.with_gpu(false);
        }
        self
    }
    
    /// 启用分布式编译
    pub fn with_distributed(mut self, enable: bool) -> Self {
        if enable {
            self.distributed_compiler = Some(distributed::DistributedCompiler::new());
            self.compiler = self.compiler.with_distributed(true);
        } else {
            self.distributed_compiler = None;
            self.compiler = self.compiler.with_distributed(false);
        }
        self
    }
    
    /// 使用自定义GPU选项
    pub fn with_gpu_options(mut self, options: gpu::GpuOptions) -> Self {
        self.gpu_optimizer = Some(gpu::GpuOptimizer::with_options(options));
        self.compiler = self.compiler.with_gpu(true);
        self
    }
    
    /// 使用自定义分布式选项
    pub fn with_distributed_options(mut self, options: distributed::DistributedOptions) -> Self {
        self.distributed_compiler = Some(distributed::DistributedCompiler::with_options(options));
        self.compiler = self.compiler.with_distributed(true);
        self
    }
    
    /// 编译单个字符串
    pub async fn compile_str(&self, source: &str, filename: Option<&str>) -> Result<CompileResult> {
        let start = Instant::now();
        info!("开始高性能编译: {}", filename.unwrap_or("未命名"));
        
        // 判断是否使用分布式编译
        if let Some(dist_compiler) = &self.distributed_compiler {
            debug!("使用分布式编译...");
            
            // 初始化分布式环境
            dist_compiler.initialize().await
                .map_err(|e| Error::DistributedError(e))?;
            
            // 创建临时文件
            let temp_dir = tempfile::tempdir()
                .map_err(|e| Error::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
                
            let input_path = temp_dir.path().join("input.js");
            let output_path = temp_dir.path().join("output.js");
            
            // 写入源代码到临时文件
            std::fs::write(&input_path, source)
                .map_err(|e| Error::IoError(e))?;
            
            // 提交分布式编译任务
            let task_id = dist_compiler.submit_task(&input_path, Some(&output_path)).await
                .map_err(|e| Error::DistributedError(e))?;
                
            // 等待任务完成
            dist_compiler.wait_for_completion(&task_id, None).await
                .map_err(|e| Error::DistributedError(e))?;
                
            // 读取编译结果
            let output = std::fs::read_to_string(&output_path)
                .map_err(|e| Error::IoError(e))?;
                
            // 关闭分布式环境
            dist_compiler.shutdown().await
                .map_err(|e| Error::DistributedError(e))?;
                
            let elapsed = start.elapsed();
            info!("分布式编译完成，耗时: {:?}", elapsed);
            
            let output_size = output.len();
            return Ok(CompileResult {
                code: output.clone(),
                source_map: None,
                time_ms: elapsed.as_millis() as u64,
                input_size: source.len(),
                output_size,
                compression_ratio: if source.len() > 0 {
                    1.0 - (output_size as f64 / source.len() as f64)
                } else {
                    0.0
                },
            });
        }
        
        // 使用本地编译
        let mut result = self.compiler.compile_str(source, filename)?;
        
        // 如果启用了GPU加速，应用GPU压缩
        if let Some(gpu_opt) = &self.gpu_optimizer {
            debug!("应用GPU代码压缩...");
            let code = match gpu_opt.minify(&result.code) {
                Ok(code) => code,
                Err(e) => {
                    warn!("GPU压缩失败，使用原始结果: {}", e);
                    result.code
                }
            };
            
            // 更新结果
            result.output_size = code.len();
            result.compression_ratio = if source.len() > 0 {
                1.0 - (code.len() as f64 / source.len() as f64)
            } else {
                0.0
            };
            result.code = code;
        }
        
        Ok(result)
    }
    
    /// 编译文件
    pub async fn compile_file<P: AsRef<Path>>(&self, input: P, output: Option<P>) -> Result<CompileResult> {
        let input_path = input.as_ref();
        info!("编译文件: {}", input_path.display());
        
        // 读取输入文件
        let source = std::fs::read_to_string(input_path)
            .map_err(|e| Error::IoError(e))?;
        
        // 编译内容
        let result = self.compile_str(&source, Some(input_path.to_str().unwrap_or("unknown"))).await?;
        
        // 如果指定了输出路径，写入文件
        if let Some(output_path) = output {
            let output_path = output_path.as_ref();
            std::fs::write(output_path, &result.code)
                .map_err(|e| Error::IoError(e))?;
            info!("输出文件已写入: {}", output_path.display());
        }
        
        Ok(result)
    }
    
    /// 批量编译文件
    pub async fn compile_files<P: AsRef<Path>>(&self, inputs: Vec<P>, output_dir: Option<P>) -> Result<Vec<CompileResult>> {
        info!("开始批量编译 {} 个文件", inputs.len());
        
        let output_dir = output_dir.map(|p| p.as_ref().to_path_buf());
        
        // 如果指定了输出目录，确保它存在
        if let Some(dir) = &output_dir {
            if !dir.exists() {
                std::fs::create_dir_all(dir)
                    .map_err(|e| Error::IoError(e))?;
            }
        }
        
        // 判断是否使用分布式编译
        if let Some(dist_compiler) = &self.distributed_compiler {
            info!("使用分布式批量编译...");
            
            // 初始化分布式环境
            dist_compiler.initialize().await
                .map_err(|e| Error::DistributedError(e))?;
                
            // 提交批量任务 - 修复类型不匹配
            let inputs_clone: Vec<_> = inputs.iter().map(|p| p.as_ref().to_path_buf()).collect();
            let task_ids = dist_compiler.submit_batch(inputs_clone, output_dir.clone())
                .await
                .map_err(|e| Error::DistributedError(e))?;
                
            // 等待所有任务完成
            let mut results = Vec::new();
            for task_id in &task_ids {
                match dist_compiler.wait_for_completion(task_id, None).await {
                    Ok(_) => {
                        // 读取任务信息
                        // 在实际应用中，这里应该从分布式编译器获取结果信息
                        results.push(CompileResult {
                            code: "".to_string(), // 这里不关心具体内容，文件已经写入
                            source_map: None,
                            time_ms: 0,
                            input_size: 0,
                            output_size: 0,
                            compression_ratio: 0.0,
                        });
                    },
                    Err(e) => {
                        warn!("任务 {} 失败: {}", task_id, e);
                        // 继续处理其他任务
                    }
                }
            }
            
            // 关闭分布式环境
            dist_compiler.shutdown().await
                .map_err(|e| Error::DistributedError(e))?;
                
            info!("分布式批量编译完成: {} 个文件", results.len());
            
            return Ok(results);
        }
        
        // 使用本地编译
        let mut results = Vec::new();
        
        // 使用迭代器替代并行迭代器
        for input in &inputs {
            let input_path = input.as_ref();
            let output_path = match &output_dir {
                Some(dir) => {
                    let file_name = input_path.file_name().unwrap_or_default();
                    let mut path = dir.join(file_name);
                    path.set_extension("js");
                    Some(path)
                },
                None => None,
            };
            
            match self.compile_file(input_path, output_path.as_ref().map(|p| p.as_ref())).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    warn!("编译文件失败: {} - {}", input_path.display(), e);
                    // 继续处理其他文件
                }
            }
        }
        
        info!("本地批量编译完成: {} 个文件", results.len());
        
        Ok(results)
    }
    
    // 其他选项方法
    pub fn with_minify(mut self, enable: bool) -> Self {
        self.compiler = self.compiler.with_minify(enable);
        self
    }
    
    pub fn with_sourcemap(mut self, enable: bool) -> Self {
        self.compiler = self.compiler.with_sourcemap(enable);
        self
    }
    
    pub fn with_target(mut self, target: &str) -> Self {
        self.compiler = self.compiler.with_target(target);
        self
    }
    
    pub fn with_cpp(mut self, enable: bool) -> Self {
        self.compiler = self.compiler.with_cpp(enable);
        self
    }
}

// 提供一个方便的函数来快速编译字符串
pub async fn compile_async(source: &str) -> Result<String> {
    let compiler = LumenCompiler::new();
    compiler.compile_str(source, None).await.map(|r| r.code)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_compile_str() {
        let source = "var x = 1 + 2;";
        let result = compile(source).unwrap();
        assert!(result.contains("let x = 1 + 2;"));
    }
    
    #[test]
    fn test_config_options() {
        let compiler = Lumen::new()
            .with_minify(true)
            .with_sourcemap(true)
            .with_target("es2015")
            .with_distributed(false)
            .with_gpu(true);
            
        assert_eq!(compiler.config.minify, true);
        assert_eq!(compiler.config.sourcemap, true);
        assert_eq!(compiler.config.target, "es2015");
        assert_eq!(compiler.config.distributed, false);
        assert_eq!(compiler.config.gpu, true);
    }
} 
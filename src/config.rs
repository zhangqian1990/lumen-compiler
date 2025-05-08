use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// 编译选项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompileOptions {
    /// 源代码字符串或文件路径
    pub input: String,
    /// 输出文件路径（可选）
    pub output: Option<String>,
    /// 是否启用代码压缩
    pub minify: bool,
    /// 是否生成sourcemap
    pub sourcemap: bool,
    /// 目标环境，如 es5, es2015, es2020 等
    pub target: String,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            input: String::new(),
            output: None,
            minify: false,
            sourcemap: false,
            target: "es2020".to_string(),
        }
    }
}

/// 完整的Lumen配置
#[derive(Debug, Clone)]
pub struct Config {
    /// 是否启用代码压缩
    pub minify: bool,
    /// 是否生成sourcemap
    pub sourcemap: bool,
    /// 目标环境，如 es5, es2015, es2020 等
    pub target: String,
    /// 是否启用分布式编译
    pub distributed: bool,
    /// 是否使用GPU加速
    pub gpu: bool,
    /// 是否启用缓存
    pub cache_enabled: bool,
    /// 缓存大小限制（字节）
    pub cache_size_limit: usize,
    /// 自定义转换器
    pub transformers: Vec<String>,
    /// 自定义插件
    pub plugins: Vec<String>,
    /// 自定义选项
    pub options: HashMap<String, String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            minify: false,
            sourcemap: false,
            target: "es2020".to_string(),
            distributed: false,
            gpu: false,
            cache_enabled: true,
            cache_size_limit: 100 * 1024 * 1024, // 100MB
            transformers: Vec::new(),
            plugins: Vec::new(),
            options: HashMap::new(),
        }
    }
}

impl Config {
    /// 创建一个新的配置实例
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 从JSON字符串加载配置
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        let config: Config = serde_json::from_str(json)?;
        Ok(config)
    }
    
    /// 转换为JSON字符串
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
    
    /// 从配置文件加载
    pub fn from_file(path: &str) -> Result<Self, std::io::Error> {
        let content = std::fs::read_to_string(path)?;
        let config = Self::from_json(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        Ok(config)
    }
    
    /// 保存到配置文件
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        let json = self.to_json()
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
        std::fs::write(path, json)
    }
    
    /// 添加自定义选项
    pub fn add_option(&mut self, key: &str, value: &str) {
        self.options.insert(key.to_string(), value.to_string());
    }
    
    /// 获取自定义选项
    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }
    
    /// 添加插件
    pub fn add_plugin(&mut self, plugin: &str) {
        self.plugins.push(plugin.to_string());
    }
    
    /// 添加转换器
    pub fn add_transformer(&mut self, transformer: &str) {
        self.transformers.push(transformer.to_string());
    }
} 
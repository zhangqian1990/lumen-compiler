use std::fmt;
use std::io;
use thiserror::Error;

/// 自定义错误类型
#[derive(Error, Debug)]
pub enum Error {
    #[error("IO错误: {0}")]
    IoError(#[from] io::Error),
    
    #[error("解析错误: {0}")]
    ParseError(String),
    
    #[error("类型错误: {0}")]
    TypeError(String),
    
    #[error("编译错误: {0}")]
    CompileError(String),
    
    #[error("未知文件类型: {0}")]
    UnknownFileType(String),
    
    #[error("不支持的语法: {0}")]
    UnsupportedSyntax(String),
    
    #[error("配置错误: {0}")]
    ConfigError(String),
    
    #[error("插件错误: {0}")]
    PluginError(String),
    
    #[error("批量编译失败: {0} 个文件出错")]
    BatchCompilationFailed(usize),
    
    #[error("分布式执行错误: {0}")]
    DistributedError(String),
    
    #[error("GPU加速错误: {0}")]
    GpuError(String),
    
    #[error("内部错误: {0}")]
    InternalError(String),
}

/// 便捷Result类型
pub type Result<T> = std::result::Result<T, Error>;

/// 编译错误的详细信息
#[derive(Debug, Clone)]
pub struct CompileErrorInfo {
    /// 源文件路径
    pub file: Option<String>,
    /// 错误所在行号
    pub line: Option<usize>,
    /// 错误所在列号
    pub column: Option<usize>,
    /// 错误代码
    pub code: Option<String>,
    /// 错误消息
    pub message: String,
    /// 相关代码片段
    pub snippet: Option<String>,
    /// 高亮范围（开始位置）
    pub highlight_start: Option<usize>,
    /// 高亮范围（结束位置）
    pub highlight_end: Option<usize>,
}

impl CompileErrorInfo {
    /// 创建一个新的编译错误信息
    pub fn new(message: &str) -> Self {
        Self {
            file: None,
            line: None,
            column: None,
            code: None,
            message: message.to_string(),
            snippet: None,
            highlight_start: None,
            highlight_end: None,
        }
    }
    
    /// 设置源文件路径
    pub fn with_file(mut self, file: &str) -> Self {
        self.file = Some(file.to_string());
        self
    }
    
    /// 设置错误位置
    pub fn with_position(mut self, line: usize, column: usize) -> Self {
        self.line = Some(line);
        self.column = Some(column);
        self
    }
    
    /// 设置错误代码
    pub fn with_code(mut self, code: &str) -> Self {
        self.code = Some(code.to_string());
        self
    }
    
    /// 设置代码片段
    pub fn with_snippet(mut self, snippet: &str) -> Self {
        self.snippet = Some(snippet.to_string());
        self
    }
    
    /// 设置高亮范围
    pub fn with_highlight(mut self, start: usize, end: usize) -> Self {
        self.highlight_start = Some(start);
        self.highlight_end = Some(end);
        self
    }
    
    /// 转换为编译错误
    pub fn into_error(self) -> Error {
        let message = if let (Some(file), Some(line), Some(column)) = (self.file.as_ref(), self.line, self.column) {
            format!("{}:{}:{} - {}", file, line, column, self.message)
        } else {
            self.message
        };
        Error::CompileError(message)
    }
}

impl fmt::Display for CompileErrorInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 错误位置信息
        if let (Some(file), Some(line), Some(column)) = (&self.file, self.line, self.column) {
            writeln!(f, "错误: {}:{}:{}", file, line, column)?;
        } else if let Some(file) = &self.file {
            writeln!(f, "错误: {}", file)?;
        } else {
            writeln!(f, "错误:")?;
        }
        
        // 错误代码和消息
        if let Some(code) = &self.code {
            writeln!(f, "[{}] {}", code, self.message)?;
        } else {
            writeln!(f, "{}", self.message)?;
        }
        
        // 代码片段
        if let Some(snippet) = &self.snippet {
            writeln!(f, "\n{}", snippet)?;
            
            // 高亮错误位置
            if let (Some(start), Some(end)) = (self.highlight_start, self.highlight_end) {
                let spaces = " ".repeat(start);
                let markers = "^".repeat(end - start);
                writeln!(f, "{}{}", spaces, markers)?;
            }
        }
        
        Ok(())
    }
} 
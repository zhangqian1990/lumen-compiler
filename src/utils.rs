use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::collections::HashSet;
use log::{info, debug, warn};
use walkdir::WalkDir;
use glob::Pattern;

/// 获取文件扩展名
pub fn get_file_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// 检查文件是否为JavaScript源文件
pub fn is_javascript_file(path: &Path) -> bool {
    matches!(get_file_extension(path).as_deref(), Some("js") | Some("jsx") | Some("mjs"))
}

/// 检查文件是否为TypeScript源文件
pub fn is_typescript_file(path: &Path) -> bool {
    matches!(get_file_extension(path).as_deref(), Some("ts") | Some("tsx"))
}

/// 检查文件是否为支持的源文件类型
pub fn is_supported_file(path: &Path) -> bool {
    is_javascript_file(path) || is_typescript_file(path)
}

/// 查找目录中与模式匹配的所有文件
pub fn find_files<P: AsRef<Path>>(dir: P, pattern: &str) -> Vec<PathBuf> {
    let glob_pattern = Pattern::new(pattern).unwrap_or_else(|_| {
        warn!("无效的glob模式: {}, 回退到默认模式", pattern);
        Pattern::new("**/*.{js,ts,jsx,tsx}").unwrap()
    });
    
    WalkDir::new(dir)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file() && 
            glob_pattern.matches_path(e.path())
        })
        .map(|e| e.path().to_path_buf())
        .collect()
}

/// 获取文件大小
pub fn get_file_size<P: AsRef<Path>>(path: P) -> std::io::Result<u64> {
    let metadata = std::fs::metadata(path)?;
    Ok(metadata.len())
}

/// 计算文件hash值，用于缓存
pub fn hash_file<P: AsRef<Path>>(path: P) -> std::io::Result<String> {
    use std::io::Read;
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut file = std::fs::File::open(path)?;
    let mut hasher = DefaultHasher::new();
    let mut buffer = [0; 1024];
    
    loop {
        let bytes_read = file.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }
        buffer[..bytes_read].hash(&mut hasher);
    }
    
    Ok(format!("{:x}", hasher.finish()))
}

/// 计算文本hash值
pub fn hash_text(text: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    
    let mut hasher = DefaultHasher::new();
    text.hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

/// 获取依赖关系图
pub fn get_dependency_graph<P: AsRef<Path>>(
    entry_points: &[P], 
    search_dirs: &[P]
) -> std::io::Result<DependencyGraph> {
    let mut graph = DependencyGraph::new();
    
    // TODO: 实现JavaScript/TypeScript依赖解析
    // 这里只是一个简单的示例实现
    for entry in entry_points {
        let path = entry.as_ref();
        graph.add_node(path.to_path_buf());
    }
    
    Ok(graph)
}

/// 性能计时器
pub struct Timer {
    start: Instant,
    checkpoints: Vec<(String, Duration)>,
}

impl Timer {
    /// 创建一个新的计时器并开始计时
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            checkpoints: Vec::new(),
        }
    }
    
    /// 记录一个检查点
    pub fn checkpoint(&mut self, name: &str) {
        let elapsed = self.start.elapsed();
        self.checkpoints.push((name.to_string(), elapsed));
        debug!("计时器 - {}: {:?}", name, elapsed);
    }
    
    /// 打印所有检查点
    pub fn print_summary(&self) {
        info!("性能计时器汇总:");
        if self.checkpoints.is_empty() {
            info!("  没有记录任何检查点");
            return;
        }
        
        // 打印每个检查点
        let mut prev_duration = Duration::new(0, 0);
        for (i, (name, duration)) in self.checkpoints.iter().enumerate() {
            let delta = *duration - prev_duration;
            info!("  {}: {:?} (增量: {:?})", name, duration, delta);
            prev_duration = *duration;
        }
        
        // 打印总耗时
        info!("  总耗时: {:?}", self.start.elapsed());
    }
    
    /// 获取总耗时
    pub fn total_elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

/// 依赖关系图
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    nodes: HashSet<PathBuf>,
    edges: Vec<(PathBuf, PathBuf)>,
}

impl DependencyGraph {
    /// 创建一个新的依赖图
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: Vec::new(),
        }
    }
    
    /// 添加节点
    pub fn add_node(&mut self, path: PathBuf) {
        self.nodes.insert(path);
    }
    
    /// 添加边（依赖关系）
    pub fn add_edge(&mut self, from: PathBuf, to: PathBuf) {
        self.edges.push((from, to));
    }
    
    /// 获取所有节点
    pub fn get_nodes(&self) -> &HashSet<PathBuf> {
        &self.nodes
    }
    
    /// 获取所有边
    pub fn get_edges(&self) -> &Vec<(PathBuf, PathBuf)> {
        &self.edges
    }
    
    /// 获取拓扑排序结果（编译顺序）
    pub fn topological_sort(&self) -> Vec<PathBuf> {
        // TODO: 实现拓扑排序算法
        // 这里只是简单地返回所有节点
        self.nodes.iter().cloned().collect()
    }
} 
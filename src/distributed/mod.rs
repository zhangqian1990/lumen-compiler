use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use log::{debug, info, warn, error};
use tokio::sync::mpsc;

// 分布式编译选项
#[derive(Debug, Clone)]
pub struct DistributedOptions {
    /// 工作节点地址列表
    pub workers: Vec<String>,
    /// 任务分配策略
    pub strategy: DistributionStrategy,
    /// 最大重试次数
    pub max_retries: usize,
    /// 任务超时时间（秒）
    pub timeout_secs: u64,
    /// 是否启用负载均衡
    pub load_balancing: bool,
    /// 任务批处理大小
    pub batch_size: usize,
}

impl Default for DistributedOptions {
    fn default() -> Self {
        Self {
            workers: vec!["localhost:8080".to_string()],
            strategy: DistributionStrategy::DependencyBased,
            max_retries: 3,
            timeout_secs: 30,
            load_balancing: true,
            batch_size: 10,
        }
    }
}

/// 任务分配策略
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistributionStrategy {
    /// 按文件随机分配
    Random,
    /// 按目录分配
    DirectoryBased,
    /// 基于依赖图的分配
    DependencyBased,
    /// 文件大小优先
    SizePriority,
}

/// 编译任务
#[derive(Debug, Clone)]
pub struct CompileTask {
    /// 任务ID
    pub id: String,
    /// 输入文件路径
    pub input_path: PathBuf,
    /// 输出文件路径
    pub output_path: Option<PathBuf>,
    /// 分配的工作节点
    pub assigned_worker: Option<String>,
    /// 任务状态
    pub status: TaskStatus,
    /// 任务优先级
    pub priority: usize,
    /// 依赖任务ID列表
    pub dependencies: Vec<String>,
    /// 任务创建时间
    pub created_at: Instant,
    /// 完成时间
    pub completed_at: Option<Instant>,
    /// 重试次数
    pub retries: usize,
}

/// 任务状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TaskStatus {
    /// 等待中
    Waiting,
    /// 正在编译
    Compiling,
    /// 已完成
    Completed,
    /// 失败
    Failed,
}

/// 工作节点信息
#[derive(Debug, Clone)]
struct WorkerInfo {
    /// 节点地址
    address: String,
    /// 活跃任务数
    active_tasks: usize,
    /// 节点状态
    status: WorkerStatus,
    /// 最近响应时间（毫秒）
    last_response_time_ms: u64,
    /// 处理能力评分（越高越好）
    performance_score: f64,
}

/// 工作节点状态
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkerStatus {
    /// 在线
    Online,
    /// 忙碌
    Busy,
    /// 离线
    Offline,
}

/// 分布式编译管理器
pub struct DistributedCompiler {
    options: DistributedOptions,
    tasks: Arc<Mutex<HashMap<String, CompileTask>>>,
    workers: Arc<Mutex<HashMap<String, WorkerInfo>>>,
    dependency_graph: Arc<Mutex<HashMap<String, HashSet<String>>>>,
}

impl DistributedCompiler {
    pub fn new() -> Self {
        Self {
            options: DistributedOptions::default(),
            tasks: Arc::new(Mutex::new(HashMap::new())),
            workers: Arc::new(Mutex::new(HashMap::new())),
            dependency_graph: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    
    pub fn with_options(options: DistributedOptions) -> Self {
        let mut compiler = Self::new();
        compiler.options = options;
        compiler
    }
    
    /// 初始化分布式编译环境
    pub async fn initialize(&self) -> Result<(), String> {
        info!("初始化分布式编译环境...");
        
        // 注册工作节点
        for worker_addr in &self.options.workers {
            self.register_worker(worker_addr).await?;
        }
        
        info!("分布式编译环境初始化完成，{} 个工作节点就绪", self.options.workers.len());
        
        Ok(())
    }
    
    /// 注册工作节点
    async fn register_worker(&self, address: &str) -> Result<(), String> {
        info!("注册工作节点: {}", address);
        
        // TODO: 实现实际的工作节点注册逻辑
        // 这里简单模拟
        
        let worker_info = WorkerInfo {
            address: address.to_string(),
            active_tasks: 0,
            status: WorkerStatus::Online,
            last_response_time_ms: 0,
            performance_score: 1.0,
        };
        
        let mut workers = self.workers.lock().unwrap();
        workers.insert(address.to_string(), worker_info);
        
        Ok(())
    }
    
    /// 提交单个编译任务
    pub async fn submit_task<P: AsRef<Path>>(&self, input: P, output: Option<P>) -> Result<String, String> {
        // 生成唯一任务ID
        let task_id = generate_uuid();
        
        // 创建任务
        let task = CompileTask {
            id: task_id.clone(),
            input_path: input.as_ref().to_path_buf(),
            output_path: output.map(|p| p.as_ref().to_path_buf()),
            assigned_worker: None,
            status: TaskStatus::Waiting,
            priority: 0,
            dependencies: Vec::new(),
            created_at: Instant::now(),
            completed_at: None,
            retries: 0,
        };
        
        // 添加任务
        {
            let mut tasks = self.tasks.lock().unwrap();
            tasks.insert(task_id.clone(), task);
        }
        
        // 尝试调度任务
        self.schedule_task(&task_id).await?;
        
        Ok(task_id)
    }
    
    /// 调度任务到工作节点
    async fn schedule_task(&self, task_id: &str) -> Result<(), String> {
        debug!("调度任务: {}", task_id);
        
        // 获取任务
        let task = {
            let tasks = self.tasks.lock().unwrap();
            match tasks.get(task_id).cloned() {
                Some(t) => t,
                None => return Err(format!("任务不存在: {}", task_id)),
            }
        };
        
        // 检查依赖是否都已完成
        if !self.check_dependencies(&task) {
            debug!("任务 {} 的依赖尚未完成，保持等待状态", task_id);
            return Ok(());
        }
        
        // 选择最佳工作节点
        let worker = self.select_worker().await?;
        
        info!("将任务 {} 分配给工作节点 {}", task_id, worker);
        
        // 更新任务状态
        {
            let mut tasks = self.tasks.lock().unwrap();
            if let Some(t) = tasks.get_mut(task_id) {
                t.assigned_worker = Some(worker.clone());
                t.status = TaskStatus::Compiling;
            }
        }
        
        // 更新工作节点状态
        {
            let mut workers = self.workers.lock().unwrap();
            if let Some(w) = workers.get_mut(&worker) {
                w.active_tasks += 1;
                if w.active_tasks >= 10 {
                    w.status = WorkerStatus::Busy;
                }
            }
        }
        
        // 在实际应用中，这里应该向工作节点发送编译请求
        // 这里简单模拟异步编译过程
        let task_id_clone = task_id.to_string();
        let worker_clone = worker.clone();
        let tasks_clone = Arc::clone(&self.tasks);
        let workers_clone = Arc::clone(&self.workers);
        
        tokio::spawn(async move {
            // 模拟编译过程
            tokio::time::sleep(Duration::from_millis(500)).await;
            
            // 更新任务状态为已完成
            {
                let mut tasks = tasks_clone.lock().unwrap();
                if let Some(t) = tasks.get_mut(&task_id_clone) {
                    t.status = TaskStatus::Completed;
                    t.completed_at = Some(Instant::now());
                }
            }
            
            // 更新工作节点状态
            {
                let mut workers = workers_clone.lock().unwrap();
                if let Some(w) = workers.get_mut(&worker_clone) {
                    w.active_tasks -= 1;
                    if w.active_tasks < 10 {
                        w.status = WorkerStatus::Online;
                    }
                }
            }
            
            info!("任务 {} 在工作节点 {} 上完成", task_id_clone, worker_clone);
        });
        
        Ok(())
    }
    
    /// 选择最佳工作节点
    async fn select_worker(&self) -> Result<String, String> {
        let workers = self.workers.lock().unwrap();
        
        if workers.is_empty() {
            return Err("没有可用的工作节点".to_string());
        }
        
        // 简单的负载均衡：选择活跃任务最少的节点
        let mut best_worker = None;
        let mut min_tasks = usize::MAX;
        
        for (addr, info) in workers.iter() {
            if info.status == WorkerStatus::Offline {
                continue;
            }
            
            if info.active_tasks < min_tasks {
                min_tasks = info.active_tasks;
                best_worker = Some(addr.clone());
            }
        }
        
        match best_worker {
            Some(w) => Ok(w),
            None => Err("没有在线的工作节点".to_string()),
        }
    }
    
    /// 检查任务依赖是否都已完成
    fn check_dependencies(&self, task: &CompileTask) -> bool {
        if task.dependencies.is_empty() {
            return true;
        }
        
        let tasks = self.tasks.lock().unwrap();
        
        for dep_id in &task.dependencies {
            if let Some(dep_task) = tasks.get(dep_id) {
                if dep_task.status != TaskStatus::Completed {
                    return false;
                }
            } else {
                // 依赖任务不存在，视为未完成
                return false;
            }
        }
        
        true
    }
    
    /// 获取任务状态
    pub fn get_task_status(&self, task_id: &str) -> Result<TaskStatus, String> {
        let tasks = self.tasks.lock().unwrap();
        
        match tasks.get(task_id) {
            Some(task) => Ok(task.status),
            None => Err(format!("任务不存在: {}", task_id)),
        }
    }
    
    /// 等待任务完成
    pub async fn wait_for_completion(&self, task_id: &str, timeout_secs: Option<u64>) -> Result<(), String> {
        let timeout = timeout_secs.unwrap_or(self.options.timeout_secs);
        let start = Instant::now();
        
        loop {
            let status = self.get_task_status(task_id)?;
            
            match status {
                TaskStatus::Completed => return Ok(()),
                TaskStatus::Failed => return Err(format!("任务失败: {}", task_id)),
                _ => {
                    // 检查是否超时
                    if start.elapsed().as_secs() > timeout {
                        return Err(format!("等待任务完成超时: {}", task_id));
                    }
                    
                    // 小睡一会再检查
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
            }
        }
    }
    
    /// 批量提交任务
    pub async fn submit_batch<P: AsRef<Path>>(&self, inputs: Vec<P>, output_dir: Option<P>) -> Result<Vec<String>, String> {
        info!("批量提交 {} 个任务", inputs.len());
        
        let output_dir = output_dir.map(|p| p.as_ref().to_path_buf());
        
        // 如果指定了输出目录，确保它存在
        if let Some(dir) = &output_dir {
            if !dir.exists() {
                std::fs::create_dir_all(dir)
                    .map_err(|e| format!("创建输出目录失败: {}", e))?;
            }
        }
        
        // 根据策略对输入文件进行排序和分组
        let sorted_inputs = self.sort_inputs_by_strategy(inputs)?;
        
        // 批量创建任务
        let mut task_ids = Vec::new();
        
        for (batch_idx, batch) in sorted_inputs.chunks(self.options.batch_size).enumerate() {
            info!("处理批次 {}, 包含 {} 个文件", batch_idx + 1, batch.len());
            
            for input in batch {
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
                
                let task_id = self.submit_task(input_path, output_path.as_ref().map(|v| &**v)).await?;
                task_ids.push(task_id);
            }
        }
        
        info!("批量提交完成，共 {} 个任务", task_ids.len());
        
        Ok(task_ids)
    }
    
    /// 根据策略对输入文件进行排序
    fn sort_inputs_by_strategy<P: AsRef<Path>>(&self, inputs: Vec<P>) -> Result<Vec<P>, String> {
        match self.options.strategy {
            DistributionStrategy::Random => Ok(inputs),
            DistributionStrategy::DirectoryBased => {
                // 按目录分组
                // 实际应用中应该进行更复杂的排序
                Ok(inputs)
            },
            DistributionStrategy::DependencyBased => {
                // 根据依赖关系排序
                // 这里应该先构建依赖图，然后进行拓扑排序
                Ok(inputs)
            },
            DistributionStrategy::SizePriority => {
                // 按文件大小排序（大文件优先）
                // 实际应用中应该获取文件大小并排序
                Ok(inputs)
            },
        }
    }
    
    /// 停止并清理分布式编译环境
    pub async fn shutdown(&self) -> Result<(), String> {
        info!("关闭分布式编译环境...");
        
        // 等待所有活跃任务完成
        let active_tasks = {
            let tasks = self.tasks.lock().unwrap();
            tasks.values()
                .filter(|t| t.status == TaskStatus::Compiling)
                .map(|t| t.id.clone())
                .collect::<Vec<_>>()
        };
        
        for task_id in active_tasks {
            match self.wait_for_completion(&task_id, Some(5)).await {
                Ok(_) => {},
                Err(e) => warn!("等待任务 {} 完成时出错: {}", task_id, e),
            }
        }
        
        // 清理工作节点连接
        let mut workers = self.workers.lock().unwrap();
        workers.clear();
        
        info!("分布式编译环境已关闭");
        
        Ok(())
    }
}

// 生成简单的UUID
fn generate_uuid() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let timestamp = now.as_secs() * 1_000_000_000 + now.subsec_nanos() as u64;
    
    let random_part = rand::random::<u64>();
    format!("{:x}{:x}", timestamp, random_part)
} 
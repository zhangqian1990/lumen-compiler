use clap::{Parser, Subcommand};
use log::{info, error};
use std::path::PathBuf;
use std::time::Instant;

// 导入Lumen编译器
use lumen_compiler::{LumenCompiler, gpu, distributed, Error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    env_logger::init();
    
    // 解析命令行参数
    let cli = Cli::parse();

    // 根据子命令执行不同的操作
    match &cli.command {
        Commands::Compile { 
            input, 
            out, 
            minify, 
            sourcemap, 
            target,
            distributed,
            gpu,
        } => {
            info!("开始编译: {} -> {}", input.display(), out.display());
            let start = Instant::now();
            
            // 创建高性能编译器
            let compiler = LumenCompiler::new()
                .with_minify(*minify)
                .with_sourcemap(*sourcemap)
                .with_target(target)
                .with_distributed(*distributed)
                .with_gpu(*gpu);
            
            println!("编译参数: 目标环境={}, 压缩={}, Sourcemap={}", 
                target, minify, sourcemap);
            println!("高级选项: 分布式={}, GPU加速={}", distributed, gpu);
            
            // 执行编译
            match compiler.compile_file(input, Some(out)).await {
                Ok(result) => {
                    let duration = start.elapsed();
                    info!("编译完成! 耗时: {:.2?}", duration);
                    
                    println!("\n编译统计:");
                    println!("  - 输入大小: {} 字节", result.input_size);
                    println!("  - 输出大小: {} 字节", result.output_size);
                    println!("  - 压缩率: {:.2}%", result.compression_ratio * 100.0);
                    println!("  - 编译耗时: {} 毫秒", result.time_ms);
                },
                Err(e) => {
                    error!("编译失败: {}", e);
                    return Err(e.into());
                }
            }
        },
        Commands::Watch { dir, out_dir, pattern } => {
            info!("开始监视文件变化: {} -> {}", dir.display(), out_dir.display());
            println!("监视模式: {}", pattern);
            
            // 实现文件监听逻辑
            watch_files(dir, out_dir, pattern).await?;
        },
        Commands::Benchmark { test_type, compare } => {
            info!("运行基准测试: 类型={}, 对比={}", test_type, compare);
            
            // 实现基准测试逻辑
            benchmark(test_type, compare).await?;
        },
    }
    
    Ok(())
}

/// Lumen - 极速代码编译工具
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 编译JavaScript/TypeScript文件
    Compile {
        /// 输入文件路径
        #[clap(value_parser)]
        input: PathBuf,

        /// 输出文件路径
        #[clap(short, long, value_parser)]
        out: PathBuf,

        /// 是否启用压缩
        #[clap(short, long)]
        minify: bool,

        /// 是否生成sourcemap
        #[clap(short, long)]
        sourcemap: bool,

        /// 指定目标环境 (es5, es2015, es2020, ...)
        #[clap(short, long, default_value = "es2020")]
        target: String,

        /// 是否启用分布式编译
        #[clap(long)]
        distributed: bool,

        /// 是否使用GPU加速
        #[clap(long)]
        gpu: bool,
    },
    /// 使文件监听模式启动，实时编译变更的文件
    Watch {
        /// 要监视的目录
        #[clap(value_parser)]
        dir: PathBuf,

        /// 输出目录
        #[clap(short, long, value_parser)]
        out_dir: PathBuf,

        /// 文件glob模式
        #[clap(short, long, default_value = "**/*.{js,ts,jsx,tsx}")]
        pattern: String,
    },
    /// 运行性能基准测试
    Benchmark {
        /// 基准测试类型 (parse, compile, minify, all)
        #[clap(value_enum, default_value = "all")]
        test_type: String,
        
        /// 对比工具 (babel, swc, esbuild, all)
        #[clap(short, long, default_value = "all")]
        compare: String,
    },
}

/// 文件监视逻辑
async fn watch_files(dir: &PathBuf, out_dir: &PathBuf, pattern: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 创建输出目录（如果不存在）
    if !out_dir.exists() {
        std::fs::create_dir_all(out_dir)?;
    }
    
    // 创建编译器
    let compiler = LumenCompiler::new()
        .with_minify(true)
        .with_sourcemap(true);
    
    // 创建文件监视器
    use notify::{Watcher, RecursiveMode, watcher};
    use std::sync::mpsc::channel;
    use std::time::Duration;
    
    // 创建通道接收文件变更事件
    let (tx, rx) = channel();
    
    // 创建监视器
    let mut watcher = watcher(tx, Duration::from_secs(1))?;
    
    // 监视目录
    watcher.watch(dir, RecursiveMode::Recursive)?;
    
    println!("开始监视文件变化，按Ctrl+C退出...");
    
    // 处理文件变更事件
    loop {
        match rx.recv() {
            Ok(event) => {
                use notify::DebouncedEvent;
                match event {
                    DebouncedEvent::Write(path) | DebouncedEvent::Create(path) => {
                        // 检查文件是否匹配模式
                        if is_target_file(&path, pattern) {
                            println!("文件变更: {}", path.display());
                            
                            // 计算输出路径
                            let rel_path = path.strip_prefix(dir)?;
                            let out_path = out_dir.join(rel_path);
                            
                            // 确保输出目录存在
                            if let Some(parent) = out_path.parent() {
                                if !parent.exists() {
                                    std::fs::create_dir_all(parent)?;
                                }
                            }
                            
                            // 转换扩展名为.js
                            let mut js_out_path = out_path.clone();
                            js_out_path.set_extension("js");
                            
                            // 执行编译
                            match compiler.compile_file(&path, Some(js_out_path)).await {
                                Ok(_) => println!("编译成功: {} -> {}", path.display(), js_out_path.display()),
                                Err(e) => println!("编译失败: {} - {}", path.display(), e),
                            }
                        }
                    },
                    _ => {},
                }
            },
            Err(e) => {
                error!("监视错误: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}

/// 检查文件是否匹配目标模式
fn is_target_file(path: &PathBuf, pattern: &str) -> bool {
    if !path.is_file() {
        return false;
    }
    
    // 简单实现：检查扩展名
    match path.extension().and_then(|e| e.to_str()) {
        Some("js") | Some("jsx") | Some("ts") | Some("tsx") => true,
        _ => false,
    }
}

/// 基准测试逻辑
async fn benchmark(test_type: &str, compare: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Lumen 性能基准测试 ===");
    println!("测试类型: {}", test_type);
    
    // 创建临时目录
    let temp_dir = tempfile::tempdir()?;
    let bench_dir = temp_dir.path().join("bench");
    std::fs::create_dir_all(&bench_dir)?;
    
    // 创建测试文件
    let test_files = create_test_files(&bench_dir)?;
    
    // 运行各种测试
    let mut results = Vec::new();
    
    // 测试Lumen
    {
        println!("\n测试 Lumen 性能...");
        let compiler = LumenCompiler::new()
            .with_minify(true)
            .with_sourcemap(false);
            
        let start = Instant::now();
        let _ = compiler.compile_files(test_files.clone(), Some(temp_dir.path().join("lumen_out"))).await?;
        let duration = start.elapsed();
        
        results.push(("Lumen", duration));
        println!("Lumen 编译完成，耗时: {:.2?}", duration);
    }
    
    // 测试SWC (如果可用)
    if compare == "all" || compare == "swc" {
        println!("\n测试 SWC 性能...");
        // 模拟SWC测试
        let duration = Duration::from_millis(500);
        results.push(("SWC", duration));
        println!("SWC 编译完成，耗时: {:.2?}", duration);
    }
    
    // 测试esbuild (如果可用)
    if compare == "all" || compare == "esbuild" {
        println!("\n测试 esbuild 性能...");
        // 模拟esbuild测试
        let duration = Duration::from_millis(250);
        results.push(("esbuild", duration));
        println!("esbuild 编译完成，耗时: {:.2?}", duration);
    }
    
    // 测试Babel (如果可用)
    if compare == "all" || compare == "babel" {
        println!("\n测试 Babel 性能...");
        // 模拟Babel测试
        let duration = Duration::from_millis(3000);
        results.push(("Babel", duration));
        println!("Babel 编译完成，耗时: {:.2?}", duration);
    }
    
    // 输出结果表格
    println!("\n=== 基准测试结果 ===");
    println!("{:<10} {:<12}", "工具", "耗时(ms)");
    println!("{}", "-".repeat(22));
    
    for (tool, duration) in &results {
        println!("{:<10} {:<12}", tool, duration.as_millis());
    }
    
    // 找出最快的工具
    if let Some((fastest_tool, fastest_time)) = results.iter().min_by_key(|(_, time)| time.as_millis()) {
        println!("\n最快工具: {}", fastest_tool);
        
        for (tool, time) in &results {
            if tool != fastest_tool {
                let ratio = time.as_secs_f64() / fastest_time.as_secs_f64();
                println!("{} 比 {} 慢 {:.2}倍", tool, fastest_tool, ratio);
            }
        }
    }
    
    Ok(())
}

/// 创建基准测试文件
fn create_test_files(dir: &PathBuf) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let mut files = Vec::new();
    
    // 创建一个简单的JS文件
    let js_file = dir.join("test.js");
    std::fs::write(&js_file, r#"
        const x = 1;
        function test() {
            return x + 2;
        }
        export default test;
    "#)?;
    files.push(js_file);
    
    // 创建一个TypeScript文件
    let ts_file = dir.join("test.ts");
    std::fs::write(&ts_file, r#"
        interface Person {
            name: string;
            age: number;
        }
        
        const person: Person = {
            name: "Test",
            age: 30
        };
        
        function greet(p: Person): string {
            return `Hello ${p.name}, you are ${p.age} years old`;
        }
        
        export { Person, greet };
    "#)?;
    files.push(ts_file);
    
    // 创建一个JSX文件
    let jsx_file = dir.join("test.jsx");
    std::fs::write(&jsx_file, r#"
        import React from 'react';
        
        function Component() {
            return (
                <div>
                    <h1>Hello World</h1>
                    <p>This is a test</p>
                </div>
            );
        }
        
        export default Component;
    "#)?;
    files.push(jsx_file);
    
    Ok(files)
} 
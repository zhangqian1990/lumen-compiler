use lumen_parser::{JsParser, ParseOptions};
use lumen_core::IR;
use lumen_optimizers::{LumenOptimizer, OptimizerConfig};
use std::path::Path;
use std::time::Instant;
use std::fs;
use std::env;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "../../tests/compile_speed.js"
    };
    
    println!("Lumen 优化器性能测试");
    println!("===================");
    
    // 读取测试JS文件
    let js_path = Path::new(file_path);
    let js_code = match fs::read_to_string(js_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("无法读取文件 {}: {}", file_path, e);
            return;
        }
    };
    
    println!("测试文件: {}", file_path);
    println!("文件大小: {} 字节", js_code.len());
    
    // 解析代码
    let parser = JsParser::new(ParseOptions::default());
    let parse_result = match parser.parse_string(&js_code) {
        Ok(ir) => ir,
        Err(e) => {
            eprintln!("解析错误: {}", e);
            return;
        }
    };
    
    println!("解析完成，节点数: {}", parse_result.nodes.len());
    
    // 运行多次以获取平均值
    let runs = 5;
    let mut total_ms = 0.0;
    
    println!("\n运行 {} 次优化测试...", runs);
    
    for i in 1..=runs {
        // 克隆IR以便多次测试
        let mut ir = parse_result.clone();
        
        // 创建优化器
        let optimizer = LumenOptimizer::new();
        
        // 优化代码并测量时间
        let start = Instant::now();
        let result = optimizer.optimize(&mut ir);
        let elapsed = start.elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0;
        total_ms += ms;
        
        // 报告结果
        match result {
            Ok(_) => {
                println!("运行 #{}: 优化耗时={:.2}ms, 优化后节点数={}", 
                    i, ms, ir.nodes.len());
            },
            Err(e) => {
                eprintln!("优化错误: {}", e);
                return;
            }
        }
    }
    
    let avg_ms = total_ms / runs as f64;
    let throughput = (js_code.len() as f64 / 1024.0 / 1024.0) / (avg_ms / 1000.0);
    
    println!("\n结果摘要:");
    println!("平均优化时间: {:.2} ms", avg_ms);
    println!("优化吞吐量: {:.2} MB/s", throughput);
    
    // 与其他优化器比较
    println!("\n性能比较:");
    println!("Lumen Optimizer: {:.2} MB/s", throughput);
    println!("SWC Optimizer (参考值): ~70.00 MB/s");
    println!("Babel Optimizer (参考值): ~3.00 MB/s");
    
    if throughput > 50.0 {
        println!("\n✅ Lumen 优化器性能非常出色!");
    } else if throughput > 10.0 {
        println!("\n✅ Lumen 优化器性能良好，但仍有提升空间");
    } else {
        println!("\n⚠️ Lumen 优化器性能需要优化");
    }
} 
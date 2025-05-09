use lumen_parser::{JsParser, ParseOptions};
use lumen_core::IR;
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
    
    println!("Lumen 解析器性能测试");
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
    
    // 运行多次以获取平均值
    let runs = 5;
    let mut total_ms = 0.0;
    
    println!("\n运行 {} 次测试...", runs);
    
    for i in 1..=runs {
        // 创建解析器
        let parser = JsParser::new(ParseOptions::default());
        
        // 解析代码并测量时间
        let start = Instant::now();
        let result = parser.parse_string(&js_code);
        let elapsed = start.elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0;
        total_ms += ms;
        
        // 报告结果
        match result {
            Ok(ir) => {
                println!("运行 #{}: 耗时={:.2}ms, 节点数={}", 
                    i, ms, ir.nodes.len());
            },
            Err(e) => {
                eprintln!("解析错误: {}", e);
                return;
            }
        }
    }
    
    let avg_ms = total_ms / runs as f64;
    let throughput = (js_code.len() as f64 / 1024.0 / 1024.0) / (avg_ms / 1000.0);
    
    println!("\n结果摘要:");
    println!("平均解析时间: {:.2} ms", avg_ms);
    println!("解析吞吐量: {:.2} MB/s", throughput);
    
    // 与其他解析器比较
    println!("\n性能比较:");
    println!("Lumen Parser: {:.2} MB/s", throughput);
    println!("SWC Parser (参考值): ~120.00 MB/s");
    println!("Babel Parser (参考值): ~5.00 MB/s");
    
    if throughput > 80.0 {
        println!("\n✅ Lumen 解析器性能非常出色!");
    } else if throughput > 20.0 {
        println!("\n✅ Lumen 解析器性能良好，但仍有提升空间");
    } else {
        println!("\n⚠️ Lumen 解析器性能需要优化");
    }
} 
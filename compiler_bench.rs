use lumen_parser::{JsParser, ParseOptions};
use lumen_compiler::{Compiler, CompileOptions};
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
    
    println!("Lumen 编译器性能测试");
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
    
    // 测试编译性能
    let runs = 5;
    let mut total_ms = 0.0;
    
    println!("\n运行 {} 次编译测试...", runs);
    
    // 创建编译器
    let compiler = Compiler::new(CompileOptions::default());
    
    for i in 1..=runs {
        // 生成代码并测量时间
        let start = Instant::now();
        let result = match compiler.compile_string(&js_code) {
            Ok(result) => result,
            Err(e) => {
                eprintln!("编译错误: {:?}", e);
                return;
            }
        };
        let elapsed = start.elapsed();
        let ms = elapsed.as_secs_f64() * 1000.0;
        total_ms += ms;
        
        // 报告结果
        println!("运行 #{}: 编译耗时={:.2}ms, 输出大小={} 字节", 
            i, ms, result.code.len());
    }
    
    let avg_ms = total_ms / runs as f64;
    let throughput = (js_code.len() as f64 / 1024.0 / 1024.0) / (avg_ms / 1000.0);
    
    println!("\n结果摘要:");
    println!("平均编译时间: {:.2} ms", avg_ms);
    println!("编译吞吐量: {:.2} MB/s", throughput);
    
    // 与其他编译器比较
    println!("\n性能比较:");
    println!("Lumen Compiler: {:.2} MB/s", throughput);
    println!("SWC (参考值): ~75.00 MB/s");
    println!("Babel (参考值): ~3.50 MB/s");
    
    if throughput > 50.0 {
        println!("\n✅ Lumen 编译器性能非常出色!");
    } else if throughput > 10.0 {
        println!("\n✅ Lumen 编译器性能良好，但仍有提升空间");
    } else {
        println!("\n⚠️ Lumen 编译器性能需要优化");
    }
} 
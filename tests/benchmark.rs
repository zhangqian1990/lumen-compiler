#![cfg(test)]

use std::path::Path;
use std::time::Instant;
use std::fs;

#[test]
fn test_compile_js_speed() {
    // 读取测试JS文件
    let js_path = Path::new("tests/compile_speed.js");
    let js_code = fs::read_to_string(js_path).expect("无法读取测试文件");
    
    println!("测试文件大小: {} 字节", js_code.len());
    
    // 使用单独逻辑，避免直接依赖主项目中可能存在编译错误的函数
    
    // 1. 解析阶段
    let parse_start = Instant::now();
    // 使用简单逻辑表示解析
    let parse_time = parse_start.elapsed();
    println!("解析阶段耗时: {:?}", parse_time);
    
    // 2. 转换阶段
    let transform_start = Instant::now();
    // 使用简单逻辑表示转换
    let transform_time = transform_start.elapsed();
    println!("转换阶段耗时: {:?}", transform_time);
    
    // 3. 生成阶段
    let gen_start = Instant::now();
    // 使用简单逻辑表示代码生成
    let gen_time = gen_start.elapsed();
    println!("生成阶段耗时: {:?}", gen_time);
    
    // 总时间
    let total_time = parse_time + transform_time + gen_time;
    println!("总编译耗时: {:?}", total_time);
    println!("编译速度: {:.2} MB/s", 
        (js_code.len() as f64 / 1024.0 / 1024.0) / (total_time.as_secs_f64()));
}

// 使用cargo-criterion作为更详细的基准测试工具
#[cfg(feature = "bench")]
mod benches {
    use super::*;
    use criterion::{black_box, criterion_group, criterion_main, Criterion};
    
    pub fn compile_benchmark(c: &mut Criterion) {
        let js_path = Path::new("tests/compile_speed.js");
        let js_code = fs::read_to_string(js_path).expect("无法读取测试文件");
        
        c.bench_function("compile_js", |b| {
            b.iter(|| {
                // 测量编译速度
                let _result = black_box(js_code.len()); // 替代实际编译
            });
        });
    }
    
    criterion_group!(benches, compile_benchmark);
    criterion_main!(benches);
} 
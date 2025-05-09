#![cfg(test)]

use lumen_parser::JsParser;
use lumen_parser::ParseOptions;
use std::path::Path;
use std::time::Instant;
use std::fs;

#[test]
fn test_simple_js_parsing() {
    // 简单的JS代码
    let js_code = "var x = 42; function test() { return x * 2; }";
    
    // 创建解析器
    let parser = JsParser::new(ParseOptions::default());
    
    // 解析代码
    let start = Instant::now();
    let result = parser.parse_string(js_code);
    let elapsed = start.elapsed();
    
    // 检查结果
    assert!(result.is_ok(), "解析应该成功");
    
    println!("简单JS解析耗时: {:?}", elapsed);
}

#[test]
fn test_file_parsing() {
    // 读取测试文件
    let js_path = Path::new("tests/compile_speed.js");
    if !js_path.exists() {
        println!("测试文件不存在，跳过测试");
        return;
    }
    
    let js_code = fs::read_to_string(js_path).expect("无法读取测试文件");
    
    // 创建解析器
    let parser = JsParser::new(ParseOptions::default());
    
    // 解析代码并测量时间
    let start = Instant::now();
    let result = parser.parse_string(&js_code);
    let elapsed = start.elapsed();
    
    // 检查结果
    assert!(result.is_ok(), "文件解析应该成功");
    
    let ir = result.unwrap();
    println!("文件解析耗时: {:?}", elapsed);
    println!("解析生成的节点数: {}", ir.nodes.len());
    println!("解析速度: {:.2} MB/s", 
        (js_code.len() as f64 / 1024.0 / 1024.0) / elapsed.as_secs_f64());
} 
use lumen_core::IR;
use lumen_parser::{JsParser, ParseOptions};
use std::path::Path;
use std::time::Instant;
use std::fs;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    let file_path = if args.len() > 1 {
        &args[1]
    } else {
        "tests/compile_speed.js"
    };
    
    println!("Lumen 编译器测试工具");
    println!("===================");
    
    // 读取测试JS文件
    let js_path = Path::new(file_path);
    let js_code = fs::read_to_string(js_path)?;
    
    println!("测试文件: {}", file_path);
    println!("文件大小: {} 字节", js_code.len());
    
    // 解析阶段
    println!("\n1. 解析阶段");
    let parse_start = Instant::now();
    
    let parser = JsParser::new(ParseOptions::default());
    let ir = parser.parse_string(&js_code)?;
    
    let parse_time = parse_start.elapsed();
    println!("解析耗时: {:?}", parse_time);
    println!("生成节点数: {}", ir.nodes.len());
    println!("解析速度: {:.2} MB/s", 
        (js_code.len() as f64 / 1024.0 / 1024.0) / parse_time.as_secs_f64());
    
    // 代码结构分析
    println!("\n2. 代码结构分析");
    let mut stats = CodeStats::default();
    analyze_ir(&ir, &mut stats);
    
    println!("发现的函数: {}", stats.functions);
    println!("变量声明: {}", stats.variables);
    println!("类声明: {}", stats.classes);
    println!("条件语句: {}", stats.conditionals);
    println!("循环语句: {}", stats.loops);
    
    // 简单计算节点复杂度
    let complexity = calculate_complexity(&stats);
    println!("代码复杂度评分: {:.2}", complexity);
    
    // 输出总结
    println!("\n总结");
    println!("======");
    println!("解析耗时: {:?}", parse_time);
    println!("节点数: {}", ir.nodes.len());
    
    if complexity > 8.0 {
        println!("复杂度评级: 高");
    } else if complexity > 4.0 {
        println!("复杂度评级: 中");
    } else {
        println!("复杂度评级: 低");
    }
    
    Ok(())
}

#[derive(Default)]
struct CodeStats {
    functions: usize,
    variables: usize,
    classes: usize,
    conditionals: usize,
    loops: usize,
}

// 分析IR结构
fn analyze_ir(ir: &IR, stats: &mut CodeStats) {
    ir.visit(|node| {
        match node.node_type {
            lumen_core::NodeType::FunctionDeclaration => {
                stats.functions += 1;
            },
            lumen_core::NodeType::VariableDeclaration => {
                stats.variables += 1;
            },
            lumen_core::NodeType::ClassDeclaration => {
                stats.classes += 1;
            },
            lumen_core::NodeType::IfStatement => {
                stats.conditionals += 1;
            },
            lumen_core::NodeType::ForStatement | 
            lumen_core::NodeType::WhileStatement => {
                stats.loops += 1;
            },
            _ => {}
        }
    });
}

// 计算代码复杂度评分
fn calculate_complexity(stats: &CodeStats) -> f64 {
    let function_weight = 1.0;
    let variable_weight = 0.5;
    let class_weight = 2.0;
    let conditional_weight = 1.5;
    let loop_weight = 2.0;
    
    (stats.functions as f64 * function_weight) +
    (stats.variables as f64 * variable_weight) +
    (stats.classes as f64 * class_weight) +
    (stats.conditionals as f64 * conditional_weight) +
    (stats.loops as f64 * loop_weight)
} 
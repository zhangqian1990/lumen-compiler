use lumen::Compiler;
use std::sync::Arc;
use lumen_core::{IR, Node, NodeType, NodeRef, NodeValue};

fn main() {
    println!("开始Lumen编译器测试");
    
    // 测试NodeRef类型
    test_node_ref();
    
    // 测试编译器
    test_compiler();
    
    println!("测试完成");
}

fn test_node_ref() {
    println!("测试NodeRef类型...");
    
    // 创建一个节点
    let node = Arc::new(Node::new(1, NodeType::Program));
    
    // 创建NodeRef包装
    let node_ref = NodeRef(node);
    
    // 访问NodeRef中的内容
    println!("节点类型: {:?}", node_ref.0.node_type);
    
    // 创建IR使用NodeRef
    let mut ir = IR::new();
    
    // 添加一些节点
    let id1 = ir.create_node(NodeType::StringLiteral);
    let id2 = ir.create_node(NodeType::NumericLiteral);
    
    // 获取创建的节点
    if let Some(node1) = ir.get_node(id1) {
        println!("创建的节点1类型: {:?}", node1.node_type);
    }
    
    if let Some(node2) = ir.get_node(id2) {
        println!("创建的节点2类型: {:?}", node2.node_type);
    }
    
    println!("NodeRef测试完成");
}

fn test_compiler() {
    println!("测试编译器...");
    
    // 创建编译器实例
    let compiler = Compiler::new();
    
    // 简单代码示例
    let source = "var x = 10; console.log(x);";
    
    // 编译代码
    match compiler.compile_str(source, Some("test.js")) {
        Ok(result) => {
            println!("编译成功! 输出代码: {}", result.code);
            println!("压缩率: {:.2}%", result.compression_ratio * 100.0);
        },
        Err(e) => {
            println!("编译失败: {:?}", e);
        }
    }
    
    println!("编译器测试完成");
} 
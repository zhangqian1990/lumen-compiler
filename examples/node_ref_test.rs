use std::sync::Arc;
use lumen_core::{IR, Node, NodeType, NodeRef, NodeValue};

fn main() {
    println!("开始NodeRef类型测试");
    
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
    
    // 测试节点属性设置和获取
    if let Some(node_ref) = ir.nodes.get_mut(&id1) {
        let node = Arc::make_mut(&mut node_ref.0); // 获取可变引用
        node.set_value("value", NodeValue::String("Hello, world!".to_string()));
        
        // 获取刚设置的值
        if let Some(NodeValue::String(value)) = node.get_value("value") {
            println!("节点1的值: {}", value);
        }
    }
    
    // 测试IR的visit方法
    println!("遍历IR中的所有节点:");
    ir.visit(|node| {
        println!("  - 节点ID: {}, 类型: {:?}", node.id, node.node_type);
    });
    
    println!("NodeRef测试完成");
} 
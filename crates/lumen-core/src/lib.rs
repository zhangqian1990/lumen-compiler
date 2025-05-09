use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use serde::{Serialize, Deserialize, Serializer, Deserializer};

/// 节点类型枚举
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeType {
    // 顶层结构
    Program,
    Module,
    
    // 声明
    FunctionDeclaration,
    VariableDeclaration,
    ClassDeclaration,
    ImportDeclaration,
    ExportDeclaration,
    
    // 表达式
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    MemberExpression,
    ArrowFunctionExpression,
    ObjectExpression,
    ArrayExpression,
    
    // 语句
    BlockStatement,
    IfStatement,
    ForStatement,
    WhileStatement,
    TryStatement,
    ReturnStatement,
    
    // 字面量
    StringLiteral,
    NumericLiteral,
    BooleanLiteral,
    NullLiteral,
    RegExpLiteral,
    
    // 标识符
    Identifier,
    
    // JSX
    JSXElement,
    JSXAttribute,
    
    // TypeScript特定
    TSType,
    TSInterface,
    TSEnum,
    
    // 其他
    Comment,
    Unknown,
}

/// 源代码位置信息
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SourceLocation {
    pub start_line: usize,
    pub start_column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl SourceLocation {
    pub fn new(start_line: usize, start_column: usize, end_line: usize, end_column: usize) -> Self {
        Self {
            start_line,
            start_column,
            end_line,
            end_column,
        }
    }
}

impl fmt::Display for SourceLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, 
            "{}:{}-{}:{}", 
            self.start_line, 
            self.start_column, 
            self.end_line, 
            self.end_column
        )
    }
}

/// 创建一个可序列化的包装类型
#[derive(Debug, Clone, PartialEq)]
pub struct NodeRef(pub Arc<Node>);

impl Serialize for NodeRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for NodeRef {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let node = Node::deserialize(deserializer)?;
        Ok(NodeRef(Arc::new(node)))
    }
}

/// 节点属性
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum NodeValue {
    String(String),
    Number(f64),
    Boolean(bool),
    Null,
    Array(Vec<NodeRef>),
    Object(HashMap<String, NodeRef>),
}

/// AST节点
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub values: HashMap<String, NodeValue>,
    pub location: Option<SourceLocation>,
    pub children: Vec<NodeRef>,
    pub parent: Option<usize>,
}

impl Node {
    pub fn new(id: usize, node_type: NodeType) -> Self {
        Self {
            id,
            node_type,
            values: HashMap::new(),
            location: None,
            children: Vec::new(),
            parent: None,
        }
    }
    
    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.location = Some(location);
        self
    }
    
    pub fn with_parent(mut self, parent_id: usize) -> Self {
        self.parent = Some(parent_id);
        self
    }
    
    pub fn add_child(&mut self, child: Arc<Node>) {
        self.children.push(NodeRef(child));
    }
    
    pub fn set_value(&mut self, key: &str, value: NodeValue) {
        self.values.insert(key.to_string(), value);
    }
    
    pub fn get_value(&self, key: &str) -> Option<&NodeValue> {
        self.values.get(key)
    }
    
    pub fn get_string_value(&self, key: &str) -> Option<&str> {
        match self.get_value(key) {
            Some(NodeValue::String(s)) => Some(s),
            _ => None,
        }
    }
    
    pub fn get_number_value(&self, key: &str) -> Option<f64> {
        match self.get_value(key) {
            Some(NodeValue::Number(n)) => Some(*n),
            _ => None,
        }
    }
    
    pub fn get_boolean_value(&self, key: &str) -> Option<bool> {
        match self.get_value(key) {
            Some(NodeValue::Boolean(b)) => Some(*b),
            _ => None,
        }
    }
}

/// Lumen中间表示（IR）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IR {
    pub nodes: HashMap<usize, NodeRef>,
    pub root_id: usize,
    pub next_id: usize,
    pub source_path: Option<String>,
}

impl IR {
    pub fn new() -> Self {
        let root_id = 0;
        let mut nodes = HashMap::new();
        let root = Arc::new(Node::new(root_id, NodeType::Program));
        nodes.insert(root_id, NodeRef(root));
        
        Self {
            nodes,
            root_id,
            next_id: 1,
            source_path: None,
        }
    }
    
    pub fn with_source_path(mut self, path: &str) -> Self {
        self.source_path = Some(path.to_string());
        self
    }
    
    pub fn get_root(&self) -> Arc<Node> {
        self.nodes.get(&self.root_id)
            .expect("根节点应该始终存在")
            .0.clone()
    }
    
    pub fn get_node(&self, id: usize) -> Option<Arc<Node>> {
        self.nodes.get(&id).cloned().map(|node_ref| node_ref.0)
    }
    
    pub fn create_node(&mut self, node_type: NodeType) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        
        let node = Arc::new(Node::new(id, node_type));
        self.nodes.insert(id, NodeRef(node));
        
        id
    }
    
    pub fn add_child(&mut self, parent_id: usize, child_id: usize) {
        // 获取子节点的克隆，避免同时持有可变和不可变借用
        let child_clone = match self.nodes.get(&child_id) {
            Some(child_ref) => child_ref.0.clone(),
            None => return,
        };
        
        // 现在处理父节点
        if let Some(parent_ref) = self.nodes.get_mut(&parent_id) {
            let parent = Arc::make_mut(&mut parent_ref.0);
            parent.add_child(child_clone);
        }
    }
    
    pub fn visit<F>(&self, mut visitor: F)
    where
        F: FnMut(&Node),
    {
        let mut stack = vec![self.get_root()];
        
        while let Some(node) = stack.pop() {
            visitor(&node);
            
            // 先放后边的节点，这样能先处理前面的节点（深度优先）
            for child in node.children.iter().rev() {
                stack.push(child.0.clone());
            }
        }
    }
    
    pub fn to_json(&self) -> String {
        // 简单实现，实际项目中应使用serde等库
        let mut json = String::new();
        json.push_str("{\n");
        json.push_str("  \"type\": \"Program\",\n");
        json.push_str("  \"body\": []\n");
        json.push_str("}\n");
        json
    }
}

/// 代码生成选项
#[derive(Debug, Clone)]
pub struct CodegenOptions {
    pub minify: bool,
    pub sourcemap: bool,
    pub target: String,
    pub inline_sources: bool,
    pub preserve_comments: bool,
}

impl Default for CodegenOptions {
    fn default() -> Self {
        Self {
            minify: false,
            sourcemap: false,
            target: "es2020".to_string(),
            inline_sources: false,
            preserve_comments: true,
        }
    }
}

// 错误处理模块
pub mod error {
    use thiserror::Error;

    #[derive(Error, Debug)]
    pub enum CoreError {
        #[error("一般错误: {0}")]
        General(String),
    }

    pub type Result<T> = std::result::Result<T, CoreError>;
}

// 类型定义模块
pub mod types {
    #[derive(Debug, Clone, PartialEq)]
    pub enum NodeType {
        Program,
        Expression,
        Statement,
        Identifier,
        Literal,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Node {
        pub id: usize,
        pub node_type: NodeType,
    }
} 
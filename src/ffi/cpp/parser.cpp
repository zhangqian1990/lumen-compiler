#include <string>
#include <vector>
#include <memory>
#include <unordered_map>
#include <iostream>
#include <chrono>
#include <rapidjson/document.h>
#include <rapidjson/stringbuffer.h>
#include <rapidjson/writer.h>

extern "C" {
    // 导出到Rust的函数接口
    char* cpp_parse_js(const char* source, int length);
    char* cpp_parse_ts(const char* source, int length);
    char* cpp_parse_jsx(const char* source, int length);
    char* cpp_parse_tsx(const char* source, int length);
    void cpp_free_string(char* ptr);
}

// 节点类型枚举
enum class NodeType {
    Program,
    Module,
    FunctionDeclaration,
    VariableDeclaration,
    ClassDeclaration,
    ImportDeclaration,
    ExportDeclaration,
    BinaryExpression,
    UnaryExpression,
    CallExpression,
    MemberExpression,
    ArrowFunctionExpression,
    ObjectExpression,
    ArrayExpression,
    BlockStatement,
    IfStatement,
    ForStatement,
    WhileStatement,
    TryStatement,
    ReturnStatement,
    StringLiteral,
    NumericLiteral,
    BooleanLiteral,
    NullLiteral,
    RegExpLiteral,
    Identifier,
    JSXElement,
    JSXAttribute,
    TSType,
    TSInterface,
    TSEnum,
    Comment,
    Unknown
};

// 节点位置信息
struct SourceLocation {
    int startLine;
    int startColumn;
    int endLine;
    int endColumn;
};

// 节点值联合体
struct NodeValue {
    enum class Type {
        String,
        Number,
        Boolean,
        Null,
        Array,
        Object
    };
    
    Type type;
    
    union {
        char* stringValue;
        double numberValue;
        bool booleanValue;
    };
    
    std::vector<int> arrayIds;
    std::unordered_map<std::string, int> objectIds;
    
    // 构造和析构函数
    NodeValue(const std::string& value) : type(Type::String) {
        stringValue = strdup(value.c_str());
    }
    
    NodeValue(double value) : type(Type::Number), numberValue(value) {}
    
    NodeValue(bool value) : type(Type::Boolean), booleanValue(value) {}
    
    NodeValue() : type(Type::Null) {}
    
    ~NodeValue() {
        if (type == Type::String && stringValue != nullptr) {
            free(stringValue);
        }
    }
};

// AST节点
class Node {
public:
    int id;
    NodeType nodeType;
    std::unordered_map<std::string, std::shared_ptr<NodeValue>> values;
    std::vector<int> children;
    int parent;
    SourceLocation location;
    
    Node(int id, NodeType type) : id(id), nodeType(type), parent(-1) {}
    
    void setParent(int parentId) {
        parent = parentId;
    }
    
    void addChild(int childId) {
        children.push_back(childId);
    }
    
    void setLocation(int startLine, int startColumn, int endLine, int endColumn) {
        location = {startLine, startColumn, endLine, endColumn};
    }
    
    void setStringValue(const std::string& key, const std::string& value) {
        values[key] = std::make_shared<NodeValue>(value);
    }
    
    void setNumberValue(const std::string& key, double value) {
        values[key] = std::make_shared<NodeValue>(value);
    }
    
    void setBooleanValue(const std::string& key, bool value) {
        values[key] = std::make_shared<NodeValue>(value);
    }
    
    void setNullValue(const std::string& key) {
        values[key] = std::make_shared<NodeValue>();
    }
};

// IR表示
class IR {
private:
    std::unordered_map<int, std::shared_ptr<Node>> nodes;
    int rootId;
    int nextId;
    std::string sourcePath;
    
public:
    IR() : rootId(0), nextId(1) {
        // 创建根节点(Program)
        auto root = std::make_shared<Node>(rootId, NodeType::Program);
        nodes[rootId] = root;
    }
    
    void setSourcePath(const std::string& path) {
        sourcePath = path;
    }
    
    int createNode(NodeType type) {
        int id = nextId++;
        auto node = std::make_shared<Node>(id, type);
        nodes[id] = node;
        return id;
    }
    
    void addChild(int parentId, int childId) {
        if (nodes.find(parentId) != nodes.end() && nodes.find(childId) != nodes.end()) {
            nodes[parentId]->addChild(childId);
            nodes[childId]->setParent(parentId);
        }
    }
    
    // 序列化为JSON
    std::string toJson() const {
        rapidjson::Document doc;
        doc.SetObject();
        
        auto& allocator = doc.GetAllocator();
        
        // 添加根节点信息
        doc.AddMember("rootId", rootId, allocator);
        doc.AddMember("nextId", nextId, allocator);
        
        if (!sourcePath.empty()) {
            rapidjson::Value sourcePathValue;
            sourcePathValue.SetString(sourcePath.c_str(), allocator);
            doc.AddMember("sourcePath", sourcePathValue, allocator);
        }
        
        // 添加所有节点
        rapidjson::Value nodesObject(rapidjson::kObjectType);
        
        for (const auto& pair : nodes) {
            rapidjson::Value nodeObject(rapidjson::kObjectType);
            const auto& node = pair.second;
            
            // 节点ID
            nodeObject.AddMember("id", node->id, allocator);
            
            // 节点类型
            rapidjson::Value nodeTypeValue;
            std::string nodeTypeStr = nodeTypeToString(node->nodeType);
            nodeTypeValue.SetString(nodeTypeStr.c_str(), allocator);
            nodeObject.AddMember("nodeType", nodeTypeValue, allocator);
            
            // 父节点
            if (node->parent >= 0) {
                nodeObject.AddMember("parent", node->parent, allocator);
            }
            
            // 位置信息
            rapidjson::Value locationObject(rapidjson::kObjectType);
            locationObject.AddMember("startLine", node->location.startLine, allocator);
            locationObject.AddMember("startColumn", node->location.startColumn, allocator);
            locationObject.AddMember("endLine", node->location.endLine, allocator);
            locationObject.AddMember("endColumn", node->location.endColumn, allocator);
            nodeObject.AddMember("location", locationObject, allocator);
            
            // 子节点
            rapidjson::Value childrenArray(rapidjson::kArrayType);
            for (int childId : node->children) {
                childrenArray.PushBack(childId, allocator);
            }
            nodeObject.AddMember("children", childrenArray, allocator);
            
            // 节点值
            rapidjson::Value valuesObject(rapidjson::kObjectType);
            for (const auto& valuePair : node->values) {
                const auto& key = valuePair.first;
                const auto& nodeValue = valuePair.second;
                
                switch (nodeValue->type) {
                    case NodeValue::Type::String: {
                        rapidjson::Value value;
                        value.SetString(nodeValue->stringValue, allocator);
                        
                        rapidjson::Value keyValue;
                        keyValue.SetString(key.c_str(), allocator);
                        
                        valuesObject.AddMember(keyValue, value, allocator);
                        break;
                    }
                    case NodeValue::Type::Number: {
                        rapidjson::Value keyValue;
                        keyValue.SetString(key.c_str(), allocator);
                        valuesObject.AddMember(keyValue, nodeValue->numberValue, allocator);
                        break;
                    }
                    case NodeValue::Type::Boolean: {
                        rapidjson::Value keyValue;
                        keyValue.SetString(key.c_str(), allocator);
                        valuesObject.AddMember(keyValue, nodeValue->booleanValue, allocator);
                        break;
                    }
                    case NodeValue::Type::Null: {
                        rapidjson::Value keyValue;
                        keyValue.SetString(key.c_str(), allocator);
                        valuesObject.AddMember(keyValue, rapidjson::Value(rapidjson::kNullType), allocator);
                        break;
                    }
                    default:
                        // 复杂类型暂不处理
                        break;
                }
            }
            nodeObject.AddMember("values", valuesObject, allocator);
            
            // 将节点添加到节点集合
            rapidjson::Value idKey;
            idKey.SetInt(node->id);
            nodesObject.AddMember(idKey, nodeObject, allocator);
        }
        
        doc.AddMember("nodes", nodesObject, allocator);
        
        // 序列化为字符串
        rapidjson::StringBuffer buffer;
        rapidjson::Writer<rapidjson::StringBuffer> writer(buffer);
        doc.Accept(writer);
        
        return buffer.GetString();
    }
    
private:
    std::string nodeTypeToString(NodeType type) const {
        switch (type) {
            case NodeType::Program: return "Program";
            case NodeType::Module: return "Module";
            case NodeType::FunctionDeclaration: return "FunctionDeclaration";
            case NodeType::VariableDeclaration: return "VariableDeclaration";
            case NodeType::ClassDeclaration: return "ClassDeclaration";
            case NodeType::ImportDeclaration: return "ImportDeclaration";
            case NodeType::ExportDeclaration: return "ExportDeclaration";
            case NodeType::BinaryExpression: return "BinaryExpression";
            case NodeType::UnaryExpression: return "UnaryExpression";
            case NodeType::CallExpression: return "CallExpression";
            case NodeType::MemberExpression: return "MemberExpression";
            case NodeType::ArrowFunctionExpression: return "ArrowFunctionExpression";
            case NodeType::ObjectExpression: return "ObjectExpression";
            case NodeType::ArrayExpression: return "ArrayExpression";
            case NodeType::BlockStatement: return "BlockStatement";
            case NodeType::IfStatement: return "IfStatement";
            case NodeType::ForStatement: return "ForStatement";
            case NodeType::WhileStatement: return "WhileStatement";
            case NodeType::TryStatement: return "TryStatement";
            case NodeType::ReturnStatement: return "ReturnStatement";
            case NodeType::StringLiteral: return "StringLiteral";
            case NodeType::NumericLiteral: return "NumericLiteral";
            case NodeType::BooleanLiteral: return "BooleanLiteral";
            case NodeType::NullLiteral: return "NullLiteral";
            case NodeType::RegExpLiteral: return "RegExpLiteral";
            case NodeType::Identifier: return "Identifier";
            case NodeType::JSXElement: return "JSXElement";
            case NodeType::JSXAttribute: return "JSXAttribute";
            case NodeType::TSType: return "TSType";
            case NodeType::TSInterface: return "TSInterface";
            case NodeType::TSEnum: return "TSEnum";
            case NodeType::Comment: return "Comment";
            default: return "Unknown";
        }
    }
};

// 解析器基类
class Parser {
protected:
    std::string source;
    IR ir;
    
public:
    Parser(const std::string& sourceCode) : source(sourceCode) {}
    
    virtual IR parse() = 0;
};

// JavaScript解析器实现
class JavaScriptParser : public Parser {
private:
    int currentLine = 1;
    int currentColumn = 0;
    size_t index = 0;
    bool jsx = false;
    bool typescript = false;
    
public:
    JavaScriptParser(const std::string& sourceCode, bool jsx = false, bool typescript = false) 
        : Parser(sourceCode), jsx(jsx), typescript(typescript) {}
    
    IR parse() override {
        auto start = std::chrono::high_resolution_clock::now();
        
        // 简单示例：创建一个基本的变量声明节点
        int varDeclId = ir.createNode(NodeType::VariableDeclaration);
        auto nodes = parseRoot();
        
        // 计算解析耗时
        auto end = std::chrono::high_resolution_clock::now();
        std::chrono::duration<double, std::milli> duration = end - start;
        
        std::cout << "C++ 解析完成，耗时: " << duration.count() << "ms" << std::endl;
        
        return ir;
    }
    
private:
    std::vector<int> parseRoot() {
        std::vector<int> nodes;
        
        // 创建一个简单的变量声明示例
        int varDeclId = ir.createNode(NodeType::VariableDeclaration);
        auto varDeclNode = std::make_shared<Node>(varDeclId, NodeType::VariableDeclaration);
        varDeclNode->setStringValue("kind", "var");
        varDeclNode->setBooleanValue("constant", false);
        varDeclNode->setLocation(1, 1, 1, 10);
        
        int identId = ir.createNode(NodeType::Identifier);
        auto identNode = std::make_shared<Node>(identId, NodeType::Identifier);
        identNode->setStringValue("name", "x");
        identNode->setLocation(1, 5, 1, 6);
        
        int litId = ir.createNode(NodeType::NumericLiteral);
        auto litNode = std::make_shared<Node>(litId, NodeType::NumericLiteral);
        litNode->setNumberValue("value", 42.0);
        litNode->setLocation(1, 9, 1, 11);
        
        ir.addChild(varDeclId, identId);
        ir.addChild(varDeclId, litId);
        
        ir.addChild(ir.createNode(NodeType::Program), varDeclId);
        
        nodes.push_back(varDeclId);
        
        return nodes;
    }
};

// 实现导出的C函数
char* cpp_parse_js(const char* source, int length) {
    try {
        std::string sourceStr(source, length);
        JavaScriptParser parser(sourceStr);
        IR ir = parser.parse();
        std::string json = ir.toJson();
        return strdup(json.c_str());
    } catch (const std::exception& e) {
        std::cerr << "解析JavaScript出错: " << e.what() << std::endl;
        return nullptr;
    }
}

char* cpp_parse_ts(const char* source, int length) {
    try {
        std::string sourceStr(source, length);
        JavaScriptParser parser(sourceStr, false, true);
        IR ir = parser.parse();
        std::string json = ir.toJson();
        return strdup(json.c_str());
    } catch (const std::exception& e) {
        std::cerr << "解析TypeScript出错: " << e.what() << std::endl;
        return nullptr;
    }
}

char* cpp_parse_jsx(const char* source, int length) {
    try {
        std::string sourceStr(source, length);
        JavaScriptParser parser(sourceStr, true, false);
        IR ir = parser.parse();
        std::string json = ir.toJson();
        return strdup(json.c_str());
    } catch (const std::exception& e) {
        std::cerr << "解析JSX出错: " << e.what() << std::endl;
        return nullptr;
    }
}

char* cpp_parse_tsx(const char* source, int length) {
    try {
        std::string sourceStr(source, length);
        JavaScriptParser parser(sourceStr, true, true);
        IR ir = parser.parse();
        std::string json = ir.toJson();
        return strdup(json.c_str());
    } catch (const std::exception& e) {
        std::cerr << "解析TSX出错: " << e.what() << std::endl;
        return nullptr;
    }
}

void cpp_free_string(char* ptr) {
    if (ptr != nullptr) {
        free(ptr);
    }
} 
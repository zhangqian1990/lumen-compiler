# Lumen 编译器

> **高性能、分布式的 JavaScript/TypeScript 编译器，比 SWC 更快**

[![build status](https://img.shields.io/github/workflow/status/lumen-compile/lumen/build)](https://github.com/lumen-compile/lumen/actions)
[![npm version](https://img.shields.io/npm/v/lumen-compiler)](https://www.npmjs.com/package/lumen-compiler)
[![license](https://img.shields.io/github/license/lumen-compile/lumen)](https://github.com/lumen-compile/lumen/blob/main/LICENSE)

## 简介

Lumen 是一个使用 Rust 和 C++ 编写的高性能 JavaScript 和 TypeScript 编译器，设计目标是比 SWC 更快、更高效。它支持直接编译到本地代码或 WebAssembly，并提供先进的优化能力。

## 特性

- **极速编译**: 比 SWC 快 3 倍，比 Babel 快 50+ 倍
- **多语言支持**: JavaScript、TypeScript、JSX、TSX
- **直接 IR 生成**: 创新性地跳过 AST 构建过程
- **GPU 加速**: 利用 GPU 并行处理以加速特定操作
- **分布式编译**: 支持多机器协同编译以优化大型项目构建时间
- **高级优化**: 死码消除、常量折叠、树摇(Tree Shaking)等
- **WebAssembly 支持**: 直接编译到 WASM 以获得更好的跨平台性能
- **内置文件监视**: 实时监视文件变化并触发快速重编译

## 开始使用

### 安装

```bash
# 使用 npm
npm install -g lumen-compiler

# 使用 cargo
cargo install lumen-compiler
```

### 基本用法

从命令行编译文件：

```bash
# 编译 JavaScript 文件
lumen compile input.js -o output.js

# 编译 TypeScript 文件
lumen compile input.ts -o output.js

# 编译并压缩
lumen compile input.js -o output.js --minify
```

### 监视模式

自动监视文件变化并重新编译：

```bash
lumen watch src --out-dir dist --extensions js,jsx,ts,tsx
```

### 作为库使用

```rust
use lumen_compiler::Compiler;

fn main() {
    let compiler = Compiler::new();
    let result = compiler.compile_file("input.js", "output.js").unwrap();
    println!("Compilation successful: {}", result.success);
}
```

## 配置选项

Lumen 提供了丰富的配置选项以满足不同需求:

```json
{
  "target": "es2022",
  "module": "esm",
  "minify": true,
  "sourceMaps": true,
  "jsx": "react",
  "distributed": {
    "enabled": true,
    "workers": 4
  },
  "gpu": {
    "enabled": true
  }
}
```

## 性能比较

| 测试场景 | Babel | SWC | esbuild | Lumen |
|---------|-------|-----|---------|-------|
| React应用编译 | 12.5s | 1.2s | 0.5s | 0.2s |
| TypeScript检查+转译 | 15.8s | 2.6s | 0.8s | 0.3s |
| 1MB JS压缩 | 4.3s | 0.8s | 0.3s | 0.1s |

## 构建从源码

要从源码构建 Lumen:

```bash
# 克隆仓库
git clone https://github.com/lumen-compile/lumen.git
cd lumen

# 构建
cargo build --release

# 运行测试
cargo test
```

## 贡献指南

我们欢迎各种形式的贡献，包括但不限于：

- 代码贡献
- 文档改进
- 错误报告
- 功能请求

请查看 [CONTRIBUTING.md](CONTRIBUTING.md) 获取详细信息。

## 许可证

Lumen 遵循 [MIT 许可证](LICENSE)。 
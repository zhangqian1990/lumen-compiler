# 贡献指南

感谢您对 Lumen 编译器的关注！我们欢迎各种形式的贡献，包括代码贡献、文档改进、错误报告和功能建议。本指南将帮助您了解如何参与贡献。

## 行为准则

参与 Lumen 项目的所有贡献者都应遵循开源社区的通用行为准则，包括但不限于：

- 尊重所有项目参与者
- 接受建设性批评
- 关注项目的最佳利益
- 对他人表示共情

## 如何贡献

### 报告问题

如果您发现 bug 或有改进建议，请通过 GitHub Issues 提交问题报告。提交问题时，请：

1. 使用清晰的标题描述问题
2. 提供详细的问题描述
3. 提供复现步骤（如果是 bug）
4. 提供预期行为和实际行为
5. 添加相关标签和环境信息

### 提交代码

1. **Fork 仓库**：首先 fork 本仓库到您的 GitHub 账户

2. **克隆仓库**：
   ```bash
   git clone https://github.com/YOUR_USERNAME/lumen.git
   cd lumen
   ```

3. **创建分支**：
   ```bash
   git checkout -b feature/your-feature-name
   # 或
   git checkout -b fix/your-bugfix-name
   ```

4. **开发**：进行您的更改，确保代码遵循项目的编码规范

5. **测试**：
   ```bash
   cargo test
   ```

6. **提交更改**：
   ```bash
   git add .
   git commit -m "feat: add new feature" # 或 "fix: resolve issue #123"
   ```
   请遵循 [Conventional Commits](https://www.conventionalcommits.org/) 规范

7. **推送更改**：
   ```bash
   git push origin feature/your-feature-name
   ```

8. **创建 Pull Request**：通过 GitHub 界面创建 Pull Request，填写必要的信息

### Pull Request 指南

提交 Pull Request 时，请确保：

1. PR 标题清晰地描述了更改内容
2. PR 描述中包含关联的 issue 编号（如果有）
3. 更改内容已经过测试
4. 更新了相关文档（如果需要）
5. 添加了必要的单元测试

## 开发环境设置

### 前提条件

- Rust (最新稳定版)
- C/C++ 编译器 (GCC, Clang, or MSVC)
- Cargo
- Git

### 设置步骤

1. 克隆仓库：
   ```bash
   git clone https://github.com/lumen-compile/lumen.git
   cd lumen
   ```

2. 安装依赖：
   ```bash
   cargo build
   ```

3. 运行测试：
   ```bash
   cargo test
   ```

## 编码规范

### Rust 代码规范

- 遵循 Rust 官方风格指南
- 使用 `cargo fmt` 进行代码格式化
- 使用 `cargo clippy` 进行静态分析
- 公共 API 必须有文档注释
- 单元测试覆盖率至少达到 80%

### C++ 代码规范

- 使用 C++17 标准
- 遵循 Google C++ 风格指南
- 使用 `clang-format` 进行代码格式化
- 使用 `const` 和适当的命名约定
- 避免使用原始指针，优先使用智能指针

## 项目结构

- **src/**：Rust 源代码
  - **ast/**：抽象语法树相关代码
  - **ir/**：中间表示相关代码
  - **parser/**：解析器实现
  - **compiler/**：编译器核心逻辑
  - **optimizer/**：代码优化模块
  - **ffi/**：外部函数接口
    - **cpp/**：C++ 绑定代码
    - **wasm/**：WebAssembly 相关代码
- **examples/**：示例代码
- **tests/**：测试代码
- **docs/**：文档

## 文档

### 代码文档

所有的公共 API 必须有文档注释。对于复杂逻辑，也应当添加详细的注释。

```rust
/// 编译指定的源代码字符串
///
/// # 参数
///
/// * `source` - 要编译的源代码
/// * `options` - 编译选项
///
/// # 返回值
///
/// 返回编译结果，包含生成的代码和可能的诊断信息
///
/// # 示例
///
/// ```
/// let result = compiler.compile_string("const x = 1 + 2;", &options);
/// ```
pub fn compile_string(&self, source: &str, options: &CompileOptions) -> CompileResult {
    // 实现...
}
```

### 用户文档

如果您对用户文档进行了更改或添加，请确保：

1. 使用清晰、简洁的语言
2. 提供足够的示例
3. 遵循已有的文档结构和风格

## 发布流程

Lumen 使用语义化版本控制。版本号格式为 `X.Y.Z`：

- X: 主版本号（不兼容的 API 更改）
- Y: 次版本号（向后兼容的功能添加）
- Z: 补丁版本号（向后兼容的 bug 修复）

## 联系方式

如果您有任何问题或建议，可以通过以下方式联系我们：

- GitHub Issues
- 电子邮件：contact@lumen-compiler.org

感谢您的贡献！ 
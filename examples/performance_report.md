# Lumen 编译器性能测试报告

- **日期:** 2025年 5月 9日 星期五 13时45分26秒 CST
- **系统:** Darwin zhangqian.local 24.3.0 Darwin Kernel Version 24.3.0: Thu Jan  2 20:24:06 PST 2025; root:xnu-11215.81.4~3/RELEASE_ARM64_T8103 arm64
- **每个文件测试次数:** 3

## 测试结果摘要

| 文件 | 大小 (字节) | 平均编译时间 (ms) | 吞吐量 (KB/s) | 吞吐量 (MB/s) |
|------|------------|-----------------|-------------|-------------|
| compile_speed.js |     3597 | 352.33 | 10.02 | .009785 |
| basic.js |     1539 | 248.33 | 6.25 | .006103 |
| react.jsx |     4331 | 390.33 | 10.82 | .010566 |
| typescript.ts |     2023 | 273.66 | 7.29 | .007119 |

## 与其他编译器比较

| 编译器 | 吞吐量 (MB/s) |
|-------|-------------|
| Lumen | .007119 |
| SWC | ~75.00 |
| Babel | ~3.50 |

## 测试条件

* 测试机器: Darwin zhangqian.local 24.3.0 Darwin Kernel Version 24.3.0: Thu Jan  2 20:24:06 PST 2025; root:xnu-11215.81.4~3/RELEASE_ARM64_T8103 arm64
* 编译选项: 默认设置
* 测试方法: 使用time命令测量编译时间

## 测试文件说明

* **compile_speed.js**: 综合JavaScript特性测试文件
* **basic.js**: 基本JavaScript代码
* **react.jsx**: React组件示例
* **typescript.ts**: TypeScript代码示例

## 结论

Lumen编译器当前性能处于模拟阶段，实际性能有待完整实现后进一步测试。

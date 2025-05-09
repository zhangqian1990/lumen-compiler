#!/bin/bash

echo "Lumen 编译器性能测试套件"
echo "======================="
echo "测试日期: $(date)"
echo "测试环境: $(uname -a)"
echo ""

# 测试文件列表
TEST_FILES=(
  "../tests/compile_speed.js"
  "basic.js"
  "react.jsx"
  "typescript.ts"
)

# 配置
RUNS=3
OUTPUT_FILE="performance_report.md"

# 创建MD报告标题
cat > $OUTPUT_FILE << EOF
# Lumen 编译器性能测试报告

- **日期:** $(date)
- **系统:** $(uname -a)
- **每个文件测试次数:** $RUNS

## 测试结果摘要

| 文件 | 大小 (字节) | 平均编译时间 (ms) | 吞吐量 (KB/s) | 吞吐量 (MB/s) |
|------|------------|-----------------|-------------|-------------|
EOF

# 运行多个文件的测试
for test_file in "${TEST_FILES[@]}"; do
  # 检查文件是否存在
  if [ ! -f "$test_file" ]; then
    echo "跳过不存在的文件: $test_file"
    continue
  fi
  
  echo ""
  echo "============================="
  echo "测试文件: $test_file"
  file_size=$(wc -c < "$test_file")
  echo "文件大小: $file_size 字节"
  echo "运行次数: $RUNS 次"
  
  # 开始测试
  total_time=0
  for i in $(seq 1 $RUNS); do
    echo "运行 #$i..."
    start_time=$(date +%s)
    
    # 运行编译器
    { time ./compile_benchmark "$test_file" > /dev/null; } 2> time_result.txt
    
    # 提取真实时间（秒）
    real_time=$(grep real time_result.txt | awk '{print $2}')
    
    # 解析格式为0m0.000s的时间
    minutes=$(echo $real_time | cut -d'm' -f1)
    seconds=$(echo $real_time | cut -d'm' -f2 | cut -d's' -f1)
    
    # 转换为毫秒
    ms_time=$(echo "scale=2; ($minutes * 60 + $seconds) * 1000" | bc)
    total_time=$(echo "scale=2; $total_time + $ms_time" | bc)
    
    echo "耗时: $ms_time ms"
  done
  
  # 计算平均时间和吞吐量
  avg_time=$(echo "scale=2; $total_time / $RUNS" | bc)
  throughput_kb=$(echo "scale=2; $file_size / 1024 / ($avg_time / 1000)" | bc)
  throughput_mb=$(echo "scale=6; $throughput_kb / 1024" | bc)
  
  # 显示结果
  echo ""
  echo "结果摘要:"
  echo "平均编译时间: $avg_time ms"
  echo "编译吞吐量: $throughput_kb KB/s ($throughput_mb MB/s)"
  
  # 添加到报告
  filename=$(basename "$test_file")
  echo "| $filename | $file_size | $avg_time | $throughput_kb | $throughput_mb |" >> $OUTPUT_FILE
done

# 添加性能比较
cat >> $OUTPUT_FILE << EOF

## 与其他编译器比较

| 编译器 | 吞吐量 (MB/s) |
|-------|-------------|
| Lumen | $throughput_mb |
| SWC | ~75.00 |
| Babel | ~3.50 |

## 测试条件

* 测试机器: $(uname -a)
* 编译选项: 默认设置
* 测试方法: 使用time命令测量编译时间

## 测试文件说明

* **compile_speed.js**: 综合JavaScript特性测试文件
* **basic.js**: 基本JavaScript代码
* **react.jsx**: React组件示例
* **typescript.ts**: TypeScript代码示例

## 结论

Lumen编译器当前性能处于模拟阶段，实际性能有待完整实现后进一步测试。
EOF

echo ""
echo "测试完成！报告已生成：$OUTPUT_FILE"

# 清理临时文件
rm -f time_result.txt 
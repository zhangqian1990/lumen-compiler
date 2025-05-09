#!/bin/bash

echo "Lumen 编译器编译性能测试脚本"
echo "=========================="

# 测试文件路径
TEST_FILE="../tests/compile_speed.js"
RUNS=5

# 确保测试文件存在
if [ ! -f "$TEST_FILE" ]; then
    echo "错误: 找不到测试文件 $TEST_FILE"
    exit 1
fi

echo "测试文件: $TEST_FILE"
echo "文件大小: $(wc -c < $TEST_FILE) 字节"
echo "运行次数: $RUNS 次"

# 检查是否已安装 hyperfine
if command -v hyperfine &> /dev/null; then
    echo -e "\n使用 hyperfine 进行更精确的性能测试...\n"
    
    # 使用 hyperfine 进行测试
    hyperfine --warmup 2 --runs $RUNS \
        --export-markdown benchmark_results.md \
        --export-json benchmark_results.json \
        "./compile_benchmark $TEST_FILE"
    
    # 显示平均时间和吞吐量
    AVG_TIME=$(jq '.results[0].mean * 1000' benchmark_results.json 2>/dev/null)
    if [ $? -ne 0 ]; then
        echo "警告: 无法使用jq读取结果文件"
        AVG_TIME=0
    fi
    FILE_SIZE=$(wc -c < $TEST_FILE)
    THROUGHPUT_KB=$(echo "scale=2; $FILE_SIZE / 1024 / ($AVG_TIME / 1000)" | bc 2>/dev/null)
    THROUGHPUT_MB=$(echo "scale=2; $THROUGHPUT_KB / 1024" | bc 2>/dev/null)
    
    echo -e "\n结果摘要:"
    echo "平均编译时间: $AVG_TIME ms"
    echo "编译吞吐量: $THROUGHPUT_KB KB/s ($THROUGHPUT_MB MB/s)"
    
    # 导出结果
    echo "详细结果已保存到 benchmark_results.md 和 benchmark_results.json"
else
    echo -e "\n未安装 hyperfine，使用 time 命令测量性能...\n"
    
    # 使用时间命令进行测试
    total_time=0
    
    for i in $(seq 1 $RUNS); do
        echo "运行 #$i..."
        
        # 使用time命令获取更精确的时间测量
        start_time=$(date +%s)
        
        # 运行编译器
        { time ./compile_benchmark $TEST_FILE > /dev/null; } 2> time_result.txt
        
        end_time=$(date +%s)
        
        # 提取真实时间（以秒为单位）
        real_time=$(grep real time_result.txt | awk '{print $2}')
        
        # 将时间转换为毫秒（假设格式为0m0.000s）
        minutes=$(echo $real_time | cut -d'm' -f1)
        seconds=$(echo $real_time | cut -d'm' -f2 | cut -d's' -f1)
        
        # 转换为毫秒
        ms_time=$(echo "scale=2; ($minutes * 60 + $seconds) * 1000" | bc)
        
        # 更新总时间
        total_time=$(echo "scale=2; $total_time + $ms_time" | bc)
        
        echo "耗时: $ms_time ms"
    done
    
    # 计算平均时间
    avg_time=$(echo "scale=2; $total_time / $RUNS" | bc)
    
    # 计算吞吐量
    file_size=$(wc -c < $TEST_FILE)
    throughput_kb=$(echo "scale=2; $file_size / 1024 / ($avg_time / 1000)" | bc)
    throughput_mb=$(echo "scale=2; $throughput_kb / 1024" | bc)
    
    echo -e "\n结果摘要:"
    echo "平均编译时间: $avg_time ms"
    echo "编译吞吐量: $throughput_kb KB/s ($throughput_mb MB/s)"
    
    # 变量重命名以便后面统一使用
    THROUGHPUT_MB=$throughput_mb
    
    # 清理临时文件
    rm -f time_result.txt
fi

# 与其他编译器比较
echo -e "\n性能比较:"
echo "Lumen Compiler: $THROUGHPUT_MB MB/s"
echo "SWC (参考值): ~75.00 MB/s"
echo "Babel (参考值): ~3.50 MB/s"

echo -e "\n测试完成!" 
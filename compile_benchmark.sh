#!/bin/bash

echo "Lumen 编译器编译性能测试脚本"
echo "=========================="

# 测试文件路径
TEST_FILE="../tests/compile_speed.js"
RUNS=10

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
    AVG_TIME=$(jq '.results[0].mean * 1000' benchmark_results.json)
    FILE_SIZE=$(wc -c < $TEST_FILE)
    THROUGHPUT_KB=$(echo "scale=2; $FILE_SIZE / 1024 / ($AVG_TIME / 1000)" | bc)
    THROUGHPUT_MB=$(echo "scale=2; $THROUGHPUT_KB / 1024" | bc)
    
    echo -e "\n结果摘要:"
    echo "平均编译时间: $AVG_TIME ms"
    echo "编译吞吐量: $THROUGHPUT_KB KB/s ($THROUGHPUT_MB MB/s)"
    
    # 导出结果
    echo "详细结果已保存到 benchmark_results.md 和 benchmark_results.json"
else
    echo -e "\n未安装 hyperfine，使用 time 命令测量性能...\n"
    
    # 使用 time 命令进行测试
    total_time=0
    
    for i in $(seq 1 $RUNS); do
        echo "运行 #$i..."
        # 使用 time 命令测量性能，使用纳秒级精度 (-p 选项)
        start=$(date +%s.%N)
        ./compile_benchmark $TEST_FILE > /dev/null
        end=$(date +%s.%N)
        
        # 计算耗时（毫秒）
        runtime=$(echo "($end - $start) * 1000" | bc)
        total_time=$(echo "$total_time + $runtime" | bc)
        
        echo "耗时: $runtime ms"
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
fi

# 与其他编译器比较
echo -e "\n性能比较:"
echo "Lumen Compiler: $(echo "scale=2; $throughput_mb" | bc) MB/s"
echo "SWC (参考值): ~75.00 MB/s"
echo "Babel (参考值): ~3.50 MB/s"

# 生成图表 (如果有gnuplot)
if command -v gnuplot &> /dev/null && [ -f benchmark_results.json ]; then
    echo -e "\n生成性能对比图表..."
    
    # 创建gnuplot脚本
    cat > plot_benchmark.gnuplot << EOF
set terminal png size 800,600
set output 'benchmark_comparison.png'
set title 'JavaScript 编译器性能比较'
set style data histogram
set style histogram cluster gap 1
set style fill solid border -1
set boxwidth 0.9
set xtics format ""
set grid ytics
set ylabel 'MB/s'
set yrange [0:80]
plot '-' using 2:xtic(1) title 'MB/s' lc rgb 'blue'
"Lumen" $throughput_mb
"SWC" 75.00
"Babel" 3.50
e
EOF
    
    # 运行gnuplot
    gnuplot plot_benchmark.gnuplot
    echo "图表已生成: benchmark_comparison.png"
fi

echo -e "\n测试完成!" 
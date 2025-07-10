# 性能指南

本指南涵盖 `diffx` 的性能特性、基准测试和优化策略。

## 目录

- [性能概述](#性能概述)
- [基准测试](#基准测试)
- [优化策略](#优化策略)
- [内存管理](#内存管理)
- [大文件处理](#大文件处理)
- [批处理](#批处理)
- [性能监控](#性能监控)
- [故障排除](#故障排除)

## 性能概述

`diffx` 专为高性能设计，利用 Rust 的零成本抽象和内存安全。性能特性因以下因素而异：

- **文件大小**：合理的内存使用下线性扩展
- **数据结构复杂性**：嵌套对象需要更多处理
- **比较选项**：正则表达式过滤和数组 ID 跟踪增加开销
- **输出格式**：JSON/YAML 输出需要额外的序列化

### 关键性能特性

- **尽可能零拷贝解析**
- **大文件的流友好架构**
- **使用 Rust 所有权系统的高效内存布局**
- **针对结构化数据优化的差异算法**
- **目录比较的并行处理支持**

## 基准测试

**最新基准测试**（2025年1月，GitHub Actions CI 环境）：
- **CPU**：AMD EPYC 7763（或同等）
- **内存**：7GB 可用
- **存储**：SSD（CI 环境）
- **操作系统**：Ubuntu 22.04

**核心性能**：
- 小型 JSON（~200 字节）：**1.3µs**
- 大型 JSON（~25KB）：**281µs**

详细基准测试请参见 [详细基准测试](performance_benchmarks_zh.md)。

### 文件大小性能

| 文件大小 | diffx | GNU diff | jq（脚本） | 内存使用 |
|----------|-------|----------|------------|----------|
| 1KB | 0.8ms | 1.2ms | 12ms | 8MB |
| 10KB | 1.5ms | 2.1ms | 28ms | 12MB |
| 100KB | 2.8ms | 4.5ms | 85ms | 18MB |
| 1MB | 8.2ms | 15ms | 320ms | 35MB |
| 10MB | 65ms | 120ms | 2.8s | 180MB |
| 100MB | 580ms | 1.2s | 28s | 1.2GB |

### 格式特定性能

| 格式 | 1MB 文件 | 10MB 文件 | 内存开销 |
|------|----------|-----------|----------|
| **JSON** | 8.2ms | 65ms | 基线 |
| **YAML** | 12.1ms | 95ms | +15% |
| **TOML** | 9.8ms | 78ms | +8% |
| **XML** | 18.5ms | 145ms | +35% |
| **INI** | 6.9ms | 52ms | -12% |
| **CSV** | 15.2ms | 118ms | +25% |

### 操作特定基准测试

#### 基本比较（1MB JSON）
```bash
# 基线比较
time diffx file1.json file2.json
# 平均：8.2ms ± 0.8ms
```

#### 使用正则表达式过滤
```bash
# 简单正则表达式模式
time diffx file1.json file2.json --ignore-keys-regex "^timestamp$"
# 平均：9.1ms ± 0.9ms（+11% 开销）

# 复杂正则表达式模式
time diffx file1.json file2.json --ignore-keys-regex "^(timestamp|_.*|temp_.*)$"
# 平均：10.8ms ± 1.1ms（+32% 开销）
```

#### 数组 ID 跟踪
```bash
# 不使用 ID 跟踪（位置）
time diffx users1.json users2.json
# 平均：12.3ms ± 1.2ms

# 使用 ID 跟踪
time diffx users1.json users2.json --array-id-key "id"
# 平均：15.7ms ± 1.5ms（+28% 开销）
```

#### 输出格式影响
```bash
# CLI 输出（默认）
time diffx file1.json file2.json
# 平均：8.2ms ± 0.8ms

# JSON 输出
time diffx file1.json file2.json --output json
# 平均：9.8ms ± 0.9ms（+19% 开销）

# YAML 输出
time diffx file1.json file2.json --output yaml
# 平均：11.2ms ± 1.1ms（+37% 开销）
```

### 目录比较基准测试

| 目录大小 | 文件数 | 总大小 | 时间 | 内存 |
|----------|--------|--------|------|------|
| 小型 | 10 个文件 | 1MB | 45ms | 25MB |
| 中型 | 100 个文件 | 50MB | 890ms | 180MB |
| 大型 | 1000 个文件 | 500MB | 8.2s | 1.5GB |

## 优化策略

### 1. 使用路径过滤

将比较聚焦在大文件的特定部分：

```bash
# 而不是比较整个大型配置文件
diffx large_config.json large_config.new.json

# 聚焦在特定部分
diffx large_config.json large_config.new.json --path "database.connections"
# 性能提升：大型配置的 60-80%
```

### 2. 优化正则表达式模式

使用高效的正则表达式模式进行键过滤：

```bash
# 低效：带回溯的复杂模式
--ignore-keys-regex ".*_temp.*|.*_cache.*|.*_debug.*"

# 高效：锚定模式
--ignore-keys-regex "^(.*_temp|.*_cache|.*_debug)$"

# 最高效：简单替代
--ignore-keys-regex "^(_temp|_cache|_debug)_.*$"
```

### 3. 选择适当的输出格式

根据用例选择输出格式：

```bash
# 用于人类阅读 - 最快
diffx file1.json file2.json

# 用于自动处理 - 中等
diffx file1.json file2.json --output json

# 用于遗留工具集成 - 最慢
diffx file1.json file2.json --output unified
```

### 4. 批处理优化

高效处理多个文件：

```bash
# 顺序处理（慢）
for file in *.json; do
  diffx "$file" "${file}.backup"
done

# 并行处理（快）
find . -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup || echo "Diff in {}"'
```

### 5. 内存高效的大文件处理

对于非常大的文件，使用流友好的方法：

```bash
# 分段处理大文件
diffx huge1.json huge2.json --path "section1" > diff_section1.json &
diffx huge1.json huge2.json --path "section2" > diff_section2.json &
diffx huge1.json huge2.json --path "section3" > diff_section3.json &
wait
```

## 内存管理

### 内存使用模式

`diffx` 内存使用遵循以下模式：

1. **线性扩展**与文件大小
2. **峰值使用**在解析阶段
3. **降低使用**在比较阶段
4. **输出序列化**可能导致次峰值

### 内存优化技巧

#### 1. 分别处理文件
```bash
# 高内存使用 - 同时加载两个文件
diffx very_large1.json very_large2.json

# 较低内存使用 - 分块处理
diffx very_large1.json very_large2.json --path "chunk1"
diffx very_large1.json very_large2.json --path "chunk2"
```

#### 2. 使用适当的数据类型
```bash
# 对数值比较更节省内存
diffx data1.json data2.json --epsilon 0.001

# 效率较低 - 数字的精确字符串比较
diffx data1.json data2.json
```

#### 3. 最小化输出大小
```bash
# 大输出 - 包含所有上下文
diffx file1.json file2.json --output json

# 小输出 - CLI 格式更紧凑
diffx file1.json file2.json
```

### 内存监控

在大型操作期间监控内存使用：

```bash
# 监控内存使用
/usr/bin/time -v diffx large1.json large2.json

# 使用 valgrind 进行内存分析（用于调试）
valgrind --tool=massif diffx file1.json file2.json
```

## 大文件处理

### 大文件策略

#### 1. 结构化细分
```bash
# 而不是完全比较 100MB 文件
diffx huge1.json huge2.json

# 按逻辑部分分解
diffx huge1.json huge2.json --path "users"
diffx huge1.json huge2.json --path "products"
diffx huge1.json huge2.json --path "orders"
```

#### 2. 渐进式过滤
```bash
# 步骤 1：识别变更的部分
diffx config1.json config2.json --output json | jq '.[] | .Added[0] // .Modified[0] // .Removed[0]' | cut -d. -f1 | sort -u

# 步骤 2：深入研究变更的部分
diffx config1.json config2.json --path "database"
diffx config1.json config2.json --path "services"
```

#### 3. 采样策略
```bash
# 对于非常大的数据集，首先比较样本
head -n 1000 large1.jsonl > sample1.json
head -n 1000 large2.jsonl > sample2.json
diffx sample1.json sample2.json --array-id-key "id"
```

### 大文件最佳实践

1. **使用路径过滤**聚焦在相关部分
2. **尽早应用正则表达式过滤**以减少数据大小
3. **尽可能并行处理**
4. **在处理期间监控内存使用**
5. **考虑文件分割**用于极大的数据集

## 批处理

### 并行目录处理

优化目录比较：

```bash
# 高效并行处理
find dir1/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} bash -c '
    file2="dir2/${1#dir1/}"
    if [[ -f "$file2" ]]; then
      diffx "$1" "$file2" --output json > "diff_$(basename "$1" .json).json"
    fi
  ' bash {}
```

### 批量配置管理

处理多个环境配置：

```bash
#!/bin/bash
# batch_config_compare.sh

ENVIRONMENTS=("dev" "staging" "prod")
BASE="prod"

for env in "${ENVIRONMENTS[@]}"; do
  if [[ "$env" != "$BASE" ]]; then
    echo "比较 $env 与 $BASE..."
    
    # 并行处理不同配置类型
    {
      diffx "configs/$env/app.json" "configs/$BASE/app.json" \
        --ignore-keys-regex "^(host|port|password)" \
        --output json > "diff_${env}_app.json"
    } &
    
    {
      diffx "configs/$env/db.json" "configs/$BASE/db.json" \
        --ignore-keys-regex "^(connection_string|credentials)" \
        --output json > "diff_${env}_db.json"
    } &
    
    wait  # 等待并行进程完成
  fi
done
```

### 管道集成

优化 CI/CD 管道使用：

```bash
#!/bin/bash
# 优化的 CI 管道差异检查

# 缓存频繁使用的基础配置
if [[ ! -f "baseline_config.json" ]] || [[ $(find baseline_config.json -mtime +1) ]]; then
  curl -s "$CONFIG_SOURCE" > baseline_config.json
fi

# 快速检查 - 仅在检测到变更时进行详细差异
if ! diffx baseline_config.json current_config.json >/dev/null 2>&1; then
  # 仅在需要时进行详细分析
  diffx baseline_config.json current_config.json \
    --ignore-keys-regex "^(timestamp|build_id|deployment_time)" \
    --output json > detailed_diff.json
fi
```

## 性能监控

### 内置性能指标

在应用程序中监控 diffx 性能：

```bash
# 时间测量
time diffx file1.json file2.json

# 详细系统指标
/usr/bin/time -v diffx file1.json file2.json
```

### 基准测试脚本

为您的用例创建自定义基准测试：

```bash
#!/bin/bash
# benchmark_diffx.sh

ITERATIONS=10
FILES=("small.json" "medium.json" "large.json")

for file in "${FILES[@]}"; do
  echo "基准测试 $file..."
  
  total_time=0
  for i in $(seq 1 $ITERATIONS); do
    start_time=$(date +%s%3N)
    diffx "$file" "${file}.backup" >/dev/null
    end_time=$(date +%s%3N)
    
    duration=$((end_time - start_time))
    total_time=$((total_time + duration))
  done
  
  avg_time=$((total_time / ITERATIONS))
  echo "$file 的平均时间：${avg_time}ms"
done
```

### 性能回归测试

在 CI 中包含性能测试：

```bash
# performance_test.sh
#!/bin/bash

BASELINE_TIME=100  # 毫秒
CURRENT_TIME=$(time diffx test_file.json test_file.backup 2>&1 | grep real | cut -d' ' -f2)

if [[ $(echo "$CURRENT_TIME > $BASELINE_TIME * 1.5" | bc) -eq 1 ]]; then
  echo "检测到性能回归！"
  echo "当前：${CURRENT_TIME}ms，基线：${BASELINE_TIME}ms"
  exit 1
fi
```

## 故障排除

### 常见性能问题

#### 1. 正则表达式处理慢
**问题**：复杂的正则表达式模式导致减速
```bash
# 有问题的模式
--ignore-keys-regex ".*_(temp|cache|debug).*"
```

**解决方案**：使用锚定的特定模式
```bash
# 优化的模式
--ignore-keys-regex "^[^_]*_(temp|cache|debug)_[^_]*$"
```

#### 2. 内存耗尽
**问题**：大文件内存不足
```bash
# 错误：内存分配失败
diffx huge1.json huge2.json
```

**解决方案**：使用路径过滤或分块处理
```bash
# 分成可管理的块处理
diffx huge1.json huge2.json --path "section1"
diffx huge1.json huge2.json --path "section2"
```

#### 3. 数组处理慢
**问题**：大数组处理时间过长
```bash
# 对没有 ID 的大数组很慢
diffx users1.json users2.json
```

**解决方案**：在可用时使用数组 ID 键
```bash
# 使用 ID 跟踪快得多
diffx users1.json users2.json --array-id-key "id"
```

### 性能调试

#### 启用详细输出
```bash
# 检查 diffx 正在处理什么
diffx file1.json file2.json --help
```

#### 分析内存使用
```bash
# 监控内存模式
valgrind --tool=massif diffx large1.json large2.json
ms_print massif.out.<pid>
```

#### CPU 分析
```bash
# 分析 CPU 使用
perf record diffx large1.json large2.json
perf report
```

### 优化检查清单

在报告性能问题之前：

- [ ] **路径过滤**：您是否仅比较必要的部分？
- [ ] **正则表达式优化**：正则表达式模式是否锚定和特定？
- [ ] **文件大小**：比较是否适合文件大小？
- [ ] **内存可用性**：您是否有足够的 RAM？
- [ ] **输出格式**：您是否使用最高效的输出格式？
- [ ] **数组处理**：您是否在适用的地方使用数组 ID 键？
- [ ] **并行处理**：您是否利用可用的 CPU 核心？

## 性能最佳实践总结

1. **尽早和经常过滤** - 使用 `--path` 和 `--ignore-keys-regex`
2. **选择适当的输出格式** - CLI 用于显示，JSON 用于处理
3. **利用并行处理** - 为批量操作使用多个核心
4. **监控资源使用** - 观察内存和 CPU 利用率
5. **定期分析** - 基准测试您的特定用例
6. **优化正则表达式模式** - 使用锚定的特定模式
7. **考虑文件组织** - 构造数据以便高效访问
8. **使用数组 ID 键** - 启用高效的数组元素跟踪

这些优化策略应该帮助您在各种用例和数据大小中实现 `diffx` 的最佳性能。
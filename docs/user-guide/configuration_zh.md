# 配置指南

本指南介绍如何配置 `diffx` 以适应您的特定需求和工作流程。

## 配置文件概述

`diffx` 支持一个可选的配置文件，允许您设置默认值，避免重复输入常用选项。配置文件使用 TOML 格式，为结构化数据工具提供了清晰且人类可读的配置语法。

### 配置文件位置

`diffx` 按以下优先级顺序查找配置文件：

1. **环境变量指定**: `DIFFX_CONFIG_PATH` 环境变量指定的路径
2. **用户配置目录**: `~/.config/diffx/config.toml`
3. **Windows 用户**: `%APPDATA%\diffx\config.toml`

### 创建配置文件

```bash
# 创建配置目录
mkdir -p ~/.config/diffx

# 创建配置文件
touch ~/.config/diffx/config.toml
```

## 配置选项参考

### 基本选项

```toml
# 默认输出格式
# 可选值: "cli", "json", "yaml", "unified"
output = "cli"

# 默认文件格式
# 可选值: "auto", "json", "yaml", "toml", "xml", "ini", "csv"
format = "auto"

# 在输出中启用颜色
colors = true

# 目录比较的默认递归模式
recursive = false
```

### 差异过滤选项

```toml
# 正则表达式忽略匹配的键
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"

# 默认数组 ID 键（用于数组元素跟踪）
array_id_key = "id"

# 浮点数比较的默认 epsilon 值
epsilon = 0.001

# 仅显示指定路径的差异
path = ""
```

### 性能选项

```toml
# 启用内存优化（等同于 --optimize 标志）
use_memory_optimization = false

# 大数据处理的批处理大小
batch_size = 1000
```

## 完整配置示例

### 开发环境配置

```toml
# ~/.config/diffx/config.toml
# 开发环境的 diffx 配置

# 默认输出设置
output = "cli"
format = "auto"
colors = true

# 常见的忽略模式
ignore_keys_regex = "^(timestamp|_.*|created_at|updated_at|build_time|version)$"

# 用户数据的数组跟踪
array_id_key = "id"

# 数值比较容差
epsilon = 0.001

# 开发期间启用递归
recursive = true

# 性能优化（大项目）
use_memory_optimization = false
batch_size = 1000
```

### 生产环境配置

```toml
# 生产环境配置
output = "json"  # 机器可读输出
colors = false   # 在日志中禁用颜色

# 严格的比较（较小的 epsilon）
epsilon = 0.0001

# 生产特定的忽略模式
ignore_keys_regex = "^(deployment_id|instance_id|pod_name|container_id)$"

# 启用大数据优化
use_memory_optimization = true
batch_size = 5000

# 递归目录扫描
recursive = true
```

### CI/CD 配置

```toml
# CI/CD 管道配置
output = "json"
format = "auto"
colors = false

# CI 特定的忽略模式
ignore_keys_regex = "^(build_number|ci_run_id|timestamp|git_sha)$"

# 性能优化以适应 CI 限制
use_memory_optimization = true
batch_size = 2000

# 保守的浮点比较
epsilon = 0.001
```

## 环境变量

除了配置文件，`diffx` 还识别环境变量：

### 配置路径变量

```bash
# 指定自定义配置文件位置
export DIFFX_CONFIG_PATH="/path/to/custom/config.toml"
```

### 性能变量

```bash
# 设置最大内存使用量（如果支持）
export DIFFX_MAX_MEMORY="2GB"

# 覆盖默认批处理大小
export DIFFX_BATCH_SIZE="5000"

# 启用调试输出
export DIFFX_DEBUG="true"

# 强制禁用颜色
export NO_COLOR="1"
```

### 与 CLI 选项的优先级

选项按以下优先级应用（最高到最低）：

1. **命令行参数** (最高优先级)
2. **环境变量**
3. **配置文件**
4. **默认值** (最低优先级)

示例：
```bash
# 即使配置文件设置 colors = true，这也会禁用颜色
diffx file1.json file2.json --no-colors

# 环境变量覆盖配置文件
export DIFFX_OUTPUT="yaml"
diffx file1.json file2.json  # 将使用 YAML 输出
```

## 特定用例配置

### 微服务配置管理

```toml
# 微服务环境配置
output = "json"
recursive = true

# 忽略服务发现和部署特定字段
ignore_keys_regex = "^(service_discovery|health_check|load_balancer|replica_count)$"

# 通过服务 ID 跟踪服务
array_id_key = "service_id"

# 网络延迟容差
epsilon = 0.01

# 大型分布式系统优化
use_memory_optimization = true
batch_size = 3000
```

### 数据科学工作流

```toml
# 数据分析配置
output = "yaml"
colors = true

# 忽略实验元数据
ignore_keys_regex = "^(experiment_id|run_timestamp|random_seed|model_version)$"

# 通过样本 ID 跟踪数据点
array_id_key = "sample_id"

# 科学计算的宽松容差
epsilon = 0.001

# 大数据集优化
use_memory_optimization = true
batch_size = 10000
```

### 基础设施即代码

```toml
# Terraform/Ansible 配置
output = "cli"
recursive = true

# 忽略基础设施特定字段
ignore_keys_regex = "^(terraform_version|last_modified|resource_id|availability_zone)$"

# 通过资源名称跟踪资源
array_id_key = "name"

# 基础设施度量的容差
epsilon = 0.1

# 保守的批处理（稳定性）
use_memory_optimization = false
batch_size = 500
```

## 团队配置

### 共享团队配置

为了确保团队的一致性，请将配置文件提交到版本控制：

```bash
# 项目根目录
mkdir -p .config/diffx
```

`.config/diffx/config.toml`:
```toml
# 团队 diffx 配置
# 在您的本地配置中复制此文件到 ~/.config/diffx/config.toml

output = "cli"
colors = true
recursive = true

# 项目特定的忽略模式
ignore_keys_regex = "^(build_id|commit_sha|deployment_time)$"

# 项目标准
array_id_key = "id"
epsilon = 0.001

# 团队标准的性能设置
use_memory_optimization = false
batch_size = 1000
```

### 项目本地配置

使用环境变量为特定项目加载配置：

```bash
# 在项目目录中
export DIFFX_CONFIG_PATH="$(pwd)/.config/diffx/config.toml"

# 或创建包装脚本
echo '#!/bin/bash
export DIFFX_CONFIG_PATH="$(pwd)/.config/diffx/config.toml"
diffx "$@"' > diffx-project
chmod +x diffx-project
```

## 验证配置

### 检查活动配置

虽然 diffx 没有内置的配置显示命令，但您可以通过运行测试来验证设置：

```bash
# 创建测试文件
echo '{"test": 1, "timestamp": "2023-01-01"}' > test1.json
echo '{"test": 2, "timestamp": "2023-01-02"}' > test2.json

# 测试您的配置
diffx test1.json test2.json

# 清理
rm test1.json test2.json
```

### 调试配置

如果配置没有按预期工作：

```bash
# 检查配置文件语法
# TOML 验证器（如果可用）
toml-lint ~/.config/diffx/config.toml

# 使用显式选项测试
diffx file1.json file2.json --output json --ignore-keys-regex "timestamp"
```

## 配置模板

### 基本模板

```toml
# 基本 diffx 配置模板
# 复制到 ~/.config/diffx/config.toml 并根据需要修改

[general]
output = "cli"          # 输出格式: cli, json, yaml, unified
format = "auto"         # 文件格式: auto, json, yaml, toml, xml, ini, csv
colors = true           # 启用彩色输出
recursive = false       # 默认目录递归

[filtering]
ignore_keys_regex = ""  # 要忽略的键的正则表达式
array_id_key = ""       # 数组元素跟踪的默认 ID 键
epsilon = 0.001         # 浮点比较容差
path = ""               # 默认路径过滤

[performance]
use_memory_optimization = false  # 启用内存优化
batch_size = 1000               # 批处理大小
```

### 高级模板

```toml
# 高级 diffx 配置模板
# 包含所有可用选项和示例

# 基本输出设置
output = "cli"
format = "auto"
colors = true
recursive = false

# 高级过滤
ignore_keys_regex = "^(timestamp|_.*|created_at|updated_at|id|guid)$"
array_id_key = "id"
epsilon = 0.001
path = ""

# 性能调优
use_memory_optimization = false
batch_size = 1000

# 特定用例示例（注释掉）
# 
# 微服务配置:
# ignore_keys_regex = "^(service_discovery|health_check|replica_count)$"
# array_id_key = "service_id"
#
# 数据科学工作流:
# epsilon = 0.01
# array_id_key = "sample_id"
# use_memory_optimization = true
# batch_size = 10000
#
# CI/CD 管道:
# output = "json"
# colors = false
# ignore_keys_regex = "^(build_number|ci_run_id|git_sha)$"
```

## 故障排除

### 常见配置问题

**配置文件未被加载:**
```bash
# 检查文件是否存在
ls -la ~/.config/diffx/config.toml

# 验证 TOML 语法
# 某些系统上可用的 toml 检查器
python3 -c "import toml; toml.load('~/.config/diffx/config.toml')"
```

**选项未生效:**
```bash
# 检查选项优先级
# CLI 参数覆盖配置文件
diffx file1.json file2.json --output json  # 强制 JSON 输出
```

**性能问题:**
```bash
# 临时启用优化
diffx large1.json large2.json --optimize

# 或通过环境变量
export DIFFX_BATCH_SIZE="5000"
diffx large1.json large2.json
```

### 验证设置

创建一个简单的测试脚本来验证您的配置：

```bash
#!/bin/bash
# test-config.sh

echo "Testing diffx configuration..."

# 创建测试文件
echo '{"name": "test", "version": "1.0", "timestamp": "2023-01-01"}' > test1.json
echo '{"name": "test", "version": "1.1", "timestamp": "2023-01-02"}' > test2.json

echo "Running diffx with current configuration:"
diffx test1.json test2.json

echo "Expected behavior based on ignore_keys_regex:"
echo "- Should show version change"
echo "- Should ignore timestamp if configured"

# 清理
rm test1.json test2.json
```

## 下一步

- 查看 [实际示例](examples_zh.md) 了解配置在真实场景中的使用
- 阅读 [CLI 参考](../reference/cli-reference_zh.md) 了解所有可用选项
- 探索 [集成指南](../guides/integrations_zh.md) 了解在自动化工作流程中使用 diffx
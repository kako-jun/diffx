# CLI 参考

`diffx` 命令行界面的完整参考文档。

## 概要

```
diffx [OPTIONS] <INPUT1> <INPUT2>
```

## 描述

`diffx` 是一个用于结构化数据文件语义比较的命令行工具。与传统的基于文本的 diff 工具不同，`diffx` 理解数据的结构和意义，专注于实际变更而非格式差异。

## 参数

### `<INPUT1>`
- **类型**: 文件路径、目录路径或标准输入 `-`
- **必需**: 是
- **描述**: 要比较的第一个输入

### `<INPUT2>`
- **类型**: 文件路径、目录路径或标准输入 `-`
- **必需**: 是
- **描述**: 要比较的第二个输入

**标准输入支持:**
- **一个标准输入，一个文件**: `diffx - file.json` 或 `diffx file.json -`
- **两个都来自标准输入**: `diffx - -` (从标准输入读取两个数据集)
  - **JSON**: 由换行符分隔或连接的两个 JSON 对象
  - **YAML**: 由 `---` 分隔的两个 YAML 文档

**示例:**
```bash
# 比较两个文件
diffx config.json config.new.json

# 标准输入和文件
cat config.json | diffx - config.new.json

# 两者都从标准输入（管道两者）
echo '{"old": "data"}
{"new": "data"}' | diffx - -

# 从标准输入读取两个 YAML 文档
echo 'name: Alice
age: 25
---
name: Bob
age: 30' | diffx - - --format yaml

# 目录比较（自动递归检测）
diffx config_dir1/ config_dir2/

# API 响应比较（通过标准输入）
(curl -s https://api.example.com/v1/config; echo; curl -s https://api.example.com/v2/config) | diffx - -
```

## 选项

### 格式选项

#### `-f, --format <FORMAT>`
- **类型**: 字符串
- **默认**: 从文件扩展名自动检测
- **值**: `json`, `yaml`, `toml`, `xml`, `ini`, `csv`
- **描述**: 强制指定特定的输入文件格式

**示例:**
```bash
# 强制 JSON 解释
diffx --format json file1.txt file2.txt

# 强制 YAML 解释
diffx -f yaml config1 config2
```

**自动检测映射:**
- `.json` → `json`
- `.yaml`, `.yml` → `yaml`
- `.toml` → `toml`
- `.xml` → `xml`
- `.ini`, `.cfg`, `.conf` → `ini`
- `.csv` → `csv`

### 输出选项

#### `-o, --output <FORMAT>`
- **类型**: 字符串
- **默认**: `diffx`（人类可读的 diffx 格式）
- **值**: `diffx`, `json`, `yaml`, `unified`
- **描述**: 差异的输出格式

**diffx 格式（默认）:**
```bash
diffx config.json config.new.json
# 输出:
# + database.port: 5432
# ~ version: "1.0" -> "1.1"
# - cache.enabled: true
```

**JSON 输出:**
```bash
diffx config.json config.new.json --output json
# 输出:
# [
#   {"Added": ["database.port", 5432]},
#   {"Modified": ["version", "1.0", "1.1"]},
#   {"Removed": ["cache.enabled", true]}
# ]
```

**YAML 输出:**
```bash
diffx config.json config.new.json --output yaml
# 输出:
# - Added:
#   - database.port
#   - 5432
# - Modified:
#   - version
#   - "1.0"
#   - "1.1"
```

**Unified 输出:**
```bash
diffx config.json config.new.json --output unified
# 输出: 传统 diff 样式格式
```

### 过滤选项

#### `--path <PATH>`
- **类型**: 字符串
- **默认**: 无（比较整个结构）
- **描述**: 将差异过滤到数据结构中的特定路径

**路径语法:**
- 对象键: `database.host`
- 数组索引: `users[0]`
- 嵌套路径: `config.database.connection.host`
- 复杂路径: `services.web.env[0].name`

**示例:**
```bash
# 仅比较数据库配置
diffx config.json config.new.json --path "database"

# 比较特定数组元素
diffx config.json config.new.json --path "users[0]"

# 深层嵌套路径
diffx config.json config.new.json --path "services.web.environment.variables"
```

#### `--ignore-keys-regex <PATTERN>`
- **类型**: 正则表达式字符串
- **默认**: 无
- **描述**: 忽略匹配指定正则表达式的键

**常见模式:**
```bash
# 忽略时间戳字段
diffx file1.json file2.json --ignore-keys-regex "^(timestamp|createdAt|updatedAt)$"

# 忽略内部字段（以下划线开头）
diffx file1.json file2.json --ignore-keys-regex "^_.*"

# 忽略多个模式
diffx file1.json file2.json --ignore-keys-regex "^(id|timestamp|_.*|temp_.*)$"

# 忽略版本相关字段
diffx file1.json file2.json --ignore-keys-regex "(version|buildNumber|revision)"
```

**正则表达式示例:**
- `^timestamp$` - "timestamp" 的精确匹配
- `^_.*` - 以下划线开头的字段
- `.*_temp$` - 以 "_temp" 结尾的字段
- `^(id|uid|pk)$` - 匹配任何一个: id, uid, pk
- `(?i)password` - "password" 的不区分大小写匹配

### 比较选项

#### `--epsilon <VALUE>`
- **类型**: 浮点数
- **默认**: `0.0`（精确比较）
- **描述**: 浮点数比较的容差

**示例:**
```bash
# 允许浮点数的小差异
diffx metrics.json metrics.new.json --epsilon 0.001

# 科学数据的更宽松容差
diffx measurements.json measurements.new.json --epsilon 0.01

# 非常严格的比较
diffx financial.json financial.new.json --epsilon 0.000001
```

**使用场景:**
- 具有测量精度的科学数据
- 有舍入差异的财务计算
- 有小变动的性能指标
- 有浮点数伪影的转换数据

#### `--array-id-key <KEY>`
- **类型**: 字符串
- **默认**: 无（位置比较）
- **描述**: 用于识别和跟踪数组元素的键

**示例:**
```bash
# 通过 ID 跟踪用户
diffx users.json users.updated.json --array-id-key "id"

# 通过 SKU 跟踪产品
diffx inventory.json inventory.new.json --array-id-key "sku"

# 通过主键跟踪数据库记录
diffx records.json records.new.json --array-id-key "primary_key"
```

**无 ID 跟踪:**
```json
// 数组比较显示位置变化
// 旧: [{"name": "Alice"}, {"name": "Bob"}]
// 新: [{"name": "Bob"}, {"name": "Alice"}]
// 结果: 所有元素都显示为已更改
```

**有 ID 跟踪:**
```json
// 旧: [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]  
// 新: [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
// 结果: 检测到无变化（相同元素，不同顺序）
```

#### `--ignore-whitespace`
- **类型**: 布尔标志
- **默认**: False
- **描述**: 忽略字符串值中的空白差异

**示例:**
```bash
# 具有不同空白的文件
echo '{"text": "Hello  World"}' > file1.json
echo '{"text": "Hello World"}' > file2.json

# 正常比较显示差异
diffx file1.json file2.json
# 输出: ~ text: "Hello  World" -> "Hello World"

# 忽略空白 - 不报告差异
diffx file1.json file2.json --ignore-whitespace
# 输出: （无差异）
```

**使用场景:**
- 空白不一致的配置文件
- 从不同系统导出的数据
- 引入额外空格的手动编辑
- 标准化与原始文本数据

#### `--ignore-case`
- **类型**: 布尔标志
- **默认**: False
- **描述**: 忽略字符串值中的大小写差异

**示例:**
```bash
# 具有不同大小写的文件
echo '{"status": "Active"}' > file1.json
echo '{"status": "ACTIVE"}' > file2.json

# 正常比较显示差异
diffx file1.json file2.json
# 输出: ~ status: "Active" -> "ACTIVE"

# 忽略大小写 - 不报告差异
diffx file1.json file2.json --ignore-case
# 输出: （无差异）
```

**使用场景:**
- 大小写变化的用户输入数据
- 遗留系统迁移
- 不区分大小写的配置值
- 数据标准化任务

**组合选项:**
```bash
# 处理空白和大小写差异
diffx config.json config.new.json --ignore-whitespace --ignore-case

# 多选项的复杂示例
diffx data.yaml data.updated.yaml \
  --ignore-case \
  --ignore-whitespace \
  --epsilon 0.001 \
  --ignore-keys-regex "^(timestamp|version)$"
```

### 输出控制选项

#### `--context <N>`
- **类型**: 整数
- **默认**: 无（显示所有上下文）
- **描述**: 在 unified 输出格式中显示差异周围的 N 行上下文

**示例:**
```bash
# 在变更周围显示 2 行上下文
diffx config.json config.new.json --output unified --context 2

# 仅显示变更行（无上下文）
diffx config.json config.new.json --output unified --context 0

# 默认行为（所有上下文）
diffx config.json config.new.json --output unified
```

**带上下文的示例输出:**
```diff
# --context 2
  "database": {
    "host": "localhost",
-   "port": 5432
+   "port": 5433
  },
  "cache": {

# --context 0  
-   "port": 5432
+   "port": 5433
```

#### `-q, --quiet`
- **类型**: 布尔标志
- **默认**: False
- **描述**: 抑制正常输出；仅返回退出状态

**示例:**
```bash
# 检查文件是否有差异（用于脚本）
diffx config.json config.new.json --quiet
echo $?  # 0 = 无差异, 1 = 发现差异, 2 = 错误

# 在 shell 脚本中使用
if diffx config.json backup.json --quiet; then
    echo "文件相同"
else
    echo "文件不同"
fi

# 与其他选项结合
diffx large.json large.new.json --quiet --ignore-whitespace
```

**退出代码:**
- `0`: 未发现差异
- `1`: 发现差异
- `2`: 发生错误（无效文件、格式错误等）

#### `--brief`
- **类型**: 布尔标志
- **默认**: False
- **描述**: 仅报告文件名，不报告差异（类似于 `diff --brief`）

**示例:**
```bash
# 仅报告文件是否不同
diffx config.json config.new.json --brief
# 输出: Files config.json and config.new.json differ

# 与目录比较一起使用（自动递归）
diffx configs/ configs.backup/ --brief
# 输出: Files configs/app.json and configs.backup/app.json differ

# 与过滤结合
diffx data.json data.new.json --brief --ignore-keys-regex "^timestamp$"
```

**使用场景:**
- 批处理脚本
- 快速文件比较检查
- 自动化测试流水线
- 文件同步验证

#### `-v, --verbose`
- **类型**: 布尔标志
- **默认**: False
- **描述**: 显示包括性能指标、配置详情和处理统计在内的全面诊断信息

**示例:**
```bash
# 基本详细输出
diffx config.json config.new.json --verbose
# 输出包括:
# Input file information: 
#   Input 1 size: 245 bytes
#   Input 2 size: 267 bytes
# Parse time: 15.2µs
# Diff computation time: 23.8µs
# Total differences found: 3
# Performance summary:
#   Total processing time: 125.4µs
#   Memory optimization: disabled

# 带过滤选项的详细信息
diffx data.json data.new.json --verbose --ignore-keys-regex "timestamp" --epsilon 0.1
# 额外输出:
# Key filtering configuration:
#   Regex pattern: timestamp
# Numerical tolerance configuration:
#   Epsilon value: 0.1

# 详细目录比较（自动递归）
diffx configs/ configs.backup/ --verbose
# 额外输出:
# Directory scan results:
#   Files in configs/: 12
#   Files in configs.backup/: 11
#   Total files to compare: 12
# Directory comparison summary:
#   Files compared: 11
#   Files only in one directory: 1
#   Differences found: Yes
```

**详细信息类别:**

1. **性能指标**
   - 文件大小和内存使用
   - 解析时间、差异计算时间
   - 总处理时间
   - 内存优化状态

2. **配置详情**
   - 活动过滤模式（正则表达式、epsilon、数组 ID 键）
   - 路径过滤设置
   - 上下文显示配置

3. **处理统计**
   - 过滤前后总差异数
   - 目录扫描结果
   - 比较效果指标

4. **诊断输出**
   - 优化决策
   - 处理批次信息
   - 错误上下文和故障排除数据

**使用场景:**
- 性能分析和优化
- 慢比较故障排除
- 理解过滤器效果
- 调试配置问题
- CI/CD 流水线诊断
- 支持和维护任务

#### `--no-color`
- **类型**: 布尔标志
- **默认**: False（启用彩色输出）
- **描述**: 禁用彩色输出以更好地兼容脚本、流水线或不支持 ANSI 颜色的终端

**示例:**
```bash
# 无颜色的基本使用
diffx config.json config.new.json --no-color
# 输出将是无颜色格式的纯文本

# 在 CI/CD 流水线中使用
diffx deploy.yaml deploy.new.yaml --no-color --output json > diff_report.json

# 与其他输出选项结合
diffx large.json large.new.json --no-color --brief --quiet

# 无颜色的目录比较（自动递归）
diffx configs/ configs.backup/ --no-color
```

**使用场景:**
- CI/CD 流水线集成，其中颜色代码干扰日志解析
- 处理 diffx 输出的自动化脚本
- 不需要 ANSI 代码的文本文件输出重定向
- 不支持颜色的终端环境
- 屏幕阅读器的可访问性合规性
- 为文档创建干净的文本报告

### 目录选项

#### **自动目录检测**
- **类型**: 自动功能（无需选项）
- **默认**: 提供目录路径时启用
- **描述**: 当提供目录路径时，diffx 自动递归比较子目录中的所有文件

**示例:**
```bash
# 目录比较（自动递归处理）
diffx config_dir1/ config_dir2/
# 自动检测目录并递归比较所有文件
# 输出:
# --- Comparing config.json ---
# ~ version: "1.0" -> "1.1"
# --- Comparing subdir/nested.json ---
# ~ data: "old" -> "new"

# 带输出格式的目录比较（自动递归）
diffx environments/dev/ environments/prod/ --output json

# 带过滤的目录比较（自动递归）
diffx configs/ configs.backup/ --ignore-keys-regex "^(timestamp|version)$"
```

**现代目录行为:**

**提供目录路径时:**
- 自动检测输入是目录
- 递归比较两个目录树中的所有文件
- 在输出中保持目录结构
- 无需手动标志 - 智能路径基础检测

**提供文件路径时:**
- 正常文件比较行为
- 无目录处理

**混合文件/目录路径:**
- 返回清晰错误: "Cannot compare file with directory"

**目录比较功能:**
- 跳过在两个目录中都不存在的文件
- 尊重每个文件的格式自动检测
- 报告仅存在于一个目录中的文件
- 默认完全递归遍历

### 性能选项

#### 自动优化
- **类型**: 自动功能
- **默认**: 对 >1MB 的文件启用
- **描述**: 对大文件和数据结构自动启用内存高效处理

**自动检测行为:**
- 文件 ≤1MB: 标准模式（快速、无限内存）
- 文件 >1MB: 优化模式（内存高效、批处理）
- 无需手动配置 - 优化完全透明

**优化功能:**
- 基于文件大小的自动检测
- 大数据集的内存高效处理
- 深层嵌套结构的批处理
- 无论模式如何都保持相同输出

**示例:**
```bash
# 自动检测（始终启用）
diffx config.json config.new.json
# 小文件使用标准模式，大文件使用优化

# 大文件自动使用优化
diffx massive_db.json massive_db.new.json --array-id-key "id" --path "users"
# 大文件自动使用优化模式

# 所有其他选项与优化透明工作
diffx complex_data.json complex_data.v2.json --ignore-keys-regex "^timestamp$"
# 根据需要自动应用优化
```

**性能行为:**
```bash
# 小文件（<1MB） - 自动标准模式
diffx config.json config.new.json
# 快速处理，无限内存使用

# 大文件（>1MB） - 自动优化模式  
diffx large_dataset.json large_dataset.v2.json
# 内存高效、批处理

# 复杂嵌套结构 - 自动优化
diffx deep_nested.json deep_nested.v2.json
# 基于数据特征的透明优化
```

### 信息选项

#### `-h, --help`
- **类型**: 布尔标志
- **描述**: 打印帮助信息并退出

#### `-V, --version`
- **类型**: 布尔标志
- **描述**: 打印版本信息并退出

**示例:**
```bash
# 显示帮助
diffx --help
diffx -h

# 显示版本
diffx --version
diffx -V
```

## 退出代码

`diffx` 使用以下退出代码：

- **0**: 成功，未发现差异
- **1**: 成功，发现差异
- **2**: 命令行参数错误
- **3**: 文件 I/O 错误
- **4**: 解析错误（无效格式）
- **5**: 内部错误

**示例:**
```bash
# 检查文件是否相同
if diffx file1.json file2.json >/dev/null 2>&1; then
    echo "文件相同"
else
    echo "文件不同"
fi

# 捕获退出代码
diffx config.json config.new.json
EXIT_CODE=$?
case $EXIT_CODE in
    0) echo "无差异" ;;
    1) echo "发现差异" ;;
    *) echo "发生错误（代码: $EXIT_CODE）" ;;
esac
```

## 使用模式

### 基本比较

```bash
# 简单文件比较
diffx file1.json file2.json

# 不同格式比较
diffx config.yaml config.toml --format yaml --format toml

# 与标准输入比较
curl -s https://api.example.com/config | diffx - local_config.json
```

### 高级过滤

```bash
# 复杂忽略模式
diffx app.json app.new.json \
  --ignore-keys-regex "^(timestamp|_.*|createdAt|updatedAt|version)$"

# 路径特定比较
diffx large_config.json large_config.new.json \
  --path "database.connections"

# 组合多个选项
diffx users.json users.new.json \
  --array-id-key "user_id" \
  --ignore-keys-regex "^(last_login|session_.*)" \
  --output json
```

### 目录操作

```bash
# Unix diff 兼容目录比较（非递归）
diffx configs/ configs.backup/
# 显示目录中的文件和 "Common subdirectories" 消息

# 递归目录比较
diffx configs/ configs.backup/ --recursive

# 带过滤的目录比较
diffx env/dev/ env/prod/ \
  --recursive \
  --ignore-keys-regex "^(host|port|password)" \
  --output json > env_diff.json
```

### 集成示例

```bash
# Git 集成
git show HEAD~1:config.json > old_config.json
diffx old_config.json config.json --output unified

# CI/CD 流水线
diffx expected_config.json actual_config.json \
  --ignore-keys-regex "^(deployment_time|build_id)" \
  --output json > config_validation.json

# 监控脚本
#!/bin/bash
if ! diffx baseline_config.json current_config.json \
     --ignore-keys-regex "^(timestamp|uptime)" >/dev/null; then
  echo "检测到配置漂移！"
  diffx baseline_config.json current_config.json --output json | \
    notify_alert_system.py
fi
```

## 错误处理

### 常见错误

**文件未找到:**
```bash
$ diffx nonexistent.json config.json
Error: No such file or directory (os error 2)
```

**无效格式:**
```bash
$ diffx invalid.json valid.json
Error: Failed to parse JSON: expected `,` or `}` at line 1 column 15
```

**权限被拒绝:**
```bash
$ diffx protected.json config.json
Error: Permission denied (os error 13)
```

**无效正则表达式:**
```bash
$ diffx file1.json file2.json --ignore-keys-regex "[invalid"
Error: Invalid regular expression: unclosed character class
```

### 调试

```bash
# 验证格式检测
diffx --format json file1.txt file2.txt
```

## 性能考虑

### 大文件

```bash
# 对大文件使用路径过滤
diffx huge1.json huge2.json --path "critical_section"

# 忽略非必要数据
diffx large1.json large2.json --ignore-keys-regex "logs|debug|metadata"
```

### 批处理

```bash
# 多文件并行处理
find configs/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup || echo "Diff in {}"'
```

### 内存使用

对于非常大的文件，考虑：
- 使用 `--path` 专注于特定部分
- 使用 `--ignore-keys-regex` 过滤大的无关部分
- 如果可能，将文件分块处理

## 按用例分类的示例

### 配置管理
```bash
# 环境比较
diffx prod.json staging.json --ignore-keys-regex "^(host|port|secret_.*)"

# Kubernetes 清单
diffx deployment.yaml deployment.new.yaml --ignore-keys-regex "^metadata\\.(creation.*|resource.*)"
```

### API 测试
```bash
# 响应验证
diffx expected_response.json actual_response.json --ignore-keys-regex "^(timestamp|request_id)"

# 模式比较
diffx api_v1_schema.json api_v2_schema.json --path "definitions"
```

### 数据处理
```bash
# ETL 验证
diffx input_data.json output_data.json --array-id-key "record_id" --epsilon 0.001

# 数据库导出比较
diffx export1.json export2.json --array-id-key "id" --ignore-keys-regex "^(updated_at|sync_time)"
```

### 安全审计
```bash
# 策略比较
diffx security_policy.json security_policy.new.json --path "permissions"

# 访问控制验证
diffx rbac.yaml rbac.new.yaml --array-id-key "name"
```

这个全面的 CLI 参考涵盖了所有可用选项，并为有效使用 `diffx` 提供了实用示例。
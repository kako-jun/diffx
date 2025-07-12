# CLI 参考

`diffx` 命令行界面的完整参考文档。

## 概要

```
diffx [OPTIONS] <INPUT1> <INPUT2>
```

## 描述

`diffx` 是一个用于结构化数据文件语义比较的命令行工具。与传统的基于文本的diff工具不同，`diffx` 理解数据的结构和含义，专注于实际变化而非格式差异。

## 参数

### `<INPUT1>`
- **类型**: 文件路径、目录路径或 `-` 表示标准输入
- **必需**: 是
- **描述**: 要比较的第一个输入

### `<INPUT2>`
- **类型**: 文件路径、目录路径或 `-` 表示标准输入  
- **必需**: 是
- **描述**: 要比较的第二个输入

**示例:**
```bash
# 比较两个文件
diffx config.json config.new.json

# 与标准输入比较
cat config.json | diffx - config.new.json

# 比较目录
diffx config_dir1/ config_dir2/
```

## 选项

### 格式选项

#### `-f, --format <FORMAT>`
- **类型**: 字符串
- **默认值**: 从文件扩展名自动检测
- **值**: `json`, `yaml`, `toml`, `xml`, `ini`, `csv`
- **描述**: 强制指定输入文件格式

**示例:**
```bash
# 强制JSON解释
diffx --format json file1.txt file2.txt

# 强制YAML解释
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
- **默认值**: `diffx` (人类可读的diffx格式)
- **值**: `diffx`, `json`, `yaml`, `unified`
- **描述**: 差异的输出格式

**diffx 格式 (默认):**
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
# - Removed:
#   - cache.enabled
#   - true
```

**统一输出 (类似传统diff):**
```bash
diffx config.json config.new.json --output unified
# 输出:
# --- config.json
# +++ config.new.json
# @@ -1,5 +1,6 @@
#  {
# +  "database": {
# +    "port": 5432
# +  },
# -  "version": "1.0"
# +  "version": "1.1"
#  }
```

### 过滤选项

#### `--ignore-keys-regex <PATTERN>`
- **类型**: 正则表达式字符串
- **默认值**: 无
- **描述**: 忽略匹配正则表达式模式的键

**示例:**
```bash
# 忽略时间戳字段
diffx data.json data.new.json --ignore-keys-regex "timestamp"

# 忽略内部字段（以下划线开头）
diffx file1.json file2.json --ignore-keys-regex "^_.*"

# 忽略版本和ID字段
diffx config.json config.new.json --ignore-keys-regex "^(version|id|_id)$"
```

**用例:**
- 忽略自动生成的时间戳
- 跳过调试或内部字段
- 过滤不相关的元数据
- 专注于业务逻辑变化

#### `--path <PATH>`
- **类型**: 字符串
- **默认值**: 无（比较整个结构）
- **描述**: 只比较指定路径下的数据

**示例:**
```bash
# 只比较数据库配置
diffx config.json config.new.json --path "database"

# 比较嵌套路径
diffx data.json data.new.json --path "users.0.profile"

# 比较数组元素
diffx data.json data.new.json --path "services.2"
```

**路径语法:**
- `.` 用于分隔对象键
- 数字用于数组索引
- 路径区分大小写
- 使用引号处理带空格的键

#### `--epsilon <VALUE>`
- **类型**: 浮点数
- **默认值**: 无（精确比较）
- **描述**: 浮点数比较的容差值

**示例:**
```bash
# 忽略小的浮点差异
diffx data1.json data2.json --epsilon 0.001

# 科学记数法的公差
diffx measurements.json measurements.new.json --epsilon 1e-6

# 百分比变化的公差
diffx stats.json stats.new.json --epsilon 0.01
```

**用例:**
- 浮点运算精度差异
- 测量数据的舍入误差
- 百分比或比率的小差异
- 科学计算中的数值容差

#### `--array-id-key <KEY>`
- **类型**: 字符串
- **默认值**: 无（基于位置的比较）
- **描述**: 用于跟踪数组元素的唯一标识符键

**示例:**
```bash
# 使用ID跟踪用户
diffx users.json users.new.json --array-id-key "id"

# 使用SKU跟踪产品
diffx inventory.json inventory.new.json --array-id-key "sku"

# 使用主键跟踪数据库记录
diffx records.json records.new.json --array-id-key "primary_key"
```

**没有ID键的情况:**
```json
// 数组比较显示基于位置的变化
// 旧: [{"name": "Alice"}, {"name": "Bob"}]
// 新: [{"name": "Bob"}, {"name": "Alice"}]
// 结果: 显示所有元素都被修改
```

**使用ID键的情况:**
```json
// 旧: [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]  
// 新: [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
// 结果: 无变化（相同元素，不同顺序）
```

### 比较选项

#### `--ignore-whitespace`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 忽略字符串值中的空白差异

**示例:**
```bash
# 不同空白的文件
echo '{"text": "Hello  World"}' > file1.json
echo '{"text": "Hello World"}' > file2.json

# 正常比较显示差异
diffx file1.json file2.json
# 输出: ~ text: "Hello  World" -> "Hello World"

# 忽略空白比较 - 不报告差异
diffx file1.json file2.json --ignore-whitespace
# 输出: (无差异)
```

**用例:**
- 间距不一致的配置文件
- 从不同系统导出的数据
- 引入额外空格的手动编辑
- 标准化与原始文本数据

#### `--ignore-case`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 忽略字符串值中的大小写差异

**示例:**
```bash
# 不同大小写的文件
echo '{"status": "Active"}' > file1.json
echo '{"status": "ACTIVE"}' > file2.json

# 正常比较显示差异
diffx file1.json file2.json
# 输出: ~ status: "Active" -> "ACTIVE"

# 忽略大小写比较 - 不报告差异
diffx file1.json file2.json --ignore-case
# 输出: (无差异)
```

**用例:**
- 不同大小写的用户输入数据
- 遗留系统迁移
- 不区分大小写的配置值
- 数据标准化任务

**组合选项:**
```bash
# 处理空白和大小写差异
diffx config.json config.new.json --ignore-whitespace --ignore-case

# 多选项复杂示例
diffx data.yaml data.updated.yaml \
  --ignore-case \
  --ignore-whitespace \
  --epsilon 0.001 \
  --ignore-keys-regex "^(timestamp|version)$"
```

### 输出控制选项

#### `--context <N>`
- **类型**: 整数
- **默认值**: 无（显示所有上下文）
- **描述**: 在统一输出格式中显示差异周围的N行上下文

**示例:**
```bash
# 在变化周围显示2行上下文
diffx config.json config.new.json --output unified --context 2

# 只显示变化行（无上下文）
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
- **默认值**: False
- **描述**: 抑制正常输出；只返回退出状态

**示例:**
```bash
# 检查文件是否不同（用于脚本）
diffx config.json config.new.json --quiet
echo $?  # 0 = 无差异, 1 = 发现差异, 2 = 错误

# 在shell脚本中使用
if diffx config.json backup.json --quiet; then
    echo "文件相同"
else
    echo "文件不同"
fi

# 与其他选项组合
diffx large.json large.new.json --quiet --ignore-whitespace
```

**退出代码:**
- `0`: 未发现差异
- `1`: 发现差异
- `2`: 发生错误（无效文件、格式错误等）

#### `--brief`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 只报告文件名，不报告差异（类似于 `diff --brief`）

**示例:**
```bash
# 只报告文件是否不同
diffx config.json config.new.json --brief
# 输出: Files config.json and config.new.json differ

# 用于目录比较
diffx configs/ configs.backup/ --recursive --brief
# 输出: Files configs/app.json and configs.backup/app.json differ

# 与过滤结合
diffx data.json data.new.json --brief --ignore-keys-regex "^timestamp$"
```

**用例:**
- 批处理脚本
- 快速文件比较检查
- 自动化测试管道
- 文件同步验证

#### `-v, --verbose`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 显示包括性能指标、配置详情和处理统计在内的综合诊断信息

**示例:**
```bash
# 基本详细输出
diffx config.json config.new.json --verbose
# 输出包含:
# Input file information: 
#   Input 1 size: 245 bytes
#   Input 2 size: 267 bytes
# Parse time: 15.2µs
# Diff computation time: 23.8µs
# Total differences found: 3
# Performance summary:
#   Total processing time: 125.4µs
#   Memory optimization: disabled

# 详细输出与过滤选项结合
diffx data.json data.new.json --verbose --ignore-keys-regex "timestamp" --epsilon 0.1
# 额外输出:
# Key filtering configuration:
#   Regex pattern: timestamp
# Numerical tolerance configuration:
#   Epsilon value: 0.1

# 目录比较的详细输出
diffx configs/ configs.backup/ --recursive --verbose
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
   - 活动过滤模式（正则表达式、容差、数组ID键）
   - 路径过滤设置
   - 上下文显示配置

3. **处理统计**
   - 过滤前后的差异总数
   - 目录扫描结果
   - 比较效果指标

4. **诊断输出**
   - 优化决策
   - 处理批次信息
   - 错误上下文和故障排除数据

**用例:**
- 性能分析和优化
- 慢速比较故障排除
- 理解过滤器效果
- 调试配置问题
- CI/CD 管道诊断
- 支持和维护任务

### 目录选项

#### `-r, --recursive`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 递归比较目录及其子目录（Unix diff 兼容）

**示例:**
```bash
# 不使用 --recursive 的目录比较（Unix diff 兼容）
# 仅比较目录直下文件，对子目录显示 "Common subdirectories" 消息
diffx config_dir1/ config_dir2/
# 输出:
# Common subdirectories: config_dir1/subdir and config_dir2/subdir
# --- Comparing config.json ---
# ~ version: "1.0" -> "1.1"

# 递归比较 - 包括子目录内的文件
diffx config_dir1/ config_dir2/ --recursive
# 输出:
# --- Comparing config.json ---
# ~ version: "1.0" -> "1.1"
# --- Comparing subdir/nested.json ---
# ~ data: "old" -> "new"

# 带输出格式的递归比较
diffx environments/dev/ environments/prod/ -r --output json

# 带过滤的递归比较
diffx configs/ configs.backup/ -r --ignore-keys-regex "^(timestamp|version)$"
```

**Unix diff 兼容行为:**

**不使用 `--recursive` 标志（默认）:**
- 仅比较指定目录直下的文件
- 对于两个位置都存在的子目录，显示 "Common subdirectories" 消息
- 不比较子目录内的文件
- 与标准 Unix `diff` 命令保持兼容性

**使用 `--recursive` 标志:**
- 递归比较所有文件，包括子目录内的文件
- 在输出中维护目录结构
- 等同于 `diff -r` 的行为

**共同行为:**
- 跳过两个目录中都不存在的文件
- 尊重每个文件的格式自动检测
- 报告仅存在于一个目录中的文件

### 性能选项

#### 自动优化
- **类型**: 自动功能
- **默认值**: >1MB文件启用
- **描述**: 大文件和数据结构自动启用内存高效处理

**自动检测行为:**
- 文件 ≤1MB: 标准模式（快速，无限内存）
- 文件 >1MB: 优化模式（内存高效，批处理）
- 无需手动配置 - 优化完全透明

**优化功能:**
- 基于文件大小自动检测
- 大数据集的内存高效处理
- 深度嵌套结构的批处理
- 无论模式如何都保持相同输出

**示例:**
```bash
# 自动检测（始终启用）
diffx config.json config.new.json
# 小文件使用标准模式，大文件使用优化模式

# 大文件自动使用优化
diffx massive_db.json massive_db.new.json --array-id-key "id" --path "users"
# 大文件自动使用优化模式

# 所有其他选项都与优化透明工作
diffx complex_data.json complex_data.v2.json --ignore-keys-regex "^timestamp$"
# 根据需要自动应用优化
```

**性能行为:**
```bash
# 小文件 (<1MB) - 自动标准模式
diffx config.json config.new.json
# 快速处理，无限内存使用

# 大文件 (>1MB) - 自动优化模式  
diffx large_dataset.json large_dataset.v2.json
# 内存高效，批处理

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

`diffx` 使用以下退出代码:

- **0**: 成功，未发现差异
- **1**: 成功，发现差异
- **2**: 命令行参数错误
- **3**: 文件I/O错误
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
    *) echo "发生错误 (代码: $EXIT_CODE)" ;;
esac
```

## 使用模式

### 基本比较

```bash
# 简单文件比较
diffx config.json config.backup.json

# 不同格式比较
diffx config.yaml config.json

# 标准输入比较
echo '{"key": "value"}' | diffx - reference.json
```

### 高级过滤

```bash
# 忽略元数据字段
diffx data.json data.new.json --ignore-keys-regex "^(created_at|updated_at|id)$"

# 专注于特定部分
diffx large_config.json large_config.new.json --path "database.connections"

# 数值容差比较
diffx measurements.json measurements.new.json --epsilon 0.001
```

### 目录工作流

```bash
# 比较配置目录
diffx env/dev/ env/prod/ --recursive

# 部署验证
diffx current_config/ new_deployment/ -r --output json > changes.json

# 备份验证
diffx production/ backup/ -r --brief --quiet
echo $? # 0 = 相同, 1 = 不同
```

### CI/CD 集成

```bash
# 配置漂移检测
diffx expected_config.json actual_config.json --quiet
if [ $? -eq 1 ]; then
    echo "配置漂移检测！"
    exit 1
fi

# 模式比较脚本
diffx schema_v1.json schema_v2.json --output json > schema_diff.json

# 快速差异检查
diffx prod_data.json test_data.json --brief --ignore-keys-regex "^test_"
```

### 数据验证

```bash
# API响应比较
curl api.com/v1/data | diffx - expected_response.json

# 数据库导出验证
diffx db_export_old.json db_export_new.json --array-id-key "primary_key"

# 配置同步检查
for env in dev staging prod; do
    diffx base_config.json configs/$env.json --path "shared_settings"
done
```

## 最佳实践

### 性能优化

1. **为大型数据集使用数组ID键**
   ```bash
   diffx users_old.json users_new.json --array-id-key "user_id"
   ```

2. **过滤不相关的字段**
   ```bash
   diffx log1.json log2.json --ignore-keys-regex "^(timestamp|session_id)$"
   ```

3. **专注于特定部分**
   ```bash
   diffx config1.json config2.json --path "critical_settings"
   ```

### 脚本编写

1. **使用quiet模式进行条件逻辑**
   ```bash
   if diffx file1.json file2.json --quiet; then
       echo "无变化"
   else
       echo "检测到变化"
   fi
   ```

2. **捕获结构化输出**
   ```bash
   diffx data1.json data2.json --output json > changes.json
   ```

3. **批量处理**
   ```bash
   diffx configs/ configs.backup/ --recursive --brief
   ```

### 错误处理

1. **验证输入格式**
   ```bash
   diffx --format json suspicious_file.txt known_good.json
   ```

2. **处理缺少的文件**
   ```bash
   diffx file1.json file2.json 2>/dev/null || echo "比较失败"
   ```

3. **调试解析问题**
   ```bash
   diffx problematic.yaml reference.yaml --output unified
   ```

### 目录操作

```bash
# 递归目录比较
diffx config_dir1/ config_dir2/ --recursive

# 带输出格式的递归比较
diffx environments/dev/ environments/prod/ -r --output json

# 带过滤的递归比较
diffx configs/ configs.backup/ -r --ignore-keys-regex "^(timestamp|version)$"
```

### 集成示例

```bash
# Git 集成
git show HEAD~1:config.json > old_config.json
diffx old_config.json config.json --output unified

# CI/CD 管道
diffx expected_config.json actual_config.json \
  --ignore-keys-regex "^(deployment_time|build_id)" \
  --output json > config_validation.json
```

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

### 调试

```bash
# 验证格式检测
diffx --format json file1.txt file2.txt
```

### 大文件

```bash
# 使用路径过滤处理大文件
diffx huge1.json huge2.json --path "critical_section"

# 忽略非必要数据
diffx large1.json large2.json --ignore-keys-regex "logs|debug|metadata"
```

### 批处理

```bash
# 并行处理多个文件
find configs/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup || echo "Diff in {}"'
```

### 内存使用

对于非常大的文件，考虑：
- 使用 `--path` 专注于特定部分
- 使用 `--ignore-keys-regex` 过滤大的不相关部分
- 如果可能，分块处理文件

### 配置管理

```bash
# 环境比较
diffx prod.json staging.json --ignore-keys-regex "^(host|port|secret_.*)"

# Kubernetes 清单
diffx deployment.yaml deployment.new.yaml --ignore-keys-regex "^metadata\\\\.(creation.*|resource.*)"
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

---

**注意**: 此参考涵盖所有可用选项。对于快速入门，请参阅[入门指南](../user-guide/getting-started_zh.md)和[示例](../user-guide/examples_zh.md)。
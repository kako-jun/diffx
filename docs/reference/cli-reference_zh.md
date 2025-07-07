# CLI 参考

`diffx` 命令行界面的完整参考文档。

## 用法

```
diffx [OPTIONS] <INPUT1> <INPUT2>
```

## 描述

`diffx` 是一个用于结构化数据文件语义比较的命令行工具。与传统的基于文本的差分工具不同，`diffx` 理解数据的结构和含义，专注于实际变化而不是格式差异。

## 参数

### `<INPUT1>`
- **类型**：文件路径、目录路径或 `-` 表示标准输入
- **必需**：是
- **描述**：要比较的第一个输入

### `<INPUT2>`
- **类型**：文件路径、目录路径或 `-` 表示标准输入
- **必需**：是
- **描述**：要比较的第二个输入

**示例：**
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
- **类型**：字符串
- **默认**：从文件扩展名自动检测
- **值**：`json`, `yaml`, `toml`, `xml`, `ini`, `csv`
- **描述**：强制指定输入文件格式

**示例：**
```bash
# 强制 JSON 解释
diffx --format json file1.txt file2.txt

# 强制 YAML 解释
diffx -f yaml config1 config2
```

**自动检测映射：**
- `.json` → `json`
- `.yaml`, `.yml` → `yaml`
- `.toml` → `toml`
- `.xml` → `xml`
- `.ini`, `.cfg`, `.conf` → `ini`
- `.csv` → `csv`

### 输出选项

#### `-o, --output <FORMAT>`
- **类型**：字符串
- **默认**：`diffx`（人类可读的 diffx 格式）
- **值**：`diffx`, `json`, `yaml`, `unified`
- **描述**：差异的输出格式

**diffx 格式（默认）：**
```bash
diffx config.json config.new.json
# 输出：
# + database.port: 5432
# ~ version: "1.0" -> "1.1"
# - cache.enabled: true
```

**JSON 输出：**
```bash
diffx config.json config.new.json --output json
# 输出：
# [
#   {"Added": ["database.port", 5432]},
#   {"Modified": ["version", "1.0", "1.1"]},
#   {"Removed": ["cache.enabled", true]}
# ]
```

**YAML 输出：**
```bash
diffx config.json config.new.json --output yaml
# 输出：
# - Added:
#   - database.port
#   - 5432
# - Modified:
#   - version
#   - "1.0"
#   - "1.1"
```

**统一输出：**
```bash
diffx config.json config.new.json --output unified
# 输出：传统差分风格格式
```

### 过滤选项

#### `--path <PATH>`
- **类型**：字符串
- **默认**：无（比较整个结构）
- **描述**：将差异过滤到数据结构中的特定路径

**路径语法：**
- 对象键：`database.host`
- 数组索引：`users[0]`
- 嵌套路径：`config.database.connection.host`
- 复杂路径：`services.web.env[0].name`

**示例：**
```bash
# 仅比较数据库配置
diffx config.json config.new.json --path "database"

# 比较特定数组元素
diffx config.json config.new.json --path "users[0]"

# 深度嵌套路径
diffx config.json config.new.json --path "services.web.environment.variables"
```

#### `--ignore-keys-regex <PATTERN>`
- **类型**：正则表达式字符串
- **默认**：无
- **描述**：忽略匹配指定正则表达式的键

**常见模式：**
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

**正则表达式示例：**
- `^timestamp$` - 精确匹配 "timestamp"
- `^_.*` - 以下划线开头的字段
- `.*_temp$` - 以 "_temp" 结尾的字段
- `^(id|uid|pk)$` - 匹配任何一个：id、uid、pk
- `(?i)password` - 不区分大小写匹配 "password"

### 比较选项

#### `--epsilon <VALUE>`
- **类型**：浮点数
- **默认**：`0.0`（精确比较）
- **描述**：浮点数比较的容差

**示例：**
```bash
# 允许浮点数的小差异
diffx metrics.json metrics.new.json --epsilon 0.001

# 科学数据的更宽松容差
diffx measurements.json measurements.new.json --epsilon 0.01

# 非常严格的比较
diffx financial.json financial.new.json --epsilon 0.000001
```

**使用场景：**
- 有测量精度的科学数据
- 有舍入差异的金融计算
- 有小变化的性能指标
- 有浮点数伪影的转换数据

#### `--array-id-key <KEY>`
- **类型**：字符串
- **默认**：无（位置比较）
- **描述**：用于识别和跟踪数组元素的键

**示例：**
```bash
# 按 ID 跟踪用户
diffx users.json users.updated.json --array-id-key "id"

# 按 SKU 跟踪产品
diffx inventory.json inventory.new.json --array-id-key "sku"

# 按主键跟踪数据库记录
diffx records.json records.new.json --array-id-key "primary_key"
```

**没有 ID 跟踪：**
```json
// 数组比较显示位置变化
// 旧：[{"name": "Alice"}, {"name": "Bob"}]
// 新：[{"name": "Bob"}, {"name": "Alice"}]
// 结果：所有元素都显示为已更改
```

**有 ID 跟踪：**
```json
// 旧：[{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
// 新：[{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
// 结果：未检测到变化（相同元素，不同顺序）
```

### 目录选项

#### `-r, --recursive`
- **类型**：布尔标志
- **默认**：False
- **描述**：启用递归目录比较

**示例：**
```bash
# 比较目录中的所有文件
diffx config_dir1/ config_dir2/ --recursive

# 递归比较与输出格式
diffx environments/dev/ environments/prod/ -r --output json

# 递归与过滤
diffx configs/ configs.backup/ -r --ignore-keys-regex "^(timestamp|version)$"
```

**行为：**
- 比较目录之间的对应文件
- 跳过在两个目录中都不存在的文件
- 在输出中维护目录结构
- 对每个文件遵循格式自动检测

### 性能选项

#### `--optimize`
- **类型**：布尔标志
- **默认**：False（标准模式）
- **描述**：为大文件和数据结构启用内存高效处理

**使用时机：**
- 大文件（>100MB）
- 深度嵌套结构（>10层）
- 大型数组（>10,000个元素）
- 内存受限环境

**示例：**
```bash
# 高效处理大数据集
diffx large_data.json large_data.v2.json --optimize

# 使用自定义批处理大小进行优化
diffx huge_config.yaml huge_config.new.yaml --optimize --batch-size 5000

# 与其他选项结合
diffx massive_db.json massive_db.new.json --optimize --array-id-key "id" --path "users"
```

**性能比较：**
```bash
# 标准模式（默认）- 可预测，无限制内存使用
diffx config.json config.new.json

# 优化模式 - 内存高效，批处理
diffx config.json config.new.json --optimize
```

#### `--batch-size <SIZE>`
- **类型**：整数
- **默认**：1000
- **描述**：优化期间每批处理的元素数量

**指导原则：**
- **小文件**（<10MB）：使用默认值（1000）
- **中等文件**（10-100MB）：使用 1000-2000
- **大文件**（100MB-1GB）：使用 2000-5000
- **巨大文件**（>1GB）：使用 5000-10000

**示例：**
```bash
# 默认批处理大小
diffx large_file.json large_file.v2.json --optimize

# 用于很大文件的自定义批处理大小
diffx massive_dataset.json massive_dataset.v2.json --optimize --batch-size 10000

# 内存约束的细调
diffx big_config.yaml big_config.new.yaml --optimize --batch-size 500
```

**内存使用估算：**
| 批处理大小 | 内存使用 | 最适合 |
|------------|----------|--------|
| 500        | ~100MB   | 内存受限系统 |
| 1000       | ~200MB   | 默认（平衡） |
| 2000       | ~400MB   | 大文件 |
| 5000       | ~800MB   | 非常大的文件 |
| 10000      | ~1.5GB   | 巨大数据集 |

### 信息选项

#### `-h, --help`
- **类型**：布尔标志
- **描述**：打印帮助信息并退出

#### `-V, --version`
- **类型**：布尔标志
- **描述**：打印版本信息并退出

**示例：**
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

- **0**：成功，未发现差异
- **1**：成功，发现差异
- **2**：命令行参数错误
- **3**：文件 I/O 错误
- **4**：解析错误（格式无效）
- **5**：内部错误

**示例：**
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
    *) echo "发生错误（代码：$EXIT_CODE）" ;;
esac
```

## 环境变量

这些环境变量可用于设置默认值：

- `DIFFX_OUTPUT` - 默认输出格式
- `DIFFX_FORMAT` - 默认输入格式
- `DIFFX_EPSILON` - 默认 epsilon 值
- `DIFFX_IGNORE_KEYS_REGEX` - 默认忽略模式
- `DIFFX_ARRAY_ID_KEY` - 默认数组 ID 键
- `DIFFX_RECURSIVE` - 默认递归模式
- `DIFFX_COLORS` - 启用/禁用彩色输出

**示例：**
```bash
# 通过环境变量设置默认值
export DIFFX_OUTPUT=json
export DIFFX_IGNORE_KEYS_REGEX="^(timestamp|_.*)"
export DIFFX_EPSILON=0.001

# 命令现在使用这些默认值
diffx config.json config.new.json
```

## 配置文件

有关使用配置文件的详细信息，请参见[配置指南](../user-guide/configuration_zh.md)。

## 使用模式

### 基本比较

```bash
# 简单文件比较
diffx file1.json file2.json

# 比较不同格式
diffx config.yaml config.toml --format yaml --format toml

# 比较标准输入与文件
curl -s https://api.example.com/config | diffx - local_config.json
```

### 高级过滤

```bash
# 复杂忽略模式
diffx app.json app.new.json \
  --ignore-keys-regex "^(timestamp|_.*|createdAt|updatedAt|version)$"

# 特定路径比较
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

# CI/CD 管道
diffx expected_config.json actual_config.json \
  --ignore-keys-regex "^(deployment_time|build_id)" \
  --output json > config_validation.json

# 监控脚本
#!/bin/bash
if ! diffx baseline_config.json current_config.json \
     --ignore-keys-regex "^(timestamp|uptime)" >/dev/null; then
  echo "检测到配置偏移！"
  diffx baseline_config.json current_config.json --output json | \
    notify_alert_system.py
fi
```

## 错误处理

### 常见错误

**文件未找到：**
```bash
$ diffx nonexistent.json config.json
错误：没有此文件或目录（os错误2）
```

**格式无效：**
```bash
$ diffx invalid.json valid.json
错误：JSON解析失败：在第1行第15列处期望`,`或`}`
```

**权限被拒绝：**
```bash
$ diffx protected.json config.json
错误：权限被拒绝（os错误13）
```

**正则表达式无效：**
```bash
$ diffx file1.json file2.json --ignore-keys-regex "[invalid"
错误：正则表达式无效：未闭合的字符类
```

### 调试

```bash
# 详细输出（如果支持）
DIFFX_VERBOSE=true diffx file1.json file2.json

# 调试模式（如果支持）
DIFFX_DEBUG=true diffx file1.json file2.json

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
# 多个文件的并行处理
find configs/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup || echo "Diff in {}"'
```

### 内存使用

对于非常大的文件，考虑：
- 使用 `--path` 专注于特定部分
- 使用 `--ignore-keys-regex` 过滤掉大的、不相关的部分
- 如果可能，将文件分成较小的块进行处理

## 按使用场景分类的示例

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

这个全面的CLI参考涵盖了所有可用选项，并提供了有效使用`diffx`的实用示例。
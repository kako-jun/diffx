# CLI 参考文档

本文件为 `diffx` 命令行工具的完整参考文档。

## 命令格式

```
diffx [OPTIONS] <INPUT1> <INPUT2>
```

## 描述

`diffx` 是一款用于结构化数据文件语义化比较的命令行工具。与传统的基于文本的比较工具不同，`diffx` 能够理解数据的结构和含义，专注于实际的数据变更，而非格式上的差异。

## 参数

### `<INPUT1>`
- **类型**: 文件路径、目录路径，或使用 `-` 代表标准输入
- **是否必需**: 是
- **描述**: 第一个比较对象。

### `<INPUT2>`
- **类型**: 文件路径、目录路径，或使用 `-` 代表标准输入
- **是否必需**: 是
- **描述**: 第二个比较对象。

**示例:**
```bash
# 比较两个文件
diffx config.json config.new.json

# 与标准输入进行比较
cat config.json | diffx - config.new.json

# 比较两个目录（默认为非递归，与Unix diff命令兼容）
diffx config_dir1/ config_dir2/
```

## 选项

### 格式选项

#### `-f, --format <FORMAT>`
- **类型**: 字符串
- **默认值**: 根据文件扩展名自动检测
- **可选值**: `json`, `yaml`, `toml`, `xml`, `ini`, `csv`
- **描述**: 强制指定输入文件的格式。

**示例:**
```bash
# 强制按JSON格式解析
diffx --format json file1.txt file2.txt

# 强制按YAML格式解析
diffx -f yaml config1 config2
```

**自动检测映射关系:**
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
- **可选值**: `diffx`, `json`, `yaml`, `unified`
- **描述**: 指定差异内容的输出格式。

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
```

**Unified 输出:**
```bash
diffx config.json config.new.json --output unified
# 输出: 传统的 diff 风格格式
```

### 过滤选项

#### `--path <PATH>`
- **类型**: 字符串
- **默认值**: 无 (比较整个数据结构)
- **描述**: 将差异比较限定在数据结构中的特定路径。

**路径语法:**
- 对象键: `database.host`
- 数组索引: `users[0]`
- 嵌套路径: `config.database.connection.host`
- 复杂路径: `services.web.env[0].name`

**示例:**
```bash
# 仅比较数据库配置
diffx config.json config.new.json --path "database"

# 比较特定的数组元素
diffx config.json config.new.json --path "users[0]"

# 比较深层嵌套的路径
diffx config.json config.new.json --path "services.web.environment.variables"
```

#### `--ignore-keys-regex <PATTERN>`
- **类型**: 正则表达式字符串
- **默认值**: 无
- **描述**: 忽略与指定正则表达式匹配的键。

**常用模式:**
```bash
# 忽略时间戳字段
diffx file1.json file2.json --ignore-keys-regex "^(timestamp|createdAt|updatedAt)$"

# 忽略内部字段（以下划线开头）
diffx file1.json file2.json --ignore-keys-regex "^_.*"

# 忽略多种模式
diffx file1.json file2.json --ignore-keys-regex "^(id|timestamp|_.*|temp_.*)$"

# 忽略版本相关字段
diffx file1.json file2.json --ignore-keys-regex "(version|buildNumber|revision)"
```

**正则表达式示例:**
- `^timestamp$` - 精确匹配 "timestamp"
- `^_.*` - 匹配以下划线开头的字段
- `.*_temp$` - 匹配以 "_temp" 结尾的字段
- `^(id|uid|pk)$` - 匹配 id, uid, pk 中的任意一个
- `(?i)password` - 不区分大小写匹配 "password"

### 比较选项

#### `--epsilon <VALUE>`
- **类型**: 浮点数
- **默认值**: `0.0` (精确比较)
- **描述**: 用于浮点数比较的容差。

**示例:**
```bash
# 允许浮点数有微小的差异
diffx metrics.json metrics.new.json --epsilon 0.001

# 为科学数据设置更宽松的容差
diffx measurements.json measurements.new.json --epsilon 0.01

# 为金融数据进行非常严格的比较
diffx financial.json financial.new.json --epsilon 0.000001
```

**使用场景:**
- 存在测量精度的科学数据
- 存在四舍五入差异的金融计算
- 有微小波动的性能指标
- 因数据转换产生的浮点数偏差

#### `--array-id-key <KEY>`
- **类型**: 字符串
- **默认值**: 无 (基于位置进行比较)
- **描述**: 用于识别和跟踪数组元素的键。

**示例:**
```bash
# 使用 ID 跟踪用户
diffx users.json users.updated.json --array-id-key "id"

# 使用 SKU 跟踪产品
diffx inventory.json inventory.new.json --array-id-key "sku"

# 使用主键跟踪数据库记录
diffx records.json records.new.json --array-id-key "primary_key"
```

**不使用ID跟踪:**
```json
// 数组比较会显示基于位置的变化
// 旧: [{"name": "Alice"}, {"name": "Bob"}]
// 新: [{"name": "Bob"}, {"name": "Alice"}]
// 结果: 所有元素都被视为已修改
```

**使用ID跟踪:**
```json
// 旧: [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]  
// 新: [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
// 结果: 未检测到任何变化（元素相同，顺序不同）
```

#### `--ignore-whitespace`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 忽略字符串值中的空白差异。

**示例:**
```bash
# 包含不同空白字符的文件
echo '{"text": "Hello  World"}' > file1.json
echo '{"text": "Hello World"}' > file2.json

# 常规比较会显示差异
diffx file1.json file2.json
# 输出: ~ text: "Hello  World" -> "Hello World"

# 忽略空白则不报告差异
diffx file1.json file2.json --ignore-whitespace
# 输出: (无差异)
```

**使用场景:**
- 空格不一致的配置文件
- 从不同系统导出的数据
- 因手动编辑引入了多余的空格
- 标准化文本与原始文本的比较

#### `--ignore-case`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 忽略字符串值中的大小写差异。

**示例:**
```bash
# 包含不同大小写的文件
echo '{"status": "Active"}' > file1.json
echo '{"status": "ACTIVE"}' > file2.json

# 常规比较会显示差异
diffx file1.json file2.json
# 输出: ~ status: "Active" -> "ACTIVE"

# 忽略大小写则不报告差异
diffx file1.json file2.json --ignore-case
# 输出: (无差异)
```

**使用场景:**
- 大小写不一的用户输入数据
- 遗留系统迁移
- 不区分大小写的配置值
- 数据标准化任务

**组合选项:**
```bash
# 同时处理空白和大小写差异
diffx config.json config.new.json --ignore-whitespace --ignore-case

# 使用多个选项的复杂示例
diffx data.yaml data.updated.yaml \
  --ignore-case \
  --ignore-whitespace \
  --epsilon 0.001 \
  --ignore-keys-regex "^(timestamp|version)$"
```

### 输出控制选项

#### `--context <N>`
- **类型**: 整数
- **默认值**: 无 (显示所有上下文)
- **描述**: 在 unified 输出格式中，显示差异点周围 N 行的上下文。

**示例:**
```bash
# 在变更点周围显示2行上下文
diffx config.json config.new.json --output unified --context 2

# 只显示变更行（无上下文）
diffx config.json config.new.json --output unified --context 0

# 默认行为（显示所有上下文）
diffx config.json config.new.json --output unified
```

**带上下文的输出示例:**
```diff
# --context 2
   "database": {
     "host": "localhost",
-    "port": 5432
+    "port": 5433
   },
   "cache": {

# --context 0  
-    "port": 5432
+    "port": 5433
```

#### `-q, --quiet`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 抑制正常输出，仅返回退出状态码。

**示例:**
```bash
# 检查文件是否有差异（用于脚本）
diffx config.json config.new.json --quiet
echo $?  # 0 = 无差异, 1 = 有差异, 2 = 错误

# 在 shell 脚本中使用
if diffx config.json backup.json --quiet; then
    echo "文件相同"
else
    echo "文件不同"
fi

# 与其他选项结合使用
diffx large.json large.new.json --quiet --ignore-whitespace
```

**退出状态码:**
- `0`: 未发现差异
- `1`: 发现差异
- `2`: 发生错误（例如，无效文件、格式错误）

#### `--brief`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 仅报告文件名，不报告具体差异（类似于 `diff --brief`）。

**示例:**
```bash
# 仅报告文件是否不同
diffx config.json config.new.json --brief
# 输出: Files config.json and config.new.json differ

# 用于目录比较
diffx configs/ configs.backup/ --recursive --brief
# 输出: Files configs/app.json and configs.backup/app.json differ

# 与过滤选项结合使用
diffx data.json data.new.json --brief --ignore-keys-regex "^timestamp$"
```

**使用场景:**
- 批处理脚本
- 快速文件比较检查
- 自动化测试流水线
- 文件同步验证

#### `-v, --verbose`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 显示全面的诊断信息，包括性能指标、配置详情和处理统计数据。

**示例:**
```bash
# 基本的详细输出
diffx config.json config.new.json --verbose
# 输出包括:
# 输入文件信息: 
#   输入 1 大小: 245 字节
#   输入 2 大小: 267 字节
# 解析时间: 15.2µs
# 差异计算时间: 23.8µs
# 发现的总差异数: 3
# 性能总结:
#   总处理时间: 125.4µs
#   内存优化: 禁用

# 结合过滤选项的详细输出
diffx data.json data.new.json --verbose --ignore-keys-regex "timestamp" --epsilon 0.1
# 额外输出:
# 键过滤配置:
#   正则表达式模式: timestamp
# 数值容差配置:
#   Epsilon 值: 0.1

# 目录比较的详细输出
diffx configs/ configs.backup/ --recursive --verbose
# 额外输出:
# 目录扫描结果:
#   configs/ 中的文件数: 12
#   configs.backup/ 中的文件数: 11
#   待比较的总文件数: 12
# 目录比较总结:
#   已比较文件数: 11
#   仅存在于一个目录中的文件数: 1
#   发现差异: 是
```

**详细信息类别:**

1.  **性能指标**
    -   文件大小和内存使用情况
    -   解析时间、差异计算时间
    -   总处理时间
    -   内存优化状态

2.  **配置详情**
    -   当前生效的过滤模式（正则表达式、epsilon、数组ID键）
    -   路径过滤设置
    -   上下文显示配置

3.  **处理统计**
    -   过滤前后的总差异数
    -   目录扫描结果
    -   比较效果指标

4.  **诊断输出**
    -   优化决策信息
    -   处理批次信息
    -   错误上下文和故障排查数据

**使用场景:**
- 性能分析与优化
- 慢速比较的故障排查
- 理解过滤器的有效性
- 调试配置问题
- CI/CD 流水线诊断
- 技术支持与维护任务

### 目录选项

#### `-r, --recursive`
- **类型**: 布尔标志
- **默认值**: False
- **描述**: 递归比较目录及其子目录（与Unix `diff` 命令兼容）。

**示例:**
```bash
# 不带 --recursive 的目录比较（Unix diff 兼容）
# 仅比较目录顶层文件，对子目录显示 "Common subdirectories" 消息
diffx config_dir1/ config_dir2/
# 输出:
# Common subdirectories: config_dir1/subdir and config_dir2/subdir
# --- Comparing config.json ---
# ~ version: "1.0" -> "1.1"

# 递归比较 - 比较所有文件，包括子目录中的文件
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

**不带 `--recursive` 标志 (默认):**
-   仅比较指定目录顶层的文件。
-   对于两个位置都存在的子目录，显示 "Common subdirectories" 消息。
-   不比较子目录内的文件。
-   与标准 Unix `diff` 命令保持兼容。

**带 `--recursive` 标志:**
-   递归比较所有文件，包括子目录中的文件。
-   在输出中保持目录结构。
-   等同于 `diff -r` 的行为。

**共同行为:**
-   跳过仅存在于一个目录中的文件。
-   对每个文件应用格式自动检测。
-   报告仅存在于一个目录中的文件。

### 性能选项

#### 自动优化
- **类型**: 自动功能
- **默认**: 对大于1MB的文件启用
- **描述**: 对大文件和复杂数据结构自动启用内存优化处理。

**自动检测行为:**
-   文件 ≤ 1MB: 标准模式（速度快，内存使用无限制）
-   文件 > 1MB: 优化模式（内存高效，分批处理）
-   无需手动配置 - 优化过程完全透明。

**优化特性:**
-   基于文件大小自动检测
-   针对大数据集的内存高效处理
-   针对深层嵌套结构的分批处理
-   无论在哪种模式下，都保持输出一致

**示例:**
```bash
# 自动检测（始终启用）
diffx config.json config.new.json
# 对小文件使用标准模式，对大文件使用优化模式

# 大文件自动使用优化
diffx massive_db.json massive_db.new.json --array-id-key "id" --path "users"
# 自动对大文件使用优化模式

# 所有其他选项均可与优化功能透明地协同工作
diffx complex_data.json complex_data.v2.json --ignore-keys-regex "^timestamp$"
# 如果需要，会自动应用优化
```

**性能表现:**
```bash
# 小文件 (<1MB) - 自动标准模式
diffx config.json config.new.json
# 处理速度快，内存使用无限制

# 大文件 (>1MB) - 自动优化模式  
diffx large_dataset.json large_dataset.v2.json
# 内存高效，分批处理

# 复杂嵌套结构 - 自动优化
diffx deep_nested.json deep_nested.v2.json
# 基于数据特征的透明优化
```

### 信息选项

#### `-h, --help`
- **类型**: 布尔标志
- **描述**: 打印帮助信息并退出。

#### `-V, --version`
- **类型**: 布尔标志
- **描述**: 打印版本信息并退出。

**示例:**
```bash
# 显示帮助信息
diffx --help
diffx -h

# 显示版本信息
diffx --version
diffx -V
```

## 退出状态码

`diffx` 使用以下退出状态码:

-   **0**: 成功，未发现差异
-   **1**: 成功，发现差异
-   **2**: 命令行参数错误
-   **3**: 文件 I/O 错误
-   **4**: 解析错误（格式无效）
-   **5**: 内部错误

**示例:**
```bash
# 检查文件是否相同
if diffx file1.json file2.json >/dev/null 2>&1; then
    echo "文件相同"
else
    echo "文件不同"
fi

# 捕获退出状态码
diffx config.json config.new.json
EXIT_CODE=$?
case $EXIT_CODE in
    0) echo "无差异" ;;
    1) echo "发现差异" ;;
    *) echo "发生错误 (状态码: $EXIT_CODE)" ;;
esac
```

## 使用模式

### 基本比较

```bash
# 简单文件比较
diffx file1.json file2.json

# 比较不同格式的文件
diffx config.yaml config.toml

# 比较标准输入与文件
curl -s https://api.example.com/config | diffx - local_config.json
```

### 高级过滤

```bash
# 复杂的忽略模式
diffx app.json app.new.json \
  --ignore-keys-regex "^(timestamp|_.*|createdAt|updatedAt|version)$"

# 特定路径的比较
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
# Unix diff 兼容的目录比较（非递归）
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
     --ignore-keys-regex "^(timestamp|uptime)$" >/dev/null; then
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

**格式无效:**
```bash
$ diffx invalid.json valid.json
Error: Failed to parse JSON: expected `,` or `}` at line 1 column 15
```

**权限被拒绝:**
```bash
$ diffx protected.json config.json
Error: Permission denied (os error 13)
```

**无效的正则表达式:**
```bash
$ diffx file1.json file2.json --ignore-keys-regex "[invalid"
Error: Invalid regular expression: unclosed character class
```

### 调试

```bash
# 验证格式检测
diffx --format json file1.txt file2.txt
```

## 性能考量

### 大文件

```bash
# 对大文件使用路径过滤
diffx huge1.json huge2.json --path "critical_section"

# 忽略不重要的数据
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

对于非常大的文件，请考虑：
- 使用 `--path` 专注于特定部分
- 使用 `--ignore-keys-regex` 过滤掉大的、不相关的部分
- 如果可能，将文件分块处理

## 按用例分类的示例

### 配置管理
```bash
# 环境比较
diffx prod.json staging.json --ignore-keys-regex "^(host|port|secret_.*)$"

# Kubernetes 清单文件
diffx deployment.yaml deployment.new.yaml --ignore-keys-regex "^metadata\\.(creation.*|resource.*)$"
```

### API 测试
```bash
# 响应验证
diffx expected_response.json actual_response.json --ignore-keys-regex "^(timestamp|request_id)$"

# 模式比较
diffx api_v1_schema.json api_v2_schema.json --path "definitions"
```

### 数据处理
```bash
# ETL 验证
diffx input_data.json output_data.json --array-id-key "record_id" --epsilon 0.001

# 数据库导出比较
diffx export1.json export2.json --array-id-key "id" --ignore-keys-regex "^(updated_at|sync_time)$"
```

### 安全审计
```bash
# 策略比较
diffx security_policy.json security_policy.new.json --path "permissions"

# 访问控制验证
diffx rbac.yaml rbac.new.yaml --array-id-key "name"
```

这份全面的CLI参考文档涵盖了`diffx`所有可用的选项，并为有效使用提供了实用的示例。

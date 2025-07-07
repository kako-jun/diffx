# diffx 入门指南

这个综合指南将帮助您快速有效地开始使用 `diffx`。

## 什么是 diffx？

`diffx` 是一个专为结构化数据格式设计的语义差异工具。与传统的基于文本的差异工具不同，`diffx` 理解数据的结构和含义，专注于实际变化而非格式差异。

### 主要优势

- **语义理解**: 忽略格式、键顺序、空白和尾随逗号
- **多种格式**: 支持 JSON、YAML、TOML、XML、INI 和 CSV
- **清晰输出**: 人类可读且机器可解析的结果
- **高级功能**: 数组跟踪、正则表达式过滤、浮点容差

## 前提条件

在开始之前，请确保您已安装 `diffx`。详细说明请参见 [安装指南](installation_zh.md)。

快速安装：
```bash
cargo install diffx
```

## 基本用法

### 简单文件比较

最基本的用法是比较两个文件：

```bash
# 比较 JSON 文件
diffx config_v1.json config_v2.json

# 比较 YAML 文件
diffx docker-compose.yml docker-compose.new.yml

# 比较 TOML 文件
diffx Cargo.toml Cargo.toml.backup

# 比较 XML 文件
diffx settings.xml settings.new.xml

# 比较 INI 文件
diffx database.ini database.prod.ini

# 比较 CSV 文件
diffx users.csv users_updated.csv
```

### 理解 diffx 格式

默认情况下，`diffx` 以 **diffx 格式** 输出差异 - 这是一种专门为语义数据比较设计的人类可读的结构化表示。diffx 格式使用直观的符号来显示不同类型的变化：

- `+` **添加**: 新的键或值
- `-` **删除**: 删除的键或值
- `~` **修改**: 更改的值
- `!` **类型变更**: 值类型转换（例如，字符串到数字）

diffx 格式输出示例：
```
+ database.port: 5432
- cache.ttl: 3600
~ version: "1.0" -> "1.1"
! debug: "true" -> true
```

## 格式检测

`diffx` 根据文件扩展名自动检测文件格式：

```bash
# 这些命令会自动工作
diffx app.json app.new.json     # 检测为 JSON
diffx config.yaml config.yml    # 检测为 YAML
diffx settings.toml backup.toml # 检测为 TOML
```

### 手动格式指定

如果自动检测失败或您使用管道，请明确指定格式：

```bash
# 强制 JSON 解释
diffx --format json file1 file2

# 为每个文件指定不同的格式
diffx --format json file1.txt --format yaml file2.txt
```

## 使用不同的数据源

### 标准输入

使用管道比较来自不同源的数据：

```bash
# 比较 API 响应
curl -s https://api.example.com/v1/config | diffx config.json -

# 比较命令输出
docker inspect container1 | diffx - <(docker inspect container2)

# 处理来自环境变量的 JSON
echo "$CONFIG_V1" | diffx - config_v2.json --format json
```

### 目录比较

递归比较整个目录：

```bash
# 比较两个目录中的所有文件
diffx config_dir1/ config_dir2/ --recursive

# 仅比较特定文件类型
diffx configs/ configs_backup/ --recursive --format json
```

## 高级功能

### 忽略特定键

使用正则表达式忽略某些键：

```bash
# 忽略时间戳和内部字段
diffx app.json app.new.json --ignore-keys-regex "^(timestamp|_.*|createdAt)$"

# 忽略版本相关字段
diffx package.json package.new.json --ignore-keys-regex "version|buildNumber"
```

### 数组元素跟踪

对于包含具有唯一标识符的对象的数组：

```bash
# 通过 ID 跟踪用户
diffx users.json users_updated.json --array-id-key "id"

# 通过 SKU 跟踪产品
diffx inventory.json inventory.new.json --array-id-key "sku"

# 通过主键跟踪数据库记录
diffx records.json records.new.json --array-id-key "pk"
```

### 浮点容差

处理浮点精度差异：

```bash
# 允许小的数值差异（0.001 容差）
diffx metrics.json metrics.new.json --epsilon 0.001

# 科学数据的更宽松容差
diffx measurements.json measurements.new.json --epsilon 0.01
```

### 路径过滤

专注于数据的特定部分：

```bash
# 仅显示数据库配置中的差异
diffx config.json config.new.json --path "database"

# 检查特定数组元素
diffx config.json config.new.json --path "servers[0]"

# 深层路径过滤
diffx app.json app.new.json --path "microservices.auth.database.connection"
```

## 输出格式

### diffx 格式（默认）

**diffx 格式** 是默认输出格式，旨在既人类可读又语义精确。与传统的基于文本的差异不同，diffx 格式专注于数据结构和含义：

**diffx 格式的关键特性：**
- **语义焦点**: 显示逻辑变化，而非文本差异
- **路径清晰**: 完整路径表示法（例如，`database.connection.host`）
- **类型感知**: 区分值变化和类型变化
- **层次结构**: 保持数据关系上下文
- **通用符号**: 在所有数据格式中一致的 `+`、`-`、`~`、`!` 表示法

**标准 diffx 格式输出：**

```bash
diffx config.json config.new.json
# 输出：
# + database.port: 5432
# ~ version: "1.0" -> "1.1"
# - cache.enabled: true
```

### JSON 输出

完美适用于程序化处理：

```bash
diffx config.json config.new.json --output json
```

输出格式：
```json
[
  {
    "Added": ["database.port", 5432]
  },
  {
    "Modified": ["version", "1.0", "1.1"]
  },
  {
    "Removed": ["cache.enabled", true]
  }
]
```

### YAML 输出

人类可读的结构化输出：

```bash
diffx config.json config.new.json --output yaml
```

### 统一差异格式

与传统差异工具兼容：

```bash
diffx config.json config.new.json --output unified
```

## 实用示例

### 配置管理

```bash
# 比较 Kubernetes 配置
diffx k8s-prod.yaml k8s-staging.yaml --ignore-keys-regex "namespace|name"

# 检查 Terraform 状态变化
diffx terraform.tfstate terraform.tfstate.backup --path "resources"

# 比较 Docker Compose 文件
diffx docker-compose.yml docker-compose.override.yml
```

### 数据验证

```bash
# 比较数据库导出
diffx users_backup.json users_current.json --array-id-key "user_id"

# 验证 API 响应
diffx expected_response.json actual_response.json --ignore-keys-regex "timestamp"

# 检查数据迁移
diffx before_migration.json after_migration.json --epsilon 0.001
```

### 开发工作流

```bash
# 比较包文件
diffx package.json package.json.template --ignore-keys-regex "^(name|version)"

# 检查配置变化
diffx .env.example .env.local --format ini

# 验证构建输出
diffx build_manifest.json build_manifest.expected.json
```

## 性能优化

对于大文件或复杂数据结构，使用 `--optimize` 标志启用内存高效处理：

### 大文件处理

```bash
# 高效处理大型 JSON 文件（>100MB）
diffx large_dataset_v1.json large_dataset_v2.json --optimize

# 使用自定义批处理大小优化
diffx huge_config.json huge_config.new.json --optimize --batch-size 5000

# 处理大型 CSV 文件
diffx sales_data_2023.csv sales_data_2024.csv --optimize --format csv
```

### 何时使用优化

在处理以下情况时使用 `--optimize`：

- **大文件**（>100MB）
- **深层嵌套结构**（>10 层）
- **大数组**（>10,000 个元素）
- **内存受限环境**

```bash
# 示例：处理大型配置文件
diffx kubernetes_config_old.yaml kubernetes_config_new.yaml --optimize

# 示例：数据库导出比较
diffx users_dump_before.json users_dump_after.json --optimize --array-id-key "id"

# 示例：内存有限的 CI/CD
diffx deployment_config.json deployment_config.prod.json --optimize --batch-size 2000
```

### 性能配置

将优化与其他选项结合使用：

```bash
# 优化比较与过滤
diffx large_data.json large_data.v2.json --optimize --path "config.database"

# 优化与正则表达式过滤
diffx huge_config.yaml huge_config.new.yaml --optimize --ignore-keys-regex "^(timestamp|_temp)"

# 优化浮点比较
diffx financial_data.json financial_data.updated.json --optimize --epsilon 0.0001
```

### 性能比较

**标准模式与优化模式：**

```bash
# 标准模式（默认）- 可预测，无限制内存使用
diffx config.json config.new.json

# 优化模式 - 内存高效，批处理
diffx config.json config.new.json --optimize
```

**实际示例：**
```bash
# 10,000 个元素的 JSON 数组（50MB 文件比较）
# 测试环境：AMD Ryzen 5 PRO 4650U
$ time diffx large_users.json large_users_v2.json
# 标准模式：~0.15s，内存使用量：~150MB

$ time diffx large_users.json large_users_v2.json --optimize
# 优化模式：~0.12s，内存使用量：~80MB
```

### 内存使用指南

| 数据大小 | 批处理大小 | 预期内存 |
|----------|------------|----------|
| < 10MB   | 默认       | < 50MB   |
| 10-100MB | 1000       | < 200MB  |
| 100MB-1GB| 5000       | < 500MB  |
| > 1GB    | 10000      | < 1GB    |

> **注意**: 默认使用标准模式以获得可预测的行为。仅在大数据处理时明确需要时使用 `--optimize`。

## 配置文件

创建 `~/.config/diffx/config.toml` 以设置默认选项：

```toml
# 默认输出格式
output = "cli"

# 浮点比较的默认 epsilon
epsilon = 0.001

# 默认忽略的键
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"

# 默认数组 ID 键
array_id_key = "id"

# 性能优化设置
use_memory_optimization = false  # 使用 --optimize 标志启用
batch_size = 1000               # 大数据处理的批处理大小

# 在输出中启用颜色
colors = true

# 目录的默认递归模式
recursive = true
```

## 与其他工具的集成

### Git 集成

```bash
# 用于结构化差异的 Git 别名
git config alias.diffx '!f() { git show "$1" | diffx - "$2"; }; f'

# 在 git hooks 中使用
diffx package.json HEAD~1:package.json --output json > package_changes.json
```

### CI/CD 管道

```bash
# GitHub Actions
diffx config/prod.yaml config/staging.yaml --output json > config_diff.json

# GitLab CI
diffx database_schema.json database_schema.backup.json --array-id-key "table_name"
```

### 监控和告警

```bash
# 检查配置漂移
if diffx config.json config.expected.json --output json | jq -e 'length > 0'; then
  echo "检测到配置漂移！"
  exit 1
fi
```

## 性能提示

### 大文件

对于非常大的文件：

```bash
# 使用路径过滤专注于特定部分
diffx large_config.json large_config.new.json --path "critical_section"

# 忽略非必要字段
diffx large_data.json large_data.new.json --ignore-keys-regex "metadata|debug_info"
```

### 批处理

```bash
# 高效处理多个文件
find configs/ -name "*.json" -print0 | \
  xargs -0 -I {} sh -c 'diffx {} {}.backup || echo "Differences in {}"'
```

## 常见模式

### 环境比较

比较不同环境的配置：

```bash
# 开发环境 vs 生产环境
diffx config/dev.json config/prod.json --ignore-keys-regex "host|port|password"

# 预发布环境验证
diffx config/staging.yaml config/prod.yaml --path "database"
```

### 备份验证

验证备份完整性：

```bash
# 数据库备份验证
diffx db_export.json db_backup.json --array-id-key "id" --epsilon 0.001

# 配置备份检查
diffx app_config.toml app_config.backup.toml
```

### API 测试

验证 API 响应：

```bash
# 响应比较
diffx expected_api_response.json actual_response.json --ignore-keys-regex "timestamp|request_id"

# 模式验证
diffx api_schema.json generated_schema.json --path "definitions"
```

## 故障排除

### 常见问题

**文件未找到错误：**
```bash
# 检查文件路径
ls -la file1.json file2.json
```

**格式检测失败：**
```bash
# 明确指定格式
diffx file1 file2 --format json
```

**输出过大：**
```bash
# 使用路径过滤
diffx large1.json large2.json --path "specific.section"
```

**大文件的内存问题：**
```bash
# 增加内存限制（如果支持）
export DIFFX_MAX_MEMORY=2GB
diffx huge1.json huge2.json
```

## 下一步

- 阅读 [配置指南](configuration_zh.md) 了解高级设置
- 浏览 [示例](examples_zh.md) 查看实际使用案例
- 查看 [CLI 参考](../reference/cli-reference_zh.md) 获取完整选项文档
- 了解 [集成模式](../guides/integrations_zh.md) 的 CI/CD 工作流

## 获取帮助

如果您需要帮助：

1. 查看 [FAQ](faq_zh.md)
2. 浏览 [示例](examples_zh.md)
3. 访问 [GitHub 仓库](https://github.com/kako-jun/diffx)
4. 为错误或功能请求创建 issue
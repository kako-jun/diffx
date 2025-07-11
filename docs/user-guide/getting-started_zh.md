# diffx 入门指南

本综合指南将帮助您快速有效地开始使用 `diffx`。

## 什么是 diffx？

`diffx` 是专门为结构化数据格式设计的语义差分工具。与传统的基于文本的差分工具不同，`diffx` 理解数据的结构和含义，专注于实际更改而非格式差异。

### 主要优势

- **语义理解**: 忽略格式化、键顺序、空白和尾随逗号
- **多格式支持**: 支持 JSON、YAML、TOML、XML、INI 和 CSV
- **清晰输出**: 人类可读且机器可解析的结果
- **高级功能**: 数组跟踪、正则表达式过滤、浮点数容差

## 前提条件

开始之前，请确保已安装 `diffx`。详细说明请参见[安装指南](installation_zh.md)。

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

默认情况下，`diffx` 以 **diffx 格式** 输出差异 - 一种专门为语义数据比较设计的人类可读的结构化表示。diffx 格式使用直观的符号来显示不同类型的更改：

- `+` **添加**: 新的键或值
- `-` **移除**: 删除的键或值
- `~` **修改**: 更改的值
- `!` **类型更改**: 值类型转换（例如，字符串转数字）

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
# 这些命令自动工作
diffx app.json app.new.json     # 检测为 JSON
diffx config.yaml config.yml    # 检测为 YAML
diffx settings.toml backup.toml # 检测为 TOML
```

### 手动格式指定

如果自动检测失败或您使用管道，请明确指定格式：

```bash
# 强制 JSON 解释
diffx --format json file1 file2

# 为每个文件指定不同格式
diffx --format json file1.txt --format yaml file2.txt
```

## 处理不同数据源

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
# 按 ID 跟踪用户
diffx users.json users_updated.json --array-id-key "id"

# 按 SKU 跟踪产品
diffx inventory.json inventory.new.json --array-id-key "sku"

# 按主键跟踪数据库记录
diffx records.json records.new.json --array-id-key "pk"
```

### 浮点数容差

处理浮点数精度差异：

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

### 新增高需求选项

#### 忽略大小写差异

忽略字符串值中的大小写差异：

```bash
# 不同大小写的配置值
diffx config.json config.new.json --ignore-case

# 与其他选项组合
diffx user_data.json user_data.updated.json \
  --ignore-case \
  --ignore-keys-regex "^(id|timestamp)$"
```

#### 忽略空白差异

忽略字符串值中的空白差异：

```bash
# 不同空白格式的文件
diffx formatted.json minified.json --ignore-whitespace

# 处理格式不一致的导出数据
diffx export1.json export2.json \
  --ignore-whitespace \
  --ignore-case
```

#### 静默模式

仅返回退出状态，不显示差异详情：

```bash
# 用于脚本的快速检查
if diffx config.json config.backup.json --quiet; then
    echo "文件相同"
else
    echo "文件不同"
fi

# 批量检查
for file in configs/*.json; do
    if ! diffx "$file" "backups/$(basename $file)" --quiet; then
        echo "$(basename $file) 已更改"
    fi
done
```

#### 简要模式

仅报告文件名，不显示差异详情：

```bash
# 快速概览哪些文件不同
diffx config.json config.new.json --brief
# 输出: Files config.json and config.new.json differ

# 目录比较时特别有用
diffx configs/ configs.backup/ --recursive --brief
```

#### 上下文行控制

在统一输出中控制上下文行数：

```bash
# 显示变化周围的 3 行上下文
diffx config.json config.new.json \
  --output unified \
  --context 3

# 仅显示变化行，无上下文
diffx config.json config.new.json \
  --output unified \
  --context 0
```

## 输出格式

### diffx 格式（默认）

**diffx 格式** 是默认输出格式，旨在既人类可读又语义精确。与传统的基于文本的差分不同，diffx 格式专注于数据结构和含义：

**diffx 格式的主要特性：**
- **语义焦点**: 显示逻辑更改，而非文本差异
- **路径清晰**: 完整路径表示法（例如，`database.connection.host`）
- **类型感知**: 区分值更改和类型更改
- **层次结构**: 保持数据关系上下文
- **通用符号**: 在所有数据格式中一致的 `+`、`-`、`~`、`!` 表示法

**标准 diffx 格式输出：**

```bash
diffx config.json config.new.json
# 输出:
# + database.port: 5432
# ~ version: "1.0" -> "1.1"
# - cache.enabled: true
```

### JSON 输出

适合程序化处理：

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

### 统一差分格式

与传统差分工具兼容：

```bash
diffx config.json config.new.json --output unified
```

## 实际示例

### 配置管理

```bash
# 比较 Kubernetes 配置
diffx k8s-prod.yaml k8s-staging.yaml --ignore-keys-regex "namespace|name"

# 检查 Terraform 状态更改
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

# 检查配置更改
diffx .env.example .env.local --format ini

# 验证构建输出
diffx build_manifest.json build_manifest.expected.json
```

## 性能优化

对于大文件或复杂数据结构，diffx **自动**启用内存高效处理：

### 自动优化

```bash
# 高效处理大型 JSON 文件（>1MB）
diffx large_dataset_v1.json large_dataset_v2.json
# 自动应用优化

# 小文件使用标准模式
diffx config.json config.new.json
# 快速标准处理

# 处理大型 CSV 文件
diffx sales_data_2023.csv sales_data_2024.csv --format csv
# 根据文件大小自动优化
```

### 何时应用优化

优化在以下情况自动应用：

- **大文件**（>1MB）
- **深度嵌套结构**（自动检测）
- **大数组**（自动检测）
- **内存受限环境**（自动处理）

```bash
# 示例：处理大型配置文件（自动优化）
diffx kubernetes_config_old.yaml kubernetes_config_new.yaml

# 示例：数据库导出比较（自动优化）
diffx users_dump_before.json users_dump_after.json --array-id-key "id"

# 示例：内存有限的 CI/CD（自动优化）
diffx deployment_config.json deployment_config.prod.json
```

### 透明性能配置

优化与所有其他选项透明工作：

```bash
# 带过滤的优化比较（自动检测）
diffx large_data.json large_data.v2.json --path "config.database"

# 带正则表达式过滤的优化（自动检测）
diffx huge_config.yaml huge_config.new.yaml --ignore-keys-regex "^(timestamp|_temp)"

# 优化的浮点数比较（自动检测）
diffx financial_data.json financial_data.updated.json --epsilon 0.0001
```

### 性能行为

**自动优化：**

```bash
# 小文件 - 标准模式（自动选择）
diffx config.json config.new.json
# 快速处理，无限内存使用

# 大文件 - 优化模式（自动选择）
diffx large_dataset.json large_dataset.v2.json
# 内存高效，批处理
```

**实际示例：**
```bash
# 10,000 元素 JSON 数组（50MB 文件比较）
# 测试环境：AMD Ryzen 5 PRO 4650U
$ time diffx large_users.json large_users_v2.json
# 自动优化模式：~0.12s，内存使用：~80MB

$ time diffx config.json config.new.json
# 标准模式：~0.05s，内存使用：~20MB
```

### 内存使用指南

| 数据大小 | 应用模式 | 预期内存 |
|----------|----------|----------|
| < 1MB    | 标准模式 | < 50MB   |
| 1-10MB   | 优化模式 | < 100MB  |
| 10-100MB | 优化模式 | < 200MB  |
| 100MB-1GB| 优化模式 | < 500MB  |
| > 1GB    | 优化模式 | < 1GB    |

> **注意**: 优化完全透明，无需用户干预。所有文件大小都保证一致的输出。

## 与其他工具集成

### Git 集成

```bash
# 结构化差分的 Git 别名
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

### 监控和警报

```bash
# 检查配置漂移
if diffx config.json config.expected.json --output json | jq -e 'length > 0'; then
  echo "检测到配置漂移！"
  exit 1
fi
```

## 性能技巧

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

跨环境比较配置：

```bash
# 开发 vs 生产
diffx config/dev.json config/prod.json --ignore-keys-regex "host|port|password"

# 暂存验证
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

## 高级使用场景

### 高需求选项的组合使用

```bash
# 容忍多种格式差异的配置比较
diffx config_template.json user_config.json \
  --ignore-case \
  --ignore-whitespace \
  --ignore-keys-regex "^(user_.*|timestamp)$"

# 快速批量验证
for env in dev staging prod; do
    if ! diffx base_config.json "config_$env.json" \
         --quiet \
         --ignore-case \
         --ignore-whitespace; then
        echo "$env 环境配置存在差异"
        diffx base_config.json "config_$env.json" --brief
    fi
done

# 部署前的详细验证
diffx current_deployment.yaml new_deployment.yaml \
  --output unified \
  --context 2 \
  --ignore-keys-regex "^(metadata\..*|status\..*)"
```

### UNIX 风格的工作流

利用新选项实现类似传统 UNIX diff 的工作流：

```bash
# 类似 diff -q 的快速检查
diffx file1.json file2.json --quiet
echo $? # 0=相同, 1=不同, 2=错误

# 类似 diff --brief 的文件名报告
diffx configs/ configs.backup/ --recursive --brief

# 类似 diff -i 的忽略大小写
diffx schema.json schema_generated.json --ignore-case

# 类似 diff -w 的忽略空白
diffx formatted.json minified.json --ignore-whitespace

# 类似 diff -C3 的上下文显示
diffx config.json config.new.json \
  --output unified \
  --context 3
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

**大量输出：**
```bash
# 使用路径过滤
diffx large1.json large2.json --path "specific.section"
```

**大文件内存问题：**
```bash
# 自动优化模式
diffx huge1.json huge2.json
```

**意外的差异：**
```bash
# 检查是否是格式差异
diffx file1.json file2.json --ignore-whitespace --ignore-case
```

## 下一步

- 探索[示例](examples_zh.md)了解实际用例
- 查看[CLI 参考](../reference/cli-reference_zh.md)获取完整选项文档
- 学习用于 CI/CD 工作流的[集成模式](../guides/integrations_zh.md)

## 获取帮助

如果您需要帮助：

1. 查看[常见问题](faq_zh.md)
2. 浏览[示例](examples_zh.md)
3. 访问[GitHub 仓库](https://github.com/kako-jun/diffx)
4. 为错误或功能请求创建 issue
# 实际应用示例

本指南提供了在实际场景中使用 `diffx` 的实用示例，按使用场景和行业进行组织。

## 目录

- [配置管理](#配置管理)
- [DevOps和基础设施](#devops和基础设施)
- [API开发和测试](#api开发和测试)
- [数据处理和ETL](#数据处理和etl)
- [数据库管理](#数据库管理)
- [监控和报警](#监控和报警)
- [软件开发](#软件开发)
- [安全与合规](#安全与合规)

## 配置管理

### 环境配置比较

比较不同环境间的配置：

```bash
# 开发环境 vs 生产环境
diffx config/dev.json config/prod.json \
  --ignore-keys-regex "^(host|port|password|secret_.*)" \
  --output json > env_diff.json

# 部署前的准备环境验证
diffx config/staging.yaml config/prod.yaml \
  --path "application" \
  --output yaml
```

**示例文件：**
```json
// config/dev.json
{
  "application": {
    "name": "myapp",
    "version": "1.0.0",
    "debug": true
  },
  "database": {
    "host": "localhost",
    "port": 5432,
    "name": "myapp_dev"
  }
}

// config/prod.json  
{
  "application": {
    "name": "myapp",
    "version": "1.0.0", 
    "debug": false
  },
  "database": {
    "host": "prod-db.example.com",
    "port": 5432,
    "name": "myapp_prod"
  }
}
```

**预期输出：**
```
~ application.debug: true -> false
```

### Kubernetes配置漂移检测

监控Kubernetes部署中的配置漂移：

```bash
# 比较当前部署与期望状态
kubectl get deployment myapp -o json > current-deployment.json
diffx desired-deployment.json current-deployment.json \
  --ignore-keys-regex "^(metadata\\.(creationTimestamp|resourceVersion|uid)|status\\.*)" \
  --output json
```

### Docker Compose环境变体

比较不同环境的Docker Compose文件：

```bash
# 比较基础compose与覆盖文件
diffx docker-compose.yml docker-compose.override.yml \
  --path "services" \
  --output unified
```

## DevOps和基础设施

### Terraform状态比较

比较Terraform状态文件以检测基础设施漂移：

```bash
# 比较当前状态与备份
diffx terraform.tfstate terraform.tfstate.backup \
  --path "resources" \
  --ignore-keys-regex "^(last_updated|timeouts)" \
  --output json > infrastructure_drift.json

# 比较计划的更改
terraform show -json plan.out > planned.json
diffx current_state.json planned.json \
  --path "planned_values.root_module"
```

### 基础设施即代码验证

在部署前验证基础设施更改：

```bash
# 比较CloudFormation模板
diffx infrastructure/base.yaml infrastructure/updated.yaml \
  --ignore-keys-regex "^(Metadata|Description)" \
  --output yaml

# 比较Ansible剧本
diffx playbook-v1.yml playbook-v2.yml \
  --path "tasks" \
  --output cli
```

### CI/CD流水线配置

监控CI/CD流水线配置更改：

```bash
# GitHub Actions工作流比较
diffx .github/workflows/ci.yml .github/workflows/ci.new.yml \
  --output unified

# GitLab CI比较
diffx .gitlab-ci.yml .gitlab-ci.backup.yml \
  --ignore-keys-regex "^(variables\\.CI_.*)"
```

## API开发和测试

### API响应验证

根据预期模式验证API响应：

```bash
# 比较API响应与预期结构
curl -s https://api.example.com/v1/users/123 > actual_response.json
diffx expected_user_response.json actual_response.json \
  --ignore-keys-regex "^(timestamp|request_id|server_time)" \
  --output json

# 验证API端点更改
diffx api/v1/schema.json api/v2/schema.json \
  --path "definitions" \
  --output yaml
```

**示例API验证：**
```bash
# 测试用户创建端点
echo '{
  "id": 123,
  "name": "John Doe",
  "email": "john@example.com",
  "created_at": "2024-01-01T00:00:00Z"
}' > expected_user.json

curl -s -X POST https://api.example.com/users \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com"}' > actual_user.json

diffx expected_user.json actual_user.json \
  --ignore-keys-regex "^(id|created_at|updated_at)$"
```

### OpenAPI规范比较

比较OpenAPI规范以检查破坏性更改：

```bash
# 比较API版本
diffx openapi-v1.yaml openapi-v2.yaml \
  --path "paths" \
  --output json > api_changes.json

# 验证向后兼容性
diffx api-spec.yaml api-spec.new.yaml \
  --ignore-keys-regex "^(info\\.(version|title)|servers)" \
  --output unified
```

### GraphQL模式验证

比较GraphQL模式：

```bash
# 将GraphQL转换为JSON并比较
graphql-to-json schema-v1.graphql > schema-v1.json
graphql-to-json schema-v2.graphql > schema-v2.json
diffx schema-v1.json schema-v2.json \
  --output yaml
```

## 数据处理和ETL

### 数据流水线验证

验证ETL流水线中的数据转换：

```bash
# 比较输入与输出数据结构
diffx input_data_sample.json output_data_sample.json \
  --array-id-key "record_id" \
  --epsilon 0.001 \
  --output json

# 验证数据迁移
diffx source_schema.json target_schema.json \
  --path "tables" \
  --output yaml
```

### 数据质量检查

监控流水线各阶段的数据质量：

```bash
# 比较数据快照
diffx data_snapshot_t1.json data_snapshot_t2.json \
  --ignore-keys-regex "^(timestamp|batch_id|process_time)" \
  --array-id-key "id" \
  --epsilon 0.01

# 验证聚合结果
diffx daily_metrics.json expected_metrics.json \
  --epsilon 0.05 \
  --output json
```

### 配置驱动ETL

比较ETL配置文件：

```bash
# 比较数据源配置
diffx etl_config_staging.yaml etl_config_prod.yaml \
  --ignore-keys-regex "^(credentials|connection_string)" \
  --path "data_sources"

# 验证转换规则
diffx transform_rules_v1.json transform_rules_v2.json \
  --array-id-key "rule_id"
```

## 数据库管理

### 模式迁移验证

验证数据库模式更改：

```bash
# 比较数据库模式
pg_dump --schema-only mydb > schema_before.sql
# 运行迁移
pg_dump --schema-only mydb > schema_after.sql

# 转换为JSON进行比较（使用自定义脚本）
sql-to-json schema_before.sql > schema_before.json
sql-to-json schema_after.sql > schema_after.json

diffx schema_before.json schema_after.json \
  --array-id-key "table_name" \
  --output json > migration_report.json
```

### 数据备份验证

验证备份完整性：

```bash
# 比较当前数据与备份
diffx production_export.json backup_export.json \
  --array-id-key "id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(last_updated|backup_timestamp)"
```

### 数据库配置管理

比较数据库配置文件：

```bash
# 比较PostgreSQL配置
diffx postgresql.conf postgresql.conf.backup \
  --format ini \
  --ignore-keys-regex "^(log_.*|shared_preload_libraries)"

# 比较MongoDB配置
diffx mongod.conf mongod.conf.new \
  --format yaml \
  --path "storage"
```

## 监控和报警

### 配置漂移检测

监控生产环境中的配置更改：

```bash
# 计划配置检查
#!/bin/bash
# check_config_drift.sh

EXPECTED_CONFIG="/opt/app/config/expected.json"
CURRENT_CONFIG="/opt/app/config/current.json"

# 获取当前配置
curl -s http://localhost:8080/api/config > "$CURRENT_CONFIG"

# 与预期配置比较
if diffx "$EXPECTED_CONFIG" "$CURRENT_CONFIG" \
   --ignore-keys-regex "^(timestamp|uptime|last_.*)" \
   --output json > config_drift.json; then
  echo "未检测到配置漂移"
else
  echo "检测到配置漂移！"
  cat config_drift.json
  # 发送警报
  alert-manager send --file config_drift.json
fi
```

### 服务健康监控

监控服务健康配置：

```bash
# 比较健康检查配置
diffx health_config_baseline.json health_config_current.json \
  --ignore-keys-regex "^(last_check|status_timestamp)" \
  --output json

# 验证监控规则
diffx prometheus_rules.yaml prometheus_rules.new.yaml \
  --path "groups" \
  --output unified
```

### 报警配置管理

管理警报规则更改：

```bash
# 比较警报管理器配置
diffx alertmanager.yml alertmanager.new.yml \
  --path "route" \
  --output yaml

# 验证Grafana仪表板更改
diffx dashboard_v1.json dashboard_v2.json \
  --ignore-keys-regex "^(id|uid|version|time)" \
  --path "panels"
```

## 软件开发

### 包依赖跟踪

跟踪包依赖的更改：

```bash
# 比较包文件
diffx package.json package.json.backup \
  --ignore-keys-regex "^(name|description|author)" \
  --path "dependencies"

# 比较锁定文件
diffx yarn.lock yarn.lock.backup \
  --output json > dependency_changes.json

# 比较Python要求
diffx requirements.txt requirements.new.txt \
  --format ini  # 作为键值对处理
```

### 构建配置更改

监控构建配置更改：

```bash
# 比较webpack配置
diffx webpack.config.js webpack.config.new.js \
  --format json \
  --output unified

# 比较Cargo.toml文件
diffx Cargo.toml Cargo.toml.backup \
  --format toml \
  --ignore-keys-regex "^(build|publish)"
```

### 代码质量配置

跟踪代码质量工具配置：

```bash
# 比较ESLint配置
diffx .eslintrc.json .eslintrc.new.json \
  --path "rules" \
  --output json

# 比较测试配置
diffx jest.config.js jest.config.new.js \
  --format json \
  --path "testMatch"
```

## 安全与合规

### 安全配置审计

审计安全配置：

```bash
# 比较安全策略
diffx security_policy_v1.json security_policy_v2.json \
  --path "permissions" \
  --output json > security_changes.json

# 验证IAM配置
diffx iam_policy_prod.json iam_policy_staging.json \
  --ignore-keys-regex "^(arn|account_id)" \
  --output yaml
```

### 合规监控

监控合规相关配置：

```bash
# 比较GDPR合规配置
diffx gdpr_config.json gdpr_config.new.json \
  --path "data_retention" \
  --output json

# 验证SOX合规性
diffx sox_controls.yaml sox_controls.updated.yaml \
  --array-id-key "control_id" \
  --output unified
```

### 访问控制验证

验证访问控制更改：

```bash
# 比较RBAC配置
diffx rbac_roles.yaml rbac_roles.new.yaml \
  --array-id-key "name" \
  --path "rules" \
  --output json

# 验证OAuth配置
diffx oauth_config.json oauth_config.backup.json \
  --ignore-keys-regex "^(client_secret|private_key)"
```

## UNIX命令模式

diffx为结构化数据提供了常见UNIX diff模式的等效功能：

### `diff -q` 等效：快速文件更改检测

```bash
# 检查配置文件是否不同（仅退出代码）
if ! diffx config.json config.backup.json --quiet; then
  echo "配置已更改，触发部署"
  deploy_app.sh
fi

# 批量检查多个配置文件
for config in configs/*.json; do
  if ! diffx "$config" "backups/$(basename $config)" --quiet; then
    echo "$(basename $config) 已更改"
  fi
done

# 使用watch进行持续监控
watch -n 30 'diffx live_config.json baseline_config.json --quiet || echo "检测到配置漂移！"'
```

### `diff --brief` 等效：仅显示已更改的文件名

```bash
# 快速目录扫描以查找更改的文件
diffx config_dir/ backup_dir/ --recursive --brief

# 在CI流水线中用于更改检测
changed_files=$(diffx current_configs/ previous_configs/ --recursive --brief)
if [ -n "$changed_files" ]; then
  echo "检测到配置更改："
  echo "$changed_files"
  trigger_validation_pipeline.sh
fi

# 结合find进行选择性检查
find . -name "*.json" -newer last_deploy.marker | while read file; do
  backup_file="backups/${file}"
  if [ -f "$backup_file" ]; then
    diffx "$file" "$backup_file" --brief
  fi
done
```

### `diff -i` 等效：大小写不敏感比较

```bash
# 忽略枚举值的大小写差异来比较API响应
curl -s https://api.example.com/status > current_status.json
diffx expected_status.json current_status.json --ignore-case

# 大小写不一致的配置文件
diffx config_template.yaml user_config.yaml \
  --ignore-case \
  --ignore-keys-regex "^(name|description)"

# 字段名大小写混合的数据库配置
diffx db_schema.json migrated_schema.json \
  --ignore-case \
  --array-id-key "table_name" \
  --output json
```

### `diff -w` 等效：忽略空白差异

```bash
# 比较可能具有不同格式的JSON文件
diffx api_response_pretty.json api_response_minified.json --ignore-whitespace

# 忽略配置中的格式差异
diffx config.json config_reformatted.json \
  --ignore-whitespace \
  --output json

# 比较具有空白变化的数据导出
diffx data_export.json data_import_processed.json \
  --ignore-whitespace \
  --array-id-key "id" \
  --epsilon 0.001
```

### `diff -C3` 等效：统一输出中的上下文行

```bash
# 在更改周围显示3行上下文
diffx large_config.json large_config_new.json \
  --output unified \
  --context 3

# 针对性差异的最小上下文
diffx api_schema.json api_schema_v2.json \
  --output unified \
  --context 1

# 仅更改视图的无上下文
diffx database_config.json database_config_updated.json \
  --output unified \
  --context 0
```

### 组合UNIX风格模式

```bash
# 等效于：diff -qiw file1 file2
diffx config.json config.backup.json \
  --quiet \
  --ignore-case \
  --ignore-whitespace

# 等效于：diff -r --brief dir1/ dir2/
diffx config_dir/ backup_dir/ \
  --recursive \
  --brief

# 高级模式：大小写不敏感与选择性字段忽略
diffx user_data.json user_data_migrated.json \
  --ignore-case \
  --ignore-whitespace \
  --ignore-keys-regex "^(created_at|updated_at|timestamp)" \
  --array-id-key "user_id"

# Git风格工作流集成
git show HEAD:config.json > /tmp/old_config.json
diffx /tmp/old_config.json config.json \
  --ignore-whitespace \
  --context 2 \
  --output unified
```

### Shell集成示例

```bash
# 预提交钩子等效
#!/bin/bash
# .git/hooks/pre-commit
if ! diffx config/production.json config/staging.json \
   --ignore-keys-regex "^(environment|debug)" \
   --quiet; then
  echo "生产和准备环境配置存在语义差异"
  echo "提交前请检查："
  diffx config/production.json config/staging.json \
    --ignore-keys-regex "^(environment|debug)"
  exit 1
fi

# 部署验证脚本
#!/bin/bash
# validate_deploy.sh
deployment_config="$1"
baseline_config="configs/baseline.json"

if diffx "$baseline_config" "$deployment_config" \
   --ignore-case \
   --ignore-whitespace \
   --quiet; then
  echo "✅ 配置未更改 - 可安全部署"
  exit 0
else
  echo "⚠️  检测到配置更改："
  diffx "$baseline_config" "$deployment_config" \
    --ignore-case \
    --ignore-whitespace \
    --brief
  echo "继续部署？(y/N)"
  read -r response
  [[ "$response" =~ ^[Yy]$ ]] || exit 1
fi

# 带报警的监控脚本
#!/bin/bash
# monitor_config_drift.sh
while true; do
  if ! diffx /etc/app/config.json /opt/app/expected_config.json \
     --ignore-keys-regex "^(hostname|instance_id|last_update)" \
     --quiet; then
    
    # 发送详细警报
    diffx /etc/app/config.json /opt/app/expected_config.json \
      --ignore-keys-regex "^(hostname|instance_id|last_update)" \
      --output json | \
      curl -X POST https://alerts.example.com/webhook \
           -H "Content-Type: application/json" \
           -d @-
  fi
  sleep 300  # 每5分钟检查一次
done
```

## 高级使用模式

### 多环境流水线

全面的环境比较流水线：

```bash
#!/bin/bash
# multi_env_compare.sh

ENVIRONMENTS=("dev" "staging" "prod")
BASE_ENV="prod"

for env in "${ENVIRONMENTS[@]}"; do
  if [ "$env" != "$BASE_ENV" ]; then
    echo "正在比较 $env 与 $BASE_ENV"
    
    # 应用程序配置
    diffx "config/$BASE_ENV.json" "config/$env.json" \
      --ignore-keys-regex "^(host|port|database|secret_.*)" \
      --output json > "diff_${env}_${BASE_ENV}_app.json"
    
    # 基础设施配置  
    diffx "infra/$BASE_ENV.yaml" "infra/$env.yaml" \
      --path "resources" \
      --output json > "diff_${env}_${BASE_ENV}_infra.json"
    
    # 生成摘要报告
    generate_report.py "diff_${env}_${BASE_ENV}_*.json" > "report_${env}.html"
  fi
done
```

### 数据迁移验证

完整的数据迁移验证工作流：

```bash
#!/bin/bash
# data_migration_validation.sh

SOURCE_DB="legacy_system"
TARGET_DB="new_system"

# 导出模式
export_schema.py "$SOURCE_DB" > source_schema.json
export_schema.py "$TARGET_DB" > target_schema.json

# 比较模式
diffx source_schema.json target_schema.json \
  --array-id-key "table_name" \
  --output json > schema_diff.json

# 导出样本数据
export_sample_data.py "$SOURCE_DB" > source_data.json
export_sample_data.py "$TARGET_DB" > target_data.json

# 比较数据结构
diffx source_data.json target_data.json \
  --array-id-key "id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(migrated_at|batch_id)" \
  --output json > data_diff.json

# 生成迁移报告
generate_migration_report.py schema_diff.json data_diff.json
```

### 自动化测试集成

与自动化测试框架的集成：

```bash
# test_api_contract.sh
#!/bin/bash

API_BASE="https://api.example.com"
EXPECTED_DIR="tests/fixtures/api_responses"

# 测试多个端点
endpoints=("users" "products" "orders")

for endpoint in "${endpoints[@]}"; do
  echo "正在测试 $endpoint 端点..."
  
  # 获取实际响应
  curl -s "$API_BASE/$endpoint" > "actual_$endpoint.json"
  
  # 与预期值比较
  if diffx "$EXPECTED_DIR/$endpoint.json" "actual_$endpoint.json" \
     --ignore-keys-regex "^(timestamp|request_id)" \
     --output json > "diff_$endpoint.json"; then
    echo "✅ $endpoint 匹配预期结构"
  else
    echo "❌ $endpoint 有意外更改"
    cat "diff_$endpoint.json"
    exit 1
  fi
done

echo "所有API合约测试通过！"
```

## 性能优化示例

### 大文件处理

为大文件优化diffx：

```bash
# 大配置文件
diffx large_config.json large_config.new.json \
  --path "critical.services" \
  --ignore-keys-regex "^(logs|metrics|debug_.*)" \
  --output json

# 批量处理多个文件
find configs/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup --output json > {}.diff || echo "在 {} 中发现差异"'
```

### 内存高效处理

高效处理大数据集：

```bash
# 流处理（概念性）
diffx --stream large_dataset_v1.json large_dataset_v2.json \
  --array-id-key "id" \
  --chunk-size 1000 \
  --output json
```

这些示例展示了 `diffx` 在各种行业和使用场景中的多功能性和强大功能。每个示例都包含实用的命令、示例数据和预期输出，以帮助您将其适应到您的特定需求。
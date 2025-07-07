# 实际应用示例

本指南提供了在实际场景中使用 `diffx` 的实用示例，按用例和行业组织。

## 目录

- [配置管理](#配置管理)
- [DevOps 和基础设施](#devops-和基础设施)
- [API 开发和测试](#api-开发和测试)
- [数据处理和 ETL](#数据处理和-etl)
- [数据库管理](#数据库管理)
- [监控和告警](#监控和告警)
- [软件开发](#软件开发)
- [安全和合规](#安全和合规)

## 配置管理

### 环境配置比较

比较不同环境的配置：

```bash
# 开发环境 vs 生产环境
diffx config/dev.json config/prod.json \
  --ignore-keys-regex "^(host|port|password|secret_.*)" \
  --output json > env_diff.json

# 部署前的预发布环境验证
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

### Kubernetes 配置漂移检测

监控 Kubernetes 部署中的配置漂移：

```bash
# 比较当前部署与期望状态
kubectl get deployment myapp -o json > current-deployment.json
diffx desired-deployment.json current-deployment.json \
  --ignore-keys-regex "^(metadata\\.(creationTimestamp|resourceVersion|uid)|status\\..*)" \
  --output json
```

### Docker Compose 环境变化

比较不同环境的 Docker Compose 文件：

```bash
# 比较基础 compose 与覆盖文件
diffx docker-compose.yml docker-compose.override.yml \
  --path "services" \
  --output unified
```

## DevOps 和基础设施

### Terraform 状态比较

比较 Terraform 状态文件以检测基础设施漂移：

```bash
# 比较当前状态与备份
diffx terraform.tfstate terraform.tfstate.backup \
  --path "resources" \
  --ignore-keys-regex "^(last_updated|timeouts)" \
  --output json > infrastructure_drift.json

# 比较计划的变更
terraform show -json plan.out > planned.json
diffx current_state.json planned.json \
  --path "planned_values.root_module"
```

### 基础设施即代码验证

部署前验证基础设施变更：

```bash
# 比较 CloudFormation 模板
diffx infrastructure/base.yaml infrastructure/updated.yaml \
  --ignore-keys-regex "^(Metadata|Description)" \
  --output yaml

# 比较 Ansible playbooks
diffx playbook-v1.yml playbook-v2.yml \
  --path "tasks" \
  --output cli
```

### CI/CD 管道配置

监控 CI/CD 管道配置变更：

```bash
# GitHub Actions 工作流比较
diffx .github/workflows/ci.yml .github/workflows/ci.new.yml \
  --output unified

# GitLab CI 比较
diffx .gitlab-ci.yml .gitlab-ci.backup.yml \
  --ignore-keys-regex "^(variables\\.CI_.*)"
```

## API 开发和测试

### API 响应验证

验证 API 响应与预期模式：

```bash
# 比较 API 响应与预期结构
curl -s https://api.example.com/v1/users/123 > actual_response.json
diffx expected_user_response.json actual_response.json \
  --ignore-keys-regex "^(timestamp|request_id|server_time)" \
  --output json

# 验证 API 端点变更
diffx api/v1/schema.json api/v2/schema.json \
  --path "definitions" \
  --output yaml
```

**示例 API 验证：**
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

### OpenAPI 规范比较

比较 OpenAPI 规范以检测破坏性变更：

```bash
# 比较 API 版本
diffx openapi-v1.yaml openapi-v2.yaml \
  --path "paths" \
  --output json > api_changes.json

# 验证向后兼容性
diffx api-spec.yaml api-spec.new.yaml \
  --ignore-keys-regex "^(info\\.(version|title)|servers)" \
  --output unified
```

### GraphQL 模式验证

比较 GraphQL 模式：

```bash
# 将 GraphQL 转换为 JSON 并比较
graphql-to-json schema-v1.graphql > schema-v1.json
graphql-to-json schema-v2.graphql > schema-v2.json
diffx schema-v1.json schema-v2.json \
  --output yaml
```

## 数据处理和 ETL

### 数据管道验证

验证 ETL 管道中的数据转换：

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

监控管道各阶段的数据质量：

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

### 配置驱动的 ETL

比较 ETL 配置文件：

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

验证数据库模式变更：

```bash
# 比较数据库模式
pg_dump --schema-only mydb > schema_before.sql
# 运行迁移
pg_dump --schema-only mydb > schema_after.sql

# 转换为 JSON 进行比较（使用自定义脚本）
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
# 比较 PostgreSQL 配置
diffx postgresql.conf postgresql.conf.backup \
  --format ini \
  --ignore-keys-regex "^(log_.*|shared_preload_libraries)"

# 比较 MongoDB 配置
diffx mongod.conf mongod.conf.new \
  --format yaml \
  --path "storage"
```

## 监控和告警

### 配置漂移检测

监控生产环境中的配置变更：

```bash
# 定时配置检查
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
  # 发送告警
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

### 告警配置管理

管理告警规则变更：

```bash
# 比较告警管理器配置
diffx alertmanager.yml alertmanager.new.yml \
  --path "route" \
  --output yaml

# 验证 Grafana 仪表板变更
diffx dashboard_v1.json dashboard_v2.json \
  --ignore-keys-regex "^(id|uid|version|time)" \
  --path "panels"
```

## 软件开发

### 包依赖跟踪

跟踪包依赖变更：

```bash
# 比较包文件
diffx package.json package.json.backup \
  --ignore-keys-regex "^(name|description|author)" \
  --path "dependencies"

# 比较锁文件
diffx yarn.lock yarn.lock.backup \
  --output json > dependency_changes.json

# 比较 Python 需求
diffx requirements.txt requirements.new.txt \
  --format ini  # 作为键值对处理
```

### 构建配置变更

监控构建配置变更：

```bash
# 比较 webpack 配置
diffx webpack.config.js webpack.config.new.js \
  --format json \
  --output unified

# 比较 Cargo.toml 文件
diffx Cargo.toml Cargo.toml.backup \
  --format toml \
  --ignore-keys-regex "^(build|publish)"
```

### 代码质量配置

跟踪代码质量工具配置：

```bash
# 比较 ESLint 配置
diffx .eslintrc.json .eslintrc.new.json \
  --path "rules" \
  --output json

# 比较测试配置
diffx jest.config.js jest.config.new.js \
  --format json \
  --path "testMatch"
```

## 安全和合规

### 安全配置审计

审计安全配置：

```bash
# 比较安全策略
diffx security_policy_v1.json security_policy_v2.json \
  --path "permissions" \
  --output json > security_changes.json

# 验证 IAM 配置
diffx iam_policy_prod.json iam_policy_staging.json \
  --ignore-keys-regex "^(arn|account_id)" \
  --output yaml
```

### 合规监控

监控合规相关配置：

```bash
# 比较 GDPR 合规配置
diffx gdpr_config.json gdpr_config.new.json \
  --path "data_retention" \
  --output json

# 验证 SOX 合规
diffx sox_controls.yaml sox_controls.updated.yaml \
  --array-id-key "control_id" \
  --output unified
```

### 访问控制验证

验证访问控制变更：

```bash
# 比较 RBAC 配置
diffx rbac_roles.yaml rbac_roles.new.yaml \
  --array-id-key "name" \
  --path "rules" \
  --output json

# 验证 OAuth 配置
diffx oauth_config.json oauth_config.backup.json \
  --ignore-keys-regex "^(client_secret|private_key)"
```

## 高级使用模式

### 多环境管道

综合环境比较管道：

```bash
#!/bin/bash
# multi_env_compare.sh

ENVIRONMENTS=("dev" "staging" "prod")
BASE_ENV="prod"

for env in "${ENVIRONMENTS[@]}"; do
  if [ "$env" != "$BASE_ENV" ]; then
    echo "比较 $env 与 $BASE_ENV"
    
    # 应用配置
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

# 导出示例数据
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

与自动化测试框架集成：

```bash
# test_api_contract.sh
#!/bin/bash

API_BASE="https://api.example.com"
EXPECTED_DIR="tests/fixtures/api_responses"

# 测试多个端点
endpoints=("users" "products" "orders")

for endpoint in "${endpoints[@]}"; do
  echo "测试 $endpoint 端点..."
  
  # 获取实际响应
  curl -s "$API_BASE/$endpoint" > "actual_$endpoint.json"
  
  # 与预期比较
  if diffx "$EXPECTED_DIR/$endpoint.json" "actual_$endpoint.json" \
     --ignore-keys-regex "^(timestamp|request_id)" \
     --output json > "diff_$endpoint.json"; then
    echo "✅ $endpoint 匹配预期结构"
  else
    echo "❌ $endpoint 有意外变更"
    cat "diff_$endpoint.json"
    exit 1
  fi
done

echo "所有 API 合约测试通过！"
```

## 性能优化示例

### 大文件处理

为大文件优化 diffx：

```bash
# 大型配置文件
diffx large_config.json large_config.new.json \
  --path "critical.services" \
  --ignore-keys-regex "^(logs|metrics|debug_.*)" \
  --output json

# 批处理多个文件
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

这些示例展示了 `diffx` 在各种行业和用例中的多功能性和强大功能。每个示例都包含实用的命令、示例数据和预期输出，帮助您将它们适配到您的特定需求。
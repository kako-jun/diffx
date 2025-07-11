# 实际使用示例

本指南提供在真实场景中使用 `diffx` 的实用示例，按用例和行业组织。

## 目录

- [配置管理](#配置管理)
- [DevOps 和基础设施](#devops-和基础设施)
- [API 开发和测试](#api-开发和测试)
- [数据处理和 ETL](#数据处理和-etl)
- [数据库管理](#数据库管理)
- [监控和警报](#监控和警报)
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

# 部署前的预发布验证
diffx config/staging.yaml config/prod.yaml \
  --path "application" \
  --output yaml
```

**示例文件:**
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

**预期输出:**
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

### Docker Compose 环境变体

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

# 比较计划更改
terraform show -json plan.out > planned.json
diffx current_state.json planned.json \
  --path "planned_values.root_module"
```

### 基础设施即代码验证

在部署前验证基础设施更改：

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

监控 CI/CD 管道配置更改：

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

根据预期模式验证 API 响应：

```bash
# 将 API 响应与预期结构比较
curl -s https://api.example.com/v1/users/123 > actual_response.json
diffx expected_user_response.json actual_response.json \
  --ignore-keys-regex "^(timestamp|request_id|server_time)" \
  --output json

# 验证 API 端点更改
diffx api/v1/schema.json api/v2/schema.json \
  --path "definitions" \
  --output yaml
```

**示例 API 验证:**
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

比较 OpenAPI 规范以检测破坏性更改：

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
npm install -g graphql-json-schema
graphql-json-schema schema-v1.graphql > schema-v1.json
graphql-json-schema schema-v2.graphql > schema-v2.json
diffx schema-v1.json schema-v2.json \
  --path "types" \
  --output yaml
```

## 数据处理和 ETL

### 数据质量验证

在 ETL 管道中验证数据质量：

```bash
# 比较处理前后的数据
diffx raw_data.json processed_data.json \
  --array-id-key "id" \
  --ignore-keys-regex "^(processed_at|etl_timestamp)" \
  --output json > data_changes.json

# 验证数据转换
diffx input_records.json output_records.json \
  --array-id-key "record_id" \
  --epsilon 0.001 \
  --output unified
```

**示例数据验证:**
```bash
# 验证用户数据处理
echo '[
  {"id": 1, "name": "Alice Smith", "age": 30, "score": 85.5},
  {"id": 2, "name": "Bob Johnson", "age": 25, "score": 92.3}
]' > input_users.json

echo '[
  {"id": 1, "full_name": "Alice Smith", "age_group": "adult", "grade": "B"},
  {"id": 2, "full_name": "Bob Johnson", "age_group": "adult", "grade": "A"}
]' > transformed_users.json

# 验证转换规则
diffx input_users.json transformed_users.json \
  --array-id-key "id" \
  --output json
```

### 数据库迁移验证

验证数据库迁移结果：

```bash
# 比较迁移前后的数据
diffx pre_migration_dump.json post_migration_dump.json \
  --array-id-key "primary_key" \
  --ignore-keys-regex "^(created_at|updated_at|version)" \
  --output json > migration_changes.json

# 验证数据完整性
diffx source_data.json migrated_data.json \
  --array-id-key "id" \
  --epsilon 0.01 \
  --quiet
echo $? # 0 = 数据一致, 1 = 发现差异
```

### 批处理作业验证

监控批处理作业的数据变化：

```bash
# 每日批处理比较
diffx daily_report_$(date -d yesterday +%Y%m%d).json \
     daily_report_$(date +%Y%m%d).json \
  --array-id-key "transaction_id" \
  --ignore-keys-regex "^(report_date|generated_at)" \
  --output json > daily_changes.json

# 验证汇总数据
diffx summary_before.json summary_after.json \
  --epsilon 0.001 \
  --output unified
```

## 数据库管理

### 模式比较

比较数据库模式：

```bash
# 比较表结构
pg_dump --schema-only mydb_dev > dev_schema.sql
pg_dump --schema-only mydb_prod > prod_schema.sql

# 转换为 JSON 进行比较（使用自定义脚本）
./sql_to_json.sh dev_schema.sql > dev_schema.json
./sql_to_json.sh prod_schema.sql > prod_schema.json

diffx dev_schema.json prod_schema.json \
  --path "tables" \
  --output unified
```

### 数据一致性检查

验证不同环境间的数据一致性：

```bash
# 比较关键表数据
diffx production_users.json staging_users.json \
  --array-id-key "user_id" \
  --ignore-keys-regex "^(last_login|session_token|password_hash)" \
  --output json > user_consistency.json

# 验证引用完整性
diffx prod_orders.json staging_orders.json \
  --array-id-key "order_id" \
  --path "order_items" \
  --output yaml
```

### 备份验证

验证数据库备份的完整性：

```bash
# 比较备份与实时数据
diffx live_data_export.json backup_restore_export.json \
  --array-id-key "id" \
  --ignore-keys-regex "^(backup_timestamp|restore_time)" \
  --output json > backup_verification.json

# 增量备份验证
diffx full_backup.json incremental_backup.json \
  --array-id-key "record_id" \
  --quiet
if [ $? -eq 0 ]; then
    echo "增量备份验证成功"
else
    echo "增量备份发现差异"
fi
```

## 监控和警报

### 系统指标比较

比较系统性能指标：

```bash
# 比较不同时间点的指标
diffx metrics_baseline.json metrics_current.json \
  --ignore-keys-regex "^(timestamp|collection_time)" \
  --epsilon 0.05 \
  --output json > performance_drift.json

# 服务健康状况比较
diffx service_health_before.json service_health_after.json \
  --path "services" \
  --output unified
```

### 日志分析

分析日志模式的变化：

```bash
# 比较日志聚合结果
diffx yesterday_log_summary.json today_log_summary.json \
  --ignore-keys-regex "^(date|timestamp)" \
  --output json > log_pattern_changes.json

# 错误率分析
diffx error_stats_week1.json error_stats_week2.json \
  --epsilon 0.001 \
  --output yaml
```

### 容量规划

监控资源使用趋势：

```bash
# 比较资源使用报告
diffx resource_usage_month1.json resource_usage_month2.json \
  --path "clusters" \
  --epsilon 0.01 \
  --output json > capacity_trends.json

# 存储增长分析
diffx storage_report_q1.json storage_report_q2.json \
  --array-id-key "volume_id" \
  --output unified
```

## 软件开发

### 配置文件版本控制

跟踪配置文件的变化：

```bash
# 功能分支配置比较
diffx main_config.json feature_config.json \
  --ignore-keys-regex "^(developer|debug_.*)" \
  --output unified

# 发布配置验证
diffx release_v1.json release_v2.json \
  --path "features" \
  --output json > release_changes.json
```

### 依赖关系管理

比较项目依赖：

```bash
# Node.js 依赖比较
diffx package-lock.json package-lock.new.json \
  --path "dependencies" \
  --output json > dependency_changes.json

# Python 依赖比较
pip freeze > requirements_current.txt
pip list --format=json > requirements_current.json
diffx requirements_baseline.json requirements_current.json \
  --array-id-key "name" \
  --output yaml
```

### 测试结果分析

分析测试结果的变化：

```bash
# 比较测试报告
diffx test_results_baseline.json test_results_current.json \
  --path "test_suites" \
  --ignore-keys-regex "^(execution_time|timestamp)" \
  --output json > test_changes.json

# 代码覆盖率比较
diffx coverage_before.json coverage_after.json \
  --path "files" \
  --epsilon 0.01 \
  --output unified
```

## 安全和合规

### 安全策略验证

验证安全配置：

```bash
# 比较防火墙规则
diffx firewall_rules_baseline.json firewall_rules_current.json \
  --array-id-key "rule_id" \
  --output json > security_changes.json

# IAM 策略比较
diffx iam_policies_v1.json iam_policies_v2.json \
  --path "policies" \
  --output yaml
```

### 合规性检查

确保配置符合合规要求：

```bash
# 审计配置更改
diffx compliant_config.json current_config.json \
  --ignore-keys-regex "^(last_modified|auditor)" \
  --output json > compliance_violations.json

# 安全基线验证
diffx security_baseline.json production_config.json \
  --path "security_settings" \
  --output unified
```

### 漏洞扫描结果比较

跟踪安全漏洞修复进度：

```bash
# 比较扫描结果
diffx vulnerability_scan_before.json vulnerability_scan_after.json \
  --array-id-key "cve_id" \
  --ignore-keys-regex "^(scan_date|scanner_version)" \
  --output json > vulnerability_changes.json

# 依赖漏洞分析
diffx security_audit_baseline.json security_audit_current.json \
  --path "vulnerabilities" \
  --output yaml
```

## 高级使用模式

### 批量文件处理

处理多个文件的批量比较：

```bash
# 批量配置文件比较
for env in dev staging prod; do
    echo "Comparing $env environment..."
    diffx config/base.json config/$env.json \
        --ignore-keys-regex "^(environment|debug)" \
        --output json > diff_$env.json
done

# 目录递归比较
diffx config_v1/ config_v2/ \
  --recursive \
  --ignore-keys-regex "^(timestamp|version)" \
  --output json > directory_changes.json
```

### 自动化工作流集成

将 diffx 集成到自动化工作流中：

```bash
#!/bin/bash
# 部署验证脚本

# 1. 获取当前配置
kubectl get configmap myapp-config -o json > current_config.json

# 2. 与期望配置比较
diffx expected_config.json current_config.json \
  --ignore-keys-regex "^(metadata\\..*)" \
  --quiet

# 3. 根据退出代码采取行动
if [ $? -eq 0 ]; then
    echo "配置一致，继续部署"
elif [ $? -eq 1 ]; then
    echo "配置漂移检测，生成报告"
    diffx expected_config.json current_config.json \
      --output json > config_drift_report.json
    # 发送警报或创建工单
else
    echo "配置比较失败"
    exit 1
fi
```

### 数据管道监控

在数据管道中监控数据质量：

```bash
# ETL 管道质量检查
#!/bin/bash
PIPELINE_NAME="user_analytics"
INPUT_DATA="input_$(date +%Y%m%d).json"
OUTPUT_DATA="output_$(date +%Y%m%d).json"
EXPECTED_SCHEMA="expected_schema.json"

# 验证输出数据结构
diffx $EXPECTED_SCHEMA $OUTPUT_DATA \
  --path "schema" \
  --quiet

if [ $? -ne 0 ]; then
    echo "数据结构验证失败"
    diffx $EXPECTED_SCHEMA $OUTPUT_DATA \
      --path "schema" \
      --output json > schema_validation_errors.json
    # 触发数据质量警报
fi

# 验证数据完整性
RECORD_COUNT_INPUT=$(jq 'length' $INPUT_DATA)
RECORD_COUNT_OUTPUT=$(jq 'length' $OUTPUT_DATA)

if [ $RECORD_COUNT_INPUT -ne $RECORD_COUNT_OUTPUT ]; then
    echo "数据记录数不匹配: 输入 $RECORD_COUNT_INPUT, 输出 $RECORD_COUNT_OUTPUT"
fi
```

## 最佳实践

### 性能优化

1. **使用适当的过滤器**
   ```bash
   # 专注于相关更改
   diffx large_file1.json large_file2.json \
     --path "critical_section" \
     --ignore-keys-regex "^(debug_|temp_)"
   ```

2. **为大型数据集使用数组ID**
   ```bash
   # 高效的数组比较
   diffx users_old.json users_new.json \
     --array-id-key "user_id"
   ```

3. **利用退出代码进行脚本编写**
   ```bash
   # 条件逻辑
   if diffx file1.json file2.json --quiet; then
       echo "无变化"
   else
       echo "检测到变化"
   fi
   ```

### 错误处理

1. **验证文件格式**
   ```bash
   # 确保正确的格式解释
   diffx --format json file1.txt file2.txt
   ```

2. **处理大文件**
   ```bash
   # 内存效率
   diffx large1.json large2.json   ```

3. **调试比较问题**
   ```bash
   # 详细输出用于调试
   diffx problematic1.json problematic2.json \
     --output unified \
     --context 5
   ```

### 集成模式

1. **CI/CD 集成**
   ```yaml
   # GitHub Actions 示例
   - name: 验证配置更改
     run: |
       diffx expected_config.json actual_config.json \
         --quiet || exit 1
   ```

2. **监控集成**
   ```bash
   # 定期配置漂移检查
   */15 * * * * diffx baseline.json current.json --quiet || \
     echo "配置漂移检测" | mail -s "警报" admin@example.com
   ```

3. **数据质量管道**
   ```bash
   # 数据验证步骤
   diffx expected_output.json actual_output.json \
     --array-id-key "id" \
     --epsilon 0.001 \
     --output json > quality_report.json
   ```

---

**提示**: 这些示例可以根据您的具体需求进行调整。有关更多选项和高级功能，请参阅 [CLI 参考](../reference/cli-reference_zh.md)。
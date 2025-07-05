# Real-World Examples

This guide provides practical examples of using `diffx` in real-world scenarios, organized by use case and industry.

## Table of Contents

- [Configuration Management](#configuration-management)
- [DevOps and Infrastructure](#devops-and-infrastructure)
- [API Development and Testing](#api-development-and-testing)
- [Data Processing and ETL](#data-processing-and-etl)
- [Database Management](#database-management)
- [Monitoring and Alerting](#monitoring-and-alerting)
- [Software Development](#software-development)
- [Security and Compliance](#security-and-compliance)

## Configuration Management

### Environment Configuration Comparison

Compare configurations across different environments:

```bash
# Development vs Production
diffx config/dev.json config/prod.json \
  --ignore-keys-regex "^(host|port|password|secret_.*)" \
  --output json > env_diff.json

# Staging validation before deployment
diffx config/staging.yaml config/prod.yaml \
  --path "application" \
  --output yaml
```

**Sample files:**
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

**Expected output:**
```
~ application.debug: true -> false
```

### Kubernetes Configuration Drift Detection

Monitor configuration drift in Kubernetes deployments:

```bash
# Compare current deployment with desired state
kubectl get deployment myapp -o json > current-deployment.json
diffx desired-deployment.json current-deployment.json \
  --ignore-keys-regex "^(metadata\\.(creationTimestamp|resourceVersion|uid)|status\\..*)" \
  --output json
```

### Docker Compose Environment Variations

Compare Docker Compose files for different environments:

```bash
# Compare base compose with override
diffx docker-compose.yml docker-compose.override.yml \
  --path "services" \
  --output unified
```

## DevOps and Infrastructure

### Terraform State Comparison

Compare Terraform state files to detect infrastructure drift:

```bash
# Compare current state with backup
diffx terraform.tfstate terraform.tfstate.backup \
  --path "resources" \
  --ignore-keys-regex "^(last_updated|timeouts)" \
  --output json > infrastructure_drift.json

# Compare planned changes
terraform show -json plan.out > planned.json
diffx current_state.json planned.json \
  --path "planned_values.root_module"
```

### Infrastructure as Code Validation

Validate infrastructure changes before deployment:

```bash
# Compare CloudFormation templates
diffx infrastructure/base.yaml infrastructure/updated.yaml \
  --ignore-keys-regex "^(Metadata|Description)" \
  --output yaml

# Compare Ansible playbooks
diffx playbook-v1.yml playbook-v2.yml \
  --path "tasks" \
  --output cli
```

### CI/CD Pipeline Configuration

Monitor CI/CD pipeline configuration changes:

```bash
# GitHub Actions workflow comparison
diffx .github/workflows/ci.yml .github/workflows/ci.new.yml \
  --output unified

# GitLab CI comparison
diffx .gitlab-ci.yml .gitlab-ci.backup.yml \
  --ignore-keys-regex "^(variables\\.CI_.*)"
```

## API Development and Testing

### API Response Validation

Validate API responses against expected schemas:

```bash
# Compare API response with expected structure
curl -s https://api.example.com/v1/users/123 > actual_response.json
diffx expected_user_response.json actual_response.json \
  --ignore-keys-regex "^(timestamp|request_id|server_time)" \
  --output json

# Validate API endpoint changes
diffx api/v1/schema.json api/v2/schema.json \
  --path "definitions" \
  --output yaml
```

**Sample API validation:**
```bash
# Test user creation endpoint
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

### OpenAPI Specification Comparison

Compare OpenAPI specifications for breaking changes:

```bash
# Compare API versions
diffx openapi-v1.yaml openapi-v2.yaml \
  --path "paths" \
  --output json > api_changes.json

# Validate backwards compatibility
diffx api-spec.yaml api-spec.new.yaml \
  --ignore-keys-regex "^(info\\.(version|title)|servers)" \
  --output unified
```

### GraphQL Schema Validation

Compare GraphQL schemas:

```bash
# Convert GraphQL to JSON and compare
graphql-to-json schema-v1.graphql > schema-v1.json
graphql-to-json schema-v2.graphql > schema-v2.json
diffx schema-v1.json schema-v2.json \
  --output yaml
```

## Data Processing and ETL

### Data Pipeline Validation

Validate data transformations in ETL pipelines:

```bash
# Compare input vs output data structure
diffx input_data_sample.json output_data_sample.json \
  --array-id-key "record_id" \
  --epsilon 0.001 \
  --output json

# Validate data migration
diffx source_schema.json target_schema.json \
  --path "tables" \
  --output yaml
```

### Data Quality Checks

Monitor data quality across pipeline stages:

```bash
# Compare data snapshots
diffx data_snapshot_t1.json data_snapshot_t2.json \
  --ignore-keys-regex "^(timestamp|batch_id|process_time)" \
  --array-id-key "id" \
  --epsilon 0.01

# Validate aggregated results
diffx daily_metrics.json expected_metrics.json \
  --epsilon 0.05 \
  --output json
```

### Configuration-Driven ETL

Compare ETL configuration files:

```bash
# Compare data source configurations
diffx etl_config_staging.yaml etl_config_prod.yaml \
  --ignore-keys-regex "^(credentials|connection_string)" \
  --path "data_sources"

# Validate transformation rules
diffx transform_rules_v1.json transform_rules_v2.json \
  --array-id-key "rule_id"
```

## Database Management

### Schema Migration Validation

Validate database schema changes:

```bash
# Compare database schemas
pg_dump --schema-only mydb > schema_before.sql
# Run migrations
pg_dump --schema-only mydb > schema_after.sql

# Convert to JSON for comparison (using custom script)
sql-to-json schema_before.sql > schema_before.json
sql-to-json schema_after.sql > schema_after.json

diffx schema_before.json schema_after.json \
  --array-id-key "table_name" \
  --output json > migration_report.json
```

### Data Backup Verification

Verify backup integrity:

```bash
# Compare current data with backup
diffx production_export.json backup_export.json \
  --array-id-key "id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(last_updated|backup_timestamp)"
```

### Database Configuration Management

Compare database configuration files:

```bash
# Compare PostgreSQL configurations
diffx postgresql.conf postgresql.conf.backup \
  --format ini \
  --ignore-keys-regex "^(log_.*|shared_preload_libraries)"

# Compare MongoDB configurations
diffx mongod.conf mongod.conf.new \
  --format yaml \
  --path "storage"
```

## Monitoring and Alerting

### Configuration Drift Detection

Monitor configuration changes in production:

```bash
# Scheduled configuration check
#!/bin/bash
# check_config_drift.sh

EXPECTED_CONFIG="/opt/app/config/expected.json"
CURRENT_CONFIG="/opt/app/config/current.json"

# Fetch current configuration
curl -s http://localhost:8080/api/config > "$CURRENT_CONFIG"

# Compare with expected
if diffx "$EXPECTED_CONFIG" "$CURRENT_CONFIG" \
   --ignore-keys-regex "^(timestamp|uptime|last_.*)" \
   --output json > config_drift.json; then
  echo "No configuration drift detected"
else
  echo "Configuration drift detected!"
  cat config_drift.json
  # Send alert
  alert-manager send --file config_drift.json
fi
```

### Service Health Monitoring

Monitor service health configurations:

```bash
# Compare health check configurations
diffx health_config_baseline.json health_config_current.json \
  --ignore-keys-regex "^(last_check|status_timestamp)" \
  --output json

# Validate monitoring rules
diffx prometheus_rules.yaml prometheus_rules.new.yaml \
  --path "groups" \
  --output unified
```

### Alert Configuration Management

Manage alerting rule changes:

```bash
# Compare alert manager configurations
diffx alertmanager.yml alertmanager.new.yml \
  --path "route" \
  --output yaml

# Validate Grafana dashboard changes
diffx dashboard_v1.json dashboard_v2.json \
  --ignore-keys-regex "^(id|uid|version|time)" \
  --path "panels"
```

## Software Development

### Package Dependency Tracking

Track changes in package dependencies:

```bash
# Compare package files
diffx package.json package.json.backup \
  --ignore-keys-regex "^(name|description|author)" \
  --path "dependencies"

# Compare lock files
diffx yarn.lock yarn.lock.backup \
  --output json > dependency_changes.json

# Compare Python requirements
diffx requirements.txt requirements.new.txt \
  --format ini  # Treat as key-value pairs
```

### Build Configuration Changes

Monitor build configuration changes:

```bash
# Compare webpack configurations
diffx webpack.config.js webpack.config.new.js \
  --format json \
  --output unified

# Compare Cargo.toml files
diffx Cargo.toml Cargo.toml.backup \
  --format toml \
  --ignore-keys-regex "^(build|publish)"
```

### Code Quality Configuration

Track code quality tool configurations:

```bash
# Compare ESLint configurations
diffx .eslintrc.json .eslintrc.new.json \
  --path "rules" \
  --output json

# Compare test configurations
diffx jest.config.js jest.config.new.js \
  --format json \
  --path "testMatch"
```

## Security and Compliance

### Security Configuration Auditing

Audit security configurations:

```bash
# Compare security policies
diffx security_policy_v1.json security_policy_v2.json \
  --path "permissions" \
  --output json > security_changes.json

# Validate IAM configurations
diffx iam_policy_prod.json iam_policy_staging.json \
  --ignore-keys-regex "^(arn|account_id)" \
  --output yaml
```

### Compliance Monitoring

Monitor compliance-related configurations:

```bash
# Compare GDPR compliance configurations
diffx gdpr_config.json gdpr_config.new.json \
  --path "data_retention" \
  --output json

# Validate SOX compliance
diffx sox_controls.yaml sox_controls.updated.yaml \
  --array-id-key "control_id" \
  --output unified
```

### Access Control Validation

Validate access control changes:

```bash
# Compare RBAC configurations
diffx rbac_roles.yaml rbac_roles.new.yaml \
  --array-id-key "name" \
  --path "rules" \
  --output json

# Validate OAuth configurations
diffx oauth_config.json oauth_config.backup.json \
  --ignore-keys-regex "^(client_secret|private_key)"
```

## Advanced Usage Patterns

### Multi-Environment Pipeline

Comprehensive environment comparison pipeline:

```bash
#!/bin/bash
# multi_env_compare.sh

ENVIRONMENTS=("dev" "staging" "prod")
BASE_ENV="prod"

for env in "${ENVIRONMENTS[@]}"; do
  if [ "$env" != "$BASE_ENV" ]; then
    echo "Comparing $env with $BASE_ENV"
    
    # Application configuration
    diffx "config/$BASE_ENV.json" "config/$env.json" \
      --ignore-keys-regex "^(host|port|database|secret_.*)" \
      --output json > "diff_${env}_${BASE_ENV}_app.json"
    
    # Infrastructure configuration  
    diffx "infra/$BASE_ENV.yaml" "infra/$env.yaml" \
      --path "resources" \
      --output json > "diff_${env}_${BASE_ENV}_infra.json"
    
    # Generate summary report
    generate_report.py "diff_${env}_${BASE_ENV}_*.json" > "report_${env}.html"
  fi
done
```

### Data Migration Validation

Complete data migration validation workflow:

```bash
#!/bin/bash
# data_migration_validation.sh

SOURCE_DB="legacy_system"
TARGET_DB="new_system"

# Export schemas
export_schema.py "$SOURCE_DB" > source_schema.json
export_schema.py "$TARGET_DB" > target_schema.json

# Compare schemas
diffx source_schema.json target_schema.json \
  --array-id-key "table_name" \
  --output json > schema_diff.json

# Export sample data
export_sample_data.py "$SOURCE_DB" > source_data.json
export_sample_data.py "$TARGET_DB" > target_data.json

# Compare data structures
diffx source_data.json target_data.json \
  --array-id-key "id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(migrated_at|batch_id)" \
  --output json > data_diff.json

# Generate migration report
generate_migration_report.py schema_diff.json data_diff.json
```

### Automated Testing Integration

Integration with automated testing frameworks:

```bash
# test_api_contract.sh
#!/bin/bash

API_BASE="https://api.example.com"
EXPECTED_DIR="tests/fixtures/api_responses"

# Test multiple endpoints
endpoints=("users" "products" "orders")

for endpoint in "${endpoints[@]}"; do
  echo "Testing $endpoint endpoint..."
  
  # Fetch actual response
  curl -s "$API_BASE/$endpoint" > "actual_$endpoint.json"
  
  # Compare with expected
  if diffx "$EXPECTED_DIR/$endpoint.json" "actual_$endpoint.json" \
     --ignore-keys-regex "^(timestamp|request_id)" \
     --output json > "diff_$endpoint.json"; then
    echo "✅ $endpoint matches expected structure"
  else
    echo "❌ $endpoint has unexpected changes"
    cat "diff_$endpoint.json"
    exit 1
  fi
done

echo "All API contract tests passed!"
```

## Performance Optimization Examples

### Large File Processing

Optimize diffx for large files:

```bash
# Large configuration files
diffx large_config.json large_config.new.json \
  --path "critical.services" \
  --ignore-keys-regex "^(logs|metrics|debug_.*)" \
  --output json

# Batch processing multiple files
find configs/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup --output json > {}.diff || echo "Diff found in {}"'
```

### Memory-Efficient Processing

Process large datasets efficiently:

```bash
# Stream processing (conceptual)
diffx --stream large_dataset_v1.json large_dataset_v2.json \
  --array-id-key "id" \
  --chunk-size 1000 \
  --output json
```

These examples demonstrate the versatility and power of `diffx` across various industries and use cases. Each example includes practical commands, sample data, and expected outputs to help you adapt them to your specific needs.
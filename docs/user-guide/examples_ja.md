# 実用例集

このガイドでは、`diffx` を実際の業務で活用するための具体的な例を、業界・用途別に整理して提供します。

## 目次

- [設定管理](#設定管理)
- [DevOps・インフラストラクチャ](#devops・インフラストラクチャ)
- [API 開発・テスト](#api-開発・テスト)
- [データ処理・ETL](#データ処理・etl)
- [データベース管理](#データベース管理)
- [監視・アラート](#監視・アラート)
- [ソフトウェア開発](#ソフトウェア開発)
- [セキュリティ・コンプライアンス](#セキュリティ・コンプライアンス)

## 設定管理

### 環境間設定比較

開発環境と本番環境の設定を比較して、意図しない差異を検出：

```bash
# 開発環境と本番環境の比較
diffx config/dev.json config/prod.json \
  --ignore-keys-regex "^(host|port|password|secret_.*)" \
  --output json > env_diff.json
```

**サンプルファイル:**
```json
// config/dev.json
{
  "application": {
    "name": "myapp",
    "version": "1.0.0",
    "debug": true,
    "log_level": "debug"
  },
  "database": {
    "host": "localhost",
    "port": 5432,
    "name": "myapp_dev",
    "ssl": false
  },
  "cache": {
    "enabled": true,
    "ttl": 300
  }
}

// config/prod.json  
{
  "application": {
    "name": "myapp",
    "version": "1.0.0",
    "debug": false,
    "log_level": "info"
  },
  "database": {
    "host": "prod-db.example.com",
    "port": 5432,
    "name": "myapp_prod", 
    "ssl": true
  },
  "cache": {
    "enabled": true,
    "ttl": 3600
  }
}
```

**期待される出力:**
```
~ application.debug: true -> false
~ application.log_level: "debug" -> "info"
+ database.ssl: true
~ cache.ttl: 300 -> 3600
```

### Kubernetes設定ドリフト検出

Kubernetesマニフェストの設定ドリフトを監視：

```bash
# 現在のデプロイメントと期待状態の比較
kubectl get deployment myapp -o json > current-deployment.json
diffx desired-deployment.json current-deployment.json \
  --ignore-keys-regex "^(metadata\\.(creationTimestamp|resourceVersion|uid)|status\\..*)" \
  --output json > k8s_drift.json

# 重要な変更を検出
if jq -e '.[] | select(.Removed or .TypeChanged)' k8s_drift.json > /dev/null; then
  echo "重要な設定変更が検出されました"
  cat k8s_drift.json
fi
```

### Docker Compose環境比較

異なる環境のDocker Compose設定を比較：

```bash
# ベース設定とオーバーライドの比較
diffx docker-compose.yml docker-compose.override.yml \
  --path "services" \
  --output unified > compose_diff.patch

# 本番用設定との比較
diffx docker-compose.yml docker-compose.prod.yml \
  --ignore-keys-regex "^(build|volumes\\.\\d+)" \
  --output yaml
```

## DevOps・インフラストラクチャ

### Terraform状態比較

Terraformの状態ファイルを比較してインフラドリフトを検出：

```bash
# 現在の状態とバックアップの比較
diffx terraform.tfstate terraform.tfstate.backup \
  --path "resources" \
  --ignore-keys-regex "^(last_updated|timeouts)" \
  --output json > infrastructure_drift.json

# 計画された変更の確認
terraform show -json plan.out > planned.json
diffx current_state.json planned.json \
  --path "planned_values.root_module" \
  --output yaml
```

### インフラストラクチャ・アズ・コード検証

デプロイ前の設定検証：

```bash
# CloudFormationテンプレートの比較
diffx infrastructure/base.yaml infrastructure/updated.yaml \
  --ignore-keys-regex "^(Metadata|Description)" \
  --output yaml > cf_changes.yaml

# Ansibleプレイブックの比較
diffx playbook-v1.yml playbook-v2.yml \
  --path "tasks" \
  --output cli

# Helmチャートの比較
helm template myapp ./chart > current_manifest.yaml
helm template myapp ./chart-new > new_manifest.yaml
diffx current_manifest.yaml new_manifest.yaml \
  --ignore-keys-regex "^(metadata\\.(labels\\.chart|labels\\.heritage))"
```

### CI/CDパイプライン設定

CI/CDパイプラインの設定変更を監視：

```bash
# GitHub Actionsワークフローの比較
diffx .github/workflows/ci.yml .github/workflows/ci.new.yml \
  --output unified > workflow_changes.patch

# GitLab CI設定の比較
diffx .gitlab-ci.yml .gitlab-ci.backup.yml \
  --ignore-keys-regex "^(variables\\.CI_.*)" \
  --output json

# Jenkins Pipelineの比較
diffx Jenkinsfile Jenkinsfile.new \
  --format yaml \
  --output cli
```

## API 開発・テスト

### APIレスポンス検証

APIレスポンスが期待される構造に準拠しているかチェック：

```bash
# APIレスポンスと期待値の比較
curl -s https://api.example.com/v1/users/123 > actual_response.json
diffx expected_user_response.json actual_response.json \
  --ignore-keys-regex "^(timestamp|request_id|server_time)" \
  --output json > api_validation.json

# 複数エンドポイントの一括検証
for endpoint in users products orders; do
  echo "=== Testing $endpoint endpoint ==="
  curl -s "https://api.example.com/v1/$endpoint" > "actual_$endpoint.json"
  
  if diffx "expected_$endpoint.json" "actual_$endpoint.json" \
     --ignore-keys-regex "^(id|timestamp|request_id)" \
     --output json > "diff_$endpoint.json"; then
    echo "✅ $endpoint: 構造が一致"
  else
    echo "❌ $endpoint: 予期しない変更"
    cat "diff_$endpoint.json"
  fi
done
```

**サンプル期待レスポンス:**
```json
// expected_user_response.json
{
  "id": 123,
  "name": "John Doe", 
  "email": "john@example.com",
  "profile": {
    "avatar": "https://example.com/avatar.jpg",
    "bio": "Software Engineer"
  },
  "created_at": "2024-01-01T00:00:00Z",
  "updated_at": "2024-01-01T00:00:00Z"
}
```

### OpenAPI仕様比較

OpenAPI仕様の変更を検出して互換性をチェック：

```bash
# API仕様のバージョン間比較
diffx openapi-v1.yaml openapi-v2.yaml \
  --path "paths" \
  --output json > api_changes.json

# 互換性破壊的変更の検出
diffx api-spec.yaml api-spec.new.yaml \
  --ignore-keys-regex "^(info\\.(version|title)|servers)" \
  --output unified > breaking_changes.diff

# スキーマ定義の比較
diffx openapi-v1.yaml openapi-v2.yaml \
  --path "components.schemas" \
  --output yaml
```

### GraphQLスキーマ検証

GraphQLスキーマの変更を追跡：

```bash
# GraphQLスキーマをJSONに変換して比較
graphql-introspection schema-v1.graphql > schema-v1.json
graphql-introspection schema-v2.graphql > schema-v2.json

diffx schema-v1.json schema-v2.json \
  --path "data.__schema.types" \
  --array-id-key "name" \
  --output yaml > schema_changes.yaml
```

## データ処理・ETL

### データパイプライン検証

ETLパイプラインでのデータ変換を検証：

```bash
# 入力データと出力データの構造比較
diffx input_data_sample.json output_data_sample.json \
  --array-id-key "record_id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(processed_at|batch_id|execution_time)" \
  --output json > transformation_validation.json

# データ移行の検証
diffx source_schema.json target_schema.json \
  --path "tables" \
  --array-id-key "table_name" \
  --output yaml
```

**サンプルデータ:**
```json
// input_data_sample.json
[
  {
    "record_id": "R001",
    "customer_name": "田中太郎",
    "order_amount": 15000,
    "currency": "JPY",
    "raw_timestamp": "2024-01-01 09:00:00"
  }
]

// output_data_sample.json
[
  {
    "record_id": "R001", 
    "customer_name": "田中太郎",
    "order_amount": 15000,
    "currency": "JPY",
    "processed_timestamp": "2024-01-01T09:00:00Z",
    "processed_at": "2024-01-01T10:00:00Z",
    "batch_id": "B20240101001"
  }
]
```

### データ品質チェック

データ品質をパイプライン段階間で監視：

```bash
# 日次データの比較
yesterday=$(date -d yesterday +%Y%m%d)
today=$(date +%Y%m%d)

diffx "daily_metrics_$yesterday.json" "daily_metrics_$today.json" \
  --epsilon 0.05 \
  --ignore-keys-regex "^(date|timestamp|execution_.*)" \
  --output json > daily_data_diff.json

# 集計結果の検証
diffx expected_aggregation.json actual_aggregation.json \
  --epsilon 0.01 \
  --output yaml
```

### データ変換設定管理

ETL設定の変更管理：

```bash
# データソース設定の比較
diffx etl_config_staging.yaml etl_config_prod.yaml \
  --ignore-keys-regex "^(credentials|connection_string|host|port)" \
  --path "data_sources" \
  --output json

# 変換ルールの比較
diffx transform_rules_v1.json transform_rules_v2.json \
  --array-id-key "rule_id" \
  --output cli
```

## データベース管理

### スキーマ移行検証

データベーススキーマの変更を追跡：

```bash
# スキーマ移行前後の比較
pg_dump --schema-only mydb > schema_before.sql
# マイグレーション実行
pg_dump --schema-only mydb > schema_after.sql

# カスタムスクリプトでSQL→JSON変換
sql-to-json schema_before.sql > schema_before.json
sql-to-json schema_after.sql > schema_after.json

diffx schema_before.json schema_after.json \
  --array-id-key "table_name" \
  --output json > migration_report.json
```

### データバックアップ検証

バックアップの整合性確認：

```bash
# 本番データとバックアップの比較
diffx production_export.json backup_export.json \
  --array-id-key "id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(last_updated|backup_timestamp|export_time)" \
  --output json > backup_validation.json

# データ一貫性チェック
diffx current_snapshot.json previous_snapshot.json \
  --array-id-key "primary_key" \
  --output cli
```

### データベース設定管理

データベース設定ファイルの比較：

```bash
# PostgreSQL設定の比較
diffx postgresql.conf postgresql.conf.backup \
  --format ini \
  --ignore-keys-regex "^(log_.*|shared_preload_libraries)" \
  --output unified

# MongoDB設定の比較
diffx mongod.conf mongod.conf.new \
  --format yaml \
  --path "storage" \
  --output json

# MySQL設定の比較
diffx my.cnf my.cnf.tuned \
  --format ini \
  --ignore-keys-regex "^(innodb_buffer_pool_size|query_cache_size)" \
  --output cli
```

## 監視・アラート

### 設定ドリフト検出

本番環境での設定変更を継続的に監視：

```bash
#!/bin/bash
# config_drift_monitor.sh - 定期実行スクリプト

EXPECTED_CONFIG="/opt/app/config/expected.json"
CURRENT_CONFIG="/opt/app/config/current.json"
ALERT_WEBHOOK="https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK"

# 現在の設定を取得
curl -s http://localhost:8080/api/config > "$CURRENT_CONFIG"

# 期待設定と比較
if ! diffx "$EXPECTED_CONFIG" "$CURRENT_CONFIG" \
     --ignore-keys-regex "^(timestamp|uptime|last_.*|temp_.*)" \
     --output json > config_drift.json; then
  
  echo "設定ドリフトが検出されました！"
  
  # 重要な変更をフィルタリング
  CRITICAL_CHANGES=$(jq -r '.[] | select(.Removed or .TypeChanged or 
    (.Modified and (.Modified[0] | contains("security") or contains("database") or contains("auth"))))' \
    config_drift.json)
  
  if [ -n "$CRITICAL_CHANGES" ]; then
    echo "🚨 重要な設定変更が検出されました"
    echo "$CRITICAL_CHANGES" | jq .
    
    # Slackにアラート送信
    curl -X POST "$ALERT_WEBHOOK" \
      -H "Content-Type: application/json" \
      -d "{
        \"text\": \"🚨 Critical configuration drift detected!\",
        \"attachments\": [{
          \"color\": \"danger\",
          \"text\": \"$(cat config_drift.json | jq -c .)\",
          \"fields\": [{
            \"title\": \"Host\",
            \"value\": \"$(hostname)\",
            \"short\": true
          }]
        }]
      }"
  else
    echo "⚠️ 軽微な設定変更が検出されました"
    cat config_drift.json
  fi
else
  echo "✅ 設定ドリフトなし"
fi
```

### サービスヘルス監視

サービスヘルス設定の変更を追跡：

```bash
# ヘルスチェック設定の比較
diffx health_config_baseline.json health_config_current.json \
  --ignore-keys-regex "^(last_check|status_timestamp|response_time)" \
  --output json > health_config_changes.json

# 監視ルールの検証
diffx prometheus_rules.yaml prometheus_rules.new.yaml \
  --path "groups" \
  --array-id-key "name" \
  --output unified
```

### アラート設定管理

アラートルールの変更管理：

```bash
# AlertManager設定の比較
diffx alertmanager.yml alertmanager.new.yml \
  --path "route" \
  --output yaml > alerting_changes.yaml

# Grafanaダッシュボードの比較
diffx dashboard_v1.json dashboard_v2.json \
  --ignore-keys-regex "^(id|uid|version|time|refresh)" \
  --path "panels" \
  --array-id-key "id" \
  --output json
```

## ソフトウェア開発

### パッケージ依存関係追跡

パッケージファイルの変更を監視：

```bash
# package.jsonの変更追跡
diffx package.json package.json.backup \
  --ignore-keys-regex "^(name|description|author|scripts\\..*)" \
  --path "dependencies" \
  --output json > dependency_changes.json

# 新しい依存関係の検出
if jq -e '.[] | select(.Added)' dependency_changes.json > /dev/null; then
  echo "新しい依存関係が追加されました："
  jq -r '.[] | select(.Added) | .Added[0] + ": " + .Added[1]' dependency_changes.json
  echo "セキュリティ監査を実行してください"
  npm audit
fi

# Python requirements.txtの比較
diffx requirements.txt requirements.new.txt \
  --format ini \
  --output cli

# Cargo.tomlの比較
diffx Cargo.toml Cargo.toml.backup \
  --format toml \
  --ignore-keys-regex "^(package\\.(build|publish))" \
  --output yaml
```

### ビルド設定変更

ビルド設定の変更を追跡：

```bash
# webpack設定の比較
diffx webpack.config.js webpack.config.new.js \
  --format json \
  --output unified > webpack_changes.diff

# TypeScript設定の比較
diffx tsconfig.json tsconfig.new.json \
  --ignore-keys-regex "^(compilerOptions\\.outDir)" \
  --output cli

# Docker設定の比較
diffx Dockerfile Dockerfile.new \
  --format yaml \
  --output cli
```

### コード品質設定

コード品質ツールの設定変更：

```bash
# ESLint設定の比較
diffx .eslintrc.json .eslintrc.new.json \
  --path "rules" \
  --output json > eslint_changes.json

# Prettier設定の比較
diffx .prettierrc.json .prettierrc.new.json \
  --output cli

# テスト設定の比較
diffx jest.config.js jest.config.new.js \
  --format json \
  --path "testMatch" \
  --output yaml
```

## セキュリティ・コンプライアンス

### セキュリティ設定監査

セキュリティ設定の変更を監視：

```bash
# セキュリティポリシーの比較
diffx security_policy_v1.json security_policy_v2.json \
  --path "permissions" \
  --array-id-key "resource" \
  --output json > security_changes.json

# 重要な権限変更の検出
CRITICAL_SECURITY=$(jq -r '.[] | select(.Removed or .TypeChanged or 
  (.Modified and (.Modified[0] | contains("admin") or contains("root") or contains("sudo"))))' \
  security_changes.json)

if [ -n "$CRITICAL_SECURITY" ]; then
  echo "🚨 重要なセキュリティ変更が検出されました"
  echo "$CRITICAL_SECURITY" | jq .
fi

# IAM設定の比較
diffx iam_policy_prod.json iam_policy_staging.json \
  --ignore-keys-regex "^(arn|account_id|creation_date)" \
  --output yaml
```

### コンプライアンス監視

コンプライアンス関連設定の追跡：

```bash
# GDPR準拠設定の比較
diffx gdpr_config.json gdpr_config.new.json \
  --path "data_retention" \
  --output json > gdpr_changes.json

# SOX準拠設定の検証
diffx sox_controls.yaml sox_controls.updated.yaml \
  --array-id-key "control_id" \
  --output unified > sox_changes.diff

# ISO27001準拠設定の監査
diffx iso27001_config.json iso27001_config.audit.json \
  --ignore-keys-regex "^(audit_date|auditor|last_review)" \
  --output json
```

### アクセス制御検証

アクセス制御設定の変更管理：

```bash
# RBAC設定の比較
diffx rbac_roles.yaml rbac_roles.new.yaml \
  --array-id-key "name" \
  --path "rules" \
  --output json > rbac_changes.json

# OAuth設定の比較
diffx oauth_config.json oauth_config.backup.json \
  --ignore-keys-regex "^(client_secret|private_key|refresh_token)" \
  --output cli

# Active Directory設定の比較
diffx ad_groups.json ad_groups.new.json \
  --array-id-key "group_name" \
  --ignore-keys-regex "^(last_modified|member_count)" \
  --output yaml
```

## 高度な使用パターン

### 複数環境パイプライン

包括的な環境比較パイプライン：

```bash
#!/bin/bash
# multi_env_compare.sh

ENVIRONMENTS=("dev" "staging" "prod")
BASE_ENV="prod"
REPORT_DIR="reports"

mkdir -p "$REPORT_DIR"

for env in "${ENVIRONMENTS[@]}"; do
  if [ "$env" != "$BASE_ENV" ]; then
    echo "=== $env と $BASE_ENV の比較 ==="
    
    # アプリケーション設定
    diffx "config/$BASE_ENV.json" "config/$env.json" \
      --ignore-keys-regex "^(host|port|database|secret_.*)" \
      --output json > "$REPORT_DIR/diff_${env}_${BASE_ENV}_app.json"
    
    # インフラ設定  
    diffx "infra/$BASE_ENV.yaml" "infra/$env.yaml" \
      --path "resources" \
      --output json > "$REPORT_DIR/diff_${env}_${BASE_ENV}_infra.json"
    
    # セキュリティ設定
    diffx "security/$BASE_ENV.json" "security/$env.json" \
      --path "policies" \
      --output json > "$REPORT_DIR/diff_${env}_${BASE_ENV}_security.json"
    
    # レポート生成
    generate_html_report.py "$REPORT_DIR/diff_${env}_${BASE_ENV}_*.json" \
      > "$REPORT_DIR/report_${env}.html"
  fi
done

echo "比較レポートが $REPORT_DIR に生成されました"
```

### データ移行検証ワークフロー

完全なデータ移行検証：

```bash
#!/bin/bash
# data_migration_validation.sh

SOURCE_DB="legacy_system"
TARGET_DB="new_system"
VALIDATION_DIR="migration_validation"

mkdir -p "$VALIDATION_DIR"

echo "=== スキーマ比較 ==="
export_schema.py "$SOURCE_DB" > "$VALIDATION_DIR/source_schema.json"
export_schema.py "$TARGET_DB" > "$VALIDATION_DIR/target_schema.json"

diffx "$VALIDATION_DIR/source_schema.json" "$VALIDATION_DIR/target_schema.json" \
  --array-id-key "table_name" \
  --output json > "$VALIDATION_DIR/schema_diff.json"

echo "=== データサンプル比較 ==="
export_sample_data.py "$SOURCE_DB" --limit 1000 > "$VALIDATION_DIR/source_data.json"
export_sample_data.py "$TARGET_DB" --limit 1000 > "$VALIDATION_DIR/target_data.json"

diffx "$VALIDATION_DIR/source_data.json" "$VALIDATION_DIR/target_data.json" \
  --array-id-key "id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(migrated_at|batch_id|source_system)" \
  --output json > "$VALIDATION_DIR/data_diff.json"

echo "=== 集約データ比較 ==="
export_aggregates.py "$SOURCE_DB" > "$VALIDATION_DIR/source_aggregates.json"
export_aggregates.py "$TARGET_DB" > "$VALIDATION_DIR/target_aggregates.json"

diffx "$VALIDATION_DIR/source_aggregates.json" "$VALIDATION_DIR/target_aggregates.json" \
  --epsilon 0.01 \
  --output json > "$VALIDATION_DIR/aggregates_diff.json"

# 移行レポート生成
generate_migration_report.py \
  "$VALIDATION_DIR/schema_diff.json" \
  "$VALIDATION_DIR/data_diff.json" \
  "$VALIDATION_DIR/aggregates_diff.json" \
  > "$VALIDATION_DIR/migration_report.html"

echo "移行検証レポートが $VALIDATION_DIR/migration_report.html に生成されました"
```

### 自動テスト統合

自動化されたテストフレームワークとの統合：

```bash
#!/bin/bash
# test_api_contract.sh

API_BASE="https://api.example.com"
EXPECTED_DIR="tests/fixtures/api_responses"
RESULTS_DIR="test_results"

mkdir -p "$RESULTS_DIR"

# 複数エンドポイントのテスト
endpoints=("users" "products" "orders" "categories")

echo "=== API契約テスト開始 ==="

for endpoint in "${endpoints[@]}"; do
  echo "Testing $endpoint endpoint..."
  
  # 実際のレスポンス取得
  curl -s "$API_BASE/$endpoint" \
    -H "Accept: application/json" \
    -H "Authorization: Bearer $API_TOKEN" \
    > "$RESULTS_DIR/actual_$endpoint.json"
  
  # 期待レスポンスと比較
  if diffx "$EXPECTED_DIR/$endpoint.json" "$RESULTS_DIR/actual_$endpoint.json" \
     --ignore-keys-regex "^(timestamp|request_id|server_time)" \
     --output json > "$RESULTS_DIR/diff_$endpoint.json"; then
    echo "✅ $endpoint: 契約に準拠"
    PASSED_TESTS=$((PASSED_TESTS + 1))
  else
    echo "❌ $endpoint: 契約違反を検出"
    cat "$RESULTS_DIR/diff_$endpoint.json"
    FAILED_TESTS=$((FAILED_TESTS + 1))
    
    # 詳細分析
    if jq -e '.[] | select(.Removed)' "$RESULTS_DIR/diff_$endpoint.json" > /dev/null; then
      echo "  ⚠️ フィールドの削除が検出されました（互換性破壊的変更）"
    fi
    
    if jq -e '.[] | select(.TypeChanged)' "$RESULTS_DIR/diff_$endpoint.json" > /dev/null; then
      echo "  ⚠️ データ型の変更が検出されました（互換性破壊的変更）"
    fi
  fi
  echo ""
done

echo "=== テスト結果 ==="
echo "成功: $PASSED_TESTS"
echo "失敗: $FAILED_TESTS"

if [ "$FAILED_TESTS" -gt 0 ]; then
  echo "API契約テストが失敗しました"
  exit 1
else
  echo "すべてのAPI契約テストが成功しました"
fi
```

---

これらの実用例を参考に、あなたの業務環境に適した `diffx` の活用方法を見つけてください。構造化データの差分を効率的に検出し、システムの品質向上と運用の自動化に役立ててください。
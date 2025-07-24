# 実用例

このガイドでは、実際のシナリオでの `diffx` の実用的な使用例を、用途と業界別に整理して提供します。

## 目次

- [設定管理](#設定管理)
- [DevOpsとインフラストラクチャ](#devopsとインフラストラクチャ)
- [API開発とテスト](#api開発とテスト)
- [データ処理とETL](#データ処理とetl)
- [データベース管理](#データベース管理)
- [監視とアラート](#監視とアラート)
- [ソフトウェア開発](#ソフトウェア開発)
- [セキュリティとコンプライアンス](#セキュリティとコンプライアンス)

## 設定管理

### 環境設定比較

異なる環境間での設定比較：

```bash
# 開発環境 vs 本番環境
diffx config/dev.json config/prod.json \
  --ignore-keys-regex "^(host|port|password|secret_.*)" \
  --output json > env_diff.json

# デプロイ前のステージング検証
diffx config/staging.yaml config/prod.yaml \
  --path "application" \
  --output yaml
```

**サンプルファイル:**
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

**期待される出力:**
```
~ application.debug: true -> false
```

### Kubernetes設定ドリフト検出

Kubernetesデプロイメントでの設定ドリフトを監視：

```bash
# 現在のデプロイメントと希望状態を比較
kubectl get deployment myapp -o json > current-deployment.json
diffx desired-deployment.json current-deployment.json \
  --ignore-keys-regex "^(metadata\\.(creationTimestamp|resourceVersion|uid)|status\\..*)" \
  --output json
```

### Docker Compose環境バリエーション

異なる環境のDocker Composeファイルを比較：

```bash
# ベースcomposeとオーバーライドを比較
diffx docker-compose.yml docker-compose.override.yml \
  --path "services" \
  --output json
```

## DevOpsとインフラストラクチャ

### Terraformステート比較

インフラストラクチャドリフト検出のためのTerraformステートファイル比較：

```bash
# 現在の状態とバックアップを比較
diffx terraform.tfstate terraform.tfstate.backup \
  --path "resources" \
  --ignore-keys-regex "^(last_updated|timeouts)" \
  --output json > infrastructure_drift.json

# 計画された変更を比較
terraform show -json plan.out > planned.json
diffx current_state.json planned.json \
  --path "planned_values.root_module"
```

### Infrastructure as Code検証

デプロイメント前のインフラストラクチャ変更検証：

```bash
# CloudFormationテンプレート比較
diffx infrastructure/base.yaml infrastructure/updated.yaml \
  --ignore-keys-regex "^(Metadata|Description)" \
  --output yaml

# Ansibleプレイブック比較
diffx playbook-v1.yml playbook-v2.yml \
  --path "tasks" \
  --output cli
```

### CI/CDパイプライン設定

CI/CDパイプライン設定変更の監視：

```bash
# GitHub Actionsワークフロー比較
diffx .github/workflows/ci.yml .github/workflows/ci.new.yml \
  --output json

# GitLab CI比較
diffx .gitlab-ci.yml .gitlab-ci.backup.yml \
  --ignore-keys-regex "^(variables\\.CI_.*)"
```

## API開発とテスト

### APIレスポンス検証

期待されるスキーマに対するAPIレスポンスの検証：

```bash
# APIレスポンスと期待される構造を比較
curl -s https://api.example.com/v1/users/123 > actual_response.json
diffx expected_user_response.json actual_response.json \
  --ignore-keys-regex "^(timestamp|request_id|server_time)" \
  --output json

# APIエンドポイント変更の検証
diffx api/v1/schema.json api/v2/schema.json \
  --path "definitions" \
  --output yaml
```

**サンプルAPI検証:**
```bash
# ユーザー作成エンドポイントのテスト
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

### OpenAPI仕様比較

破壊的変更のためのOpenAPI仕様比較：

```bash
# APIバージョン比較
diffx openapi-v1.yaml openapi-v2.yaml \
  --path "paths" \
  --output json > api_changes.json

# 後方互換性検証
diffx api-spec.yaml api-spec.new.yaml \
  --ignore-keys-regex "^(info\\.(version|title)|servers)" \
  --output json
```

### GraphQLスキーマ検証

GraphQLスキーマの比較：

```bash
# GraphQLをJSONに変換して比較
graphql-to-json schema-v1.graphql > schema-v1.json
graphql-to-json schema-v2.graphql > schema-v2.json
diffx schema-v1.json schema-v2.json \
  --output yaml
```

## データ処理とETL

### データパイプライン検証

ETLパイプラインでのデータ変換検証：

```bash
# 入力データと出力データ構造を比較
diffx input_data_sample.json output_data_sample.json \
  --array-id-key "record_id" \
  --epsilon 0.001 \
  --output json

# データマイグレーション検証
diffx source_schema.json target_schema.json \
  --path "tables" \
  --output yaml
```

### データ品質チェック

パイプライン段階でのデータ品質監視：

```bash
# データスナップショット比較
diffx data_snapshot_t1.json data_snapshot_t2.json \
  --ignore-keys-regex "^(timestamp|batch_id|process_time)" \
  --array-id-key "id" \
  --epsilon 0.01

# 集計結果検証
diffx daily_metrics.json expected_metrics.json \
  --epsilon 0.05 \
  --output json
```

### 設定駆動ETL

ETL設定ファイルの比較：

```bash
# データソース設定比較
diffx etl_config_staging.yaml etl_config_prod.yaml \
  --ignore-keys-regex "^(credentials|connection_string)" \
  --path "data_sources"

# 変換ルール検証
diffx transform_rules_v1.json transform_rules_v2.json \
  --array-id-key "rule_id"
```

## データベース管理

### スキーママイグレーション検証

データベーススキーマ変更の検証：

```bash
# データベーススキーマ比較
pg_dump --schema-only mydb > schema_before.sql
# マイグレーション実行
pg_dump --schema-only mydb > schema_after.sql

# 比較用のJSONに変換（カスタムスクリプト使用）
sql-to-json schema_before.sql > schema_before.json
sql-to-json schema_after.sql > schema_after.json

diffx schema_before.json schema_after.json \
  --array-id-key "table_name" \
  --output json > migration_report.json
```

### データバックアップ検証

バックアップの整合性検証：

```bash
# 現在のデータとバックアップを比較
diffx production_export.json backup_export.json \
  --array-id-key "id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(last_updated|backup_timestamp)"
```

### データベース設定管理

データベース設定ファイルの比較：

```bash
# PostgreSQL設定比較
diffx postgresql.conf postgresql.conf.backup \
  --format ini \
  --ignore-keys-regex "^(log_.*|shared_preload_libraries)"

# MongoDB設定比較
diffx mongod.conf mongod.conf.new \
  --format yaml \
  --path "storage"
```

## 監視とアラート

### 設定ドリフト検出

本番環境での設定変更監視：

```bash
# スケジュールされた設定チェック
#!/bin/bash
# check_config_drift.sh

EXPECTED_CONFIG="/opt/app/config/expected.json"
CURRENT_CONFIG="/opt/app/config/current.json"

# 現在の設定を取得
curl -s http://localhost:8080/api/config > "$CURRENT_CONFIG"

# 期待値と比較
if diffx "$EXPECTED_CONFIG" "$CURRENT_CONFIG" \
   --ignore-keys-regex "^(timestamp|uptime|last_.*)" \
   --output json > config_drift.json; then
  echo "設定ドリフトは検出されませんでした"
else
  echo "設定ドリフトが検出されました！"
  cat config_drift.json
  # アラートを送信
  alert-manager send --file config_drift.json
fi
```

### サービスヘルス監視

サービスヘルス設定の監視：

```bash
# ヘルスチェック設定比較
diffx health_config_baseline.json health_config_current.json \
  --ignore-keys-regex "^(last_check|status_timestamp)" \
  --output json

# 監視ルール検証
diffx prometheus_rules.yaml prometheus_rules.new.yaml \
  --path "groups" \
  --output json
```

### アラート設定管理

アラートルール変更の管理：

```bash
# アラートマネージャー設定比較
diffx alertmanager.yml alertmanager.new.yml \
  --path "route" \
  --output yaml

# Grafanaダッシュボード変更検証
diffx dashboard_v1.json dashboard_v2.json \
  --ignore-keys-regex "^(id|uid|version|time)" \
  --path "panels"
```

## ソフトウェア開発

### パッケージ依存性追跡

パッケージ依存性の変更追跡：

```bash
# パッケージファイル比較
diffx package.json package.json.backup \
  --ignore-keys-regex "^(name|description|author)" \
  --path "dependencies"

# ロックファイル比較
diffx yarn.lock yarn.lock.backup \
  --output json > dependency_changes.json

# Python要件比較
diffx requirements.txt requirements.new.txt \
  --format ini  # キー値ペアとして扱う
```

### ビルド設定変更

ビルド設定変更の監視：

```bash
# webpack設定比較
diffx webpack.config.js webpack.config.new.js \
  --format json \
  --output json

# Cargo.tomlファイル比較
diffx Cargo.toml Cargo.toml.backup \
  --format toml \
  --ignore-keys-regex "^(build|publish)"
```

### コード品質設定

コード品質ツール設定の追跡：

```bash
# ESLint設定比較
diffx .eslintrc.json .eslintrc.new.json \
  --path "rules" \
  --output json

# テスト設定比較
diffx jest.config.js jest.config.new.js \
  --format json \
  --path "testMatch"
```

## セキュリティとコンプライアンス

### セキュリティ設定監査

セキュリティ設定の監査：

```bash
# セキュリティポリシー比較
diffx security_policy_v1.json security_policy_v2.json \
  --path "permissions" \
  --output json > security_changes.json

# IAM設定検証
diffx iam_policy_prod.json iam_policy_staging.json \
  --ignore-keys-regex "^(arn|account_id)" \
  --output yaml
```

### コンプライアンス監視

コンプライアンス関連設定の監視：

```bash
# GDPR準拠設定比較
diffx gdpr_config.json gdpr_config.new.json \
  --path "data_retention" \
  --output json

# SOX準拠検証
diffx sox_controls.yaml sox_controls.updated.yaml \
  --array-id-key "control_id" \
  --output json
```

### アクセス制御検証

アクセス制御変更の検証：

```bash
# RBAC設定比較
diffx rbac_roles.yaml rbac_roles.new.yaml \
  --array-id-key "name" \
  --path "rules" \
  --output json

# OAuth設定検証
diffx oauth_config.json oauth_config.backup.json \
  --ignore-keys-regex "^(client_secret|private_key)"
```

## UNIXコマンドパターン

diffxは、構造化データに適応した一般的なUNIX diffパターンの同等機能を提供します：

### `diff -q` 同等: 高速ファイル変更検出

```bash
# 設定ファイルが異なるかチェック（終了コードのみ）
if ! diffx config.json config.backup.json --quiet; then
  echo "設定が変更されました、デプロイメントをトリガーします"
  deploy_app.sh
fi

# 複数の設定ファイルのバッチチェック
for config in configs/*.json; do
  if ! diffx "$config" "backups/$(basename $config)" --quiet; then
    echo "$(basename $config) が変更されました"
  fi
done

# 継続的監視のためのwatch使用
watch -n 30 'diffx live_config.json baseline_config.json --quiet || echo "設定ドリフトが検出されました！"'
```

### `diff --brief` 同等: 変更されたファイル名のみ表示

```bash
# 変更されたファイルの高速ディレクトリスキャン
# diffxはディレクトリを自動検出し、再帰的に比較します
diffx config_dir/ backup_dir/ --brief

# 変更検出のためのCIパイプラインでの使用
# diffxはディレクトリを自動検出し、再帰的に比較します
changed_files=$(diffx current_configs/ previous_configs/ --brief)
if [ -n "$changed_files" ]; then
  echo "設定変更が検出されました:"
  echo "$changed_files"
  trigger_validation_pipeline.sh
fi

# 選択的チェックのためのfindとの組み合わせ
find . -name "*.json" -newer last_deploy.marker | while read file; do
  backup_file="backups/${file}"
  if [ -f "$backup_file" ]; then
    diffx "$file" "$backup_file" --brief
  fi
done
```

### `diff -i` 同等: 大文字小文字を区別しない比較

```bash
# 列挙値の大文字小文字の違いを無視したAPIレスポンス比較
curl -s https://api.example.com/status > current_status.json
diffx expected_status.json current_status.json --ignore-case

# 一貫性のない大文字小文字の設定ファイル
diffx config_template.yaml user_config.yaml \
  --ignore-case \
  --ignore-keys-regex "^(name|description)"

# 大文字小文字が混在するフィールド名のデータベース設定
diffx db_schema.json migrated_schema.json \
  --ignore-case \
  --array-id-key "table_name" \
  --output json
```

### `diff -w` 同等: 空白の違いを無視

```bash
# 異なるフォーマットの可能性があるJSONファイル比較
diffx api_response_pretty.json api_response_minified.json --ignore-whitespace

# 設定内のフォーマット差異を無視
diffx config.json config_reformatted.json \
  --ignore-whitespace \
  --output json

# 空白のバリエーションがあるデータエクスポート比較
diffx data_export.json data_import_processed.json \
  --ignore-whitespace \
  --array-id-key "id" \
  --epsilon 0.001
```

### YAML出力による可読性の向上

```bash
# 可読性の向上のためのYAML形式での差分表示
diffx large_config.json large_config_new.json \
  --output yaml

# 特定セクションにフォーカスしたYAML出力
diffx api_schema.json api_schema_v2.json \
  --path \"definitions\" \
  --output yaml

# プログラム処理のためのJSON出力
diffx database_config.json database_config_updated.json \
  --output json
```

### 組み合わせUNIXスタイルパターン

```bash
# 同等: diff -qiw file1 file2
diffx config.json config.backup.json \
  --quiet \
  --ignore-case \
  --ignore-whitespace

# 同等: diff -r --brief dir1/ dir2/
# diffxはディレクトリを自動検出し、再帰的に比較します
diffx config_dir/ backup_dir/ \
  --brief

# 高度なパターン: 選択的フィールド無視での大文字小文字無視
diffx user_data.json user_data_migrated.json \
  --ignore-case \
  --ignore-whitespace \
  --ignore-keys-regex "^(created_at|updated_at|timestamp)" \
  --array-id-key "user_id"

# Gitスタイルワークフロー統合
git show HEAD:config.json > /tmp/old_config.json
diffx /tmp/old_config.json config.json \
  --ignore-whitespace \
  --output json
```

### シェル統合例

```bash
# pre-commitフック同等
#!/bin/bash
# .git/hooks/pre-commit
if ! diffx config/production.json config/staging.json \
   --ignore-keys-regex "^(environment|debug)" \
   --quiet; then
  echo "本番環境とステージング環境の設定にセマンティックな差異があります"
  echo "コミット前に確認してください:"
  diffx config/production.json config/staging.json \
    --ignore-keys-regex "^(environment|debug)"
  exit 1
fi

# デプロイメント検証スクリプト
#!/bin/bash
# validate_deploy.sh
deployment_config="$1"
baseline_config="configs/baseline.json"

if diffx "$baseline_config" "$deployment_config" \
   --ignore-case \
   --ignore-whitespace \
   --quiet; then
  echo "✅ 設定は変更されていません - デプロイ可能です"
  exit 0
else
  echo "⚠️  設定変更が検出されました:"
  diffx "$baseline_config" "$deployment_config" \
    --ignore-case \
    --ignore-whitespace \
    --brief
  echo "デプロイメントを続行しますか？ (y/N)"
  read -r response
  [[ "$response" =~ ^[Yy]$ ]] || exit 1
fi

# アラート付き監視スクリプト
#!/bin/bash
# monitor_config_drift.sh
while true; do
  if ! diffx /etc/app/config.json /opt/app/expected_config.json \
     --ignore-keys-regex "^(hostname|instance_id|last_update)" \
     --quiet; then
    
    # 詳細でアラートを送信
    diffx /etc/app/config.json /opt/app/expected_config.json \
      --ignore-keys-regex "^(hostname|instance_id|last_update)" \
      --output json | \
      curl -X POST https://alerts.example.com/webhook \
           -H "Content-Type: application/json" \
           -d @-
  fi
  sleep 300  # 5分ごとにチェック
done
```

## 高度な使用パターン

### マルチ環境パイプライン

包括的な環境比較パイプライン：

```bash
#!/bin/bash
# multi_env_compare.sh

ENVIRONMENTS=("dev" "staging" "prod")
BASE_ENV="prod"

for env in "${ENVIRONMENTS[@]}"; do
  if [ "$env" != "$BASE_ENV" ]; then
    echo "$envと$BASE_ENVを比較中"
    
    # アプリケーション設定
    diffx "config/$BASE_ENV.json" "config/$env.json" \
      --ignore-keys-regex "^(host|port|database|secret_.*)" \
      --output json > "diff_${env}_${BASE_ENV}_app.json"
    
    # インフラストラクチャ設定  
    diffx "infra/$BASE_ENV.yaml" "infra/$env.yaml" \
      --path "resources" \
      --output json > "diff_${env}_${BASE_ENV}_infra.json"
    
    # サマリーレポート生成
    generate_report.py "diff_${env}_${BASE_ENV}_*.json" > "report_${env}.html"
  fi
done
```

### データマイグレーション検証

完全なデータマイグレーション検証ワークフロー：

```bash
#!/bin/bash
# data_migration_validation.sh

SOURCE_DB="legacy_system"
TARGET_DB="new_system"

# スキーマエクスポート
export_schema.py "$SOURCE_DB" > source_schema.json
export_schema.py "$TARGET_DB" > target_schema.json

# スキーマ比較
diffx source_schema.json target_schema.json \
  --array-id-key "table_name" \
  --output json > schema_diff.json

# サンプルデータエクスポート
export_sample_data.py "$SOURCE_DB" > source_data.json
export_sample_data.py "$TARGET_DB" > target_data.json

# データ構造比較
diffx source_data.json target_data.json \
  --array-id-key "id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(migrated_at|batch_id)" \
  --output json > data_diff.json

# マイグレーションレポート生成
generate_migration_report.py schema_diff.json data_diff.json
```

### 自動テスト統合

自動テストフレームワークとの統合：

```bash
# test_api_contract.sh
#!/bin/bash

API_BASE="https://api.example.com"
EXPECTED_DIR="tests/fixtures/api_responses"

# 複数エンドポイントのテスト
endpoints=("users" "products" "orders")

for endpoint in "${endpoints[@]}"; do
  echo "$endpointエンドポイントをテスト中..."
  
  # 実際のレスポンスを取得
  curl -s "$API_BASE/$endpoint" > "actual_$endpoint.json"
  
  # 期待値と比較
  if diffx "$EXPECTED_DIR/$endpoint.json" "actual_$endpoint.json" \
     --ignore-keys-regex "^(timestamp|request_id)" \
     --output json > "diff_$endpoint.json"; then
    echo "✅ $endpointは期待される構造と一致しています"
  else
    echo "❌ $endpointに予期しない変更があります"
    cat "diff_$endpoint.json"
    exit 1
  fi
done

echo "すべてのAPIコントラクトテストが成功しました！"
```

## パフォーマンス最適化例

### 大容量ファイル処理

大容量ファイルのためのdiffx最適化：

```bash
# 大容量設定ファイル
diffx large_config.json large_config.new.json \
  --path "critical.services" \
  --ignore-keys-regex "^(logs|metrics|debug_.*)" \
  --output json

# 複数ファイルのバッチ処理
find configs/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup --output json > {}.diff || echo "{}で差分が見つかりました"'
```

### メモリ効率的処理

大容量データセットの効率的処理：

```bash
# ストリーム処理（概念的）
diffx --stream large_dataset_v1.json large_dataset_v2.json \
  --array-id-key "id" \
  --chunk-size 1000 \
  --output json
```

これらの例は、様々な業界と用途にわたる `diffx` の汎用性とパワーを実証しています。各例には実用的なコマンド、サンプルデータ、期待される出力が含まれており、特定のニーズに適応するのに役立ちます。
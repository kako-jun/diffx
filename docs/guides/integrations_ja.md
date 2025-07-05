# CI/CD統合例

このセクションでは、`diffx` を継続的インテグレーション/継続的デプロイメント（CI/CD）パイプラインに統合する方法の例を提供します。

## GitHub Actions

GitHub Actionsワークフローで `diffx` を使用して、設定ファイル、データ、その他の構造化アセットの構造的差分を自動的にチェックできます。これは、変更が期待されるパターンに準拠していることを確認したり、予期しない変更にフラグを立てたりするのに特に有用です。

以下は、`diffx` を使用して2つのJSONファイルを比較するGitHub Actionsワークフローの基本例です。このワークフローは、プルリクエストでトリガーされ、設定変更の構造的影響をレビューするために使用できます。

```yaml
name: Check Config Differences

on:
  pull_request:
    paths:
      - 'config/*.json'
      - 'config/*.yaml'
      - 'config/*.toml'

jobs:
  check-config-diff:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v3
      with:
        fetch-depth: 0  # すべての履歴を取得

    - name: Install Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        
    - name: Install diffx
      run: cargo install diffx
      
    - name: Check configuration differences
      run: |
        # プルリクエストで変更されたファイルを取得
        CHANGED_FILES=$(git diff --name-only HEAD^ HEAD | grep -E '\.(json|yaml|yml|toml)$' || true)
        
        if [ -n "$CHANGED_FILES" ]; then
          echo "変更された設定ファイル:"
          echo "$CHANGED_FILES"
          
          for file in $CHANGED_FILES; do
            if [ -f "$file" ]; then
              echo "=== $file の差分 ==="
              # 前のバージョンと比較
              git show HEAD^:"$file" > /tmp/old_file 2>/dev/null || continue
              diffx /tmp/old_file "$file" --output json > /tmp/diff_output.json
              
              # 差分がある場合は詳細を表示
              if [ -s /tmp/diff_output.json ]; then
                echo "構造的変更が検出されました:"
                cat /tmp/diff_output.json
              else
                echo "構造的変更はありません（フォーマットのみの変更）"
              fi
              echo ""
            fi
          done
        else
          echo "変更された設定ファイルはありません"
        fi
```

## GitLab CI

GitLab CIでも同様に `diffx` を使用できます：

```yaml
check_config_changes:
  stage: test
  image: rust:latest
  before_script:
    - cargo install diffx
  script:
    - |
      if [ "$CI_PIPELINE_SOURCE" = "merge_request_event" ]; then
        git fetch origin $CI_MERGE_REQUEST_TARGET_BRANCH_NAME
        CHANGED_FILES=$(git diff --name-only origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME...HEAD | grep -E '\.(json|yaml|yml|toml)$' || true)
        
        for file in $CHANGED_FILES; do
          echo "=== $file の差分チェック ==="
          git show origin/$CI_MERGE_REQUEST_TARGET_BRANCH_NAME:"$file" > /tmp/old_file 2>/dev/null || continue
          diffx /tmp/old_file "$file" || echo "差分が検出されました"
        done
      fi
  only:
    - merge_requests
```

## Jenkins Pipeline

```groovy
pipeline {
    agent any
    
    stages {
        stage('Install diffx') {
            steps {
                sh 'cargo install diffx'
            }
        }
        
        stage('Check Config Changes') {
            when {
                changeRequest()
            }
            steps {
                script {
                    def changedFiles = sh(
                        script: "git diff --name-only HEAD^ HEAD | grep -E '\\.(json|yaml|yml|toml)\$' || true",
                        returnStdout: true
                    ).trim()
                    
                    if (changedFiles) {
                        changedFiles.split('\n').each { file ->
                            echo "=== ${file} の差分チェック ==="
                            sh """
                                git show HEAD^:${file} > /tmp/old_file 2>/dev/null || exit 0
                                diffx /tmp/old_file ${file} --output json > diff_report.json || true
                                if [ -s diff_report.json ]; then
                                    echo "構造的変更が検出されました:"
                                    cat diff_report.json
                                fi
                            """
                        }
                    }
                }
            }
        }
    }
}
```

## Docker統合

Dockerfileで `diffx` を使用する例：

```dockerfile
FROM rust:latest as builder
RUN cargo install diffx

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y git && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/diffx /usr/local/bin/
COPY check_config.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/check_config.sh

# 使用例
# docker run --rm -v $(pwd):/workspace config-checker
```

## 実用的なユースケース

### 1. データベース移行チェック

```bash
# マイグレーション前後のスキーマ比較
diffx schema_before.json schema_after.json --output yaml > migration_report.yaml
```

### 2. API設定変更の監視

```bash
# API設定の変更を追跡
diffx api_config_v1.json api_config_v2.json --ignore-keys-regex "^(timestamp|version)$"
```

### 3. 複数環境間の設定比較

```bash
# 本番環境と開発環境の設定を比較
diffx config/production.yaml config/development.yaml --output json > env_diff.json
```
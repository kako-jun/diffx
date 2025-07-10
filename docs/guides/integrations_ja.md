# 統合ガイド

この包括的なガイドでは、`diffx` を様々な開発ワークフロー、CI/CD パイプライン、自動化システムに統合する方法を説明します。

## 目次

- [CI/CD プラットフォーム](#cicd-プラットフォーム)
- [バージョン管理統合](#バージョン管理統合)
- [コンテナエコシステム](#コンテナエコシステム)
- [クラウドプラットフォーム](#クラウドプラットフォーム)
- [監視とアラート](#監視とアラート)
- [開発ツール](#開発ツール)
- [自動化スクリプト](#自動化スクリプト)

## CI/CD プラットフォーム

### GitHub Actions

#### 基本的な設定検証

```yaml
name: Configuration Validation

on:
  pull_request:
    paths:
      - 'config/**'
      - '**/*.json'
      - '**/*.yaml'
      - '**/*.yml'

jobs:
  validate-config:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout code
      uses: actions/checkout@v4
      with:
        fetch-depth: 0
    
    - name: Install diffx
      run: |
        curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz
        sudo mv diffx /usr/local/bin/
        diffx --version
    
    - name: Validate configuration changes
      run: |
        # 変更されたファイルを取得
        CHANGED_FILES=$(git diff --name-only origin/${{ github.base_ref }}...HEAD | grep -E '\.(json|yaml|yml|toml)$' || true)
        
        if [ -n "$CHANGED_FILES" ]; then
          echo "変更された設定ファイルを検証中:"
          echo "$CHANGED_FILES"
          
          for file in $CHANGED_FILES; do
            if [ -f "$file" ]; then
              echo "=== $file を分析中 ==="
              
              # ベースブランチ版と比較
              git show origin/${{ github.base_ref }}:"$file" > /tmp/base_file 2>/dev/null || {
                echo "新しいファイル: $file"
                continue
              }
              
              # 設定固有の設定で diffx を実行
              diffx /tmp/base_file "$file" \
                --ignore-keys-regex "^(timestamp|lastModified|createdAt|updatedAt|buildTime)$" \
                --output json > "/tmp/diff_${file//\//_}.json"
              
              # 重要な変更をチェック
              if [ -s "/tmp/diff_${file//\//_}.json" ]; then
                echo "$file で変更を検出:"
                cat "/tmp/diff_${file//\//_}.json" | jq -r '.[] | 
                  if .Added then "  + \(.Added[0]): \(.Added[1])"
                  elif .Removed then "  - \(.Removed[0]): \(.Removed[1])"
                  elif .Modified then "  ~ \(.Modified[0]): \(.Modified[1]) → \(.Modified[2])"
                  elif .TypeChanged then "  ! \(.TypeChanged[0]): \(.TypeChanged[1]) → \(.TypeChanged[2]) (型変更)"
                  else . end'
                
                # 重要な変更にフラグを立てる
                CRITICAL=$(cat "/tmp/diff_${file//\//_}.json" | jq -r '.[] | 
                  select(.Removed or .TypeChanged or 
                         (.Modified and (.Modified[0] | contains("security") or contains("database") or contains("auth"))))')
                
                if [ -n "$CRITICAL" ]; then
                  echo "⚠️ $file で重要な変更を検出 - レビューが必要"
                  echo "$CRITICAL" | jq -r '.[]'
                  echo "::warning title=Critical Config Change::Critical changes detected in $file"
                fi
              else
                echo "✅ $file にセマンティックな変更なし（フォーマットのみ）"
              fi
              echo ""
            fi
          done
        else
          echo "設定ファイルの変更なし"
        fi
```

#### API 契約テスト

```yaml
name: API Contract Validation

on:
  schedule:
    - cron: '0 */4 * * *'  # 4時間ごと
  workflow_dispatch:

jobs:
  api-contract-test:
    runs-on: ubuntu-latest
    
    steps:
    - name: Checkout repository
      uses: actions/checkout@v4
    
    - name: Install diffx
      run: cargo install diffx
    
    - name: Test API contracts
      env:
        API_BASE_URL: ${{ secrets.API_BASE_URL }}
        API_KEY: ${{ secrets.API_KEY }}
      run: |
        #!/bin/bash
        set -e
        
        # テストするエンドポイントを定義
        ENDPOINTS=("users" "products" "orders" "health")
        FAILED_TESTS=()
        
        for endpoint in "${ENDPOINTS[@]}"; do
          echo "$endpoint エンドポイントをテスト中..."
          
          # 現在のレスポンスを取得
          curl -H "Authorization: Bearer $API_KEY" \
               -H "Accept: application/json" \
               "$API_BASE_URL/$endpoint" > "actual_$endpoint.json"
          
          # 期待されるスキーマと比較
          if diffx "tests/api_contracts/$endpoint.json" "actual_$endpoint.json" \
             --ignore-keys-regex "^(timestamp|requestId|serverId|responseTime)$" \
             --output json > "diff_$endpoint.json"; then
            echo "✅ $endpoint 契約は一致"
          else
            echo "❌ $endpoint 契約違反を検出"
            FAILED_TESTS+=("$endpoint")
            
            # 詳細レポートを作成
            echo "## $endpoint Contract Violation" >> contract_violations.md
            echo '```json' >> contract_violations.md
            cat "diff_$endpoint.json" >> contract_violations.md
            echo '```' >> contract_violations.md
            echo "" >> contract_violations.md
          fi
        done
        
        # 結果をレポート
        if [ ${#FAILED_TESTS[@]} -gt 0 ]; then
          echo "契約違反が見つかりました: ${FAILED_TESTS[*]}"
          
          # 違反に対してGitHubイシューを作成
          if [ -f contract_violations.md ]; then
            gh issue create \
              --title "API Contract Violations Detected" \
              --body-file contract_violations.md \
              --label "api,contract-violation,automation"
          fi
          
          exit 1
        else
          echo "すべてのAPI契約が正常に検証されました"
        fi
```

### GitLab CI

#### マルチ環境設定検証

```yaml
# .gitlab-ci.yml
stages:
  - validate
  - deploy

variables:
  DIFFX_VERSION: "0.2.0"

.install_diffx: &install_diffx
  - |
    if ! command -v diffx &> /dev/null; then
      cargo install diffx --version $DIFFX_VERSION
    fi

validate_configs:
  stage: validate
  image: rust:latest
  before_script:
    - *install_diffx
  script:
    - |
      # 環境間の設定一貫性を検証
      ENVIRONMENTS=("development" "staging" "production")
      
      for env in "${ENVIRONMENTS[@]}"; do
        if [ "$env" != "production" ]; then
          echo "$env と本番環境の設定を比較中..."
          
          # アプリ設定を比較
          diffx "config/production.yaml" "config/$env.yaml" \
            --ignore-keys-regex "^(environment|host|port|replicas|resources\..*)" \
            --output json > "diff_${env}_prod.json"
          
          # 予期しない差分をチェック
          UNEXPECTED_DIFFS=$(cat "diff_${env}_prod.json" | jq -r '.[] | 
            select(.Added or .Removed or 
                   (.Modified and (.Modified[0] | 
                    contains("security") or contains("auth") or contains("database"))))')
          
          if [ -n "$UNEXPECTED_DIFFS" ]; then
            echo "⚠️ $env と本番環境間で予期しない設定差分:"
            echo "$UNEXPECTED_DIFFS" | jq -r '.'
            echo "セキュリティと互換性についてこれらの変更を確認してください。"
          fi
        fi
      done
  artifacts:
    reports:
      junit: config_validation_report.xml
    paths:
      - diff_*.json
    expire_in: 1 week
  only:
    changes:
      - config/**/*
```

#### Infrastructure as Code 検証

```yaml
validate_terraform:
  stage: validate
  image: hashicorp/terraform:latest
  before_script:
    - apk add --no-cache curl jq
    - *install_diffx
  script:
    - |
      # Terraform プランの変更を検証
      terraform init
      terraform plan -out=tfplan
      terraform show -json tfplan > planned_changes.json
      
      # 現在の状態と比較
      terraform show -json > current_state.json
      
      # リソースの変更に焦点
      diffx current_state.json planned_changes.json \
        --path "planned_values.root_module.resources" \
        --ignore-keys-regex "^(timeouts|creation_time|last_updated)" \
        --output json > terraform_diff.json
      
      # 影響を分析
      CRITICAL_CHANGES=$(cat terraform_diff.json | jq -r '.[] | 
        select(.Removed or (.Modified and (.Modified[0] | 
          contains("security_group") or contains("iam") or contains("vpc"))))')
      
      if [ -n "$CRITICAL_CHANGES" ]; then
        echo "🔴 重要なインフラ変更を検出!"
        echo "$CRITICAL_CHANGES" | jq -r '.'
        echo "デプロイには手動承認が必要です。"
        exit 1
      fi
  when: manual
  allow_failure: false
```

### Jenkins Pipeline

#### 設定管理用宣言型パイプライン

```groovy
pipeline {
    agent any
    
    environment {
        DIFFX_VERSION = '0.2.0'
    }
    
    stages {
        stage('Setup') {
            steps {
                script {
                    // diffx がない場合はインストール
                    sh '''
                        if ! command -v diffx &> /dev/null; then
                            curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz
                            sudo mv diffx /usr/local/bin/
                        fi
                        diffx --version
                    '''
                }
            }
        }
        
        stage('Validate Configuration Changes') {
            when {
                changeset "config/**"
            }
            steps {
                script {
                    def changedFiles = sh(
                        script: "git diff --name-only HEAD~1 HEAD | grep -E '\\.(json|yaml|yml|toml)\$' || true",
                        returnStdout: true
                    ).trim()
                    
                    if (changedFiles) {
                        changedFiles.split('\n').each { file ->
                            echo "${file} を分析中..."
                            
                            sh """
                                git show HEAD~1:${file} > old_${file} 2>/dev/null || echo '{}' > old_${file}
                                
                                diffx old_${file} ${file} \\
                                    --ignore-keys-regex "^(timestamp|version|buildNumber)\$" \\
                                    --output json > diff_${file.replaceAll('/', '_')}.json || true
                                
                                if [ -s diff_${file.replaceAll('/', '_')}.json ]; then
                                    echo "${file} で変更を検出:"
                                    cat diff_${file.replaceAll('/', '_')}.json | jq -r '.[]'
                                else
                                    echo "${file} にセマンティックな変更なし"
                                fi
                            """
                        }
                    }
                }
            }
            post {
                always {
                    archiveArtifacts artifacts: 'diff_*.json', allowEmptyArchive: true
                }
            }
        }
        
        stage('Deploy') {
            when {
                branch 'main'
            }
            steps {
                script {
                    // ここにデプロイロジック
                    echo "設定変更をデプロイ中..."
                }
            }
        }
    }
    
    post {
        failure {
            emailext (
                subject: "設定検証失敗: ${env.JOB_NAME} - ${env.BUILD_NUMBER}",
                body: "設定検証が失敗しました。詳細はビルドログを確認してください。",
                to: "${env.CHANGE_AUTHOR_EMAIL}"
            )
        }
    }
}
```

## バージョン管理統合

### Git フック

#### 設定検証用プリコミットフック

```bash
#!/bin/bash
# .git/hooks/pre-commit

# diffx が利用可能かチェック
if ! command -v diffx &> /dev/null; then
    echo "警告: diffx が見つかりません。次でインストール: cargo install diffx"
    exit 0
fi

# ステージされたファイルを取得
STAGED_FILES=$(git diff --cached --name-only --diff-filter=AM | grep -E '\.(json|yaml|yml|toml)$' || true)

if [ -z "$STAGED_FILES" ]; then
    exit 0
fi

echo "ステージされた設定ファイルを検証中..."

VALIDATION_FAILED=false

for file in $STAGED_FILES; do
    echo "$file を検証中..."
    
    # ファイルが HEAD に存在するかチェック（変更の場合）
    if git cat-file -e HEAD:"$file" 2>/dev/null; then
        # ステージ版と HEAD を比較
        git show HEAD:"$file" > /tmp/head_version
        git show :"$file" > /tmp/staged_version
        
        # 厳密な検証で diffx を実行
        if diffx /tmp/head_version /tmp/staged_version \
           --ignore-keys-regex "^(timestamp|lastModified)$" \
           --output json > /tmp/diff_output.json; then
            echo "✅ $file: セマンティックな変更なし"
        else
            echo "📝 $file: 変更を検出"
            
            # 潜在的に危険な変更をチェック
            DANGEROUS_CHANGES=$(cat /tmp/diff_output.json | jq -r '.[] | 
                select(.Removed or .TypeChanged or 
                       (.Modified and (.Modified[0] | 
                        contains("security") or contains("password") or 
                        contains("secret") or contains("key"))))')
            
            if [ -n "$DANGEROUS_CHANGES" ]; then
                echo "⚠️  警告: $file で潜在的に危険な変更:"
                echo "$DANGEROUS_CHANGES" | jq -r '.'
                echo ""
                read -p "コミットを続行しますか? (y/N): " -n 1 -r
                echo
                if [[ ! $REPLY =~ ^[Yy]$ ]]; then
                    VALIDATION_FAILED=true
                fi
            fi
        fi
        
        # クリーンアップ
        rm -f /tmp/head_version /tmp/staged_version /tmp/diff_output.json
    else
        echo "ℹ️  $file: 新しいファイル"
    fi
done

if [ "$VALIDATION_FAILED" = true ]; then
    echo "検証の懸念によりコミットが中止されました。"
    exit 1
fi

echo "設定検証が正常に完了しました。"
```

#### デプロイ検証用ポストレシーブフック

```bash
#!/bin/bash
# hooks/post-receive

while read oldrev newrev refname; do
    # main ブランチのみ処理
    if [ "$refname" = "refs/heads/main" ]; then
        echo "main ブランチのデプロイ準備を検証中..."
        
        # 変更された設定ファイルを取得
        CHANGED_CONFIGS=$(git diff --name-only $oldrev..$newrev | grep -E 'config/.*\.(json|yaml|yml)$' || true)
        
        if [ -n "$CHANGED_CONFIGS" ]; then
            echo "設定変更を検出:"
            echo "$CHANGED_CONFIGS"
            
            # 変更された各設定を検証
            for config in $CHANGED_CONFIGS; do
                echo "$config を検証中..."
                
                # 旧バージョンと新バージョンを抽出
                git show $oldrev:$config > /tmp/old_config 2>/dev/null || echo '{}' > /tmp/old_config
                git show $newrev:$config > /tmp/new_config
                
                # 包括的な検証を実行
                diffx /tmp/old_config /tmp/new_config \
                    --ignore-keys-regex "^(version|buildNumber|timestamp)$" \
                    --output json > /tmp/config_diff.json
                
                if [ -s /tmp/config_diff.json ]; then
                    # デプロイパイプラインをトリガー
                    echo "設定変更にはデプロイ更新が必要"
                    
                    # 例: Jenkins ジョブをトリガー
                    curl -X POST "$JENKINS_URL/job/deploy-config/build" \
                         --user "$JENKINS_USER:$JENKINS_TOKEN" \
                         --data-urlencode "json={\"parameter\": [{\"name\":\"config_file\", \"value\":\"$config\"}]}"
                fi
                
                rm -f /tmp/old_config /tmp/new_config /tmp/config_diff.json
            done
        fi
    fi
done
```

### Git エイリアス

`.gitconfig` に追加:

```ini
[alias]
    # 現在のファイルを前のコミットと比較
    diffx-prev = "!f() { git show HEAD~1:\"$1\" | diffx - \"$1\"; }; f"
    
    # 2つのコミット間でファイルを比較
    diffx-commits = "!f() { git show \"$1\":\"$3\" | diffx - <(git show \"$2\":\"$3\"); }; f"
    
    # git log でセマンティック差分を表示
    logx = "!f() { git log --oneline \"$@\" | while read commit msg; do echo \"$commit: $msg\"; git diffx-prev HEAD~1 HEAD 2>/dev/null | head -5; echo; done; }; f"
    
    # プッシュ前にすべての設定を検証
    validate-configs = "!find . -name '*.json' -o -name '*.yaml' -o -name '*.yml' | xargs -I {} sh -c 'echo \"Validating {}\"; diffx {} {} --output json > /dev/null && echo \"✅ {}\" || echo \"❌ {}\"'"
```

## コンテナエコシステム

### Docker 統合

#### 設定検証付きマルチステージビルド

```dockerfile
# Dockerfile
FROM rust:1.70-alpine AS diffx-builder
RUN cargo install diffx

FROM node:18-alpine AS app-builder
COPY --from=diffx-builder /usr/local/cargo/bin/diffx /usr/local/bin/

# 設定ファイルをコピー
COPY config/ ./config/
COPY config.schema.json ./

# ビルド中に設定を検証
RUN diffx config/default.json config/production.json \
    --ignore-keys-regex "^(environment|host|port)$" \
    --output json > /tmp/config_diff.json && \
    if [ -s /tmp/config_diff.json ]; then \
        echo "設定検証完了"; \
        cat /tmp/config_diff.json; \
    fi

# アプリビルドを続行...
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

FROM node:18-alpine
COPY --from=diffx-builder /usr/local/cargo/bin/diffx /usr/local/bin/
COPY --from=app-builder /app/dist ./dist
COPY --from=app-builder /app/config ./config
COPY --from=app-builder /tmp/config_diff.json ./

# 設定検証を含むヘルスチェックを追加
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD node health-check.js && diffx config/runtime.json config/expected.json --output json > /dev/null
```

#### 設定監視付き Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  app:
    build: .
    volumes:
      - ./config:/app/config:ro
      - config-monitor:/tmp/config-monitor
    environment:
      - CONFIG_MONITOR_ENABLED=true
    
  config-monitor:
    image: rust:1.70-alpine
    volumes:
      - ./config:/config:ro
      - config-monitor:/tmp/config-monitor
      - ./scripts:/scripts:ro
    command: |
      sh -c "
        cargo install diffx
        /scripts/config-monitor.sh
      "
    restart: unless-stopped

volumes:
  config-monitor:
```

```bash
#!/bin/bash
# scripts/config-monitor.sh

BASELINE_CONFIG="/config/baseline.json"
CURRENT_CONFIG="/config/current.json"
MONITOR_FILE="/tmp/config-monitor/status"

while true; do
    if [ -f "$CURRENT_CONFIG" ] && [ -f "$BASELINE_CONFIG" ]; then
        # 設定ドリフトをチェック
        if ! diffx "$BASELINE_CONFIG" "$CURRENT_CONFIG" \
             --ignore-keys-regex "^(timestamp|uptime|pid)$" \
             --output json > /tmp/config_drift.json; then
            
            echo "$(date): 設定ドリフトを検出" >> "$MONITOR_FILE"
            cat /tmp/config_drift.json >> "$MONITOR_FILE"
            
            # アラートメカニズム（webhook、slack等）
            curl -X POST "$ALERT_WEBHOOK_URL" \
                 -H "Content-Type: application/json" \
                 -d "{\"message\": \"設定ドリフトを検出\", \"details\": $(cat /tmp/config_drift.json)}"
        else
            echo "$(date): 設定安定" >> "$MONITOR_FILE"
        fi
    fi
    
    sleep 300  # 5分ごとにチェック
done
```

### Kubernetes 統合

#### ConfigMap 検証

```yaml
# k8s-config-validator.yaml
apiVersion: batch/v1
kind: Job
metadata:
  name: config-validator
spec:
  template:
    spec:
      containers:
      - name: validator
        image: rust:1.70-alpine
        command:
        - sh
        - -c
        - |
          cargo install diffx
          
          # 現在の ConfigMap を取得
          kubectl get configmap app-config -o jsonpath='{.data.config\.json}' > current_config.json
          
          # 期待される設定と比較
          diffx expected_config.json current_config.json \
            --ignore-keys-regex "^(namespace|resourceVersion|creationTimestamp)$" \
            --output json > config_validation.json
          
          if [ -s config_validation.json ]; then
            echo "設定検証の問題を発見:"
            cat config_validation.json
            exit 1
          else
            echo "設定検証が通過"
          fi
        volumeMounts:
        - name: expected-config
          mountPath: /expected_config.json
          subPath: config.json
      volumes:
      - name: expected-config
        configMap:
          name: expected-app-config
      restartPolicy: Never
  backoffLimit: 4
```

#### 設定検証付き Helm チャート

```yaml
# templates/config-validation-job.yaml
{{- if .Values.configValidation.enabled }}
apiVersion: batch/v1
kind: Job
metadata:
  name: {{ include "myapp.fullname" . }}-config-validation
  annotations:
    "helm.sh/hook": pre-install,pre-upgrade
    "helm.sh/hook-weight": "-5"
spec:
  template:
    spec:
      containers:
      - name: config-validator
        image: {{ .Values.configValidation.image }}
        command:
        - sh
        - -c
        - |
          # diffx をインストール
          cargo install diffx
          
          # Helm で生成された設定をスキーマと検証
          echo '{{ .Values.config | toJson }}' > generated_config.json
          
          diffx schema_config.json generated_config.json \
            --ignore-keys-regex "{{ .Values.configValidation.ignoreKeys }}" \
            --output json > validation_result.json
          
          if [ -s validation_result.json ]; then
            echo "Helm 設定検証失敗:"
            cat validation_result.json
            exit 1
          fi
        volumeMounts:
        - name: config-schema
          mountPath: /schema_config.json
          subPath: schema.json
      volumes:
      - name: config-schema
        configMap:
          name: {{ include "myapp.fullname" . }}-config-schema
      restartPolicy: Never
{{- end }}
```

## クラウドプラットフォーム

### AWS 統合

#### S3 設定監視用 Lambda 関数

```python
# lambda_function.py
import json
import boto3
import subprocess
import os
from datetime import datetime

def lambda_handler(event, context):
    s3 = boto3.client('s3')
    
    # diffx バイナリをダウンロード（Lambda 用にプリコンパイル済み）
    if not os.path.exists('/tmp/diffx'):
        s3.download_file('my-tools-bucket', 'diffx-lambda', '/tmp/diffx')
        os.chmod('/tmp/diffx', 0o755)
    
    # この関数をトリガーした S3 オブジェクトを取得
    bucket = event['Records'][0]['s3']['bucket']['name']
    key = event['Records'][0]['s3']['object']['key']
    
    if not key.endswith(('.json', '.yaml', '.yml')):
        return {'statusCode': 200, 'body': '設定ファイルではありません'}
    
    # 現在とベースラインの設定をダウンロード
    s3.download_file(bucket, key, '/tmp/current_config')
    
    baseline_key = key.replace('current/', 'baseline/')
    try:
        s3.download_file(bucket, baseline_key, '/tmp/baseline_config')
    except:
        return {'statusCode': 200, 'body': 'ベースライン設定が見つかりません'}
    
    # diffx 比較を実行
    result = subprocess.run([
        '/tmp/diffx', 
        '/tmp/baseline_config', 
        '/tmp/current_config',
        '--ignore-keys-regex', '^(timestamp|lastModified|version)$',
        '--output', 'json'
    ], capture_output=True, text=True)
    
    if result.returncode != 0:
        # 設定ドリフトを検出
        diff_data = json.loads(result.stdout) if result.stdout else []
        
        # アラート用に SNS に送信
        sns = boto3.client('sns')
        message = {
            'bucket': bucket,
            'key': key,
            'timestamp': datetime.utcnow().isoformat(),
            'differences': diff_data
        }
        
        sns.publish(
            TopicArn=os.environ['SNS_TOPIC_ARN'],
            Message=json.dumps(message),
            Subject=f'設定ドリフトを検出: {key}'
        )
        
        return {
            'statusCode': 200,
            'body': json.dumps({
                'message': '設定ドリフトを検出してアラートを送信',
                'differences': diff_data
            })
        }
    
    return {'statusCode': 200, 'body': '設定ドリフトは検出されませんでした'}
```

#### CloudFormation テンプレート検証

```yaml
# cloudformation-config-validator.yaml
AWSTemplateFormatVersion: '2010-09-09'
Description: '設定検証パイプライン'

Parameters:
  ConfigBucket:
    Type: String
    Description: 設定ファイルを含む S3 バケット

Resources:
  ConfigValidationRole:
    Type: AWS::IAM::Role
    Properties:
      AssumeRolePolicyDocument:
        Version: '2012-10-17'
        Statement:
          - Effect: Allow
            Principal:
              Service: codebuild.amazonaws.com
            Action: sts:AssumeRole
      ManagedPolicyArns:
        - arn:aws:iam::aws:policy/AWSCodeBuildDeveloperAccess
      Policies:
        - PolicyName: S3Access
          PolicyDocument:
            Version: '2012-10-17'
            Statement:
              - Effect: Allow
                Action:
                  - s3:GetObject
                  - s3:PutObject
                Resource: !Sub '${ConfigBucket}/*'

  ConfigValidationProject:
    Type: AWS::CodeBuild::Project
    Properties:
      ServiceRole: !GetAtt ConfigValidationRole.Arn
      Artifacts:
        Type: NO_ARTIFACTS
      Environment:
        Type: LINUX_CONTAINER
        ComputeType: BUILD_GENERAL1_SMALL
        Image: aws/codebuild/amazonlinux2-x86_64-standard:3.0
        EnvironmentVariables:
          - Name: CONFIG_BUCKET
            Value: !Ref ConfigBucket
      Source:
        Type: NO_SOURCE
        BuildSpec: |
          version: 0.2
          phases:
            install:
              runtime-versions:
                rust: 1.70
              commands:
                - cargo install diffx
            build:
              commands:
                - |
                  # S3 から設定をダウンロード
                  aws s3 cp s3://$CONFIG_BUCKET/production.json production.json
                  aws s3 cp s3://$CONFIG_BUCKET/staging.json staging.json
                  
                  # 一貫性を検証
                  diffx production.json staging.json \
                    --ignore-keys-regex "^(environment|host|replicas)$" \
                    --output json > validation_result.json
                  
                  # 結果をアップロード
                  aws s3 cp validation_result.json s3://$CONFIG_BUCKET/validation/
                  
                  if [ -s validation_result.json ]; then
                    echo "設定の不整合を発見"
                    cat validation_result.json
                    exit 1
                  fi
```

### Azure DevOps

#### 設定検証付きパイプライン

```yaml
# azure-pipelines.yml
trigger:
  branches:
    include:
    - main
    - develop
  paths:
    include:
    - config/*

pool:
  vmImage: 'ubuntu-latest'

variables:
  DIFFX_VERSION: '0.2.0'

stages:
- stage: Validate
  displayName: '設定検証'
  jobs:
  - job: ValidateConfigs
    displayName: '設定ファイル検証'
    steps:
    - task: Bash@3
      displayName: 'diffx インストール'
      inputs:
        targetType: 'inline'
        script: |
          curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz
          sudo mv diffx /usr/local/bin/
          diffx --version
    
    - task: Bash@3
      displayName: '設定変更検証'
      inputs:
        targetType: 'inline'
        script: |
          # このコミットで変更されたファイルを取得
          CHANGED_FILES=$(git diff HEAD~1 HEAD --name-only | grep -E '\.(json|yaml|yml)$' || true)
          
          if [ -n "$CHANGED_FILES" ]; then
            echo "変更された設定ファイルを検証中:"
            echo "$CHANGED_FILES"
            
            for file in $CHANGED_FILES; do
              echo "=== $file を検証中 ==="
              
              # 前のバージョンを取得
              git show HEAD~1:"$file" > "previous_$file" 2>/dev/null || echo '{}' > "previous_$file"
              
              # バージョンを比較
              diffx "previous_$file" "$file" \
                --ignore-keys-regex "^(buildId|timestamp|version)$" \
                --output json > "diff_$(echo $file | tr '/' '_').json"
              
              # 結果を処理
              if [ -s "diff_$(echo $file | tr '/' '_').json" ]; then
                echo "$file で変更を検出:"
                cat "diff_$(echo $file | tr '/' '_').json" | jq -r '.[] | 
                  if .Added then "  + \(.Added[0]): \(.Added[1])"
                  elif .Removed then "  - \(.Removed[0]): \(.Removed[1])"
                  elif .Modified then "  ~ \(.Modified[0]): \(.Modified[1]) → \(.Modified[2])"
                  else . end'
              else
                echo "$file にセマンティックな変更なし"
              fi
              echo ""
            done
          else
            echo "設定ファイルの変更なし"
          fi
    
    - task: PublishBuildArtifacts@1
      displayName: '検証結果公開'
      inputs:
        pathtoPublish: 'diff_*.json'
        artifactName: 'config-validation-results'
      condition: always()
```

### Google Cloud Platform

#### 設定監視用 Cloud Function

```python
# main.py for Google Cloud Function
import json
import subprocess
import tempfile
import os
from google.cloud import storage
from google.cloud import pubsub_v1

def validate_config_change(event, context):
    """Cloud Storage バケットへの変更によってトリガーされる。"""
    
    file_name = event['name']
    bucket_name = event['bucket']
    
    if not file_name.endswith(('.json', '.yaml', '.yml')):
        print(f'非設定ファイルを無視: {file_name}')
        return
    
    # diffx バイナリをダウンロード（関数にプリデプロイ済み）
    diffx_path = '/workspace/diffx'  # デプロイメントに含まれる
    
    client = storage.Client()
    bucket = client.bucket(bucket_name)
    
    # 現在のファイルをダウンロード
    blob = bucket.blob(file_name)
    current_config = tempfile.NamedTemporaryFile(mode='w+b', delete=False)
    blob.download_to_filename(current_config.name)
    
    # ベースライン設定をダウンロード
    baseline_name = file_name.replace('current/', 'baseline/')
    try:
        baseline_blob = bucket.blob(baseline_name)
        baseline_config = tempfile.NamedTemporaryFile(mode='w+b', delete=False)
        baseline_blob.download_to_filename(baseline_config.name)
    except:
        print(f'{file_name} のベースラインが見つかりません')
        return
    
    # diffx を実行
    result = subprocess.run([
        diffx_path,
        baseline_config.name,
        current_config.name,
        '--ignore-keys-regex', '^(timestamp|gcp_metadata)$',
        '--output', 'json'
    ], capture_output=True, text=True)
    
    # 一時ファイルをクリーンアップ
    os.unlink(current_config.name)
    os.unlink(baseline_config.name)
    
    if result.returncode != 0:
        # 設定ドリフトを検出
        diff_data = json.loads(result.stdout) if result.stdout else []
        
        # アラート用に Pub/Sub に公開
        publisher = pubsub_v1.PublisherClient()
        topic_path = publisher.topic_path(
            os.environ['GCP_PROJECT'], 
            os.environ['PUBSUB_TOPIC']
        )
        
        message_data = {
            'bucket': bucket_name,
            'file': file_name,
            'differences': diff_data
        }
        
        publisher.publish(topic_path, json.dumps(message_data).encode('utf-8'))
        print(f'{file_name} で設定ドリフトを検出')
    else:
        print(f'{file_name} でドリフト検出なし')
```

## 監視とアラート

### Prometheus 統合

#### 設定ドリフト エクスポーター

```python
#!/usr/bin/env python3
# config_drift_exporter.py

import time
import subprocess
import json
import os
from prometheus_client import start_http_server, Gauge, Counter, Info
import schedule

# Prometheus メトリクス
config_drift_detected = Gauge('config_drift_detected', 'Configuration drift detected', ['config_file'])
config_validation_errors = Counter('config_validation_errors_total', 'Total configuration validation errors')
config_last_check = Gauge('config_last_check_timestamp', 'Last configuration check timestamp')
config_info = Info('config_info', 'Configuration file information')

class ConfigDriftMonitor:
    def __init__(self, config_dir='/etc/app/config'):
        self.config_dir = config_dir
        self.baseline_dir = os.path.join(config_dir, 'baseline')
        self.current_dir = os.path.join(config_dir, 'current')
    
    def check_drift(self):
        """監視対象すべてのファイルで設定ドリフトをチェック。"""
        try:
            config_files = [f for f in os.listdir(self.current_dir) 
                          if f.endswith(('.json', '.yaml', '.yml'))]
            
            for config_file in config_files:
                current_path = os.path.join(self.current_dir, config_file)
                baseline_path = os.path.join(self.baseline_dir, config_file)
                
                if not os.path.exists(baseline_path):
                    continue
                
                # diffx を実行
                result = subprocess.run([
                    'diffx',
                    baseline_path,
                    current_path,
                    '--ignore-keys-regex', '^(timestamp|pid|uptime)$',
                    '--output', 'json'
                ], capture_output=True, text=True)
                
                if result.returncode != 0:
                    # ドリフトを検出
                    config_drift_detected.labels(config_file=config_file).set(1)
                    
                    # 詳細をログ
                    diff_data = json.loads(result.stdout) if result.stdout else []
                    print(f"{config_file} でドリフト検出: {len(diff_data)} 個の差分")
                else:
                    config_drift_detected.labels(config_file=config_file).set(0)
            
            config_last_check.set(time.time())
            
        except Exception as e:
            config_validation_errors.inc()
            print(f"設定チェック中のエラー: {e}")

def main():
    monitor = ConfigDriftMonitor()
    
    # 定期チェックをスケジュール
    schedule.every(5).minutes.do(monitor.check_drift)
    
    # Prometheus メトリクスサーバー開始
    start_http_server(8000)
    print("設定ドリフトエクスポーターがポート8000で開始されました")
    
    # 初期チェック
    monitor.check_drift()
    
    # メインループ
    while True:
        schedule.run_pending()
        time.sleep(1)

if __name__ == '__main__':
    main()
```

#### Grafana ダッシュボード設定

```json
{
  "dashboard": {
    "title": "設定ドリフト監視",
    "panels": [
      {
        "title": "設定ドリフト状態",
        "type": "stat",
        "targets": [
          {
            "expr": "sum(config_drift_detected) by (config_file)",
            "legendFormat": "{{config_file}}"
          }
        ],
        "fieldConfig": {
          "defaults": {
            "thresholds": {
              "steps": [
                {"color": "green", "value": 0},
                {"color": "red", "value": 1}
              ]
            }
          }
        }
      },
      {
        "title": "設定チェックエラー",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(config_validation_errors_total[5m])",
            "legendFormat": "検証エラー/秒"
          }
        ]
      },
      {
        "title": "最終設定チェック",
        "type": "stat",
        "targets": [
          {
            "expr": "time() - config_last_check_timestamp",
            "legendFormat": "最終チェックからの秒数"
          }
        ]
      }
    ]
  }
}
```

### Slack 統合

#### 設定変更通知

```bash
#!/bin/bash
# slack_config_notifier.sh

SLACK_WEBHOOK_URL="$1"
CONFIG_FILE="$2"
DIFF_FILE="$3"

if [ ! -f "$DIFF_FILE" ] || [ ! -s "$DIFF_FILE" ]; then
    exit 0  # レポートする差分なし
fi

# 差分データを解析
DIFF_SUMMARY=$(cat "$DIFF_FILE" | jq -r '
    group_by(keys[0]) | 
    map({
        type: .[0] | keys[0],
        count: length,
        items: map(.[keys[0]][0]) | join(", ")
    }) | 
    map("\(.type): \(.count) (\(.items))") | 
    join("\n")'
)

# Slack メッセージを作成
SLACK_MESSAGE=$(cat <<EOF
{
    "text": "設定変更を検出",
    "attachments": [
        {
            "color": "warning",
            "fields": [
                {
                    "title": "ファイル",
                    "value": "$CONFIG_FILE",
                    "short": true
                },
                {
                    "title": "タイムスタンプ",
                    "value": "$(date -u +%Y-%m-%dT%H:%M:%SZ)",
                    "short": true
                },
                {
                    "title": "変更概要",
                    "value": "\`\`\`$DIFF_SUMMARY\`\`\`",
                    "short": false
                }
            ],
            "actions": [
                {
                    "type": "button",
                    "text": "詳細を見る",
                    "url": "$BUILD_URL"
                }
            ]
        }
    ]
}
EOF
)

# Slack に送信
curl -X POST -H 'Content-type: application/json' \
     --data "$SLACK_MESSAGE" \
     "$SLACK_WEBHOOK_URL"
```

## 開発ツール

### VSCode 拡張統合

#### diffx 言語サーバー設定

```json
// .vscode/settings.json
{
    "diffx.enabled": true,
    "diffx.configPath": ".diffx.toml",
    "diffx.ignorePatterns": [
        "^(timestamp|_.*|createdAt|updatedAt)$"
    ],
    "diffx.autoValidate": true,
    "diffx.outputFormat": "cli",
    "files.associations": {
        "*.diffx": "json",
        ".diffx": "toml"
    }
}
```

#### タスク設定

```json
// .vscode/tasks.json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "diffx: ベースラインと比較",
            "type": "shell",
            "command": "diffx",
            "args": [
                "config/baseline.json",
                "${file}",
                "--output", "cli"
            ],
            "group": "test",
            "presentation": {
                "echo": true,
                "reveal": "always",
                "focus": false,
                "panel": "shared"
            },
            "problemMatcher": {
                "pattern": {
                    "regexp": "^([+~!-])\\s+(.*?):\\s+(.*)$",
                    "file": 1,
                    "location": 2,
                    "message": 3
                }
            }
        },
        {
            "label": "diffx: すべての設定を検証",
            "type": "shell",
            "command": "find",
            "args": [
                "config/",
                "-name", "*.json",
                "-exec", "diffx", "{}", "{}.backup", ";"
            ],
            "group": "test"
        }
    ]
}
```

### IntelliJ IDEA 統合

#### 外部ツール設定

```xml
<!-- File: .idea/tools/External Tools.xml -->
<toolSet name="diffx">
  <tool name="ベースラインと比較" showInMainMenu="true" showInEditor="true" showInProject="true" showInSearchPopup="true" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="diffx" />
      <option name="PARAMETERS" value="config/baseline/$FileNameWithoutExtension$.$FileExt$ $FilePath$ --output cli" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
  <tool name="git とのセマンティック差分" showInMainMenu="true" showInEditor="true" showInProject="false" showInSearchPopup="false" disabled="false" useConsole="true" showConsoleOnStdOut="false" showConsoleOnStdErr="false" synchronizeAfterRun="true">
    <exec>
      <option name="COMMAND" value="bash" />
      <option name="PARAMETERS" value="-c &quot;git show HEAD~1:$FileRelativePath$ | diffx - $FilePath$&quot;" />
      <option name="WORKING_DIRECTORY" value="$ProjectFileDir$" />
    </exec>
  </tool>
</toolSet>
```

## 自動化スクリプト

### 包括的設定管理スクリプト

```bash
#!/bin/bash
# config-manager.sh - diffx を使った包括的設定管理

set -euo pipefail

# 設定
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
CONFIG_DIR="${CONFIG_DIR:-./config}"
BASELINE_DIR="${BASELINE_DIR:-./config/baseline}"
BACKUP_DIR="${BACKUP_DIR:-./config/backups}"
LOG_FILE="${LOG_FILE:-/var/log/config-manager.log}"

# デフォルト diffx オプション
IGNORE_REGEX="${IGNORE_REGEX:-^(timestamp|lastModified|createdAt|updatedAt|buildTime|version)$}"
OUTPUT_FORMAT="${OUTPUT_FORMAT:-json}"

# ログ関数
log() {
    echo "[$(date +'%Y-%m-%d %H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

# diffx が利用可能かチェック
check_diffx() {
    if ! command -v diffx &> /dev/null; then
        log "エラー: diffx が見つかりません。次でインストール: cargo install diffx"
        exit 1
    fi
}

# ディレクトリ構造を作成
setup_directories() {
    mkdir -p "$CONFIG_DIR" "$BASELINE_DIR" "$BACKUP_DIR"
    log "ディレクトリ構造を作成しました"
}

# 現在の設定をバックアップ
backup_configs() {
    local backup_timestamp=$(date +'%Y%m%d_%H%M%S')
    local backup_path="$BACKUP_DIR/$backup_timestamp"
    
    mkdir -p "$backup_path"
    
    find "$CONFIG_DIR" -name "*.json" -o -name "*.yaml" -o -name "*.yml" -o -name "*.toml" | \
    while read -r config_file; do
        if [[ ! "$config_file" =~ backup ]]; then
            cp "$config_file" "$backup_path/"
            log "バックアップ済み: $config_file"
        fi
    done
    
    log "バックアップ完了: $backup_path"
}

# ベースラインに対して設定を検証
validate_config() {
    local config_file="$1"
    local baseline_file="$BASELINE_DIR/$(basename "$config_file")"
    
    if [[ ! -f "$baseline_file" ]]; then
        log "警告: $config_file のベースラインが見つかりません"
        return 0
    fi
    
    local diff_output=$(mktemp)
    
    if diffx "$baseline_file" "$config_file" \
       --ignore-keys-regex "$IGNORE_REGEX" \
       --output "$OUTPUT_FORMAT" > "$diff_output"; then
        log "✅ $config_file: ベースラインとのセマンティックな差分なし"
        rm "$diff_output"
        return 0
    else
        log "⚠️  $config_file: ベースラインとの差分を検出"
        
        # 差分を分析
        local added=$(jq '[.[] | select(.Added)] | length' "$diff_output" 2>/dev/null || echo "0")
        local removed=$(jq '[.[] | select(.Removed)] | length' "$diff_output" 2>/dev/null || echo "0")
        local modified=$(jq '[.[] | select(.Modified)] | length' "$diff_output" 2>/dev/null || echo "0")
        local type_changed=$(jq '[.[] | select(.TypeChanged)] | length' "$diff_output" 2>/dev/null || echo "0")
        
        log "  追加: $added, 削除: $removed, 変更: $modified, 型変更: $type_changed"
        
        # 重要な変更をチェック
        local critical_changes=$(jq '[.[] | select(.Removed or .TypeChanged or 
            (.Modified and (.Modified[0] | 
             contains("security") or contains("auth") or contains("password") or contains("key"))))]' "$diff_output" 2>/dev/null || echo "[]")
        
        if [[ "$critical_changes" != "[]" ]]; then
            log "🔴 重要: $config_file で潜在的に危険な変更を検出"
            echo "$critical_changes" | jq -r '.[] | 
                if .Removed then "  削除: \(.Removed[0])"
                elif .TypeChanged then "  型変更: \(.TypeChanged[0])"
                elif .Modified then "  変更: \(.Modified[0])"
                else . end' >> "$LOG_FILE"
            
            # レビュー用の詳細差分を保存
            cp "$diff_output" "$BACKUP_DIR/critical_diff_$(basename "$config_file")_$(date +'%Y%m%d_%H%M%S').json"
        fi
        
        rm "$diff_output"
        return 1
    fi
}

# すべての設定を検証
validate_all() {
    local validation_failed=false
    
    log "設定検証を開始中..."
    
    find "$CONFIG_DIR" -name "*.json" -o -name "*.yaml" -o -name "*.yml" -o -name "*.toml" | \
    while read -r config_file; do
        if [[ ! "$config_file" =~ (baseline|backup) ]]; then
            if ! validate_config "$config_file"; then
                validation_failed=true
            fi
        fi
    done
    
    if [[ "$validation_failed" == "true" ]]; then
        log "❌ 設定検証が問題を伴って完了"
        return 1
    else
        log "✅ すべての設定が正常に検証されました"
        return 0
    fi
}

# 現在の設定でベースラインを更新
update_baseline() {
    local config_file="$1"
    local baseline_file="$BASELINE_DIR/$(basename "$config_file")"
    
    cp "$config_file" "$baseline_file"
    log "ベースライン更新: $baseline_file"
}

# 2つの設定ファイルを比較
compare_configs() {
    local file1="$1"
    local file2="$2"
    
    log "$file1 と $file2 を比較中"
    
    diffx "$file1" "$file2" \
        --ignore-keys-regex "$IGNORE_REGEX" \
        --output cli
}

# 設定レポートを生成
generate_report() {
    local report_file="${1:-config_report_$(date +'%Y%m%d_%H%M%S').html}"
    
    log "設定レポート生成中: $report_file"
    
    cat > "$report_file" << 'EOF'
<!DOCTYPE html>
<html>
<head>
    <title>設定検証レポート</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 20px; }
        .summary { background: #f5f5f5; padding: 10px; border-radius: 5px; }
        .success { color: green; }
        .warning { color: orange; }
        .error { color: red; }
        pre { background: #f0f0f0; padding: 10px; overflow-x: auto; }
    </style>
</head>
<body>
    <h1>設定検証レポート</h1>
    <div class="summary">
        <h2>概要</h2>
        <p>生成日時: $(date)</p>
        <p>設定ディレクトリ: $CONFIG_DIR</p>
        <p>ベースラインディレクトリ: $BASELINE_DIR</p>
    </div>
    
    <h2>検証結果</h2>
EOF
    
    find "$CONFIG_DIR" -name "*.json" -o -name "*.yaml" -o -name "*.yml" -o -name "*.toml" | \
    while read -r config_file; do
        if [[ ! "$config_file" =~ (baseline|backup) ]]; then
            echo "<h3>$(basename "$config_file")</h3>" >> "$report_file"
            
            local baseline_file="$BASELINE_DIR/$(basename "$config_file")"
            if [[ -f "$baseline_file" ]]; then
                local diff_output=$(mktemp)
                if diffx "$baseline_file" "$config_file" \
                   --ignore-keys-regex "$IGNORE_REGEX" \
                   --output json > "$diff_output"; then
                    echo '<p class="success">✅ ベースラインとの差分なし</p>' >> "$report_file"
                else
                    echo '<p class="warning">⚠️ 差分を検出:</p>' >> "$report_file"
                    echo '<pre>' >> "$report_file"
                    cat "$diff_output" | jq -r '.[] | 
                        if .Added then "+ \(.Added[0]): \(.Added[1])"
                        elif .Removed then "- \(.Removed[0]): \(.Removed[1])"
                        elif .Modified then "~ \(.Modified[0]): \(.Modified[1]) → \(.Modified[2])"
                        elif .TypeChanged then "! \(.TypeChanged[0]): \(.TypeChanged[1]) → \(.TypeChanged[2]) (型変更)"
                        else . end' >> "$report_file"
                    echo '</pre>' >> "$report_file"
                fi
                rm "$diff_output"
            else
                echo '<p class="warning">⚠️ ベースライン設定が見つかりません</p>' >> "$report_file"
            fi
        fi
    done
    
    echo '</body></html>' >> "$report_file"
    log "レポート生成完了: $report_file"
}

# メイン関数
main() {
    local command="${1:-help}"
    
    case "$command" in
        "setup")
            setup_directories
            ;;
        "backup")
            check_diffx
            backup_configs
            ;;
        "validate")
            check_diffx
            validate_all
            ;;
        "validate-file")
            check_diffx
            if [[ -z "${2:-}" ]]; then
                echo "使用法: $0 validate-file <config-file>"
                exit 1
            fi
            validate_config "$2"
            ;;
        "update-baseline")
            if [[ -z "${2:-}" ]]; then
                echo "使用法: $0 update-baseline <config-file>"
                exit 1
            fi
            update_baseline "$2"
            ;;
        "compare")
            check_diffx
            if [[ -z "${2:-}" ]] || [[ -z "${3:-}" ]]; then
                echo "使用法: $0 compare <file1> <file2>"
                exit 1
            fi
            compare_configs "$2" "$3"
            ;;
        "report")
            check_diffx
            generate_report "${2:-}"
            ;;
        "monitor")
            check_diffx
            log "設定監視を開始中..."
            while true; do
                validate_all || log "⚠️ 設定検証が失敗"
                sleep "${MONITOR_INTERVAL:-300}"  # デフォルト: 5分
            done
            ;;
        "help"|*)
            cat << EOF
diffx を使った設定マネージャー

使用法: $0 <コマンド> [オプション]

コマンド:
    setup                   ディレクトリ構造を作成
    backup                  現在の設定をバックアップ
    validate               すべての設定をベースラインと検証
    validate-file <file>   特定の設定ファイルを検証
    update-baseline <file> 現在の設定でベースラインを更新
    compare <file1> <file2> 2つの設定ファイルを比較
    report [file]          HTML検証レポートを生成
    monitor                継続監視を開始（MONITOR_INTERVAL環境変数を使用）
    help                   このヘルプメッセージを表示

環境変数:
    CONFIG_DIR             設定ディレクトリ (デフォルト: ./config)
    BASELINE_DIR           ベースライン設定ディレクトリ (デフォルト: ./config/baseline)
    BACKUP_DIR             バックアップディレクトリ (デフォルト: ./config/backups)
    LOG_FILE               ログファイルパス (デフォルト: /var/log/config-manager.log)
    IGNORE_REGEX     無視するキーの正規表現パターン
    OUTPUT_FORMAT    diffx の出力フォーマット (デフォルト: json)
    MONITOR_INTERVAL       監視間隔（秒） (デフォルト: 300)

例:
    $0 setup
    $0 backup
    $0 validate
    $0 validate-file config/app.json
    $0 compare config/prod.json config/staging.json
    $0 report config_report.html
    MONITOR_INTERVAL=60 $0 monitor
EOF
            ;;
    esac
}

# すべての引数でメイン関数を実行
main "$@"
```

この包括的な統合ガイドにより、diffx をあらゆる開発・運用環境に効果的に組み込むことができます。
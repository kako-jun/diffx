# はじめに

`diffx` へようこそ！このガイドでは、構造化データのためのセマンティック差分ツールの基本的な使い方から応用まで、実践的に学んでいきます。

## 目次

- [diffx とは](#diffx-とは)
- [基本概念](#基本概念)
- [基本的な使い方](#基本的な使い方)
- [コマンドラインオプション](#コマンドラインオプション)
- [出力形式](#出力形式)
- [高度な機能](#高度な機能)
- [設定とカスタマイズ](#設定とカスタマイズ)
- [統合例](#統合例)
- [よくある使用パターン](#よくある使用パターン)
- [次のステップ](#次のステップ)

## diffx とは

`diffx` は、JSON、YAML、TOML、XML、INI、CSVなどの構造化データファイルの **意味的な差分** を抽出するツールです。

### 従来のdiffとの違い

**従来のdiff:**
```bash
$ diff config1.json config2.json
< {
<   "name": "myapp",
<   "version": "1.0",
<   "debug": true
< }
> {
<   "debug": false,
<   "version": "1.1",
<   "name": "myapp"
> }
```

**diffx:**
```bash
$ diffx config1.json config2.json
~ version: "1.0" -> "1.1"
~ debug: true -> false
```

### 主な特徴

- **🎯 セマンティック理解**: キーの順序、空白、ケツカンマを無視
- **🔧 多様なフォーマット**: 6つの構造化データ形式をサポート
- **🤖 AI対応**: 機械可読な出力形式でAIツールとの連携が容易
- **⚡ 高速**: Rustで実装された高性能
- **🔗 メタチェイン**: 差分レポートの差分も取得可能

## 基本概念

### セマンティック差分とは

セマンティック差分とは、データの **意味的な変更** のみに注目した差分のことです。

```json
// ファイル1: config_old.json
{
  "database": {
    "host": "localhost",
    "port": 5432
  },
  "cache": {
    "enabled": true
  }
}

// ファイル2: config_new.json  
{
  "cache": {
    "enabled": false,
    "ttl": 3600
  },
  "database": {
    "port": 5432,
    "host": "localhost"
  }
}
```

**従来のdiff** は多くの変更を報告しますが、**diffx** は実際の意味的変更のみを報告します：

```bash
$ diffx config_old.json config_new.json
+ cache.ttl: 3600
~ cache.enabled: true -> false
```

### 差分の種類

diffx は4種類の変更を検出します：

- **➕ 追加 (Added)**: 新しいキー・値の追加
- **➖ 削除 (Removed)**: 既存のキー・値の削除  
- **🔄 変更 (Modified)**: 値の変更
- **⚠️ 型変更 (TypeChanged)**: データ型の変更

## 基本的な使い方

### 初回実行

まず、簡単な例で動作を確認してみましょう：

```bash
# サンプルファイルの作成
echo '{"name": "test", "version": "1.0"}' > file1.json
echo '{"name": "test", "version": "1.1"}' > file2.json

# 基本的な比較
diffx file1.json file2.json
```

**出力:**
```
~ version: "1.0" -> "1.1"
```

### 複数フォーマットの対応

diffx は6つのフォーマットをサポートしています：

```bash
# JSON
diffx config.json config_new.json

# YAML  
diffx deploy.yaml deploy_new.yaml

# TOML
diffx Cargo.toml Cargo_new.toml

# XML
diffx config.xml config_new.xml

# INI
diffx database.ini database_new.ini

# CSV
diffx data.csv data_new.csv
```

### 自動フォーマット検出

diffx は拡張子から自動的にフォーマットを判別します：

```bash
# 拡張子による自動判別
diffx config.json config.yaml     # JSONとYAMLを自動判別
diffx app.toml app.xml            # TOMLとXMLを自動判別

# 手動指定も可能
diffx --format json file1.txt file2.txt
```

## コマンドラインオプション

### 基本オプション

#### ヘルプと情報
```bash
# ヘルプ表示
diffx --help
diffx -h

# バージョン確認  
diffx --version
diffx -V
```

#### 入力形式の指定
```bash
# フォーマット指定
diffx --format json file1.txt file2.txt
diffx -f yaml config1 config2

# 標準入力の使用
cat config.json | diffx - config_new.json
echo '{"test": 1}' | diffx - '{"test": 2}'
```

### 出力オプション

#### 出力形式の選択
```bash
# CLI表示（デフォルト）
diffx file1.json file2.json

# JSON出力
diffx file1.json file2.json --output json
diffx file1.json file2.json -o json

# YAML出力
diffx file1.json file2.json --output yaml

# 統一diff形式
diffx file1.json file2.json --output unified
```

#### 出力例の比較

**CLI出力（推奨）:**
```bash
$ diffx config.json config_new.json
+ database.port: 5432
~ version: "1.0" -> "1.1"  
- cache.enabled: true
```

**JSON出力:**
```bash
$ diffx config.json config_new.json --output json
[
  {"Added": ["database.port", 5432]},
  {"Modified": ["version", "1.0", "1.1"]},
  {"Removed": ["cache.enabled", true]}
]
```

**YAML出力:**
```bash
$ diffx config.json config_new.json --output yaml
- Added:
  - database.port
  - 5432
- Modified:
  - version
  - "1.0"
  - "1.1"
```

## 高度な機能

### フィルタリング

#### キーの無視（正規表現）
```bash
# タイムスタンプフィールドを無視
diffx config.json config_new.json --ignore-keys-regex "^timestamp$"

# 内部フィールドを無視（_で始まるもの）
diffx data.json data_new.json --ignore-keys-regex "^_.*"

# 複数パターンの無視
diffx log.json log_new.json --ignore-keys-regex "^(timestamp|_.*|temp_.*)$"

# 大文字小文字を無視
diffx config.json config_new.json --ignore-keys-regex "(?i)password"
```

#### パスフィルタリング
```bash
# 特定のセクションのみ比較
diffx large_config.json large_config_new.json --path "database"

# ネストしたパス
diffx config.json config_new.json --path "services.web.environment"

# 配列の特定要素
diffx users.json users_new.json --path "users[0]"

# 複雑なパス
diffx config.json config_new.json --path "services.api.env[0].name"
```

### 配列の処理

#### IDベースの配列追跡
```bash
# IDキーを指定した配列比較
diffx users.json users_new.json --array-id-key "id"

# ユニークキーでの追跡
diffx products.json products_new.json --array-id-key "sku"

# プライマリキーでの追跡
diffx records.json records_new.json --array-id-key "primary_key"
```

**例:**
```json
// users.json
[
  {"id": 1, "name": "Alice"},
  {"id": 2, "name": "Bob"}
]

// users_new.json
[
  {"id": 2, "name": "Bob"},
  {"id": 1, "name": "Alice Smith"}
]
```

```bash
# IDキーなしの場合
$ diffx users.json users_new.json
~ [0]: {"id": 1, "name": "Alice"} -> {"id": 2, "name": "Bob"}
~ [1]: {"id": 2, "name": "Bob"} -> {"id": 1, "name": "Alice Smith"}

# IDキーありの場合
$ diffx users.json users_new.json --array-id-key "id"
~ users[id=1].name: "Alice" -> "Alice Smith"
```

### 数値の許容誤差

#### 浮動小数点数の比較
```bash
# 小さな差を無視
diffx metrics.json metrics_new.json --epsilon 0.001

# 科学データでの使用
diffx measurements.json measurements_new.json --epsilon 0.01

# 金融データでの厳密な比較
diffx financial.json financial_new.json --epsilon 0.000001
```

**例:**
```json
// file1.json
{"pi": 3.14159}

// file2.json  
{"pi": 3.14160}
```

```bash
# 厳密比較（差分を検出）
$ diffx file1.json file2.json
~ pi: 3.14159 -> 3.14160

# 許容誤差付き（差分なし）
$ diffx file1.json file2.json --epsilon 0.001
# 出力なし（差分なし）
```

### ディレクトリ比較

#### 再帰比較
```bash
# ディレクトリ全体の比較
diffx config_dir1/ config_dir2/ --recursive
diffx config_dir1/ config_dir2/ -r

# フィルタリングと組み合わせ
diffx configs/ configs_backup/ -r --ignore-keys-regex "^(timestamp|version)$"

# 特定出力形式で
diffx env/dev/ env/prod/ -r --output json > env_diff.json
```

## 設定とカスタマイズ

### 設定ファイル

diffx は TOML 形式の設定ファイルをサポートしています：

```bash
# 設定ファイルの場所
~/.config/diffx/config.toml
```

**設定例:**
```toml
# ~/.config/diffx/config.toml

# デフォルト出力形式
output = "json"

# デフォルトで無視するキー
ignore_keys_regex = "^(timestamp|_.*|createdAt|updatedAt)$"

# デフォルトの許容誤差
epsilon = 0.001

# デフォルトの配列IDキー
array_id_key = "id"

# 再帰比較を有効化
recursive = true
```

### 環境変数

設定を環境変数で上書きできます：

```bash
# 出力形式
export DIFFX_OUTPUT=json

# 無視パターン
export DIFFX_IGNORE_KEYS_REGEX="^(timestamp|_.*)"

# 許容誤差
export DIFFX_EPSILON=0.001

# 配列IDキー
export DIFFX_ARRAY_ID_KEY="id"

# 実行
diffx config.json config_new.json  # 上記設定が適用される
```

### 優先順位

設定の優先順位（高い順）：

1. **コマンドライン引数**
2. **環境変数**  
3. **設定ファイル**
4. **デフォルト値**

## 統合例

### Git 統合

#### Git フック
```bash
#!/bin/bash
# .git/hooks/pre-commit

# package.json の変更をチェック
if git diff --cached --name-only | grep -q "package.json"; then
  # 新しい依存関係の追加をチェック
  git show HEAD:package.json > /tmp/package_old.json
  git show :package.json > /tmp/package_new.json
  
  if diffx /tmp/package_old.json /tmp/package_new.json --output json | jq -e '.[] | select(.Added)' > /dev/null; then
    echo "新しい依存関係が検出されました。セキュリティ監査を実行します..."
    npm audit
  fi
fi
```

#### Git alias
```bash
# Git alias の設定
git config alias.diffx '!f() { git show HEAD~1:"$1" > /tmp/git_diffx_old && diffx /tmp/git_diffx_old "$1"; }; f'

# 使用例
git diffx config.json
```

### CI/CD 統合

#### GitHub Actions
```yaml
name: Configuration Validation
on:
  pull_request:
    paths: ['config/**/*.json', '**/*.yaml']

jobs:
  validate-config:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v4
      with:
        fetch-depth: 0
        
    - name: Install diffx
      run: |
        curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz
        sudo mv diffx /usr/local/bin/
        
    - name: Check configuration changes
      run: |
        CHANGED_FILES=$(git diff --name-only origin/${{ github.base_ref }}...HEAD | grep -E '\\.(json|yaml|yml)$' || true)
        
        for file in $CHANGED_FILES; do
          if [ -f "$file" ]; then
            git show origin/${{ github.base_ref }}:"$file" > /tmp/base_file 2>/dev/null || continue
            
            diffx /tmp/base_file "$file" \
              --ignore-keys-regex "^(timestamp|lastModified|buildTime)$" \
              --output json > "diff_$file.json"
              
            # 重要な変更をチェック
            if jq -e '.[] | select(.Removed or .TypeChanged)' "diff_$file.json" > /dev/null; then
              echo "::warning title=Critical Config Change::Critical changes detected in $file"
            fi
          fi
        done
```

### スクリプト統合

#### 設定ドリフト検出
```bash
#!/bin/bash
# config_drift_monitor.sh

BASELINE_CONFIG="/opt/app/config/baseline.json"
CURRENT_CONFIG="/opt/app/config/current.json"

# 現在の設定を取得
curl -s http://localhost:8080/api/config > "$CURRENT_CONFIG"

# ベースラインと比較
if ! diffx "$BASELINE_CONFIG" "$CURRENT_CONFIG" \
     --ignore-keys-regex "^(timestamp|uptime|last_.*)$" \
     --output json > config_drift.json; then
  
  echo "設定ドリフトが検出されました！"
  
  # 重要な変更をチェック
  CRITICAL=$(jq -r '.[] | select(.Removed or .TypeChanged or (.Modified and (.Modified[0] | contains("security") or contains("database"))))' config_drift.json)
  
  if [ -n "$CRITICAL" ]; then
    echo "⚠️ 重要な設定変更が検出されました"
    echo "$CRITICAL" | jq .
    
    # アラート送信
    curl -X POST "https://hooks.slack.com/services/YOUR/SLACK/WEBHOOK" \
      -H "Content-Type: application/json" \
      -d "{\"text\": \"Critical configuration drift detected!\", \"attachments\": [{\"text\": \"$(cat config_drift.json)\"}]}"
  fi
else
  echo "設定ドリフトなし"
fi
```

## よくある使用パターン

### 1. 設定ファイル管理

#### 環境間の設定比較
```bash
# 開発環境と本番環境の比較
diffx config/dev.json config/prod.json \
  --ignore-keys-regex "^(host|port|password|secret_.*)$"

# Kubernetes マニフェストの比較
diffx k8s/staging.yaml k8s/production.yaml \
  --ignore-keys-regex "^metadata\\.(creationTimestamp|resourceVersion)"
```

#### 設定変更の検証
```bash
# デプロイ前の設定検証
diffx config/current.json config/proposed.json \
  --output json | jq '.[] | select(.Removed or .TypeChanged)'
```

### 2. データ処理

#### ETL パイプライン検証
```bash
# データ変換の検証
diffx input_data.json output_data.json \
  --array-id-key "record_id" \
  --epsilon 0.001 \
  --ignore-keys-regex "^(processed_at|batch_id)$"
```

#### データ品質チェック
```bash
# 日次データ比較
diffx daily_metrics_$(date -d yesterday +%Y%m%d).json \
     daily_metrics_$(date +%Y%m%d).json \
  --epsilon 0.05
```

### 3. API テスト

#### レスポンス構造検証
```bash
# API スキーマ変更検出
diffx api_v1_schema.json api_v2_schema.json \
  --path "definitions" \
  --output yaml
```

#### 契約テスト
```bash
# API 契約の検証
for endpoint in users products orders; do
  curl -s "https://api.example.com/$endpoint" > "actual_$endpoint.json"
  diffx "expected_$endpoint.json" "actual_$endpoint.json" \
    --ignore-keys-regex "^(id|timestamp|request_id)$"
done
```

### 4. インフラストラクチャ

#### Terraform 状態比較
```bash
# インフラドリフト検出
diffx terraform.tfstate terraform.tfstate.backup \
  --path "resources" \
  --ignore-keys-regex "^(last_updated|timeouts)"
```

#### Docker Compose 比較
```bash
# 環境固有設定の比較
diffx docker-compose.yml docker-compose.override.yml \
  --path "services" \
  --output unified
```

## 次のステップ

### より詳しく学ぶ

1. **[設定ガイド](configuration.md)** - 詳細な設定オプション
2. **[実用例](examples.md)** - 業界別の実用例
3. **[CLI リファレンス](../reference/cli-reference.md)** - 全コマンドオプション
4. **[統合ガイド](../guides/integrations.md)** - CI/CD統合の詳細

### 高度な機能

- **[API リファレンス](../reference/api-reference.md)** - Rust ライブラリとして使用
- **[パフォーマンスガイド](../guides/performance.md)** - 最適化とベンチマーク
- **[ツール比較](../reference/comparison.md)** - 他ツールとの比較

### コミュニティ

- **[GitHub Issues](https://github.com/kako-jun/diffx/issues)** - バグ報告・機能要望
- **[GitHub Discussions](https://github.com/kako-jun/diffx/discussions)** - コミュニティディスカッション

### 貢献

diffx プロジェクトへの貢献に興味がある場合は、[コントリビューションガイド](../../CONTRIBUTING.md) をご確認ください。

---

これで `diffx` の基本的な使い方がわかりましたね！構造化データの意味的な差分を効率的に取得して、開発やデータ管理の品質向上に役立ててください。

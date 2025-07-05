# 設定ガイド

このガイドでは、`diffx` の設定オプションと、様々な環境や用途に応じたカスタマイズ方法について説明します。

## 目次

- [設定の概要](#設定の概要)
- [設定ファイル](#設定ファイル)
- [環境変数](#環境変数)
- [設定の優先順位](#設定の優先順位)
- [設定例](#設定例)
- [チーム設定](#チーム設定)
- [トラブルシューティング](#トラブルシューティング)

## 設定の概要

`diffx` は柔軟な設定システムを提供しており、以下の方法で設定できます：

1. **コマンドライン引数** - 一時的な設定や特定の用途
2. **環境変数** - シェルセッションやスクリプト内での設定
3. **設定ファイル** - 永続的な個人/プロジェクト設定
4. **デフォルト値** - 何も設定しない場合の標準動作

## 設定ファイル

### 設定ファイルの場所

`diffx` は以下の場所から設定ファイルを読み込みます（優先順位順）：

1. **カレントディレクトリ**: `./diffx.toml`
2. **プロジェクトルート**: `./.diffx/config.toml` 
3. **ユーザー設定**: `~/.config/diffx/config.toml`
4. **システム設定**: `/etc/diffx/config.toml` (Linux/macOS)

### 基本的な設定ファイル

`~/.config/diffx/config.toml` の例：

```toml
# diffx 設定ファイル
# ファイル形式: TOML

# == 基本設定 ==

# デフォルト出力形式
# 値: "cli", "json", "yaml", "unified"
output = "cli"

# デフォルト入力形式（自動判別が失敗した場合）
# 値: "json", "yaml", "toml", "xml", "ini", "csv"
# format = "json"

# == フィルタリング設定 ==

# 無視するキーのパターン（正規表現）
ignore_keys_regex = "^(timestamp|createdAt|updatedAt|_.*|temp_.*)$"

# パスフィルタリング（特定のパスのみ比較）
# path = "config.database"

# == 比較設定 ==

# 浮動小数点数の許容誤差
epsilon = 0.001

# 配列要素の識別キー
array_id_key = "id"

# == ディレクトリ設定 ==

# 再帰的なディレクトリ比較を有効化
recursive = false

# == 出力設定 ==

# カラー出力の制御
# 値: "auto", "always", "never"
colors = "auto"

# 詳細出力モード
verbose = false
```

### 設定値の詳細

#### 出力設定

**`output`** - 出力形式
```toml
output = "cli"     # 人間向け表示（推奨）
output = "json"    # 機械可読JSON
output = "yaml"    # 機械可読YAML
output = "unified" # 従来のdiff形式
```

**`colors`** - カラー出力制御
```toml
colors = "auto"   # ターミナル検出で自動判定
colors = "always" # 常にカラー出力
colors = "never"  # カラー出力無効
```

#### フィルタリング設定

**`ignore_keys_regex`** - キー無視パターン
```toml
# タイムスタンプフィールドを無視
ignore_keys_regex = "^(timestamp|createdAt|updatedAt)$"

# 内部フィールド（_開始）を無視
ignore_keys_regex = "^_.*$"

# 一時ファイル関連を無視
ignore_keys_regex = "^(temp_.*|cache_.*|tmp_.*)$"

# 大文字小文字を無視した無視パターン
ignore_keys_regex = "(?i)(password|secret|token)"
```

**`path`** - パスフィルタリング
```toml
path = "database"              # トップレベルのdatabaseセクション
path = "services.web"          # ネストしたパス
path = "users[0]"              # 配列の特定要素
path = "config.env[*].name"    # ワイルドカードパス
```

#### 比較設定

**`epsilon`** - 数値比較の許容誤差
```toml
epsilon = 0.001      # 一般的な用途
epsilon = 0.0001     # 高精度データ
epsilon = 0.01       # 測定データ
epsilon = 0.000001   # 金融データ
```

**`array_id_key`** - 配列要素識別
```toml
array_id_key = "id"          # 一般的なIDフィールド
array_id_key = "name"        # 名前をキーとして使用
array_id_key = "uuid"        # UUIDをキーとして使用
array_id_key = "primary_key" # データベース主キー
```

### プロジェクト固有設定

プロジェクトルートに `.diffx/config.toml` を作成して、プロジェクト固有の設定を行います：

```toml
# .diffx/config.toml - プロジェクト固有設定

# このプロジェクトでは常にJSON出力
output = "json"

# Kubernetesマニフェスト用の無視パターン
ignore_keys_regex = "^(metadata\\.(creationTimestamp|resourceVersion|uid|generation)|status\\..*)"

# 設定ファイルのルートパス
path = "spec"

# ポッド配列をnameで識別
array_id_key = "name"

# 本番環境用設定
[production]
ignore_keys_regex = "^(metadata\\.(creationTimestamp|resourceVersion)|status\\..*|spec\\.template\\.metadata\\.labels\\.version)"
epsilon = 0.0

# 開発環境用設定
[development]
verbose = true
colors = "always"
```

## 環境変数

すべての設定は環境変数で上書き可能です。環境変数名は `DIFFX_` プレフィックスを付けて大文字にします。

### 基本的な環境変数

```bash
# 出力形式
export DIFFX_OUTPUT=json

# 入力形式
export DIFFX_FORMAT=yaml

# 無視パターン
export DIFFX_IGNORE_KEYS_REGEX="^(timestamp|_.*)"

# パスフィルタリング
export DIFFX_PATH="database.connections"

# 許容誤差
export DIFFX_EPSILON=0.001

# 配列IDキー
export DIFFX_ARRAY_ID_KEY="id"

# 再帰モード
export DIFFX_RECURSIVE=true

# カラー出力
export DIFFX_COLORS=always

# 詳細モード
export DIFFX_VERBOSE=true
```

### CI/CD環境での活用

```bash
# GitHub Actions
- name: Compare configurations
  env:
    DIFFX_OUTPUT: json
    DIFFX_IGNORE_KEYS_REGEX: "^(timestamp|buildId|deploymentTime)$"
  run: |
    diffx config/baseline.json config/current.json

# Docker環境
docker run --rm \
  -e DIFFX_OUTPUT=json \
  -e DIFFX_IGNORE_KEYS_REGEX="^(createdAt|updatedAt)$" \
  -v $(pwd):/data \
  diffx-image /data/file1.json /data/file2.json
```

### シェル関数での活用

```bash
# .bashrc または .zshrc
diffx_config() {
  DIFFX_IGNORE_KEYS_REGEX="^(timestamp|_.*|createdAt|updatedAt)$" \
  DIFFX_OUTPUT=json \
  diffx "$@"
}

diffx_k8s() {
  DIFFX_IGNORE_KEYS_REGEX="^(metadata\\.(creationTimestamp|resourceVersion)|status\\..*)" \
  DIFFX_OUTPUT=yaml \
  diffx "$@"
}

diffx_api() {
  DIFFX_IGNORE_KEYS_REGEX="^(timestamp|request_id|server_time)$" \
  DIFFX_OUTPUT=json \
  diffx "$@"
}
```

## 設定の優先順位

設定値は以下の優先順位で決定されます（高い順）：

1. **コマンドライン引数**
2. **環境変数**
3. **プロジェクト設定ファイル** (`.diffx/config.toml`)
4. **ユーザー設定ファイル** (`~/.config/diffx/config.toml`)
5. **システム設定ファイル** (`/etc/diffx/config.toml`)
6. **デフォルト値**

### 優先順位の例

```bash
# ~/.config/diffx/config.toml
output = "yaml"
ignore_keys_regex = "^timestamp$"

# 環境変数
export DIFFX_OUTPUT=json

# コマンド実行
diffx file1.json file2.json --output cli --ignore-keys-regex "^_.*$"

# 実際に適用される設定:
# output = "cli"          (コマンドライン引数)
# ignore_keys_regex = "^_.*$"  (コマンドライン引数)
```

## 設定例

### 用途別設定例

#### 1. 設定ファイル管理用

```toml
# ~/.config/diffx/config.toml
output = "cli"
ignore_keys_regex = "^(timestamp|lastModified|version|buildTime)$"
colors = "auto"

[kubernetes]
ignore_keys_regex = "^(metadata\\.(creationTimestamp|resourceVersion|uid)|status\\..*)"
array_id_key = "name"

[docker-compose]
ignore_keys_regex = "^(build\\..*)"
path = "services"
```

#### 2. API テスト用

```toml
# ~/.config/diffx/config.toml
output = "json"
ignore_keys_regex = "^(timestamp|request_id|server_time|duration)$"
epsilon = 0.001

[api-testing]
ignore_keys_regex = "^(id|timestamp|request_id|server_time|etag|last_modified)$"
output = "json"
```

#### 3. データ処理用

```toml
# ~/.config/diffx/config.toml
output = "yaml"
array_id_key = "record_id"
epsilon = 0.01
ignore_keys_regex = "^(processed_at|batch_id|execution_time)$"

[etl-pipeline]
epsilon = 0.001
ignore_keys_regex = "^(timestamp|batch_.*|process_.*|_metadata)"
array_id_key = "id"
```

#### 4. 開発環境用

```toml
# .diffx/config.toml (プロジェクトルート)
output = "cli"
colors = "always"
verbose = true

# 開発中は厳密な比較
epsilon = 0.0

# デバッグ情報を無視
ignore_keys_regex = "^(debug_.*|dev_.*|local_.*)"

[testing]
output = "json"
ignore_keys_regex = "^(test_.*|mock_.*|fixture_.*)"
```

### 環境別設定

#### 開発環境
```bash
# ~/.bashrc
export DIFFX_OUTPUT=cli
export DIFFX_COLORS=always
export DIFFX_VERBOSE=true
export DIFFX_IGNORE_KEYS_REGEX="^(timestamp|debug_.*)"
```

#### 本番環境
```bash
# 本番環境スクリプト
export DIFFX_OUTPUT=json
export DIFFX_COLORS=never
export DIFFX_IGNORE_KEYS_REGEX="^(timestamp|last_.*|temp_.*)"
export DIFFX_EPSILON=0.001
```

#### CI/CD環境
```yaml
# .github/workflows/config-check.yml
env:
  DIFFX_OUTPUT: json
  DIFFX_IGNORE_KEYS_REGEX: "^(timestamp|buildId|commitSha)$"
  DIFFX_COLORS: never
```

## チーム設定

### 共有設定ファイル

プロジェクトに設定ファイルを含めて、チーム全体で共有：

```toml
# .diffx/config.toml（バージョン管理に含める）
# チーム共有設定

# 基本設定
output = "cli"
colors = "auto"

# プロジェクト固有の無視パターン
ignore_keys_regex = "^(metadata\\.(timestamp|version)|_internal.*)"

# 配列識別設定
array_id_key = "id"

# 環境別設定
[environments.development]
verbose = true
ignore_keys_regex = "^(metadata\\.(timestamp|version)|_internal.*|debug_.*)"

[environments.staging]
ignore_keys_regex = "^(metadata\\.(timestamp|version)|_internal.*)"
epsilon = 0.001

[environments.production]
ignore_keys_regex = "^(metadata\\.(timestamp|version)|_internal.*)"
epsilon = 0.0
colors = "never"
```

### チーム向けドキュメント

```markdown
# diffx 設定ガイド（チーム用）

## 基本設定
プロジェクトルートの `.diffx/config.toml` を使用してください。

## 個人設定
個人的な設定は `~/.config/diffx/config.toml` に記載してください。

## CI/CD設定
以下の環境変数を設定してください：
- DIFFX_OUTPUT=json
- DIFFX_IGNORE_KEYS_REGEX="^(timestamp|buildId)$"
```

### 設定の検証

```bash
# 設定確認スクリプト
#!/bin/bash

echo "=== diffx 設定確認 ==="

# 現在の設定を表示
echo "Output format: $DIFFX_OUTPUT"
echo "Ignore pattern: $DIFFX_IGNORE_KEYS_REGEX"
echo "Array ID key: $DIFFX_ARRAY_ID_KEY"

# テスト実行
echo '{"name": "test", "timestamp": "2024-01-01"}' > test1.json
echo '{"name": "test", "timestamp": "2024-01-02"}' > test2.json

echo "=== テスト結果 ==="
diffx test1.json test2.json

# クリーンアップ
rm test1.json test2.json
```

## トラブルシューティング

### 設定が反映されない

#### 1. 設定ファイルの場所を確認
```bash
# 設定ファイルの場所をチェック
ls -la ~/.config/diffx/config.toml
ls -la .diffx/config.toml
ls -la diffx.toml
```

#### 2. 設定ファイルの構文を確認
```bash
# TOML構文のチェック
toml-validator ~/.config/diffx/config.toml

# または、diffxで構文確認
diffx --help  # エラーが出れば構文エラー
```

#### 3. 環境変数の確認
```bash
# diffx関連の環境変数をチェック
env | grep DIFFX_

# 設定の競合をチェック
echo "Current settings:"
echo "DIFFX_OUTPUT: $DIFFX_OUTPUT"
echo "DIFFX_IGNORE_KEYS_REGEX: $DIFFX_IGNORE_KEYS_REGEX"
```

### 正規表現が動作しない

#### 1. 正規表現のテスト
```bash
# 正規表現の動作確認
echo "timestamp" | grep -E "^(timestamp|_.*)"
echo $?  # 0なら一致、1なら不一致
```

#### 2. エスケープの確認
```bash
# TOMLファイルでの正しいエスケープ
ignore_keys_regex = "^metadata\\.(timestamp|version)$"

# 環境変数での正しいエスケープ
export DIFFX_IGNORE_KEYS_REGEX="^metadata\\.(timestamp|version)$"
```

### パフォーマンスの問題

#### 1. 過度なフィルタリング
```toml
# 問題のある設定（遅い）
ignore_keys_regex = ".*metadata.*|.*timestamp.*|.*debug.*"

# 改善された設定（速い）
ignore_keys_regex = "^(metadata_.*|timestamp|debug_.*)"
```

#### 2. 不適切な epsilon 設定
```toml
# 問題のある設定（精度が高すぎる）
epsilon = 0.000000001

# 適切な設定
epsilon = 0.001
```

### 設定のデバッグ

#### デバッグモードでの実行
```bash
# 詳細情報を表示
DIFFX_VERBOSE=true diffx file1.json file2.json

# 設定値の確認
DIFFX_DEBUG=true diffx --help
```

#### 設定値の段階的テスト
```bash
# デフォルト設定でテスト
diffx file1.json file2.json

# 環境変数を追加してテスト
DIFFX_OUTPUT=json diffx file1.json file2.json

# 設定ファイルを追加してテスト
echo 'output = "yaml"' > test_config.toml
DIFFX_CONFIG=test_config.toml diffx file1.json file2.json
```

### よくある問題と解決策

#### 1. 設定が無視される
**原因**: コマンドライン引数が設定ファイルを上書き  
**解決**: コマンドライン引数を削除するか、意図的な設定か確認

#### 2. 正規表現がマッチしない
**原因**: TOMLエスケープやパス区切り文字の問題  
**解決**: バックスラッシュを二重にエスケープ

#### 3. 配列比較が期待通りに動かない
**原因**: array_id_keyが存在しない、または重複している  
**解決**: データ内でユニークなキーを使用

#### 4. 設定ファイルが読み込まれない
**原因**: ファイルの権限やパスが間違っている  
**解決**: ファイルパスと権限を確認

---

この設定ガイドを参考に、あなたの使用環境や用途に最適な `diffx` 設定を行ってください。設定により、効率的で精確な差分検出が可能になります。
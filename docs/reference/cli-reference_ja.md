# CLI リファレンス

`diffx` コマンドラインインターフェースの完全なリファレンスドキュメントです。

## 概要

```
diffx [OPTIONS] <INPUT1> <INPUT2>
```

## 説明

`diffx` は構造化データファイルのセマンティック比較を行うコマンドラインツールです。従来のテキストベースのdiffツールとは異なり、`diffx` はデータの構造と意味を理解し、フォーマットの違いではなく実際の変更に焦点を当てます。

## 引数

### `<INPUT1>`
- **型**: ファイルパス、ディレクトリパス、または標準入力の `-`
- **必須**: はい
- **説明**: 比較する最初の入力

### `<INPUT2>`
- **型**: ファイルパス、ディレクトリパス、または標準入力の `-`
- **必須**: はい
- **説明**: 比較する2番目の入力

**例:**
```bash
# 2つのファイルを比較
diffx config.json config.new.json

# 標準入力と比較
cat config.json | diffx - config.new.json

# ディレクトリ比較（Unixのdiff互換 - デフォルトは非再帰）
diffx config_dir1/ config_dir2/
```

## オプション

### フォーマットオプション

#### `-f, --format <FORMAT>`
- **型**: 文字列
- **デフォルト**: ファイル拡張子から自動検出
- **値**: `json`, `yaml`, `toml`, `xml`, `ini`, `csv`
- **説明**: 特定の入力ファイル形式を強制指定

**例:**
```bash
# JSON解釈を強制
diffx --format json file1.txt file2.txt

# YAML解釈を強制
diffx -f yaml config1 config2
```

**自動検出マッピング:**
- `.json` → `json`
- `.yaml`, `.yml` → `yaml`
- `.toml` → `toml`
- `.xml` → `xml`
- `.ini`, `.cfg`, `.conf` → `ini`
- `.csv` → `csv`

### 出力オプション

#### `-o, --output <FORMAT>`
- **型**: 文字列
- **デフォルト**: `diffx`（人間が読みやすいdiffx形式）
- **値**: `diffx`, `json`, `yaml`, `unified`
- **説明**: 差分の出力形式

**diffx形式（デフォルト）:**
```bash
diffx config.json config.new.json
# 出力:
# + database.port: 5432
# ~ version: "1.0" -> "1.1"
# - cache.enabled: true
```

**JSON出力:**
```bash
diffx config.json config.new.json --output json
# 出力:
# [
#   {"Added": ["database.port", 5432]},
#   {"Modified": ["version", "1.0", "1.1"]},
#   {"Removed": ["cache.enabled", true]}
# ]
```

**YAML出力:**
```bash
diffx config.json config.new.json --output yaml
# 出力:
# - Added:
#   - database.port
#   - 5432
# - Modified:
#   - version
#   - "1.0"
#   - "1.1"
```

**Unified出力:**
```bash
diffx config.json config.new.json --output unified
# 出力: 従来のdiffスタイル形式
```

### フィルタリングオプション

#### `--path <PATH>`
- **型**: 文字列
- **デフォルト**: なし（構造全体を比較）
- **説明**: データ構造内の特定のパスに差分をフィルタリング

**パス構文:**
- オブジェクトキー: `database.host`
- 配列インデックス: `users[0]`
- ネストしたパス: `config.database.connection.host`
- 複雑なパス: `services.web.env[0].name`

**例:**
```bash
# データベース設定のみを比較
diffx config.json config.new.json --path "database"

# 特定の配列要素を比較
diffx config.json config.new.json --path "users[0]"

# 深くネストしたパス
diffx config.json config.new.json --path "services.web.environment.variables"
```

#### `--ignore-keys-regex <PATTERN>`
- **型**: 正規表現文字列
- **デフォルト**: なし
- **説明**: 指定した正規表現にマッチするキーを無視

**一般的なパターン:**
```bash
# タイムスタンプフィールドを無視
diffx file1.json file2.json --ignore-keys-regex "^(timestamp|createdAt|updatedAt)$"

# 内部フィールド（アンダースコアで始まる）を無視
diffx file1.json file2.json --ignore-keys-regex "^_.*"

# 複数パターンを無視
diffx file1.json file2.json --ignore-keys-regex "^(id|timestamp|_.*|temp_.*)$"

# バージョン関連フィールドを無視
diffx file1.json file2.json --ignore-keys-regex "(version|buildNumber|revision)"
```

**正規表現の例:**
- `^timestamp$` - "timestamp"の完全一致
- `^_.*` - アンダースコアで始まるフィールド
- `.*_temp$` - "_temp"で終わるフィールド
- `^(id|uid|pk)$` - id、uid、pkのいずれかに一致
- `(?i)password` - "password"の大文字小文字を区別しない一致

### 比較オプション

#### `--epsilon <VALUE>`
- **型**: 浮動小数点数
- **デフォルト**: `0.0`（完全比較）
- **説明**: 浮動小数点数比較の許容値

**例:**
```bash
# 浮動小数点数の小さな差異を許可
diffx metrics.json metrics.new.json --epsilon 0.001

# 科学データ用のより寛容な許容値
diffx measurements.json measurements.new.json --epsilon 0.01

# 非常に厳密な比較
diffx financial.json financial.new.json --epsilon 0.000001
```

**使用例:**
- 測定精度のある科学データ
- 丸め誤差のある財務計算
- 小さな変動のあるパフォーマンスメトリクス
- 浮動小数点アーティファクトのある変換データ

#### `--array-id-key <KEY>`
- **型**: 文字列
- **デフォルト**: なし（位置による比較）
- **説明**: 配列要素の識別と追跡に使用するキー

**例:**
```bash
# ユーザーをIDで追跡
diffx users.json users.updated.json --array-id-key "id"

# 製品をSKUで追跡
diffx inventory.json inventory.new.json --array-id-key "sku"

# データベースレコードを主キーで追跡
diffx records.json records.new.json --array-id-key "primary_key"
```

**ID追跡なし:**
```json
// 配列比較は位置の変更を表示
// 旧: [{"name": "Alice"}, {"name": "Bob"}]
// 新: [{"name": "Bob"}, {"name": "Alice"}]
// 結果: すべての要素が変更されたように表示
```

**ID追跡あり:**
```json
// 旧: [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]  
// 新: [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
// 結果: 変更なしを検出（同じ要素、異なる順序）
```

#### `--ignore-whitespace`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: 文字列値内の空白の違いを無視

**例:**
```bash
# 異なる空白のファイル
echo '{"text": "Hello  World"}' > file1.json
echo '{"text": "Hello World"}' > file2.json

# 通常の比較では差分を表示
diffx file1.json file2.json
# 出力: ~ text: "Hello  World" -> "Hello World"

# 空白を無視 - 差分は報告されない
diffx file1.json file2.json --ignore-whitespace
# 出力: （差分なし）
```

**使用例:**
- 空白が一貫しない設定ファイル
- 異なるシステムからエクスポートされたデータ
- 余分なスペースが導入された手動編集
- 正規化された対生データ

#### `--ignore-case`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: 文字列値の大文字小文字の違いを無視

**例:**
```bash
# 異なる大文字小文字のファイル
echo '{"status": "Active"}' > file1.json
echo '{"status": "ACTIVE"}' > file2.json

# 通常の比較では差分を表示
diffx file1.json file2.json
# 出力: ~ status: "Active" -> "ACTIVE"

# 大文字小文字を無視 - 差分は報告されない
diffx file1.json file2.json --ignore-case
# 出力: （差分なし）
```

**使用例:**
- 大文字小文字が異なるユーザー入力データ
- レガシーシステムマイグレーション
- 大文字小文字を区別しない設定値
- データ正規化タスク

**オプションの組み合わせ:**
```bash
# 空白と大文字小文字の両方の違いを処理
diffx config.json config.new.json --ignore-whitespace --ignore-case

# 複数オプションの複雑な例
diffx data.yaml data.updated.yaml \
  --ignore-case \
  --ignore-whitespace \
  --epsilon 0.001 \
  --ignore-keys-regex "^(timestamp|version)$"
```

### 出力制御オプション

#### `--context <N>`
- **型**: 整数
- **デフォルト**: なし（すべてのコンテキストを表示）
- **説明**: unified出力形式で差分の周りにN行のコンテキストを表示

**例:**
```bash
# 変更周辺に2行のコンテキストを表示
diffx config.json config.new.json --output unified --context 2

# 変更行のみを表示（コンテキストなし）
diffx config.json config.new.json --output unified --context 0

# デフォルト動作（すべてのコンテキスト）
diffx config.json config.new.json --output unified
```

**コンテキスト付きサンプル出力:**
```diff
# --context 2
  "database": {
    "host": "localhost",
-   "port": 5432
+   "port": 5433
  },
  "cache": {

# --context 0  
-   "port": 5432
+   "port": 5433
```

#### `-q, --quiet`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: 通常出力を抑制し、終了ステータスのみを返す

**例:**
```bash
# ファイルが異なるかチェック（スクリプト用）
diffx config.json config.new.json --quiet
echo $?  # 0 = 差分なし, 1 = 差分あり, 2 = エラー

# シェルスクリプトで使用
if diffx config.json backup.json --quiet; then
    echo "ファイルは同一です"
else
    echo "ファイルが異なります"
fi

# 他のオプションと組み合わせ
diffx large.json large.new.json --quiet --ignore-whitespace
```

**終了コード:**
- `0`: 差分なし
- `1`: 差分あり
- `2`: エラー発生（無効なファイル、形式エラーなど）

#### `--brief`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: ファイル名のみを報告し、差分は報告しない（`diff --brief`と類似）

**例:**
```bash
# ファイルが異なるかのみ報告
diffx config.json config.new.json --brief
# 出力: Files config.json and config.new.json differ

# ディレクトリ比較で使用
diffx configs/ configs.backup/ --recursive --brief
# 出力: Files configs/app.json and configs.backup/app.json differ

# フィルタリングと組み合わせ
diffx data.json data.new.json --brief --ignore-keys-regex "^timestamp$"
```

**使用例:**
- バッチ処理スクリプト
- クイックファイル比較チェック
- 自動テストパイプライン
- ファイル同期検証

#### `-v, --verbose`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: パフォーマンスメトリクス、設定詳細、処理統計を含む包括的な診断情報を表示

**例:**
```bash
# 基本的なverbose出力
diffx config.json config.new.json --verbose
# 出力に含まれる:
# Input file information: 
#   Input 1 size: 245 bytes
#   Input 2 size: 267 bytes
# Parse time: 15.2µs
# Diff computation time: 23.8µs
# Total differences found: 3
# Performance summary:
#   Total processing time: 125.4µs
#   Memory optimization: disabled

# フィルタリングオプション付きverbose
diffx data.json data.new.json --verbose --ignore-keys-regex "timestamp" --epsilon 0.1
# 追加出力:
# Key filtering configuration:
#   Regex pattern: timestamp
# Numerical tolerance configuration:
#   Epsilon value: 0.1

# verboseディレクトリ比較
diffx configs/ configs.backup/ --recursive --verbose
# 追加出力:
# Directory scan results:
#   Files in configs/: 12
#   Files in configs.backup/: 11
#   Total files to compare: 12
# Directory comparison summary:
#   Files compared: 11
#   Files only in one directory: 1
#   Differences found: Yes
```

**Verbose情報カテゴリ:**

1. **パフォーマンスメトリクス**
   - ファイルサイズとメモリ使用量
   - 解析時間、差分計算時間
   - 総処理時間
   - メモリ最適化ステータス

2. **設定詳細**
   - アクティブなフィルタリングパターン（正規表現、イプシロン、配列IDキー）
   - パスフィルタリング設定
   - コンテキスト表示設定

3. **処理統計**
   - フィルタリング前後の総差分数
   - ディレクトリスキャン結果
   - 比較効果性メトリクス

4. **診断出力**
   - 最適化決定
   - 処理バッチ情報
   - エラーコンテキストとトラブルシューティングデータ

**使用例:**
- パフォーマンス分析と最適化
- 遅い比較のトラブルシューティング
- フィルター効果の理解
- 設定問題のデバッグ
- CI/CDパイプライン診断
- サポートとメンテナンスタスク

#### `--no-color`
- **型**: ブールフラグ
- **デフォルト**: False（カラー出力有効）
- **説明**: スクリプト、パイプライン、またはANSIカラーをサポートしないターミナルとの互換性向上のため、カラー出力を無効化

**例:**
```bash
# カラーなしの基本使用
diffx config.json config.new.json --no-color
# 出力はカラーフォーマットなしのプレーンテキスト

# CI/CDパイプラインで使用
diffx deploy.yaml deploy.new.yaml --no-color --output json > diff_report.json

# 他の出力オプションと組み合わせ
diffx large.json large.new.json --no-color --brief --quiet

# カラーなしのディレクトリ比較
diffx configs/ configs.backup/ --recursive --no-color
```

**使用例:**
- カラーコードがログ解析を妨げるCI/CDパイプライン統合
- diffx出力を処理する自動スクリプト
- ANSIコードが不要なテキストファイル出力リダイレクト
- カラーをサポートしないターミナル環境
- スクリーンリーダーのアクセシビリティ対応
- ドキュメント用のクリーンなテキストレポート作成

### ディレクトリオプション

#### `-r, --recursive`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: サブディレクトリを通じてディレクトリを再帰的に比較（Unix diff互換）

**例:**
```bash
# --recursiveなしのディレクトリ比較（Unix diff互換）
# ディレクトリ内のファイルを直接比較、サブディレクトリには「Common subdirectories」を表示
diffx config_dir1/ config_dir2/
# 出力:
# Common subdirectories: config_dir1/subdir and config_dir2/subdir
# --- Comparing config.json ---
# ~ version: "1.0" -> "1.1"

# 再帰比較 - サブディレクトリを含むすべてのファイルを比較
diffx config_dir1/ config_dir2/ --recursive
# 出力:
# --- Comparing config.json ---
# ~ version: "1.0" -> "1.1"
# --- Comparing subdir/nested.json ---
# ~ data: "old" -> "new"

# 出力形式付き再帰比較
diffx environments/dev/ environments/prod/ -r --output json

# フィルタリング付き再帰
diffx configs/ configs.backup/ -r --ignore-keys-regex "^(timestamp|version)$"
```

**Unix diff互換動作:**

**`--recursive`フラグなし（デフォルト）:**
- 指定されたディレクトリ内のファイルのみを直接比較
- 両方の場所にあるサブディレクトリには「Common subdirectories」メッセージを表示
- サブディレクトリ内のファイルは比較しない
- 標準Unix `diff`コマンドとの互換性を維持

**`--recursive`フラグあり:**
- サブディレクトリを通じてすべてのファイルを再帰的に比較
- 出力でディレクトリ構造を維持
- `diff -r`動作と同等

**共通動作:**
- 両方のディレクトリに存在しないファイルをスキップ
- 各ファイルの形式自動検出を尊重
- 一方のディレクトリにのみ存在するファイルを報告

### パフォーマンスオプション

#### 自動最適化
- **型**: 自動機能
- **デフォルト**: 1MB以上のファイルで有効
- **説明**: 大容量ファイルとデータ構造に対してメモリ効率的処理が自動的に有効化

**自動検出動作:**
- ファイル ≤1MB: 標準モード（高速、無制限メモリ）
- ファイル >1MB: 最適化モード（メモリ効率的、バッチ処理）
- 手動設定不要 - 最適化は完全に透明

**最適化機能:**
- ファイルサイズに基づく自動検出
- 大容量データセット用のメモリ効率的処理
- 深くネストした構造用のバッチ処理
- モードに関係なく同一の出力を維持

**例:**
```bash
# 自動検出（常に有効）
diffx config.json config.new.json
# 小ファイルには標準モード、大ファイルには最適化を使用

# 大ファイルは自動的に最適化を使用
diffx massive_db.json massive_db.new.json --array-id-key "id" --path "users"
# 大ファイルに最適化モードを自動使用

# 他のすべてのオプションは最適化と透明に動作
diffx complex_data.json complex_data.v2.json --ignore-keys-regex "^timestamp$"
# 必要に応じて最適化が自動適用
```

**パフォーマンス動作:**
```bash
# 小ファイル（<1MB） - 自動標準モード
diffx config.json config.new.json
# 高速処理、無制限メモリ使用

# 大ファイル（>1MB） - 自動最適化モード  
diffx large_dataset.json large_dataset.v2.json
# メモリ効率的、バッチ処理

# 複雑なネスト構造 - 自動最適化
diffx deep_nested.json deep_nested.v2.json
# データ特性に基づく透明最適化
```

### 情報オプション

#### `-h, --help`
- **型**: ブールフラグ
- **説明**: ヘルプ情報を印刷して終了

#### `-V, --version`
- **型**: ブールフラグ
- **説明**: バージョン情報を印刷して終了

**例:**
```bash
# ヘルプを表示
diffx --help
diffx -h

# バージョンを表示
diffx --version
diffx -V
```

## 終了コード

`diffx` は以下の終了コードを使用します：

- **0**: 成功、差分なし
- **1**: 成功、差分あり
- **2**: コマンドライン引数エラー
- **3**: ファイルI/Oエラー
- **4**: 解析エラー（無効な形式）
- **5**: 内部エラー

**例:**
```bash
# ファイルが同一かチェック
if diffx file1.json file2.json >/dev/null 2>&1; then
    echo "ファイルは同一です"
else
    echo "ファイルが異なります"
fi

# 終了コードをキャプチャ
diffx config.json config.new.json
EXIT_CODE=$?
case $EXIT_CODE in
    0) echo "差分なし" ;;
    1) echo "差分あり" ;;
    *) echo "エラーが発生 (コード: $EXIT_CODE)" ;;
esac
```

## 使用パターン

### 基本比較

```bash
# シンプルなファイル比較
diffx file1.json file2.json

# 異なる形式での比較
diffx config.yaml config.toml --format yaml --format toml

# 標準入力とファイルの比較
curl -s https://api.example.com/config | diffx - local_config.json
```

### 高度なフィルタリング

```bash
# 複雑な無視パターン
diffx app.json app.new.json \
  --ignore-keys-regex "^(timestamp|_.*|createdAt|updatedAt|version)$"

# パス固有の比較
diffx large_config.json large_config.new.json \
  --path "database.connections"

# 複数オプションの組み合わせ
diffx users.json users.new.json \
  --array-id-key "user_id" \
  --ignore-keys-regex "^(last_login|session_.*)" \
  --output json
```

### ディレクトリ操作

```bash
# Unix diff互換ディレクトリ比較（非再帰）
diffx configs/ configs.backup/
# ディレクトリ内のファイルと「Common subdirectories」メッセージを表示

# 再帰ディレクトリ比較
diffx configs/ configs.backup/ --recursive

# フィルタリング付きディレクトリ比較
diffx env/dev/ env/prod/ \
  --recursive \
  --ignore-keys-regex "^(host|port|password)" \
  --output json > env_diff.json
```

### 統合例

```bash
# Git統合
git show HEAD~1:config.json > old_config.json
diffx old_config.json config.json --output unified

# CI/CDパイプライン
diffx expected_config.json actual_config.json \
  --ignore-keys-regex "^(deployment_time|build_id)" \
  --output json > config_validation.json

# 監視スクリプト
#!/bin/bash
if ! diffx baseline_config.json current_config.json \
     --ignore-keys-regex "^(timestamp|uptime)" >/dev/null; then
  echo "設定ドリフトが検出されました！"
  diffx baseline_config.json current_config.json --output json | \
    notify_alert_system.py
fi
```

## エラーハンドリング

### 一般的なエラー

**ファイルが見つからない:**
```bash
$ diffx nonexistent.json config.json
Error: No such file or directory (os error 2)
```

**無効な形式:**
```bash
$ diffx invalid.json valid.json
Error: Failed to parse JSON: expected `,` or `}` at line 1 column 15
```

**権限拒否:**
```bash
$ diffx protected.json config.json
Error: Permission denied (os error 13)
```

**無効な正規表現:**
```bash
$ diffx file1.json file2.json --ignore-keys-regex "[invalid"
Error: Invalid regular expression: unclosed character class
```

### デバッグ

```bash
# 形式検出を検証
diffx --format json file1.txt file2.txt
```

## パフォーマンスの考慮事項

### 大容量ファイル

```bash
# 大ファイルにはパスフィルタリングを使用
diffx huge1.json huge2.json --path "critical_section"

# 非必須データを無視
diffx large1.json large2.json --ignore-keys-regex "logs|debug|metadata"
```

### バッチ処理

```bash
# 複数ファイルの並列処理
find configs/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup || echo "Diff in {}"'
```

### メモリ使用量

非常に大容量のファイルの場合は以下を検討：
- 特定のセクションに焦点を当てるため `--path` を使用
- `--ignore-keys-regex` で大きな無関係なセクションをフィルタアウト
- 可能であれば小さなチャンクでファイルを処理

## 使用例（用途別）

### 設定管理
```bash
# 環境比較
diffx prod.json staging.json --ignore-keys-regex "^(host|port|secret_.*)"

# Kubernetesマニフェスト
diffx deployment.yaml deployment.new.yaml --ignore-keys-regex "^metadata\\.(creation.*|resource.*)"
```

### APIテスト
```bash
# レスポンス検証
diffx expected_response.json actual_response.json --ignore-keys-regex "^(timestamp|request_id)"

# スキーマ比較
diffx api_v1_schema.json api_v2_schema.json --path "definitions"
```

### データ処理
```bash
# ETL検証
diffx input_data.json output_data.json --array-id-key "record_id" --epsilon 0.001

# データベースエクスポート比較
diffx export1.json export2.json --array-id-key "id" --ignore-keys-regex "^(updated_at|sync_time)"
```

### セキュリティ監査
```bash
# ポリシー比較
diffx security_policy.json security_policy.new.json --path "permissions"

# アクセス制御検証
diffx rbac.yaml rbac.new.yaml --array-id-key "name"
```

この包括的なCLIリファレンスは、利用可能なすべてのオプションをカバーし、`diffx` の効果的な使用のための実用的な例を提供します。
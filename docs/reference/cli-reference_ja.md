# CLIリファレンス

`diffx` コマンドラインインターフェースの完全なリファレンスドキュメントです。

## 概要

```
diffx [オプション] <入力1> <入力2>
```

## 説明

`diffx` は、構造化データファイルのセマンティック（意味的）な比較を行うコマンドラインツールです。従来のテキストベースの差分ツールとは異なり、`diffx` はデータの構造と意味を理解し、フォーマットの違いではなく実際の変更点に焦点を当てます。

## 引数

### `<入力1>`
- **型**: ファイルパス、ディレクトリパス、または標準入力の場合は `-`
- **必須**: はい
- **説明**: 比較対象の最初の入力。

### `<入力2>`
- **型**: ファイルパス、ディレクトリパス、または標準入力の場合は `-`
- **必須**: はい
- **説明**: 比較対象の2番目の入力。

**使用例:**
```bash
# 2つのファイルを比較
diffx config.json config.new.json

# 標準入力からの入力と比較
cat config.json | diffx - config.new.json

# ディレクトリを比較（デフォルトでは非再帰的、Unixのdiffコマンドと互換性あり）
diffx config_dir1/ config_dir2/
```

## オプション

### フォーマット関連オプション

#### `-f, --format <フォーマット>`
- **型**: 文字列
- **デフォルト**: ファイル拡張子から自動検出
- **指定可能な値**: `json`, `yaml`, `toml`, `xml`, `ini`, `csv`
- **説明**: 特定の入力ファイルフォーマットを強制します。

**使用例:**
```bash
# JSON形式として強制的に解釈
diffx --format json file1.txt file2.txt

# YAML形式として強制的に解釈
diffx -f yaml config1 config2
```

**自動検出のマッピング:**
- `.json` → `json`
- `.yaml`, `.yml` → `yaml`
- `.toml` → `toml`
- `.xml` → `xml`
- `.ini`, `.cfg`, `.conf` → `ini`
- `.csv` → `csv`

### 出力関連オプション

#### `-o, --output <フォーマット>`
- **型**: 文字列
- **デフォルト**: `diffx` (人間が読みやすいdiffx形式)
- **指定可能な値**: `diffx`, `json`, `yaml`, `unified`
- **説明**: 差分の出力フォーマットを指定します。

**diffxフォーマット (デフォルト):**
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
# 出力: 従来のdiff形式のフォーマット
```

### フィルタリング関連オプション

#### `--path <パス>`
- **型**: 文字列
- **デフォルト**: なし (構造全体を比較)
- **説明**: データ構造内の特定のパスに差分の比較を限定します。

**パスの構文:**
- オブジェクトキー: `database.host`
- 配列インデックス: `users[0]`
- ネストしたパス: `config.database.connection.host`
- 複雑なパス: `services.web.env[0].name`

**使用例:**
```bash
# データベース設定のみを比較
diffx config.json config.new.json --path "database"

# 特定の配列要素を比較
diffx config.json config.new.json --path "users[0]"

# 深くネストしたパスを比較
diffx config.json config.new.json --path "services.web.environment.variables"
```

#### `--ignore-keys-regex <パターン>`
- **型**: 正規表現文字列
- **デフォルト**: なし
- **説明**: 指定された正規表現に一致するキーを無視します。

**一般的なパターン:**
```bash
# タイムスタンプ関連のフィールドを無視
diffx file1.json file2.json --ignore-keys-regex "^(timestamp|createdAt|updatedAt)$"

# 内部フィールド（アンダースコアで始まる）を無視
diffx file1.json file2.json --ignore-keys-regex "^_.*"

# 複数のパターンを無視
diffx file1.json file2.json --ignore-keys-regex "^(id|timestamp|_.*|temp_.*)$"

# バージョン関連のフィールドを無視
diffx file1.json file2.json --ignore-keys-regex "(version|buildNumber|revision)"
```

**正規表現の例:**
- `^timestamp$` - "timestamp"に完全に一致
- `^_.*` - アンダースコアで始まるフィールド
- `.*_temp$` - "_temp"で終わるフィールド
- `^(id|uid|pk)$` - id, uid, pkのいずれかに一致
- `(?i)password` - "password"に大文字小文字を区別せずに一致

### 比較関連オプション

#### `--epsilon <値>`
- **型**: 浮動小数点数
- **デフォルト**: `0.0` (厳密な比較)
- **説明**: 浮動小数点数を比較する際の許容誤差。

**使用例:**
```bash
# 浮動小数点数の小さな差を許容
diffx metrics.json metrics.new.json --epsilon 0.001

# 科学技術データ用に、より緩い許容誤差を設定
diffx measurements.json measurements.new.json --epsilon 0.01

# 金融データ用に、非常に厳密な比較を実施
diffx financial.json financial.new.json --epsilon 0.000001
```

**主な使用場面:**
- 測定精度が問題となる科学技術データ
- 丸め誤差が生じる金融計算
- 小さな変動があるパフォーマンス指標
- 浮動小数点数に変換されたデータ

#### `--array-id-key <キー>`
- **型**: 文字列
- **デフォルト**: なし (位置に基づいて比較)
- **説明**: 配列の要素を識別し、追跡するために使用するキー。

**使用例:**
```bash
# IDを使ってユーザーを追跡
diffx users.json users.updated.json --array-id-key "id"

# SKUを使って製品を追跡
diffx inventory.json inventory.new.json --array-id-key "sku"

# 主キーを使ってデータベースのレコードを追跡
diffx records.json records.new.json --array-id-key "primary_key"
```

**ID追跡なしの場合:**
```json
// 配列の比較では、位置に基づいた変更が表示される
// 旧: [{"name": "Alice"}, {"name": "Bob"}]
// 新: [{"name": "Bob"}, {"name": "Alice"}]
// 結果: すべての要素が変更されたと見なされる
```

**ID追跡ありの場合:**
```json
// 旧: [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]  
// 新: [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
// 結果: 変更は検出されない（要素は同じで順序が違うだけ）
```

#### `--ignore-whitespace`
- **型**: ブール値フラグ
- **デフォルト**: False
- **説明**: 文字列の値に含まれる空白文字の違いを無視します。

**使用例:**
```bash
# 空白の扱いが異なるファイル
echo '{"text": "Hello  World"}' > file1.json
echo '{"text": "Hello World"}' > file2.json

# 通常の比較では差分が検出される
diffx file1.json file2.json
# 出力: ~ text: "Hello  World" -> "Hello World"

# 空白を無視すると差分は報告されない
diffx file1.json file2.json --ignore-whitespace
# 出力: (差分なし)
```

**主な使用場面:**
- スペースの使い方が一貫しない設定ファイル
- 異なるシステムからエクスポートされたデータ
- 手動編集によって余分なスペースが挿入された場合
- 正規化されたテキストと未加工のテキストの比較

#### `--ignore-case`
- **型**: ブール値フラグ
- **デフォルト**: False
- **説明**: 文字列の値に含まれる大文字と小文字の違いを無視します。

**使用例:**
```bash
# 大文字小文字の使い方が異なるファイル
echo '{"status": "Active"}' > file1.json
echo '{"status": "ACTIVE"}' > file2.json

# 通常の比較では差分が検出される
diffx file1.json file2.json
# 出力: ~ status: "Active" -> "ACTIVE"

# 大文字小文字を無視すると差分は報告されない
diffx file1.json file2.json --ignore-case
# 出力: (差分なし)
```

**主な使用場面:**
- 大文字小文字が混在するユーザー入力データ
- レガシーシステムの移行
- 大文字小文字を区別しない設定値
- データ正規化のタスク

**オプションの組み合わせ:**
```bash
# 空白と大文字小文字の両方の違いを無視
diffx config.json config.new.json --ignore-whitespace --ignore-case

# 複数のオプションを組み合わせた複雑な例
diffx data.yaml data.updated.yaml \
  --ignore-case \
  --ignore-whitespace \
  --epsilon 0.001 \
  --ignore-keys-regex "^(timestamp|version)$"
```

### 出力制御オプション

#### `--context <N>`
- **型**: 整数
- **デフォルト**: なし (すべてのコンテキストを表示)
- **説明**: unified出力形式で、差分の周囲に指定したN行のコンテキストを表示します。

**使用例:**
```bash
# 変更点の周囲に2行のコンテキストを表示
diffx config.json config.new.json --output unified --context 2

# 変更された行のみを表示（コンテキストなし）
diffx config.json config.new.json --output unified --context 0

# デフォルトの動作（すべてのコンテキストを表示）
diffx config.json config.new.json --output unified
```

**コンテキスト付きの出力例:**
```diff
# --context 2 の場合
   "database": {
     "host": "localhost",
-    "port": 5432
+    "port": 5433
   },
   "cache": {

# --context 0 の場合
-    "port": 5432
+    "port": 5433
```

#### `-q, --quiet`
- **型**: ブール値フラグ
- **デフォルト**: False
- **説明**: 通常の出力を抑制し、終了ステータスのみを返します。

**使用例:**
```bash
# ファイルが異なるかどうかをスクリプトで確認
diffx config.json config.new.json --quiet
echo $?  # 0 = 差分なし, 1 = 差分あり, 2 = エラー

# シェルスクリプトでの使用
if diffx config.json backup.json --quiet; then
    echo "ファイルは同一です"
else
    echo "ファイルが異なります"
fi

# 他のオプションとの組み合わせ
diffx large.json large.new.json --quiet --ignore-whitespace
```

**終了コード:**
- `0`: 差分は見つかりませんでした
- `1`: 差分が見つかりました
- `2`: エラーが発生しました（ファイルが無効、フォーマットエラーなど）

#### `--brief`
- **型**: ブール値フラグ
- **デフォルト**: False
- **説明**: 差分の詳細ではなく、ファイル名のみを報告します（`diff --brief`と同様の動作）。

**使用例:**
```bash
# ファイルが異なるかどうかだけを報告
diffx config.json config.new.json --brief
# 出力: Files config.json and config.new.json differ

# ディレクトリ比較での使用
diffx configs/ configs.backup/ --recursive --brief
# 出力: Files configs/app.json and configs.backup/app.json differ

# フィルタリングとの組み合わせ
diffx data.json data.new.json --brief --ignore-keys-regex "^timestamp$"
```

**主な使用場面:**
- バッチ処理スクリプト
- 簡単なファイル比較チェック
- 自動化されたテストパイプライン
- ファイル同期の検証

#### `-v, --verbose`
- **型**: ブール値フラグ
- **デフォルト**: False
- **説明**: パフォーマンス指標、設定詳細、処理統計など、包括的な診断情報を表示します。

**使用例:**
```bash
# 基本的な詳細出力
diffx config.json config.new.json --verbose
# 出力に含まれる情報:
# 入力ファイル情報: 
#   入力1のサイズ: 245バイト
#   入力2のサイズ: 267バイト
# パース時間: 15.2µs
# 差分計算時間: 23.8µs
# 見つかった差分の合計: 3
# パフォーマンス概要:
#   総処理時間: 125.4µs
#   メモリ最適化: 無効

# フィルタリングオプションと組み合わせた詳細出力
diffx data.json data.new.json --verbose --ignore-keys-regex "timestamp" --epsilon 0.1
# 追加の出力:
# キーフィルタリング設定:
#   正規表現パターン: timestamp
# 数値許容誤差設定:
#   イプシロン値: 0.1

# ディレクトリ比較の詳細出力
diffx configs/ configs.backup/ --recursive --verbose
# 追加の出力:
# ディレクトリのスキャン結果:
#   configs/ 内のファイル数: 12
#   configs.backup/ 内のファイル数: 11
#   比較対象の合計ファイル数: 12
# ディレクトリ比較の概要:
#   比較したファイル数: 11
#   片方のディレクトリにのみ存在するファイル数: 1
#   見つかった差分: あり
```

**詳細情報のカテゴリ:**

1.  **パフォーマンス指標**
    -   ファイルサイズとメモリ使用量
    -   パース時間、差分計算時間
    -   総処理時間
    -   メモリ最適化の状態

2.  **設定詳細**
    -   有効なフィルタリングパターン（正規表現、イプシロン、配列IDキー）
    -   パスフィルタリングの設定
    -   コンテキスト表示の設定

3.  **処理統計**
    -   フィルタリング前後の差分総数
    -   ディレクトリのスキャン結果
    -   比較の有効性に関する指標

4.  **診断出力**
    -   最適化の決定に関する情報
    -   処理バッチの情報
    -   エラーコンテキストとトラブルシューティングデータ

**主な使用場面:**
- パフォーマンス分析と最適化
- 時間のかかる比較のトラブルシューティング
- フィルタの効果を理解する
- 設定問題のデバッグ
- CI/CDパイプラインの診断
- サポートとメンテナンス作業

### ディレクトリ関連オプション

#### `-r, --recursive`
- **型**: ブール値フラグ
- **デフォルト**: False
- **説明**: サブディレクトリを含めて再帰的にディレクトリを比較します（Unixの`diff`コマンドと互換性あり）。

**使用例:**
```bash
# --recursiveなしのディレクトリ比較（Unix diff互換）
# ディレクトリ直下のファイルを比較し、共通のサブディレクトリはメッセージで表示
diffx config_dir1/ config_dir2/
# 出力:
# Common subdirectories: config_dir1/subdir and config_dir2/subdir
# --- Comparing config.json ---
# ~ version: "1.0" -> "1.1"

# 再帰比較 - サブディレクトリ内のすべてのファイルを比較
diffx config_dir1/ config_dir2/ --recursive
# 出力:
# --- Comparing config.json ---
# ~ version: "1.0" -> "1.1"
# --- Comparing subdir/nested.json ---
# ~ data: "old" -> "new"

# 出力フォーマットを指定した再帰比較
diffx environments/dev/ environments/prod/ -r --output json

# フィルタリングと組み合わせた再帰比較
diffx configs/ configs.backup/ -r --ignore-keys-regex "^(timestamp|version)$"
```

**Unix diff互換の動作:**

**`--recursive`フラグなし（デフォルト）:**
-   指定されたディレクトリ直下のファイルのみを比較します。
-   両方の場所に存在するサブディレクトリについては、「Common subdirectories」というメッセージを表示します。
-   サブディレクトリ内のファイルは比較しません。
-   標準的なUnixの`diff`コマンドとの互換性を維持します。

**`--recursive`フラグあり:**
-   サブディレクトリを含め、すべてのファイルを再帰的に比較します。
-   出力においてディレクトリ構造を維持します。
-   `diff -r`の動作に相当します。

**共通の動作:**
-   片方のディレクトリにしか存在しないファイルはスキップします。
-   各ファイルのフォーマット自動検出を尊重します。
-   片方のディレクトリにのみ存在するファイルを報告します。

### パフォーマンス関連オプション

#### 自動最適化
- **型**: 自動機能
- **デフォルト**: 1MBを超えるファイルで有効
- **説明**: 大きなファイルやデータ構造に対して、メモリ効率の良い処理が自動的に有効になります。

**自動検出の動作:**
-   1MB以下のファイル: 標準モード（高速、メモリ使用量に制限なし）
-   1MBを超えるファイル: 最適化モード（メモリ効率が良く、バッチ処理）
-   手動での設定は不要 - 最適化は完全に透過的に行われます。

**最適化の機能:**
-   ファイルサイズに基づく自動検出
-   大規模データセットに対するメモリ効率の良い処理
-   深いネスト構造に対するバッチ処理
-   どのモードでも同一の出力を維持

**使用例:**
```bash
# 自動検出（常に有効）
diffx config.json config.new.json
# 小さなファイルには標準モード、大きなファイルには最適化モードが自動的に使われる

# 大きなファイルでは自動的に最適化が使用される
diffx massive_db.json massive_db.new.json --array-id-key "id" --path "users"
# 大きなファイルのため、自動的に最適化モードが使用される

# 他のすべてのオプションは最適化と透過的に連携
diffx complex_data.json complex_data.v2.json --ignore-keys-regex "^timestamp$"
# 必要に応じて最適化が自動的に適用される
```

**パフォーマンスの動作:**
```bash
# 小さなファイル（<1MB） - 自動で標準モード
diffx config.json config.new.json
# 高速処理、メモリ使用量に制限なし

# 大きなファイル（>1MB） - 自動で最適化モード
diffx large_dataset.json large_dataset.v2.json
# メモリ効率の良いバッチ処理

# 複雑なネスト構造 - 自動で最適化
diffx deep_nested.json deep_nested.v2.json
# データ特性に基づいた透過的な最適化
```

### 情報オプション

#### `-h, --help`
- **型**: ブール値フラグ
- **説明**: ヘルプ情報を表示して終了します。

#### `-V, --version`
- **型**: ブール値フラグ
- **説明**: バージョン情報を表示して終了します。

**使用例:**
```bash
# ヘルプの表示
diffx --help
diffx -h

# バージョンの表示
diffx --version
diffx -V
```

## 終了コード

`diffx` は以下の終了コードを使用します:

-   **0**: 成功、差分なし
-   **1**: 成功、差分あり
-   **2**: コマンドライン引数のエラー
-   **3**: ファイルI/Oエラー
-   **4**: パースエラー（無効なフォーマット）
-   **5**: 内部エラー

**使用例:**
```bash
# ファイルが同一かどうかを確認
if diffx file1.json file2.json >/dev/null 2>&1; then
    echo "ファイルは同一です"
else
    echo "ファイルが異なります"
fi

# 終了コードを取得
diffx config.json config.new.json
EXIT_CODE=$?
case $EXIT_CODE in
    0) echo "差分なし" ;;
    1) echo "差分が見つかりました" ;;
    *) echo "エラーが発生しました (コード: $EXIT_CODE)" ;;
esac
```

## 使用パターン

### 基本的な比較

```bash
# 簡単なファイル比較
diffx file1.json file2.json

# 異なるフォーマットのファイルを比較
diffx config.yaml config.toml

# 標準入力とファイルを比較
curl -s https://api.example.com/config | diffx - local_config.json
```

### 高度なフィルタリング

```bash
# 複雑な無視パターン
diffx app.json app.new.json \
  --ignore-keys-regex "^(timestamp|_.*|createdAt|updatedAt|version)$"

# 特定のパスに限定した比較
diffx large_config.json large_config.new.json \
  --path "database.connections"

# 複数のオプションを組み合わせ
diffx users.json users.new.json \
  --array-id-key "user_id" \
  --ignore-keys-regex "^(last_login|session_.*)" \
  --output json
```

### ディレクトリ操作

```bash
# Unix diff互換のディレクトリ比較（非再帰的）
diffx configs/ configs.backup/
# ディレクトリ内のファイルと「Common subdirectories」メッセージを表示

# 再帰的なディレクトリ比較
diffx configs/ configs.backup/ --recursive

# フィルタリング付きのディレクトリ比較
diffx env/dev/ env/prod/ \
  --recursive \
  --ignore-keys-regex "^(host|port|password)" \
  --output json > env_diff.json
```

### 統合例

```bash
# Gitとの統合
git show HEAD~1:config.json > old_config.json
diffx old_config.json config.json --output unified

# CI/CDパイプライン
diffx expected_config.json actual_config.json \
  --ignore-keys-regex "^(deployment_time|build_id)" \
  --output json > config_validation.json

# 監視スクリプト
#!/bin/bash
if ! diffx baseline_config.json current_config.json \
     --ignore-keys-regex "^(timestamp|uptime)$" >/dev/null; then
  echo "設定のドリフトが検出されました！"
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

**無効なフォーマット:**
```bash
$ diffx invalid.json valid.json
Error: Failed to parse JSON: expected `,` or `}` at line 1 column 15
```

**権限が拒否されました:**
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
# フォーマット検出の検証
diffx --format json file1.txt file2.txt
```

## パフォーマンスに関する考慮事項

### 大きなファイル

```bash
# 大きなファイルにはパスフィルタリングを使用
diffx huge1.json huge2.json --path "critical_section"

# 不要なデータを無視
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

非常に大きなファイルの場合は、以下を検討してください:
- `--path` を使用して特定のセクションに焦点を当てる
- `--ignore-keys-regex` を使用して、大きく無関係なセクションを除外する
- 可能であれば、ファイルをより小さなチャンクで処理する

## ユースケース別使用例

### 設定管理
```bash
# 環境間の比較
diffx prod.json staging.json --ignore-keys-regex "^(host|port|secret_.*)$"

# Kubernetesマニフェスト
diffx deployment.yaml deployment.new.yaml --ignore-keys-regex "^metadata\\.(creation.*|resource.*)$"
```

### APIテスト
```bash
# レスポンスの検証
diffx expected_response.json actual_response.json --ignore-keys-regex "^(timestamp|request_id)$"

# スキーマの比較
diffx api_v1_schema.json api_v2_schema.json --path "definitions"
```

### データ処理
```bash
# ETLの検証
diffx input_data.json output_data.json --array-id-key "record_id" --epsilon 0.001

# データベースエクスポートの比較
diffx export1.json export2.json --array-id-key "id" --ignore-keys-regex "^(updated_at|sync_time)$"
```

### セキュリティ監査
```bash
# ポリシーの比較
diffx security_policy.json security_policy.new.json --path "permissions"

# アクセス制御の検証
diffx rbac.yaml rbac.new.yaml --array-id-key "name"
```

この包括的なCLIリファレンスは、`diffx`で利用可能なすべてのオプションを網羅し、効果的な使用法のための実用的な例を提供します。

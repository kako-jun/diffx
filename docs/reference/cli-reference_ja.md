# CLIリファレンス

`diffx` コマンドライン・インターフェースの完全なリファレンスドキュメントです。

## 概要

```
diffx [オプション] <入力1> <入力2>
```

## 説明

`diffx` は構造化データファイルのセマンティック比較を行うコマンドラインツールです。従来のテキストベース差分ツールとは異なり、`diffx` はデータの構造と意味を理解し、フォーマットの違いではなく実際の変更に焦点を当てます。

## 引数

### `<入力1>`
- **型**: ファイルパス、ディレクトリパス、または標準入力の `-`
- **必須**: はい
- **説明**: 比較する最初の入力

### `<入力2>`
- **型**: ファイルパス、ディレクトリパス、または標準入力の `-`
- **必須**: はい
- **説明**: 比較する2番目の入力

**例:**
```bash
# 2つのファイルを比較
diffx config.json config.new.json

# 標準入力と比較
cat config.json | diffx - config.new.json

# ディレクトリを比較
diffx config_dir1/ config_dir2/
```

## オプション

### フォーマット・オプション

#### `-f, --format <フォーマット>`
- **型**: 文字列
- **デフォルト**: ファイル拡張子から自動検出
- **値**: `json`, `yaml`, `toml`, `xml`, `ini`, `csv`
- **説明**: 入力ファイルの形式を強制指定

**例:**
```bash
# JSON形式として解釈を強制
diffx --format json file1.txt file2.txt

# YAML形式として解釈を強制
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

#### `-o, --output <フォーマット>`
- **型**: 文字列
- **デフォルト**: `cli`
- **値**: `cli`, `json`, `yaml`, `unified`
- **説明**: 差分の出力形式

**CLI出力（デフォルト）:**
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
# 出力: 従来のdiff形式
```

### フィルタリング・オプション

#### `--path <パス>`
- **型**: 文字列
- **デフォルト**: なし（全体を比較）
- **説明**: データ構造の特定パスの差分のみにフィルタリング

**パス構文:**
- オブジェクトキー: `database.host`
- 配列インデックス: `users[0]`
- ネストしたパス: `config.database.connection.host`
- 複雑なパス: `services.web.env[0].name`

**例:**
```bash
# データベース設定のみ比較
diffx config.json config.new.json --path "database"

# 特定の配列要素を比較
diffx config.json config.new.json --path "users[0]"

# 深くネストしたパス
diffx config.json config.new.json --path "services.web.environment.variables"
```

#### `--ignore-keys-regex <パターン>`
- **型**: 正規表現文字列
- **デフォルト**: なし
- **説明**: 指定した正規表現にマッチするキーを無視

**よくあるパターン:**
```bash
# タイムスタンプフィールドを無視
diffx file1.json file2.json --ignore-keys-regex "^(timestamp|createdAt|updatedAt)$"

# 内部フィールド（アンダースコア開始）を無視
diffx file1.json file2.json --ignore-keys-regex "^_.*"

# 複数パターンを無視
diffx file1.json file2.json --ignore-keys-regex "^(id|timestamp|_.*|temp_.*)$"

# バージョン関連フィールドを無視
diffx file1.json file2.json --ignore-keys-regex "(version|buildNumber|revision)"
```

**正規表現例:**
- `^timestamp$` - "timestamp"の完全一致
- `^_.*` - アンダースコアで始まるフィールド
- `.*_temp$` - "_temp"で終わるフィールド
- `^(id|uid|pk)$` - id、uid、pkのいずれか
- `(?i)password` - "password"の大文字小文字無視マッチ

### 比較オプション

#### `--epsilon <値>`
- **型**: 浮動小数点数
- **デフォルト**: `0.0`（厳密比較）
- **説明**: 浮動小数点数比較の許容誤差

**例:**
```bash
# 小さな差を許容
diffx metrics.json metrics.new.json --epsilon 0.001

# より寛容な許容誤差（科学データ）
diffx measurements.json measurements.new.json --epsilon 0.01

# 非常に厳密な比較（金融データ）
diffx financial.json financial.new.json --epsilon 0.000001
```

**用途:**
- 科学データの測定精度
- 金融計算の丸め誤差
- パフォーマンス指標の小さな変動
- 変換データの浮動小数点アーティファクト

#### `--array-id-key <キー>`
- **型**: 文字列
- **デフォルト**: なし（位置ベース比較）
- **説明**: 配列要素の識別と追跡に使用するキー

**例:**
```bash
# ユーザーをIDで追跡
diffx users.json users.updated.json --array-id-key "id"

# 商品をSKUで追跡
diffx inventory.json inventory.new.json --array-id-key "sku"

# データベースレコードを主キーで追跡
diffx records.json records.new.json --array-id-key "primary_key"
```

**IDキーなしの場合:**
```json
// 配列比較は位置ベースの変更を表示
// 旧: [{"name": "Alice"}, {"name": "Bob"}]
// 新: [{"name": "Bob"}, {"name": "Alice"}]
// 結果: すべての要素が変更されたと表示
```

**IDキー使用の場合:**
```json
// 旧: [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]  
// 新: [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
// 結果: 変更なし（同じ要素、異なる順序）
```

#### `--ignore-whitespace`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: 文字列値の空白差異を無視

**例:**
```bash
# 異なる空白を持つファイル
echo '{"text": "Hello  World"}' > file1.json
echo '{"text": "Hello World"}' > file2.json

# 通常の比較では差分を表示
diffx file1.json file2.json
# 出力: ~ text: "Hello  World" -> "Hello World"

# 空白無視で比較 - 差分なしと報告
diffx file1.json file2.json --ignore-whitespace
# 出力: (差分なし)
```

**使用ケース:**
- 一貫性のない間隔を持つ設定ファイル
- 異なるシステムからエクスポートされたデータ
- 余分なスペースを導入する手動編集
- 正規化済み vs 生のテキストデータ

#### `--ignore-case`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: 文字列値の大文字小文字差異を無視

**例:**
```bash
# 異なる大文字小文字を持つファイル
echo '{"status": "Active"}' > file1.json
echo '{"status": "ACTIVE"}' > file2.json

# 通常の比較では差分を表示
diffx file1.json file2.json
# 出力: ~ status: "Active" -> "ACTIVE"

# 大文字小文字無視で比較 - 差分なしと報告
diffx file1.json file2.json --ignore-case
# 出力: (差分なし)
```

**使用ケース:**
- 様々な大文字小文字を持つユーザー入力データ
- レガシーシステムの移行
- 大文字小文字を区別しない設定値
- データ正規化タスク

**オプションの組み合わせ:**
```bash
# 空白と大文字小文字の両方の差異を処理
diffx config.json config.new.json --ignore-whitespace --ignore-case

# 複数オプションを使用した複雑な例
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
- **説明**: unified出力形式で差分周辺のN行のコンテキストを表示

**例:**
```bash
# 変更箇所周辺の2行のコンテキストを表示
diffx config.json config.new.json --output unified --context 2

# 変更行のみ表示（コンテキストなし）
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
- **説明**: 通常の出力を抑制し、終了ステータスのみを返す

**例:**
```bash
# ファイルが異なるかをチェック（スクリプト用）
diffx config.json config.new.json --quiet
echo $?  # 0 = 差分なし, 1 = 差分あり, 2 = エラー

# シェルスクリプトで使用
if diffx config.json backup.json --quiet; then
    echo "ファイルは同一"
else
    echo "ファイルが異なる"
fi

# 他のオプションと組み合わせ
diffx large.json large.new.json --quiet --ignore-whitespace
```

**終了コード:**
- `0`: 差分なし
- `1`: 差分あり
- `2`: エラー発生（無効なファイル、フォーマットエラーなど）

#### `--brief`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: 差分内容ではなく、ファイル名のみを報告（`diff --brief`に類似）

**例:**
```bash
# ファイルが異なるかのみを報告
diffx config.json config.new.json --brief
# 出力: Files config.json and config.new.json differ

# ディレクトリ比較で使用
diffx configs/ configs.backup/ --recursive --brief
# 出力: Files configs/app.json and configs.backup/app.json differ

# フィルタリングと組み合わせ
diffx data.json data.new.json --brief --ignore-keys-regex "^timestamp$"
```

**使用ケース:**
- バッチ処理スクリプト
- 迅速なファイル比較チェック
- 自動テストパイプライン
- ファイル同期検証

### ディレクトリオプション

#### `-r, --recursive`
- **型**: ブールフラグ
- **デフォルト**: False
- **説明**: 再帰的ディレクトリ比較を有効化

**例:**
```bash
# ディレクトリ内のすべてのファイルを比較
diffx config_dir1/ config_dir2/ --recursive

# 出力形式付きの再帰比較
diffx environments/dev/ environments/prod/ -r --output json

# フィルタリング付きの再帰比較
diffx configs/ configs.backup/ -r --ignore-keys-regex "^(timestamp|version)$"
```

**動作:**
- ディレクトリ間で対応するファイルを比較
- 両方のディレクトリに存在しないファイルをスキップ
- 出力でディレクトリ構造を維持
- 各ファイルのフォーマット自動検出を尊重

### パフォーマンスオプション

#### 自動最適化
- **型**: 自動機能
- **デフォルト**: >1MBファイルで有効
- **説明**: 大きなファイルやデータ構造に対して自動的にメモリ効率的な処理を有効化

**自動検出の動作:**
- ファイル ≤1MB: 標準モード（高速、無制限メモリ）
- ファイル >1MB: 最適化モード（メモリ効率、バッチ処理）
- 手動設定不要 - 最適化は完全に透明

**最適化機能:**
- ファイルサイズに基づく自動検出
- 大きなデータセットでのメモリ効率的処理
- 深いネスト構造でのバッチ処理
- モードに関係なく同一出力を維持

**例:**
```bash
# 自動検出（常に有効）
diffx config.json config.new.json
# 小ファイルは標準モード、大ファイルは最適化モード

# 大きなファイルは自動的に最適化を使用
diffx massive_db.json massive_db.new.json --array-id-key "id" --path "users"
# 大きなファイルは自動的に最適化モードを使用

# 他の全オプションは最適化と透明に動作
diffx complex_data.json complex_data.v2.json --ignore-keys-regex "^timestamp$"
# 必要に応じて自動的に最適化を適用
```

**パフォーマンス動作:**
```bash
# 小ファイル (<1MB) - 自動標準モード
diffx config.json config.new.json
# 高速処理、無制限メモリ使用

# 大ファイル (>1MB) - 自動最適化モード
diffx large_dataset.json large_dataset.v2.json
# メモリ効率、バッチ処理

# 複雑なネスト構造 - 自動最適化
diffx deep_nested.json deep_nested.v2.json
# データ特性に基づく透明な最適化
```

### 情報オプション

#### `-h, --help`
- **型**: ブールフラグ
- **説明**: ヘルプ情報を表示して終了

#### `-V, --version`
- **型**: ブールフラグ  
- **説明**: バージョン情報を表示して終了

**例:**
```bash
# ヘルプ表示
diffx --help
diffx -h

# バージョン表示
diffx --version
diffx -V
```

## 終了コード

`diffx` は以下の終了コードを使用します：

- **0**: 成功、差分なし
- **1**: 成功、差分あり
- **2**: コマンドライン引数エラー
- **3**: ファイルI/Oエラー
- **4**: パースエラー（無効なフォーマット）
- **5**: 内部エラー

**例:**
```bash
# ファイルが同一かチェック
if diffx file1.json file2.json >/dev/null 2>&1; then
    echo "ファイルは同一"
else
    echo "ファイルが異なる"
fi

# 終了コードをキャプチャ
diffx config.json config.new.json
EXIT_CODE=$?
case $EXIT_CODE in
    0) echo "差分なし" ;;
    1) echo "差分あり" ;;
    *) echo "エラーが発生 (code: $EXIT_CODE)" ;;
esac
```


## 使用パターン

### 基本的な比較

```bash
# シンプルなファイル比較
diffx file1.json file2.json

# 異なるフォーマットとの比較
diffx config.yaml config.toml --format yaml --format toml

# 標準入力との比較
curl -s https://api.example.com/config | diffx - local_config.json
```

### 高度なフィルタリング

```bash
# 複雑な無視パターン
diffx app.json app.new.json \
  --ignore-keys-regex "^(timestamp|_.*|createdAt|updatedAt|version)$"

# パス特化比較
diffx large_config.json large_config.new.json \
  --path "database.connections"

# 複数オプションの組み合わせ
diffx users.json users.new.json \
  --array-id-key "user_id" \
  --ignore-keys-regex "^(last_login|session_.*)$" \
  --output json
```

### ディレクトリ操作

```bash
# 再帰ディレクトリ比較
diffx configs/ configs.backup/ --recursive

# フィルタリング付きディレクトリ比較
diffx env/dev/ env/prod/ \
  --recursive \
  --ignore-keys-regex "^(host|port|password)$" \
  --output json > env_diff.json
```

### 統合例

```bash
# Git統合
git show HEAD~1:config.json > old_config.json
diffx old_config.json config.json --output unified

# CI/CDパイプライン
diffx expected_config.json actual_config.json \
  --ignore-keys-regex "^(deployment_time|build_id)$" \
  --output json > config_validation.json

# 監視スクリプト
#!/bin/bash
if ! diffx baseline_config.json current_config.json \
     --ignore-keys-regex "^(timestamp|uptime)$" >/dev/null; then
  echo "設定ドリフトが検出されました！"
  diffx baseline_config.json current_config.json --output json | \
    notify_alert_system.py
fi
```

## エラー処理

### よくあるエラー

**ファイルが見つからない:**
```bash
$ diffx nonexistent.json config.json
Error: そのようなファイルまたはディレクトリはありません (os error 2)
```

**無効なフォーマット:**
```bash
$ diffx invalid.json valid.json
Error: JSONの解析に失敗: 1行15列目で `,` または `}` が期待されます
```

**アクセス権限エラー:**
```bash
$ diffx protected.json config.json
Error: アクセスが拒否されました (os error 13)
```

**無効な正規表現:**
```bash
$ diffx file1.json file2.json --ignore-keys-regex "[invalid"
Error: 無効な正規表現: 文字クラスが閉じられていません
```

### デバッグ

```bash
# フォーマット検出の検証
diffx --format json file1.txt file2.txt
```

## パフォーマンス考慮事項

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

大きなファイルには以下を検討：
- `--path` を使用して特定セクションに焦点
- `--ignore-keys-regex` で大きな無関係セクションをフィルタリング
- 可能であればファイルを小さなチャンクに分割処理

## 用途別例

### 設定管理
```bash
# 環境比較
diffx prod.json staging.json --ignore-keys-regex "^(host|port|secret_.*)$"

# Kubernetesマニフェスト  
diffx deployment.yaml deployment.new.yaml --ignore-keys-regex "^metadata\\.(creation.*|resource.*)$"
```

### APIテスト
```bash
# レスポンス検証
diffx expected_response.json actual_response.json --ignore-keys-regex "^(timestamp|request_id)$"

# スキーマ比較
diffx api_v1_schema.json api_v2_schema.json --path "definitions"
```

### データ処理
```bash
# ETL検証
diffx input_data.json output_data.json --array-id-key "record_id" --epsilon 0.001

# データベースエクスポート比較
diffx export1.json export2.json --array-id-key "id" --ignore-keys-regex "^(updated_at|sync_time)$"
```

### セキュリティ監査
```bash
# ポリシー比較
diffx security_policy.json security_policy.new.json --path "permissions"

# アクセス制御検証
diffx rbac.yaml rbac.new.yaml --array-id-key "name"
```

この包括的なCLIリファレンスは、`diffx` の利用可能なすべてのオプションと効果的な使用のための実用例を提供します。
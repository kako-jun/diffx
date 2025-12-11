# diffx CLI 仕様書

バージョン: 0.6.0
最終更新: 2025-12-11
ステータス: **確定**

## 概要

diffx は構造化データ（JSON, YAML, TOML, XML, INI, CSV）の差分を検出するCLIツール。
テキストではなく**意味的な差分**を抽出する。キー順序、フォーマット、空白は無視。

## 基本構文

```
diffx [OPTIONS] <FILE1> <FILE2>
diffx [OPTIONS] -r <DIR1> <DIR2>
diffx - - < input                    # stdin から2つのデータを読む
diffx - file.json < input            # stdin と file を比較
```

## 終了コード

| コード | 意味 |
|--------|------|
| 0 | 差分なし |
| 1 | 差分あり |
| 2 | 引数エラー、パースエラー、ディレクトリ比較エラー等 |
| 3 | ファイルI/Oエラー（ファイルが存在しない、読み取り失敗等） |

## 引数

### FILE1, FILE2

- 両方必須
- ファイルパス、ディレクトリパス、または `-`（stdin）
- ディレクトリの場合は `-r` が必須

---

## オプション

### 入力制御

#### `-f, --format <FORMAT>`

入力ファイルのフォーマットを明示的に指定する。

| 値 | 対応拡張子 |
|----|-----------|
| `json` | .json |
| `yaml` | .yaml, .yml |
| `toml` | .toml |
| `xml` | .xml |
| `ini` | .ini |
| `csv` | .csv |

**デフォルト**: 拡張子から自動検出

**動作**:
- 指定時: 両ファイルを指定フォーマットとしてパース
- 未指定時: 各ファイルの拡張子から判定
- stdin使用時で拡張子がない場合: `--format` 必須（またはJSON/YAMLを自動判定）

---

### 出力制御

#### `-o, --output <OUTPUT>`

出力フォーマットを指定する。

| 値 | 説明 |
|----|------|
| `diffx` | 人間可読な差分表示（デフォルト） |
| `json` | JSON配列形式 |
| `yaml` | YAML配列形式 |

**JSON出力例**:
```json
[
  {"Modified": ["age", 30, 31]},
  {"Added": ["items[2]", "orange"]}
]
```

**YAML出力例**:
```yaml
- Modified:
  - age
  - 30
  - 31
- Added:
  - items[2]
  - orange
```

**diffx形式の記号**:
- `+` 追加（青）: `+ key: value`
- `-` 削除（黄）: `- key: value`
- `~` 変更（シアン）: `~ key: old -> new`
- `!` 型変更（マゼンタ）: `! key: old (OldType) -> new (NewType)`

**インデント**:
- ネストレベルに応じて2スペースずつインデント
- 例: `root.nested.key` の変更は `    ~ key: ...` と表示（4スペース）

**色付けについて**:
- TTYに出力時のみ色付け
- パイプやファイルリダイレクト時は色なし
- `--no-color` で強制的に無効化

---

#### `--no-color`

出力の色付けを無効にする。

**動作**: ANSIエスケープシーケンスを出力しない

---

#### `-q, --quiet`

出力を抑制し、終了コードのみを返す。

**動作**:
- stdout に何も出力しない
- 終了コードで結果を判定

**例**:
```bash
diffx -q file1.json file2.json && echo "同じ" || echo "異なる"
```

---

#### `--brief`

差分の有無のみを報告する。

**動作**:
- 差分あり: `Files FILE1 and FILE2 differ` を出力
- 差分なし: 何も出力しない

---

#### `-v, --verbose`

処理の詳細情報を stderr に表示する。

**出力内容**:
```
Input file information:
  Input 1 size: 87 bytes
  Input 2 size: 95 bytes
[通常の差分出力]
Parse time: 103.916µs
Diff computation time: 103.916µs
Total differences found: 3
Performance summary:
  Total processing time: 194.289µs
```

**注意**: 差分出力自体は stdout、詳細情報は stderr

---

### 比較オプション

#### `--epsilon <EPSILON>`

数値比較時の許容誤差。

**動作**: `|a - b| <= epsilon` なら同値とみなす

**適用対象**: 全ての数値（**整数も f64 に変換して比較**）

**例**:
```bash
diffx --epsilon 0.001 file1.json file2.json
# 1.0 と 1.0005 は同値
# 100 と 100.0005 も同値
```

---

#### `--ignore-keys-regex <PATTERN>`

指定した正規表現にマッチするキーを無視する。

**動作**:
- マッチしたキーとその値を比較対象から除外
- **ネストしたオブジェクトにも再帰的に適用**
- パスではなくキー名のみにマッチ

**例**:
```bash
diffx --ignore-keys-regex "^(timestamp|updated_at)$" file1.json file2.json
# root.timestamp, root.data.timestamp, root.nested.deep.timestamp 全て無視
```

---

#### `--array-id-key <KEY>`

配列要素をインデックスではなく指定キーで対応付ける。

**動作**:
- 配列内のオブジェクトを指定キーの値でマッチング
- 順序の変更は差分として検出しない
- **IDキーを持たない要素はインデックスで比較**（フォールバック）

**例**:
```json
// file1.json: [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
// file2.json: [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Alice"}]
```
```bash
diffx --array-id-key id file1.json file2.json
# 差分なし（順序は無視される）
```

**IDがない要素の例**:
```json
// file1.json: [{"id": 1, "name": "Alice"}, {"name": "NoID"}]
// file2.json: [{"id": 1, "name": "Alice"}, {"name": "Changed"}]
```
```bash
diffx --array-id-key id file1.json file2.json
# {"name": "NoID"} と {"name": "Changed"} はインデックスで比較され差分検出
```

---

#### `--ignore-whitespace`

文字列比較時に空白を無視する。

**動作**: 文字列から**全ての空白文字を削除**して比較

**実装詳細**:
```rust
old_str.chars().filter(|c| !c.is_whitespace()).collect()
```

**例**:
- `"hello world"` と `"helloworld"` → 同値
- `"  abc  "` と `"abc"` → 同値

---

#### `--ignore-case`

文字列比較時に大文字・小文字を無視する。

**動作**: 文字列値を**小文字に変換**して比較

**適用対象**: 文字列値のみ（キー名には適用しない）

**例**:
- `"Hello"` と `"hello"` → 同値
- キー `"Name"` と `"name"` → 別のキーとして扱う

---

### フィルタリング

#### `--path <PATH>`

指定文字列を含むパスの差分のみを表示する。

**動作**:
- 全ての差分を計算した後にフィルタリング
- パスに指定文字列が**含まれる**（部分一致）場合のみ出力

**例**:
```bash
diffx --path "database" config1.json config2.json
# "database" を含むパスの差分のみ表示
# 例: root.database.host, root.database.port など
```

---

### ディレクトリ比較

#### `-r, --recursive`

ディレクトリを再帰的に比較する。

**必須条件**: 両引数がディレクトリの場合

**動作**:
1. 両ディレクトリのファイルを再帰的に収集
2. 相対パスでマッチング
3. 片方にのみ存在 → Added/Removed
4. 両方に存在 → ファイル内容を比較

**マッチングルール**:
- 相対パスが完全一致するファイル同士を比較
- 拡張子からフォーマットを自動検出

**エラー**:
- ディレクトリを指定して `-r` がない場合: 終了コード 2、メッセージ「Use --recursive (-r) to compare directories」

---

### stdin 入力

`-` を引数に指定すると stdin から読み込む。

**パターン**:

1. **片方がstdin**: `diffx - file.json` または `diffx file.json -`
   - stdin からデータを読み込み、ファイルと比較

2. **両方がstdin**: `diffx - -`
   - JSON: 2つのJSONオブジェクト/配列を順番に読む
   - YAML: `---` で区切られた2つのドキュメント

**フォーマット指定**:
- stdin のみの場合、`--format` で明示するか、JSON/YAMLを自動判定

---

### ユーティリティ

#### `--completions <SHELL>`

シェル補完スクリプトを生成する。

| 値 | シェル |
|----|--------|
| `bash` | Bash |
| `zsh` | Zsh |
| `fish` | Fish |
| `elvish` | Elvish |
| `powershell` | PowerShell |

**例**:
```bash
diffx --completions bash > ~/.local/share/bash-completion/completions/diffx
```

---

#### `-h, --help`

ヘルプを表示。

#### `-V, --version`

バージョンを表示。形式: `diffx X.Y.Z`

---

## 対応フォーマット

| フォーマット | 拡張子 | パーサー |
|--------------|--------|----------|
| JSON | .json | serde_json |
| YAML | .yaml, .yml | serde_yaml |
| TOML | .toml | toml |
| XML | .xml | quick-xml |
| INI | .ini | rust-ini |
| CSV | .csv | csv |

---

## 差分の種類

| 種類 | 記号 | 説明 |
|------|------|------|
| Added | `+` | キー/要素が追加された |
| Removed | `-` | キー/要素が削除された |
| Modified | `~` | 値が変更された |
| TypeChanged | `!` | 値の型が変更された（例: 数値→文字列） |

---

## メタチェイン

差分レポート同士を比較して、変更の進化を追跡できる。

```bash
diffx config_v1.json config_v2.json --output json > diff1.json
diffx config_v2.json config_v3.json --output json > diff2.json
diffx diff1.json diff2.json  # 差分の差分
```

---

## 変更履歴

- 2025-12-11: 初版確定
  - コード調査に基づき `--ignore-whitespace`, `--ignore-case`, `--path` の正確な動作を記載
  - stdin サポートを追加
  - `-r` 必須を明記（READMEの「自動再帰」は廃止された動作）

# diffx-core API 仕様書

バージョン: 0.6.0
最終更新: 2025-12-11
ステータス: **確定**

## 概要

`diffx-core` は構造化データの意味的差分を検出するRustライブラリ。
CLIツール (`diffx`) および言語バインディング (JS, Python) の基盤。

## クレート構造

```
diffx-core/
├── src/
│   ├── lib.rs          # 公開API再エクスポート
│   ├── types.rs        # 型定義
│   ├── parser/         # パーサーモジュール
│   │   ├── mod.rs
│   │   ├── format.rs   # フォーマット検出
│   │   ├── json.rs
│   │   ├── yaml.rs
│   │   ├── toml.rs
│   │   ├── xml.rs
│   │   ├── ini.rs
│   │   └── csv.rs
│   ├── diff/           # 差分検出モジュール
│   │   ├── mod.rs
│   │   ├── core.rs     # diff, diff_paths
│   │   ├── recursive.rs
│   │   ├── objects.rs
│   │   └── arrays.rs
│   ├── io/             # ファイル操作
│   │   └── mod.rs
│   └── utils.rs        # ユーティリティ
```

---

## 公開API

### 主要関数

#### `diff_paths`

ファイルパスまたはディレクトリパスを指定して差分を検出する。

```rust
pub fn diff_paths(
    old_path: &str,
    new_path: &str,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>>
```

**引数**:
- `old_path`: 比較元のパス（ファイルまたはディレクトリ）
- `new_path`: 比較先のパス（ファイルまたはディレクトリ）
- `options`: 比較オプション（省略可）

**戻り値**: `Result<Vec<DiffResult>>`

**動作**:
- ファイル vs ファイル: ファイル内容を比較
- ディレクトリ vs ディレクトリ: `options.recursive = Some(true)` が必要
- ファイル vs ディレクトリ: エラー

**例**:
```rust
use diffx_core::{diff_paths, DiffOptions};

let results = diff_paths("old.json", "new.json", None)?;
for result in &results {
    println!("{}", result);
}
```

---

#### `diff`

パース済みの `serde_json::Value` を直接比較する。

```rust
pub fn diff(
    old: &Value,
    new: &Value,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>>
```

**引数**:
- `old`: 比較元の値
- `new`: 比較先の値
- `options`: 比較オプション（省略可）

**戻り値**: `Result<Vec<DiffResult>>`

**例**:
```rust
use diffx_core::diff;
use serde_json::json;

let old = json!({"name": "Alice", "age": 30});
let new = json!({"name": "Alice", "age": 31});

let results = diff(&old, &new, None)?;
// results: [Modified("age", 30, 31)]
```

---

### パーサー関数

#### `parse_json`, `parse_yaml`, `parse_toml`, `parse_xml`, `parse_ini`, `parse_csv`

各フォーマットの文字列を `serde_json::Value` にパースする。

```rust
pub fn parse_json(content: &str) -> Result<Value>
pub fn parse_yaml(content: &str) -> Result<Value>
pub fn parse_toml(content: &str) -> Result<Value>
pub fn parse_xml(content: &str) -> Result<Value>
pub fn parse_ini(content: &str) -> Result<Value>
pub fn parse_csv(content: &str) -> Result<Value>
```

**戻り値**: `Result<Value>` - 全てのフォーマットは `serde_json::Value` に統一される

---

#### `detect_format_from_path`

ファイルパスの拡張子からフォーマットを検出する。

```rust
pub fn detect_format_from_path(path: &Path) -> FileFormat
```

**マッピング**:
| 拡張子 | FileFormat |
|--------|------------|
| `.json` | `Json` |
| `.yaml`, `.yml` | `Yaml` |
| `.toml` | `Toml` |
| `.xml` | `Xml` |
| `.ini`, `.cfg` | `Ini` |
| `.csv` | `Csv` |
| その他 | `Json`（デフォルト） |

---

#### `parse_content_by_format`

指定フォーマットでコンテンツをパースする。

```rust
pub fn parse_content_by_format(content: &str, format: FileFormat) -> Result<Value>
```

---

## 型定義

### `DiffResult`

差分の結果を表すenum。

```rust
#[derive(Debug, PartialEq, Serialize, Clone)]
pub enum DiffResult {
    Added(String, Value),           // パス, 追加された値
    Removed(String, Value),         // パス, 削除された値
    Modified(String, Value, Value), // パス, 旧値, 新値
    TypeChanged(String, Value, Value), // パス, 旧値, 新値
}
```

**Display実装**（ライブラリのデフォルト）:
```
  + path: value           # Added
  - path: value           # Removed
  ~ path: old -> new      # Modified
  # path: old -> new (type changed)  # TypeChanged
```

**注意**: CLIでは異なる形式を使用（`!` 記号と型名表示）。詳細は `docs/specs/cli.md` 参照。

---

### `DiffOptions`

差分検出のオプション。

```rust
#[derive(Debug, Clone, Default)]
pub struct DiffOptions {
    // 比較オプション
    pub epsilon: Option<f64>,              // 浮動小数点許容誤差
    pub array_id_key: Option<String>,      // 配列要素のIDキー
    pub ignore_keys_regex: Option<Regex>,  // 無視するキーの正規表現
    pub path_filter: Option<String>,       // パスフィルタ（部分一致）

    // ディレクトリ比較
    pub recursive: Option<bool>,           // 再帰比較フラグ

    // 出力制御
    pub output_format: Option<OutputFormat>,

    // diffx固有オプション
    pub diffx_options: Option<DiffxSpecificOptions>,
}
```

---

### `DiffxSpecificOptions`

diffx CLI固有のオプション。

```rust
#[derive(Debug, Clone, Default)]
pub struct DiffxSpecificOptions {
    pub ignore_whitespace: Option<bool>,  // 空白を無視
    pub ignore_case: Option<bool>,        // 大文字小文字を無視
    pub brief_mode: Option<bool>,         // 簡易モード
    pub quiet_mode: Option<bool>,         // 静粛モード
}
```

---

### `OutputFormat`

出力フォーマット。

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Diffx,  // 人間可読形式
    Json,   // JSON配列
    Yaml,   // YAML配列
}
```

---

### `FileFormat`

入力ファイルフォーマット。

```rust
#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    Json,
    Yaml,
    Csv,
    Toml,
    Ini,
    Xml,
}
```

---

### `LightweightDiffResult`

メモリ効率の良い差分結果（値をStringで保持）。

```rust
#[derive(Debug, PartialEq, Serialize)]
pub enum LightweightDiffResult {
    Added(String, String),
    Removed(String, String),
    Modified(String, String, String),
    TypeChanged(String, String, String),
}
```

`DiffResult` から変換可能:
```rust
impl From<&DiffResult> for LightweightDiffResult
```

---

## ユーティリティ関数

### `value_type_name`

JSON値の型名を取得する（内部用）。

```rust
pub fn value_type_name(value: &Value) -> &str
```

**戻り値**: `"Null"`, `"Boolean"`, `"Number"`, `"String"`, `"Array"`, `"Object"`

---

### `format_output`

差分結果を指定フォーマットの文字列に変換する（内部用）。

```rust
pub fn format_output<T: Serialize>(results: &[T], format: OutputFormat) -> Result<String>
```

---

### `format_diff_output`

`DiffResult` を指定フォーマットの文字列に変換する。

```rust
pub fn format_diff_output(
    results: &[DiffResult],
    format: OutputFormat,
    options: Option<&DiffOptions>,
) -> Result<String>
```

---

## 差分検出アルゴリズム

### オブジェクト比較

1. 両オブジェクトのキーを列挙
2. `ignore_keys_regex` にマッチするキーはスキップ（**再帰的に適用**）
3. 片方にのみ存在するキー → Added/Removed
4. 両方に存在するキー → 値を再帰比較

### 配列比較

**デフォルト（インデックスベース）**:
1. 長い方の長さまでインデックスで対応付け
2. 各要素を再帰比較
3. 片方にのみ存在する要素 → Added/Removed

**`array_id_key` 指定時（IDベース）**:
1. 各要素から指定キーの値を取得
2. **IDを持つ要素**: 同じID値を持つ要素同士を対応付け
3. **IDを持たない要素**: インデックスで対応付け（フォールバック）
4. 対応する要素を再帰比較
5. 対応しない要素 → Added/Removed

### 文字列比較

**`ignore_whitespace` 有効時**:
```rust
old.chars().filter(|c| !c.is_whitespace()).collect()
```
- 全ての空白文字（スペース、タブ、改行等）を削除して比較

**`ignore_case` 有効時**:
```rust
old.to_lowercase()
```
- 小文字に変換して比較
- **キー名には適用されない**（値のみ）

### 数値比較

**`epsilon` 指定時**:
```rust
let old_f = old_num.as_f64().unwrap_or(0.0);
let new_f = new_num.as_f64().unwrap_or(0.0);
(old_f - new_f).abs() <= epsilon
```
- **整数も f64 に変換して比較される**
- epsilon 未指定時は厳密比較（`old != new`）

### 型変更の検出

```rust
if value_type_name(old) != value_type_name(new) {
    DiffResult::TypeChanged(...)
} else {
    DiffResult::Modified(...)
}
```

**型名**: `"Null"`, `"Boolean"`, `"Number"`, `"String"`, `"Array"`, `"Object"`

- 同じ型で値が異なる → `Modified`
- 異なる型 → `TypeChanged`

### ディレクトリ比較

1. 両ディレクトリのファイルを再帰的に収集
2. 相対パスでマッチング
3. 片方にのみ存在 → Added/Removed（ファイル全体を値として）
4. 両方に存在 → ファイル内容を比較
5. **パースできないファイルは静かにスキップ**（エラーにならない）

---

## エラー処理

全ての公開関数は `anyhow::Result` を返す。

**主なエラー**:
- ファイルが存在しない
- パースエラー（不正なフォーマット）
- ファイルとディレクトリの混在比較
- ディレクトリ比較で `recursive` 未指定

---

## 使用例

### 基本的な使用

```rust
use diffx_core::{diff, DiffOptions, DiffxSpecificOptions};
use serde_json::json;

let old = json!({"name": "Alice", "age": 30});
let new = json!({"name": "ALICE", "age": 31});

// オプション設定
let options = DiffOptions {
    diffx_options: Some(DiffxSpecificOptions {
        ignore_case: Some(true),
        ..Default::default()
    }),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
// name は ignore_case により同値、age のみ差分
```

### ファイル比較

```rust
use diffx_core::{diff_paths, DiffOptions};

let options = DiffOptions {
    epsilon: Some(0.001),
    ..Default::default()
};

let results = diff_paths("config_v1.json", "config_v2.json", Some(&options))?;
```

### ディレクトリ比較

```rust
use diffx_core::{diff_paths, DiffOptions};

let options = DiffOptions {
    recursive: Some(true),
    ..Default::default()
};

let results = diff_paths("dir1/", "dir2/", Some(&options))?;
```

---

## 変更履歴

- 2025-12-11: 初版確定
  - lib.rs の再エクスポート構造を文書化
  - 全公開API、型、オプションを記載
  - 差分検出アルゴリズムの概要を追加

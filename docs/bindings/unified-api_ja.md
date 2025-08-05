# diffx 統一API リファレンス

*diffx-python および diffx-js 言語バインディング API ドキュメント*

## 概要

diffx は構造化データファイル（JSON、YAML、TOML、XML、INI、CSV）を比較するための統一APIを提供します。このライブラリは書式の変更ではなく、セマンティックな差分に焦点を当てています。

**統一API設計**: コアAPIは、すべての比較操作に対して単一のメイン関数 `diff()` のみを公開します。すべての機能はオプションパラメータを使用してこの統一インターフェースからアクセスできます。この設計により、すべてのユースケースにおいて一貫性とシンプルさが保証されます。

## メイン関数

### `diff(old, new, options)`

2つの構造化データ値を比較し、差分を返します。

#### パラメータ

- `old` (Value): 比較する元の/古いデータ構造
- `new` (Value): 比較する新しい/更新されたデータ構造
- `options` (DiffOptions, optional): 比較の設定オプション

#### 戻り値

- `Result<Vec<DiffResult>, Error>`: 2つの構造間で見つかった差分のベクター

#### 例

```rust
use diffx_core::{diff, DiffOptions, OutputFormat};
use serde_json::json;

let old = json!({
    "name": "John",
    "age": 30,
    "city": "New York"
});

let new = json!({
    "name": "John",
    "age": 31,
    "city": "Boston"
});

let options = DiffOptions {
    output_format: Some(OutputFormat::Json),
    show_unchanged: Some(false),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```

## オプション

### DiffOptions 構造体

```rust
pub struct DiffOptions {
    // コア比較オプション
    pub epsilon: Option<f64>,
    pub array_id_key: Option<String>,
    pub ignore_keys_regex: Option<Regex>,
    pub path_filter: Option<String>,
    
    // 出力制御
    pub output_format: Option<OutputFormat>,
    pub show_unchanged: Option<bool>,
    pub show_types: Option<bool>,
    
    // メモリ最適化
    pub use_memory_optimization: Option<bool>,
    pub batch_size: Option<usize>,
    
    // diffx固有オプション
    pub diffx_options: Option<DiffxSpecificOptions>,
}

pub struct DiffxSpecificOptions {
    pub context_lines: Option<usize>,
    pub ignore_whitespace: Option<bool>,
    pub ignore_case: Option<bool>,
    pub brief_mode: Option<bool>,
    pub quiet_mode: Option<bool>,
}
```

### オプション詳細

#### 数値比較
- **`epsilon`**: 浮動小数点比較の許容誤差。この値以内の値は等しいとみなされます。
  - デフォルト: `0.0`（厳密な比較）
  - 例: `0.001`（0.1%の許容誤差）

#### 配列比較
- **`array_id_key`**: 配列比較時に配列要素を識別するために使用するキー
  - デフォルト: `None`（インデックスベース比較）
  - 例: `"id"`（配列要素を`id`フィールドで照合）

#### フィルタリング
- **`ignore_keys_regex`**: 比較時に無視するキーの正規表現
  - 例: `"^(timestamp|metadata)"`（timestampとmetadataフィールドを無視）
- **`path_filter`**: 比較を特定のパスに限定するJSONPath形式のフィルター
  - 例: `"$.users[*].name"`（ユーザー名のみ比較）

#### 出力制御
- **`output_format`**: 出力形式
  - オプション: `Json`、`Yaml`、`Csv`、`Diffx`（カスタム形式）
  - デフォルト: `Json`
- **`show_unchanged`**: 出力に変更されていない値を含める
  - デフォルト: `false`
- **`show_types`**: 出力に型情報を含める
  - デフォルト: `false`

#### メモリ最適化
- **`use_memory_optimization`**: 大きなファイルのメモリ効率的な処理を有効にする
  - デフォルト: `false`
- **`batch_size`**: メモリ最適化が有効な時の各バッチで処理するアイテム数
  - デフォルト: `1000`

#### diffx固有オプション（DiffxSpecificOptions）
これらのオプションは`diffx_options`フィールド内にネストされています：

- **`context_lines`**: 統一diff形式で表示するコンテキスト行数
  - デフォルト: `3`
- **`ignore_whitespace`**: 文字列比較で空白の違いを無視
  - デフォルト: `false`
- **`ignore_case`**: 大文字小文字を区別しない文字列比較
  - デフォルト: `false`
- **`brief_mode`**: 差分ではなく、ファイルが異なるかどうかのみ表示
  - デフォルト: `false`
- **`quiet_mode`**: すべての通常出力を抑制
  - デフォルト: `false`

## 結果タイプ

### DiffResult 列挙型

```rust
pub enum DiffResult {
    Added(String, Value),
    Removed(String, Value),
    Modified(String, Value, Value),
    TypeChanged(String, String, String),
}
```

- **`Added(path, value)`**: 指定されたパスに新しいフィールド/値が追加された
- **`Removed(path, value)`**: 指定されたパスからフィールド/値が削除された
- **`Modified(path, old_value, new_value)`**: 指定されたパスで値が変更された
- **`TypeChanged(path, old_type, new_type)`**: 指定されたパスで値の型が変更された

## 言語バインディング

### Python

```python
import diffx_python

# 基本使用法
results = diffx_python.diff(old_dict, new_dict)

# オプション付き
results = diffx_python.diff(
    old_dict, 
    new_dict,
    epsilon=0.001,
    array_id_key="id",
    ignore_keys_regex="^(timestamp|metadata)",
    output_format="json",
    show_unchanged=False
)
```

### TypeScript/JavaScript

```typescript
import { diff, DiffOptions } from 'diffx-js';
import * as fs from 'fs';

// 基本使用法 - ユーザーがファイルを自分で解析
const oldData = JSON.parse(fs.readFileSync('old.json', 'utf8'));
const newData = JSON.parse(fs.readFileSync('new.json', 'utf8'));
const results = diff(oldData, newData);

// オプション付き
const options: DiffOptions = {
    epsilon: 0.001,
    arrayIdKey: 'id',
    ignoreKeysRegex: '^(timestamp|metadata)',
    outputFormat: 'json',
    showUnchanged: false
};
const results = diff(oldData, newData, options);
```

## エラーハンドリング

ライブラリは以下に対して詳細なエラーメッセージを返します：
- 解析エラー（無効なJSON、YAMLなど）
- ファイルI/Oエラー
- 無効な正規表現
- メモリ割り当て失敗
- 無効なオプション

## パフォーマンスの考慮事項

- 100MBより大きなファイルには`use_memory_optimization`を使用
- 利用可能メモリに基づいて`batch_size`を調整
- より良いパフォーマンスのために`path_filter`を使用して比較範囲を制限
- `ignore_keys_regex`の正規表現は大きなデータセットでパフォーマンスに影響する可能性があります

## 例

### JSONファイルの比較

```rust
use diffx_core::{diff, DiffOptions};
use serde_json;

// ユーザーは標準ライブラリを使用してデータを自分で解析する必要があります
let old_content = std::fs::read_to_string("old.json")?;
let new_content = std::fs::read_to_string("new.json")?;

let old: serde_json::Value = serde_json::from_str(&old_content)?;
let new: serde_json::Value = serde_json::from_str(&new_content)?;

let results = diff(&old, &new, None)?;
```

### タイムスタンプの無視

```rust
use regex::Regex;

let options = DiffOptions {
    ignore_keys_regex: Some(Regex::new("timestamp|updated_at|created_at")?),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```

### IDによる配列比較

```rust
let options = DiffOptions {
    array_id_key: Some("id".to_string()),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```

### diffx固有オプションの使用

```rust
use diffx_core::{DiffOptions, DiffxSpecificOptions};

let diffx_opts = DiffxSpecificOptions {
    ignore_whitespace: Some(true),
    ignore_case: Some(true),
    brief_mode: Some(false),
    ..Default::default()
};

let options = DiffOptions {
    diffx_options: Some(diffx_opts),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```
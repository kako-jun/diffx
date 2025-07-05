# APIリファレンス - diffx-core

構造化データのセマンティック差分機能を提供する `diffx-core` Rust クレートの完全なAPIドキュメントです。

## 概要

`diffx-core` クレートは diffx エコシステムの中核であり、構造化データ形式の高速で正確なセマンティック差分操作を提供します。他のRustアプリケーションに組み込んで、セマンティック比較機能を追加することができます。

## インストール

`Cargo.toml` に `diffx-core` を追加：

```toml
[dependencies]
diffx-core = "0.2.0"
```

### 機能フラグ

```toml
[dependencies]
diffx-core = { version = "0.2.0", features = ["all-formats"] }
```

利用可能な機能：
- `json` (デフォルト) - JSON形式サポート
- `yaml` (デフォルト) - YAML形式サポート  
- `toml` (デフォルト) - TOML形式サポート
- `xml` - XML形式サポート
- `ini` - INI形式サポート
- `csv` - CSV形式サポート
- `all-formats` - 全形式パーサーを有効化

## パブリックAPI

### コア型

#### `DiffResult`

2つの構造化された値間の単一のセマンティック差分を表します。

```rust
#[derive(Debug, PartialEq, Serialize)]
pub enum DiffResult {
    Added(String, Value),           // 新しいキー・値の追加
    Removed(String, Value),         // キー・値の削除
    Modified(String, Value, Value), // 値の変更（旧、新）
    TypeChanged(String, Value, Value), // 型の変更（旧、新）
}
```

**フィールド：**
- **パス** (`String`): 変更された要素へのJSONパス（例：`"config.database.port"`）
- **値** (`Value`): データを表すserde_json::Value

**例：**
```rust
use diffx_core::DiffResult;
use serde_json::Value;

// キーの追加
let added = DiffResult::Added(
    "database.port".to_string(),
    Value::Number(5432.into())
);

// 値の変更  
let modified = DiffResult::Modified(
    "version".to_string(),
    Value::String("1.0".to_string()),
    Value::String("1.1".to_string())
);

// 型の変更
let type_changed = DiffResult::TypeChanged(
    "debug".to_string(),
    Value::String("true".to_string()),
    Value::Bool(true)
);
```

### コア関数

#### `diff()`

2つの構造化された値間のセマンティック差分を計算するメイン関数。

```rust
pub fn diff(
    v1: &Value,
    v2: &Value,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Vec<DiffResult>
```

**パラメータ：**
- `v1`: 比較する最初の値（ベースライン）
- `v2`: 比較する2番目の値（ターゲット）
- `ignore_keys_regex`: 特定キーを無視するオプションの正規表現
- `epsilon`: 浮動小数点比較の許容誤差（オプション）
- `array_id_key`: 配列要素識別のためのオプションキー

**戻り値：** 見つかったすべての差分を表す `DiffResult` のベクター

**例：**
```rust
use diffx_core::{diff, DiffResult};
use serde_json::{json, Value};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let v1 = json!({
        "name": "myapp",
        "version": "1.0",
        "timestamp": "2024-01-01T00:00:00Z"
    });
    
    let v2 = json!({
        "name": "myapp",
        "version": "1.1", 
        "timestamp": "2024-01-02T00:00:00Z",
        "port": 8080
    });
    
    // タイムスタンプの変更を無視
    let ignore_regex = Regex::new(r"^timestamp$")?;
    
    let differences = diff(&v1, &v2, Some(&ignore_regex), None, None);
    
    for diff in differences {
        match diff {
            DiffResult::Added(path, value) => {
                println!("追加 {}: {}", path, value);
            }
            DiffResult::Modified(path, old, new) => {
                println!("変更 {}: {} -> {}", path, old, new);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### フォーマットパーサー

#### `parse_ini()`

INI形式のコンテンツをJSON Valueに解析します。

```rust
pub fn parse_ini(content: &str) -> Result<Value>
```

**例：**
```rust
use diffx_core::parse_ini;

let ini_content = r#"
[database]
host = localhost
port = 5432

[cache]
enabled = true
ttl = 3600
"#;

let parsed = parse_ini(ini_content)?;
println!("{}", serde_json::to_string_pretty(&parsed)?);
```

#### `parse_xml()`

XML形式のコンテンツをJSON Valueに解析します。

```rust
pub fn parse_xml(content: &str) -> Result<Value>
```

**例：**
```rust
use diffx_core::parse_xml;

let xml_content = r#"
<config>
    <database>
        <host>localhost</host>
        <port>5432</port>
    </database>
    <cache enabled="true">
        <ttl>3600</ttl>
    </cache>
</config>
"#;

let parsed = parse_xml(xml_content)?;
println!("{}", serde_json::to_string_pretty(&parsed)?);
```

#### `parse_csv()`

CSV形式のコンテンツをJSON Value（オブジェクトの配列）に解析します。

```rust
pub fn parse_csv(content: &str) -> Result<Value>
```

**例：**
```rust
use diffx_core::parse_csv;

let csv_content = r#"
name,age,city
Alice,25,New York
Bob,30,San Francisco
Charlie,35,Chicago
"#;

let parsed = parse_csv(csv_content)?;
println!("{}", serde_json::to_string_pretty(&parsed)?);
// 出力: [{"name": "Alice", "age": "25", "city": "New York"}, ...]
```

### ユーティリティ関数

#### `value_type_name()`

JSON値の人間が読める型名を取得します。

```rust
pub fn value_type_name(value: &Value) -> &str
```

**戻り値：** 型名の文字列スライス: `"null"`, `"boolean"`, `"number"`, `"string"`, `"array"`, `"object"`

**例：**
```rust
use diffx_core::value_type_name;
use serde_json::{json, Value};

let values = vec![
    json!(null),
    json!(true),
    json!(42),
    json!("hello"),
    json!([1, 2, 3]),
    json!({"key": "value"})
];

for value in values {
    println!("{}: {}", value, value_type_name(&value));
}
// 出力:
// null: null
// true: boolean  
// 42: number
// "hello": string
// [1,2,3]: array
// {"key":"value"}: object
```

## 高度な使用法

### カスタム比較ロジック

#### Epsilon比較

浮動小数点精度の違いを処理：

```rust
use diffx_core::diff;
use serde_json::json;

let v1 = json!({"pi": 3.14159});
let v2 = json!({"pi": 3.14160});

// epsilonなし - 差分を報告
let diffs_strict = diff(&v1, &v2, None, None, None);
assert!(!diffs_strict.is_empty());

// epsilonあり - 差分なし
let diffs_epsilon = diff(&v1, &v2, None, Some(0.001), None);
assert!(diffs_epsilon.is_empty());
```

#### 正規表現キーフィルタリング

特定のキーやパターンを無視：

```rust
use diffx_core::diff;
use serde_json::json;
use regex::Regex;

let v1 = json!({
    "data": {"important": "value"},
    "timestamp": "2024-01-01T00:00:00Z",
    "_internal": "system_data"
});

let v2 = json!({
    "data": {"important": "new_value"},
    "timestamp": "2024-01-02T00:00:00Z", 
    "_internal": "different_system_data"
});

// タイムスタンプと内部フィールドを無視
let ignore_regex = Regex::new(r"^(timestamp|_.*)")?;
let differences = diff(&v1, &v2, Some(&ignore_regex), None, None);

// 重要なデータの変更のみ報告
assert_eq!(differences.len(), 1);
```

#### 配列要素追跡

位置ではなくIDで配列要素を追跡：

```rust
use diffx_core::diff;
use serde_json::json;

let v1 = json!({
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]
});

let v2 = json!({
    "users": [
        {"id": 2, "name": "Bob"}, 
        {"id": 1, "name": "Alice Smith"}  // 名前が変更
    ]
});

// ID追跡あり - 名前変更を検出
let differences = diff(&v1, &v2, None, None, Some("id"));
// 報告: Modified users[id=1].name: "Alice" -> "Alice Smith"

// ID追跡なし - 位置のため全て変更として報告
let differences_positional = diff(&v1, &v2, None, None, None);
// 位置の違いにより複数の変更を報告
```

### 異なる形式での作業

#### 完全な形式処理パイプライン

```rust
use diffx_core::{diff, parse_ini, parse_xml, parse_csv};
use serde_json::{from_str, Value};
use std::fs;

fn compare_files(
    file1_path: &str,
    file2_path: &str,
    format: &str
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = fs::read_to_string(file1_path)?;
    let content2 = fs::read_to_string(file2_path)?;
    
    let (value1, value2) = match format {
        "json" => {
            (from_str(&content1)?, from_str(&content2)?)
        }
        "yaml" => {
            (serde_yml::from_str(&content1)?, serde_yml::from_str(&content2)?)
        }
        "toml" => {
            (toml::from_str(&content1)?, toml::from_str(&content2)?)
        }
        "ini" => {
            (parse_ini(&content1)?, parse_ini(&content2)?)
        }
        "xml" => {
            (parse_xml(&content1)?, parse_xml(&content2)?)
        }
        "csv" => {
            (parse_csv(&content1)?, parse_csv(&content2)?)
        }
        _ => return Err(format!("サポートされていない形式: {}", format).into())
    };
    
    Ok(diff(&value1, &value2, None, None, None))
}
```

### 統合パターン

#### カスタム差分処理

```rust
use diffx_core::{diff, DiffResult};
use serde_json::Value;

struct DiffProcessor {
    pub additions: Vec<(String, Value)>,
    pub removals: Vec<(String, Value)>,
    pub modifications: Vec<(String, Value, Value)>,
    pub type_changes: Vec<(String, Value, Value)>,
}

impl DiffProcessor {
    pub fn new() -> Self {
        Self {
            additions: Vec::new(),
            removals: Vec::new(),
            modifications: Vec::new(),
            type_changes: Vec::new(),
        }
    }
    
    pub fn process(&mut self, differences: Vec<DiffResult>) {
        for diff in differences {
            match diff {
                DiffResult::Added(path, value) => {
                    self.additions.push((path, value));
                }
                DiffResult::Removed(path, value) => {
                    self.removals.push((path, value));
                }
                DiffResult::Modified(path, old, new) => {
                    self.modifications.push((path, old, new));
                }
                DiffResult::TypeChanged(path, old, new) => {
                    self.type_changes.push((path, old, new));
                }
            }
        }
    }
    
    pub fn has_critical_changes(&self) -> bool {
        // 「重要な」変更を定義
        !self.removals.is_empty() || 
        !self.type_changes.is_empty() ||
        self.modifications.iter().any(|(path, _, _)| {
            path.contains("security") || path.contains("database")
        })
    }
}
```

#### 非同期処理

```rust
use diffx_core::{diff, DiffResult};
use serde_json::Value;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = vec![
        process_diff_async("file1.json", "file2.json"),
        process_diff_async("file3.json", "file4.json"),
    ];
    
    let results = futures::future::try_join_all(tasks).await?;
    
    for (i, diffs) in results.into_iter().enumerate() {
        println!("ファイルペア {}: {} 個の差分", i + 1, diffs.len());
    }
    
    Ok(())
}

async fn process_diff_async(
    file1: &str,
    file2: &str
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = tokio::fs::read_to_string(file1).await?;
    let content2 = tokio::fs::read_to_string(file2).await?;
    
    // ブロッキングを避けるためバックグラウンドタスクで解析
    let result = tokio::task::spawn_blocking(move || {
        let v1: Value = serde_json::from_str(&content1)?;
        let v2: Value = serde_json::from_str(&content2)?;
        Ok::<_, serde_json::Error>(diff(&v1, &v2, None, None, None))
    }).await??;
    
    Ok(result)
}
```

## エラー処理

### エラー型

ライブラリはエラー処理に `anyhow::Error` を使用します：

```rust
use diffx_core::parse_ini;
use anyhow::Result;

fn handle_parse_errors() -> Result<()> {
    let invalid_ini = "invalid [section syntax";
    
    match parse_ini(invalid_ini) {
        Ok(value) => println!("正常に解析されました: {}", value),
        Err(e) => {
            eprintln!("解析エラー: {}", e);
            
            // エラー原因のチェーン
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("原因: {}", err);
                source = err.source();
            }
        }
    }
    
    Ok(())
}
```

### 一般的なエラーシナリオ

```rust
use diffx_core::{diff, parse_xml};
use serde_json::json;

// 不正なデータを処理
fn robust_comparison(
    data1: &str,
    data2: &str
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    // 最初にJSONとして解析を試行
    let v1 = match serde_json::from_str(data1) {
        Ok(v) => v,
        Err(_) => {
            // JSONが失敗した場合はXMLを試行
            parse_xml(data1)?
        }
    };
    
    let v2 = match serde_json::from_str(data2) {
        Ok(v) => v,
        Err(_) => parse_xml(data2)?
    };
    
    Ok(diff(&v1, &v2, None, None, None))
}
```

## パフォーマンス考慮事項

### メモリ使用量

大きなデータセットの場合：

```rust
use diffx_core::diff;
use serde_json::Value;

// 大きなファイルを効率的に処理
fn process_large_diff(
    v1: &Value,
    v2: &Value,
    focus_path: Option<&str>
) -> Vec<DiffResult> {
    // 特定のパスに焦点を当てる場合、その部分のみを抽出
    if let Some(path) = focus_path {
        if let (Some(sub1), Some(sub2)) = (
            extract_path(v1, path),
            extract_path(v2, path)
        ) {
            return diff(&sub1, &sub2, None, None, None);
        }
    }
    
    diff(v1, v2, None, None, None)
}

fn extract_path(value: &Value, path: &str) -> Option<Value> {
    // ネストしたパスを抽出する実装
    // これはJSONパスを辿る
    todo!("パス抽出を実装")
}
```

### 最適化のヒント

1. **正規表現フィルタリングを使用** して大きな無関係なセクションを無視
2. **epsilon値を指定** して浮動小数点が多いデータを処理
3. **配列IDキーを使用** して識別可能な要素を持つ大きな配列を処理
4. **パスフィルタリングを検討** して非常に大きなオブジェクトを処理

## テスト

### ユニットテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_basic_diff() {
        let v1 = json!({"a": 1, "b": 2});
        let v2 = json!({"a": 1, "b": 3, "c": 4});
        
        let diffs = diff(&v1, &v2, None, None, None);
        
        assert_eq!(diffs.len(), 2);
        // 特定の差分をテスト...
    }
    
    #[test]
    fn test_epsilon_comparison() {
        let v1 = json!({"value": 1.0});
        let v2 = json!({"value": 1.0001});
        
        let diffs_strict = diff(&v1, &v2, None, None, None);
        assert!(!diffs_strict.is_empty());
        
        let diffs_epsilon = diff(&v1, &v2, None, Some(0.001), None);
        assert!(diffs_epsilon.is_empty());
    }
}
```

## バージョン互換性

- **0.2.x**: 現在の安定版
- **最小Rustバージョン**: 1.70.0
- **依存関係**: 現在のバージョンについては `Cargo.toml` を参照

## 関連項目

- [CLIリファレンス](cli-reference_ja.md) - コマンドライン使用方法
- [はじめにガイド](../user-guide/getting-started_ja.md) - 基本概念
- [実用例](../user-guide/examples_ja.md) - 実用的な使用例
- [設定ガイド](../user-guide/configuration_ja.md) - 高度な設定
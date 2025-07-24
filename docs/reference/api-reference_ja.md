# API リファレンス - diffx-core

構造化データのセマンティック差分機能を提供する `diffx-core` Rust クレートの完全な API ドキュメント。

## 概要

`diffx-core` クレートは diffx エコシステムの中核であり、構造化データフォーマットに対する高速で正確なセマンティック差分操作を提供します。他の Rust アプリケーションに組み込んで、セマンティック比較機能を追加できます。

**統一API設計**: コアAPIは、すべての比較操作に対して単一のメイン関数 `diff()` のみを公開します。すべての機能はオプションパラメータを使用してこの統一インターフェースからアクセスできます。この設計により、すべてのユースケースにおいて一貫性とシンプルさが保証されます。

## インストール

`Cargo.toml` に `diffx-core` を追加：

```toml
[dependencies]
diffx-core = "0.2.0"
```

### フィーチャーフラグ

```toml
[dependencies]
diffx-core = { version = "0.2.0", features = ["all-formats"] }
```

利用可能なフィーチャー：
- `json` (デフォルト) - JSON フォーマットサポート
- `yaml` (デフォルト) - YAML フォーマットサポート  
- `toml` (デフォルト) - TOML フォーマットサポート
- `xml` - XML フォーマットサポート
- `ini` - INI フォーマットサポート
- `csv` - CSV フォーマットサポート
- `all-formats` - すべてのフォーマットパーサーを有効化

## パブリック API

### コアタイプ

#### `DiffResult`

2つの構造化された値の間の単一のセマンティック差分を表します。

```rust
#[derive(Debug, PartialEq, Serialize)]
pub enum DiffResult {
    Added(String, Value),           // 新しいキー/値が追加された
    Removed(String, Value),         // キー/値が削除された
    Modified(String, Value, Value), // 値が変更された (古い値, 新しい値)
    TypeChanged(String, String, String), // 型が変更された (パス, 古い型, 新しい型)
}
```

**フィールド:**
- **パス** (`String`): 変更された要素への JSON パス（例: `"config.database.port"`）
- **値** (`Value`): データを表す serde_json::Value

**例:**
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
    "string".to_string(),
    "boolean".to_string()
);
```

### コア関数

#### `diff()`

2つの構造化された値の間のセマンティック差分を計算するための主要関数。これはすべての比較操作のための統一APIエントリーポイントです。

```rust
pub fn diff(
    old: &Value,
    new: &Value,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>, Error>
```

**パラメータ:**
- `old`: 元の/ベースラインの値
- `new`: 新しい/ターゲットの値  
- `options`: 比較のためのオプション設定

**戻り値:** 見つかったすべての差分を表す `Result<Vec<DiffResult>, Error>`

#### DiffOptions 構造体

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
```

**例:**
```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::{json, Value};
use regex::Regex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let old = json!({
        "name": "myapp",
        "version": "1.0",
        "timestamp": "2024-01-01T00:00:00Z"
    });
    
    let new = json!({
        "name": "myapp",
        "version": "1.1", 
        "timestamp": "2024-01-02T00:00:00Z",
        "port": 8080
    });
    
    // タイムスタンプの変更を無視する設定
    let options = DiffOptions {
        ignore_keys_regex: Some(Regex::new(r"^timestamp$")?),
        show_unchanged: Some(false),
        ..Default::default()
    };
    
    let differences = diff(&old, &new, Some(&options))?;
    
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

## 高度な使用法

### カスタム比較ロジック

#### イプシロン比較

浮動小数点精度の差を扱う：

```rust
use diffx_core::{diff, DiffOptions};
use serde_json::json;

let old = json!({"pi": 3.14159});
let new = json!({"pi": 3.14160});

// イプシロンなし - 差分を報告
let diffs_strict = diff(&old, &new, None)?;
assert!(!diffs_strict.is_empty());

// イプシロンあり - 差分なし
let options = DiffOptions {
    epsilon: Some(0.001),
    ..Default::default()
};
let diffs_epsilon = diff(&old, &new, Some(&options))?;
assert!(diffs_epsilon.is_empty());
```

#### 正規表現キーフィルタリング

特定のキーやパターンを無視：

```rust
use diffx_core::{diff, DiffOptions};
use serde_json::json;
use regex::Regex;

let old = json!({
    "data": {"important": "value"},
    "timestamp": "2024-01-01T00:00:00Z",
    "_internal": "system_data"
});

let new = json!({
    "data": {"important": "new_value"},
    "timestamp": "2024-01-02T00:00:00Z", 
    "_internal": "different_system_data"
});

// タイムスタンプと内部フィールドを無視
let options = DiffOptions {
    ignore_keys_regex: Some(Regex::new(r"^(timestamp|_.*)")?),
    ..Default::default()
};
let differences = diff(&old, &new, Some(&options))?;

// 重要なデータの変更のみ報告
assert_eq!(differences.len(), 1);
```

#### 配列要素トラッキング

位置ではなくIDで配列要素を追跡：

```rust
use diffx_core::{diff, DiffOptions};
use serde_json::json;

let old = json!({
    "users": [
        {"id": 1, "name": "Alice"},
        {"id": 2, "name": "Bob"}
    ]
});

let new = json!({
    "users": [
        {"id": 2, "name": "Bob"}, 
        {"id": 1, "name": "Alice Smith"}  // 名前が変更された
    ]
});

// IDトラッキングあり - 名前の変更を検出
let options = DiffOptions {
    array_id_key: Some("id".to_string()),
    ..Default::default()
};
let differences = diff(&old, &new, Some(&options))?;
// 報告: Modified users[id=1].name: "Alice" -> "Alice Smith"

// IDトラッキングなし - 位置のため全て変更として報告
let differences_positional = diff(&old, &new, None)?;
// 位置の違いによる複数の変更を報告
```

### 異なるフォーマットでの作業

#### 完全なフォーマット処理パイプライン

```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::{from_str, Value};
use std::fs;

fn compare_files(
    file1_path: &str,
    file2_path: &str,
    format: &str,
    options: Option<&DiffOptions>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = fs::read_to_string(file1_path)?;
    let content2 = fs::read_to_string(file2_path)?;
    
    // ユーザーは各フォーマット用の標準パーサーを使用すべき
    let (old, new) = match format {
        "json" => {
            (from_str(&content1)?, from_str(&content2)?)
        }
        "yaml" => {
            (serde_yml::from_str(&content1)?, serde_yml::from_str(&content2)?)
        }
        "toml" => {
            (toml::from_str(&content1)?, toml::from_str(&content2)?)
        }
        _ => return Err(format!("サポートされていないフォーマット: {}", format).into())
    };
    
    Ok(diff(&old, &new, options)?)
}
```

### 統合パターン

#### カスタム差分処理

```rust
use diffx_core::{diff, DiffOptions, DiffResult};
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
        for diff_result in differences {
            match diff_result {
                DiffResult::Added(path, value) => {
                    self.additions.push((path, value));
                }
                DiffResult::Removed(path, value) => {
                    self.removals.push((path, value));
                }
                DiffResult::Modified(path, old, new) => {
                    self.modifications.push((path, old, new));
                }
                DiffResult::TypeChanged(path, old_type, new_type) => {
                    // 注意: TypeChanged は型文字列を含む（値ではない）
                    self.type_changes.push((path, old_type.into(), new_type.into()));
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
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::Value;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tasks = vec![
        process_diff_async("file1.json", "file2.json", None),
        process_diff_async("file3.json", "file4.json", None),
    ];
    
    let results = futures::future::try_join_all(tasks).await?;
    
    for (i, diffs) in results.into_iter().enumerate() {
        println!("ファイルペア {}: {} 個の差分", i + 1, diffs.len());
    }
    
    Ok(())
}

async fn process_diff_async(
    file1: &str,
    file2: &str,
    options: Option<DiffOptions>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = tokio::fs::read_to_string(file1).await?;
    let content2 = tokio::fs::read_to_string(file2).await?;
    
    // ブロッキングを避けるためバックグラウンドタスクで解析
    let result = tokio::task::spawn_blocking(move || {
        let old: Value = serde_json::from_str(&content1)?;
        let new: Value = serde_json::from_str(&content2)?;
        diff(&old, &new, options.as_ref())
    }).await??;
    
    Ok(result)
}
```

## エラーハンドリング

### エラータイプ

ライブラリはエラーハンドリングに `anyhow::Error` を使用します：

```rust
use diffx_core::{diff, DiffOptions};
use anyhow::Result;

fn handle_parse_errors() -> Result<()> {
    let invalid_data = "invalid json";
    
    match serde_json::from_str::<Value>(invalid_data) {
        Ok(value) => println!("解析成功: {}", value),
        Err(e) => {
            eprintln!("解析エラー: {}", e);
            
            // エラー原因の連鎖
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
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::Value;

// 複数のフォーマットの可能性を扱う
fn robust_comparison(
    data1: &str,
    data2: &str,
    options: Option<&DiffOptions>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    // 最初に JSON として解析を試みる
    let old = serde_json::from_str::<Value>(data1)
        .or_else(|_| serde_yml::from_str::<Value>(data1))
        .or_else(|_| toml::from_str::<Value>(data1))?;
    
    let new = serde_json::from_str::<Value>(data2)
        .or_else(|_| serde_yml::from_str::<Value>(data2))
        .or_else(|_| toml::from_str::<Value>(data2))?;
    
    Ok(diff(&old, &new, options)?)
}
```

## パフォーマンスの考慮事項

### メモリ使用量

大きなデータセットの場合：

```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::Value;

// 大きなファイルを効率的に処理
fn process_large_diff(
    old: &Value,
    new: &Value,
    focus_path: Option<&str>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let options = if let Some(path) = focus_path {
        DiffOptions {
            path_filter: Some(path.to_string()),
            use_memory_optimization: Some(true),
            ..Default::default()
        }
    } else {
        DiffOptions {
            use_memory_optimization: Some(true),
            ..Default::default()
        }
    };
    
    Ok(diff(old, new, Some(&options))?)
}
```

### 最適化のヒント

1. **正規表現フィルタリングを使用** して大きな無関係なセクションを無視
2. **イプシロンを指定** して浮動小数点の多いデータに対応
3. **配列IDキーを使用** して識別可能な要素を持つ大きな配列に対応
4. **パスフィルタリングを検討** して非常に大きなオブジェクトに対応

## テスト

### ユニットテスト

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_basic_diff() {
        let old = json!({"a": 1, "b": 2});
        let new = json!({"a": 1, "b": 3, "c": 4});
        
        let diffs = diff(&old, &new, None).unwrap();
        
        assert_eq!(diffs.len(), 2);
        // 特定の差分をテスト...
    }
    
    #[test]
    fn test_epsilon_comparison() {
        let old = json!({"value": 1.0});
        let new = json!({"value": 1.0001});
        
        let diffs_strict = diff(&old, &new, None).unwrap();
        assert!(!diffs_strict.is_empty());
        
        let options = DiffOptions {
            epsilon: Some(0.001),
            ..Default::default()
        };
        let diffs_epsilon = diff(&old, &new, Some(&options)).unwrap();
        assert!(diffs_epsilon.is_empty());
    }
}
```

## バージョン互換性

- **0.2.x**: 現在の安定版
- **最小 Rust バージョン**: 1.70.0
- **依存関係**: 現在のバージョンは `Cargo.toml` を参照

## 関連項目

- [CLI リファレンス](cli-reference_ja.md) - コマンドライン使用法
- [はじめに](../user-guide/getting-started_ja.md) - 基本概念
- [実用例](../user-guide/examples_ja.md) - 実用的なユースケース
- [統一API リファレンス](../bindings/unified-api_ja.md) - 言語バインディング
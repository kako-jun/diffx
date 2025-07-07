# API 参考 - diffx-core

为结构化数据提供语义差异功能的 `diffx-core` Rust crate 的完整 API 文档。

## 概述

`diffx-core` crate 是 diffx 生态系统的核心，为结构化数据格式提供快速准确的语义差异操作。它可以嵌入到其他 Rust 应用程序中以添加语义比较功能。

## 安装

将 `diffx-core` 添加到您的 `Cargo.toml`：

```toml
[dependencies]
diffx-core = "0.2.0"
```

### 功能标志

```toml
[dependencies]
diffx-core = { version = "0.2.0", features = ["all-formats"] }
```

可用功能：
- `json`（默认）- JSON 格式支持
- `yaml`（默认）- YAML 格式支持  
- `toml`（默认）- TOML 格式支持
- `xml` - XML 格式支持
- `ini` - INI 格式支持
- `csv` - CSV 格式支持
- `all-formats` - 启用所有格式解析器

## 公共 API

### 核心类型

#### `DiffResult`

表示两个结构化值之间的单个语义差异。

```rust
#[derive(Debug, PartialEq, Serialize)]
pub enum DiffResult {
    Added(String, Value),           // 新增键/值
    Removed(String, Value),         // 删除键/值
    Modified(String, Value, Value), // 值变更（旧，新）
    TypeChanged(String, Value, Value), // 类型变更（旧，新）
}
```

**字段：**
- **路径**（`String`）：到变更元素的 JSON 路径（例如，`"config.database.port"`）
- **值**（`Value`）：表示数据的 serde_json::Value

**示例：**
```rust
use diffx_core::DiffResult;
use serde_json::Value;

// 键添加
let added = DiffResult::Added(
    "database.port".to_string(),
    Value::Number(5432.into())
);

// 值修改  
let modified = DiffResult::Modified(
    "version".to_string(),
    Value::String("1.0".to_string()),
    Value::String("1.1".to_string())
);

// 类型变更
let type_changed = DiffResult::TypeChanged(
    "debug".to_string(),
    Value::String("true".to_string()),
    Value::Bool(true)
);
```

### 核心函数

#### `diff()`

计算两个结构化值之间语义差异的主要函数。

```rust
pub fn diff(
    v1: &Value,
    v2: &Value,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
) -> Vec<DiffResult>
```

**参数：**
- `v1`：要比较的第一个值（基线）
- `v2`：要比较的第二个值（目标）
- `ignore_keys_regex`：忽略某些键的可选正则表达式
- `epsilon`：浮点数比较的可选容差
- `array_id_key`：数组元素识别的可选键

**返回：**表示所有发现差异的 `DiffResult` 向量

**示例：**
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
    
    // 忽略时间戳变更
    let ignore_regex = Regex::new(r"^timestamp$")?;
    
    let differences = diff(&v1, &v2, Some(&ignore_regex), None, None);
    
    for diff in differences {
        match diff {
            DiffResult::Added(path, value) => {
                println!("添加 {}: {}", path, value);
            }
            DiffResult::Modified(path, old, new) => {
                println!("修改 {}: {} -> {}", path, old, new);
            }
            _ => {}
        }
    }
    
    Ok(())
}
```

### 格式解析器

#### `parse_ini()`

将 INI 格式内容解析为 JSON Value。

```rust
pub fn parse_ini(content: &str) -> Result<Value>
```

**示例：**
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

将 XML 格式内容解析为 JSON Value。

```rust
pub fn parse_xml(content: &str) -> Result<Value>
```

**示例：**
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

将 CSV 格式内容解析为 JSON Value（对象数组）。

```rust
pub fn parse_csv(content: &str) -> Result<Value>
```

**示例：**
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
// 输出：[{"name": "Alice", "age": "25", "city": "New York"}, ...]
```

### 工具函数

#### `value_type_name()`

获取 JSON 值的人类可读类型名称。

```rust
pub fn value_type_name(value: &Value) -> &str
```

**返回：**带类型名称的字符串切片：`"null"`、`"boolean"`、`"number"`、`"string"`、`"array"`、`"object"`

**示例：**
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
// 输出：
// null: null
// true: boolean  
// 42: number
// "hello": string
// [1,2,3]: array
// {"key":"value"}: object
```

## 高级用法

### 自定义比较逻辑

#### Epsilon 比较

处理浮点精度差异：

```rust
use diffx_core::diff;
use serde_json::json;

let v1 = json!({"pi": 3.14159});
let v2 = json!({"pi": 3.14160});

// 不使用 epsilon - 报告差异
let diffs_strict = diff(&v1, &v2, None, None, None);
assert!(!diffs_strict.is_empty());

// 使用 epsilon - 无差异
let diffs_epsilon = diff(&v1, &v2, None, Some(0.001), None);
assert!(diffs_epsilon.is_empty());
```

#### 正则表达式键过滤

忽略特定键或模式：

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

// 忽略时间戳和内部字段
let ignore_regex = Regex::new(r"^(timestamp|_.*)")?;
let differences = diff(&v1, &v2, Some(&ignore_regex), None, None);

// 仅报告重要数据变更
assert_eq!(differences.len(), 1);
```

#### 数组元素跟踪

按 ID 而不是位置跟踪数组元素：

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
        {"id": 1, "name": "Alice Smith"}  // 名称变更
    ]
});

// 使用 ID 跟踪 - 检测名称变更
let differences = diff(&v1, &v2, None, None, Some("id"));
// 报告：修改 users[id=1].name: "Alice" -> "Alice Smith"

// 不使用 ID 跟踪 - 由于位置原因报告所有变更
let differences_positional = diff(&v1, &v2, None, None, None);
// 由于位置差异报告多个变更
```

### 处理不同格式

#### 完整格式处理管道

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
        _ => return Err(format!("不支持的格式：{}", format).into())
    };
    
    Ok(diff(&value1, &value2, None, None, None))
}
```

### 集成模式

#### 自定义差异处理

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
        // 定义什么构成"关键"变更
        !self.removals.is_empty() || 
        !self.type_changes.is_empty() ||
        self.modifications.iter().any(|(path, _, _)| {
            path.contains("security") || path.contains("database")
        })
    }
}
```

#### 异步处理

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
        println!("文件对 {}: {} 个差异", i + 1, diffs.len());
    }
    
    Ok(())
}

async fn process_diff_async(
    file1: &str,
    file2: &str
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    let content1 = tokio::fs::read_to_string(file1).await?;
    let content2 = tokio::fs::read_to_string(file2).await?;
    
    // 在后台任务中解析以避免阻塞
    let result = tokio::task::spawn_blocking(move || {
        let v1: Value = serde_json::from_str(&content1)?;
        let v2: Value = serde_json::from_str(&content2)?;
        Ok::<_, serde_json::Error>(diff(&v1, &v2, None, None, None))
    }).await??;
    
    Ok(result)
}
```

## 错误处理

### 错误类型

库使用 `anyhow::Error` 进行错误处理：

```rust
use diffx_core::parse_ini;
use anyhow::Result;

fn handle_parse_errors() -> Result<()> {
    let invalid_ini = "invalid [section syntax";
    
    match parse_ini(invalid_ini) {
        Ok(value) => println!("解析成功：{}", value),
        Err(e) => {
            eprintln!("解析错误：{}", e);
            
            // 错误原因链
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("由以下引起：{}", err);
                source = err.source();
            }
        }
    }
    
    Ok(())
}
```

### 常见错误场景

```rust
use diffx_core::{diff, parse_xml};
use serde_json::json;

// 处理格式错误的数据
fn robust_comparison(
    data1: &str,
    data2: &str
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    // 首先尝试解析为 JSON
    let v1 = match serde_json::from_str(data1) {
        Ok(v) => v,
        Err(_) => {
            // JSON 失败时尝试 XML
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

## 性能考虑

### 内存使用

对于大数据集：

```rust
use diffx_core::diff;
use serde_json::Value;

// 高效处理大文件
fn process_large_diff(
    v1: &Value,
    v2: &Value,
    focus_path: Option<&str>
) -> Vec<DiffResult> {
    // 如果专注于特定路径，只提取该部分
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
    // 实现提取嵌套路径
    // 这将遍历 JSON 路径
    todo!("实现路径提取")
}
```

### 优化技巧

1. **使用正则表达式过滤**忽略大的、不相关的部分
2. **指定 epsilon**用于浮点数密集的数据
3. **使用数组 ID 键**用于具有可识别元素的大数组
4. **考虑路径过滤**用于非常大的对象

## 测试

### 单元测试

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
        // 测试特定差异...
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

## 版本兼容性

- **0.2.x**：当前稳定版本
- **最低 Rust 版本**：1.70.0
- **依赖项**：当前版本请参见 `Cargo.toml`

## 另请参阅

- [CLI 参考](cli-reference_zh.md) 命令行用法
- [入门指南](../user-guide/getting-started_zh.md) 基本概念
- [示例](../user-guide/examples_zh.md) 实际用例
- [配置指南](../user-guide/configuration_zh.md) 高级设置
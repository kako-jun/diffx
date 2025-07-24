# API 参考 - diffx-core

`diffx-core` Rust crate 的完整 API 文档，提供结构化数据的语义差异功能。

## 概述

`diffx-core` crate 是 diffx 生态系统的核心，为结构化数据格式提供快速、准确的语义差异操作。它可以嵌入到其他 Rust 应用程序中以添加语义比较功能。

**统一API设计**：核心 API 仅公开一个主函数 `diff()` 用于所有比较操作。所有功能都通过选项参数从这个统一接口访问。这种设计确保了所有用例的一致性和简单性。

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
    Modified(String, Value, Value), // 值已更改（旧值，新值）
    TypeChanged(String, String, String), // 类型已更改（路径，旧类型，新类型）
}
```

**字段：**
- **路径** (`String`)：更改元素的 JSON 路径（例如：`"config.database.port"`）
- **值** (`Value`)：表示数据的 serde_json::Value

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

// 类型更改
let type_changed = DiffResult::TypeChanged(
    "debug".to_string(),
    "string".to_string(),
    "boolean".to_string()
);
```

### 核心函数

#### `diff()`

计算两个结构化值之间语义差异的主要函数。这是所有比较操作的统一 API 入口点。

```rust
pub fn diff(
    old: &Value,
    new: &Value,
    options: Option<&DiffOptions>,
) -> Result<Vec<DiffResult>, Error>
```

**参数：**
- `old`：原始/基准值
- `new`：新/目标值  
- `options`：比较的可选配置选项

**返回值：**表示找到的所有差异的 `Result<Vec<DiffResult>, Error>`

#### DiffOptions 结构体

```rust
pub struct DiffOptions {
    // 核心比较选项
    pub epsilon: Option<f64>,
    pub array_id_key: Option<String>,
    pub ignore_keys_regex: Option<Regex>,
    pub path_filter: Option<String>,
    
    // 输出控制
    pub output_format: Option<OutputFormat>,
    pub show_unchanged: Option<bool>,
    pub show_types: Option<bool>,
    
    // 内存优化
    pub use_memory_optimization: Option<bool>,
    pub batch_size: Option<usize>,
    
    // diffx特定选项
    pub diffx_options: Option<DiffxSpecificOptions>,
}
```

**示例：**
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
    
    // 配置选项以忽略时间戳更改
    let options = DiffOptions {
        ignore_keys_regex: Some(Regex::new(r"^timestamp$")?),
        show_unchanged: Some(false),
        ..Default::default()
    };
    
    let differences = diff(&old, &new, Some(&options))?;
    
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

## 高级用法

### 自定义比较逻辑

#### Epsilon 比较

处理浮点精度差异：

```rust
use diffx_core::{diff, DiffOptions};
use serde_json::json;

let old = json!({"pi": 3.14159});
let new = json!({"pi": 3.14160});

// 无 epsilon - 报告差异
let diffs_strict = diff(&old, &new, None)?;
assert!(!diffs_strict.is_empty());

// 有 epsilon - 无差异
let options = DiffOptions {
    epsilon: Some(0.001),
    ..Default::default()
};
let diffs_epsilon = diff(&old, &new, Some(&options))?;
assert!(diffs_epsilon.is_empty());
```

#### 正则表达式键过滤

忽略特定键或模式：

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

// 忽略时间戳和内部字段
let options = DiffOptions {
    ignore_keys_regex: Some(Regex::new(r"^(timestamp|_.*)")?),
    ..Default::default()
};
let differences = diff(&old, &new, Some(&options))?;

// 仅报告重要的数据更改
assert_eq!(differences.len(), 1);
```

#### 数组元素跟踪

通过 ID 而不是位置跟踪数组元素：

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
        {"id": 1, "name": "Alice Smith"}  // 名字已更改
    ]
});

// 使用 ID 跟踪 - 检测名字更改
let options = DiffOptions {
    array_id_key: Some("id".to_string()),
    ..Default::default()
};
let differences = diff(&old, &new, Some(&options))?;
// 报告：Modified users[id=1].name: "Alice" -> "Alice Smith"

// 不使用 ID 跟踪 - 由于位置原因报告所有更改
let differences_positional = diff(&old, &new, None)?;
// 由于位置差异报告多个更改
```

### 处理不同格式

#### 完整的格式处理管道

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
    
    // 用户应为其格式使用标准解析器
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
        _ => return Err(format!("不支持的格式：{}", format).into())
    };
    
    Ok(diff(&old, &new, options)?)
}
```

### 集成模式

#### 自定义差异处理

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
                    // 注意：TypeChanged 现在包含类型字符串，而不是值
                    self.type_changes.push((path, old_type.into(), new_type.into()));
                }
            }
        }
    }
    
    pub fn has_critical_changes(&self) -> bool {
        // 定义什么构成"关键"更改
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
        println!("文件对 {}: {} 个差异", i + 1, diffs.len());
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
    
    // 在后台任务中解析以避免阻塞
    let result = tokio::task::spawn_blocking(move || {
        let old: Value = serde_json::from_str(&content1)?;
        let new: Value = serde_json::from_str(&content2)?;
        diff(&old, &new, options.as_ref())
    }).await??;
    
    Ok(result)
}
```

## 错误处理

### 错误类型

该库使用 `anyhow::Error` 进行错误处理：

```rust
use diffx_core::{diff, DiffOptions};
use anyhow::Result;

fn handle_parse_errors() -> Result<()> {
    let invalid_data = "invalid json";
    
    match serde_json::from_str::<Value>(invalid_data) {
        Ok(value) => println!("解析成功：{}", value),
        Err(e) => {
            eprintln!("解析错误：{}", e);
            
            // 错误原因链
            let mut source = e.source();
            while let Some(err) = source {
                eprintln!("原因：{}", err);
                source = err.source();
            }
        }
    }
    
    Ok(())
}
```

### 常见错误场景

```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::Value;

// 处理多种格式可能性
fn robust_comparison(
    data1: &str,
    data2: &str,
    options: Option<&DiffOptions>
) -> Result<Vec<DiffResult>, Box<dyn std::error::Error>> {
    // 首先尝试解析为 JSON
    let old = serde_json::from_str::<Value>(data1)
        .or_else(|_| serde_yml::from_str::<Value>(data1))
        .or_else(|_| toml::from_str::<Value>(data1))?;
    
    let new = serde_json::from_str::<Value>(data2)
        .or_else(|_| serde_yml::from_str::<Value>(data2))
        .or_else(|_| toml::from_str::<Value>(data2))?;
    
    Ok(diff(&old, &new, options)?)
}
```

## 性能考虑

### 内存使用

对于大型数据集：

```rust
use diffx_core::{diff, DiffOptions, DiffResult};
use serde_json::Value;

// 高效处理大文件
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

### 优化提示

1. **使用正则表达式过滤** 来忽略大型无关部分
2. **指定 epsilon** 用于浮点数较多的数据
3. **使用数组 ID 键** 用于具有可识别元素的大型数组
4. **考虑路径过滤** 用于非常大的对象

## 测试

### 单元测试

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
        // 测试特定差异...
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

## 版本兼容性

- **0.2.x**：当前稳定版本
- **最低 Rust 版本**：1.70.0
- **依赖项**：请参阅 `Cargo.toml` 了解当前版本

## 另请参阅

- [CLI 参考](cli-reference_zh.md) - 命令行使用
- [入门指南](../user-guide/getting-started_zh.md) - 基本概念
- [示例](../user-guide/examples_zh.md) - 实际用例
- [统一API参考](../bindings/unified-api_zh.md) - 语言绑定
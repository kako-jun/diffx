# diffx 统一API参考

*diffx-python 和 diffx-js 语言绑定API文档*

## 概述

diffx 提供用于比较结构化数据文件（JSON、YAML、TOML、XML、INI、CSV）的统一API。该库专注于语义差异而非格式变化。

**统一API设计**: 核心API仅公开一个主函数 `diff()` 用于所有比较操作。所有功能都通过此统一接口使用选项参数访问。这种设计确保了所有用例的一致性和简洁性。

## 主函数

### `diff(old, new, options)`

比较两个结构化数据值并返回差异。

#### 参数

- `old` (Value): 要比较的原始/旧数据结构
- `new` (Value): 要比较的新/更新的数据结构
- `options` (DiffOptions, optional): 比较的配置选项

#### 返回值

- `Result<Vec<DiffResult>, Error>`: 在两个结构之间找到的差异向量

#### 示例

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

## 选项

### DiffOptions 结构体

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

pub struct DiffxSpecificOptions {
    pub context_lines: Option<usize>,
    pub ignore_whitespace: Option<bool>,
    pub ignore_case: Option<bool>,
    pub brief_mode: Option<bool>,
    pub quiet_mode: Option<bool>,
}
```

### 选项详情

#### 数值比较
- **`epsilon`**: 浮点数比较容差。在此epsilon内的值被认为相等。
  - 默认值: `0.0`（精确比较）
  - 示例: `0.001` 为 0.1% 容差

#### 数组比较
- **`array_id_key`**: 在比较数组时用于识别数组元素的键
  - 默认值: `None`（基于索引的比较）
  - 示例: `"id"` 通过 `id` 字段匹配数组元素

#### 过滤
- **`ignore_keys_regex`**: 在比较期间忽略键的正则表达式
  - 示例: `"^(timestamp|metadata)"` 忽略timestamp和metadata字段
- **`path_filter`**: 将比较限制为特定路径的JSONPath风格过滤器
  - 示例: `"$.users[*].name"` 仅比较用户名

#### 输出控制
- **`output_format`**: 输出格式
  - 选项: `Json`, `Yaml`, `Csv`, `Diffx`（自定义格式）
  - 默认值: `Json`
- **`show_unchanged`**: 在输出中包含未更改的值
  - 默认值: `false`
- **`show_types`**: 在输出中包含类型信息
  - 默认值: `false`

#### 内存优化
- **`use_memory_optimization`**: 为大文件启用内存高效处理
  - 默认值: `false`
- **`batch_size`**: 启用内存优化时每批处理的项目数
  - 默认值: `1000`

#### diffx特定选项 (DiffxSpecificOptions)
这些选项嵌套在 `diffx_options` 字段中：

- **`context_lines`**: 在统一差异格式中显示的上下文行数
  - 默认值: `3`
- **`ignore_whitespace`**: 在字符串比较中忽略空白差异
  - 默认值: `false`
- **`ignore_case`**: 不区分大小写的字符串比较
  - 默认值: `false`
- **`brief_mode`**: 仅显示文件是否不同，不显示差异
  - 默认值: `false`
- **`quiet_mode`**: 抑制所有正常输出
  - 默认值: `false`

## 结果类型

### DiffResult 枚举

```rust
pub enum DiffResult {
    Added(String, Value),
    Removed(String, Value),
    Modified(String, Value, Value),
    TypeChanged(String, String, String),
}
```

- **`Added(path, value)`**: 在给定路径添加了新字段/值
- **`Removed(path, value)`**: 从给定路径删除了字段/值
- **`Modified(path, old_value, new_value)`**: 在给定路径值发生了变化
- **`TypeChanged(path, old_type, new_type)`**: 在给定路径值的类型发生了变化

## 语言绑定

### Python

```python
import diffx_python

# 基本用法
results = diffx_python.diff(old_dict, new_dict)

# 带选项
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

// 基本用法 - 用户自己解析文件
const oldData = JSON.parse(fs.readFileSync('old.json', 'utf8'));
const newData = JSON.parse(fs.readFileSync('new.json', 'utf8'));
const results = await diff(oldData, newData);

// 带选项
const options: DiffOptions = {
    epsilon: 0.001,
    arrayIdKey: 'id',
    ignoreKeysRegex: '^(timestamp|metadata)',
    outputFormat: 'json',
    showUnchanged: false
};
const results = await diff(oldData, newData, options);
```

## 错误处理

库为以下情况提供详细错误：
- 解析错误（无效的JSON、YAML等）
- 文件I/O错误
- 无效的正则表达式
- 内存分配失败
- 无效选项

## 性能考虑

- 对于超过100MB的文件使用 `use_memory_optimization`
- 根据可用内存调整 `batch_size`
- 使用 `path_filter` 限制比较范围以获得更好的性能
- `ignore_keys_regex` 中的正则表达式可能影响大数据集的性能

## 示例

### 比较JSON文件

```rust
use diffx_core::{diff, DiffOptions};
use serde_json;

// 用户使用适当的库自己解析文件
let old_content = std::fs::read_to_string("old.json")?;
let new_content = std::fs::read_to_string("new.json")?;

let old: serde_json::Value = serde_json::from_str(&old_content)?;
let new: serde_json::Value = serde_json::from_str(&new_content)?;

let results = diff(&old, &new, None)?;
```

### 忽略时间戳

```rust
use regex::Regex;

let options = DiffOptions {
    ignore_keys_regex: Some(Regex::new("timestamp|updated_at|created_at")?),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```

### 按ID比较数组

```rust
let options = DiffOptions {
    array_id_key: Some("id".to_string()),
    ..Default::default()
};

let results = diff(&old, &new, Some(&options))?;
```

### 使用diffx特定选项

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
# diffx 格式规范

**diffx 格式**是专为结构化数据比较设计的人类可读语义差分表示格式。与传统的基于文本的差分格式不同，diffx 格式关注数据的含义和结构，而不是文本外观。

## 概述

diffx 格式解决了传统差分工具在处理结构化数据时的局限性：

- **语义聚焦**：显示逻辑变化，而非文本差异
- **格式无关**：在 JSON、YAML、TOML、XML、INI、CSV 之间提供一致的表示
- **基于路径**：完整的层次路径表示法，精确定位变化位置
- **类型感知**：区分值变化和类型变化
- **人类可读**：直观的符号和清晰的格式

## 规范

### 基本语法

diffx 格式使用四个主要符号表示变化类型：

```
+ path: value    # 添加
- path: value    # 删除
~ path: old -> new    # 修改
! path: old -> new    # 类型变化
```

### 路径表示法

路径对对象使用点标记法，对数组使用方括号标记法：

```
database.host                    # 对象属性
servers[0].port                  # 数组元素属性
config.users[2].permissions[1]   # 嵌套数组访问
```

### 值表示

值以类似 JSON 的标准表示形式显示：

```
+ enabled: true                  # 布尔值
+ port: 8080                     # 数字
+ name: "production"             # 字符串
+ tags: ["web", "api"]           # 数组
+ config: {"debug": false}       # 对象
+ value: null                    # 空值
```

### 变化类型示例

#### 添加
当新增键或数组元素时：

```
+ database.port: 5432
+ servers[2]: {"name": "web-03", "port": 8080}
+ features[0]: "authentication"
```

#### 删除
当删除键或数组元素时：

```
- cache.ttl: 3600
- servers[1]: {"name": "web-02", "port": 8080}
- features[2]: "legacy-api"
```

#### 修改
当值发生变化但保持相同类型时：

```
~ database.host: "localhost" -> "prod-db.example.com"
~ servers[0].port: 8080 -> 9090
~ config.debug: false -> true
```

#### 类型变化
当值改变类型时（修改的特殊情况）：

```
! port: "8080" -> 8080           # 字符串转数字
! enabled: "true" -> true        # 字符串转布尔值
! config: {} -> null             # 对象转空值
```

### 复杂示例

#### 嵌套对象变化
```
~ user.profile.settings.theme: "light" -> "dark"
+ user.profile.preferences.notifications: true
- user.profile.cache.lastLogin: "2024-01-01T00:00:00Z"
```

#### 数组修改
```
+ items[3]: "new-item"
- items[1]: "removed-item"
~ items[0].name: "old-name" -> "new-name"
```

#### 混合变化
```
+ database.port: 5432
~ database.host: "localhost" -> "prod-db.example.com"
- cache.enabled: true
! debug: "false" -> false
```

## 设计原则

### 1. 语义清晰度
diffx 格式优先考虑理解**发生了什么变化**，而不是**文本如何变化**：

- 显示 `database.port: 5432 -> 6432` 而不是逐行文本差异
- 按语义含义对相关变化进行分组
- 维护数据结构上下文

### 2. 格式独立性
相同的 diffx 格式输出在所有支持的数据格式中一致地表示变化：

- JSON、YAML、TOML、XML、INI、CSV 都产生统一的 diffx 格式输出
- 用户学习一种格式而不是特定格式的差分表示
- 工具可以处理 diffx 格式输出，不管源数据格式如何

### 3. 路径精确性
完整的路径表示法消除了变化位置的歧义：

- `config.database.connection.host` 而不是含糊的行号
- 数组索引清晰指定：`users[2].email`
- 嵌套变化保持完整上下文

### 4. 类型安全
显式的类型变化检测防止数据损坏：

- 区分 `"8080" -> 8080`（类型变化）和 `8080 -> 9090`（值变化）
- 帮助识别意外的类型转换
- 对 API 架构演进和配置管理至关重要

## 使用场景

### DevOps 和配置管理
```bash
# 基础设施配置比较
diffx infrastructure.json infrastructure.new.json
# 输出：
# ~ services.database.instance_type: "t3.micro" -> "t3.small"
# + services.cache.enabled: true
# - services.legacy.port: 3000
```

### API 架构演进
```bash
# OpenAPI 规范比较
diffx api-v1.yaml api-v2.yaml --path "paths"
# 输出：
# + /users.post.responses.201: {"description": "Created"}
# ~ /users/{id}.get.parameters[0].schema.type: "integer" -> "string"
```

### 数据管道验证
```bash
# ETL 输出验证
diffx expected_output.json actual_output.json --array-id-key "id"
# 输出：
# ~ records[id=123].status: "pending" -> "completed"
# + records[id=456]: {"status": "new", "timestamp": "2024-01-01T12:00:00Z"}
```

## 相对于传统差分的优势

| 传统差分 | diffx 格式 |
|----------|-----------|
| `- "port": 8080,`<br>`+ "port": 9090,` | `~ port: 8080 -> 9090` |
| 显示行变化 | 显示语义变化 |
| 格式相关的输出 | 所有格式一致 |
| 对格式敏感 | 忽略无关格式 |
| 无类型感知 | 显式类型变化检测 |
| 上下文贫乏 | 完整层次上下文 |

## 集成和工具

diffx 格式设计用于人类使用和机器处理：

### 人类使用
- 清晰、直观的符号
- 层次路径上下文
- 一致的格式化规则

### 机器处理
- 可预测的解析语法
- 结构化变化表示
- 工具友好的输出格式

### 命令行集成
```bash
# 生成 diffx 格式输出
diffx config.json config.new.json > changes.diffx

# 使用标准工具处理 diffx 格式
grep "^+" changes.diffx | wc -l    # 计算添加项
grep "database\." changes.diffx    # 查找数据库变化
```

## 未来扩展

diffx 格式规范可能会扩展以支持：

- **置信度等级**：指示检测到的变化的确定性
- **变化元数据**：包含时间戳、作者或变化原因
- **语义注释**：为技术变化添加业务上下文
- **差分压缩**：大型变化集的紧凑表示

## 采用和标准化

为了将 diffx 格式确立为行业标准：

1. **开放规范**：公开的版本化规范文档
2. **参考实现**：`diffx` 工具作为标准实现
3. **工具生态系统**：在编辑器、CI/CD 工具和分析软件中的支持
4. **社区反馈**：基于实际使用的迭代改进

目标是让"diffx 格式"在开发者生态系统中变得像"JSON 格式"或"YAML 格式"一样被认可和有用。

---

*本规范反映了 diffx 格式版本 1.0。如需最新更新和社区讨论，请参见 [diffx 项目仓库](https://github.com/kako-jun/diffx)。*
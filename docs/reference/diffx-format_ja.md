# diffx 形式仕様

**diffx形式**は、構造化データ比較専用に設計された、人間が読みやすいセマンティック差分表現です。従来のテキストベースの差分形式とは異なり、diffx形式はテキストの見た目ではなく、データの意味と構造に焦点を当てています。

## 概要

diffx形式は、構造化データを扱う際の従来の差分ツールの限界に対処します：

- **セマンティック重視**: テキストの違いではなく、論理的な変更を表示
- **形式非依存**: JSON、YAML、TOML、XML、INI、CSVで一貫した表現
- **パスベース**: 正確な変更位置のための完全な階層パス記法
- **型対応**: 値の変更と型の変更を区別
- **人間が読みやすい**: 直感的なシンボルと明確なフォーマット

## 仕様

### 基本構文

diffx形式は、変更タイプを示すために4つの主要シンボルを使用します：

```
+ path: value    # 追加
- path: value    # 削除  
~ path: old -> new    # 修正
! path: old -> new    # 型変更
```

### パス記法

パスは、オブジェクトにドット記法、配列にブラケット記法を使用します：

```
database.host                    # オブジェクトプロパティ
servers[0].port                  # 配列要素プロパティ
config.users[2].permissions[1]   # ネストした配列アクセス
```

### 値表現

値は正規のJSON風表現で表示されます：

```
+ enabled: true                  # ブール値
+ port: 8080                     # 数値
+ name: "production"             # 文字列
+ tags: ["web", "api"]           # 配列
+ config: {"debug": false}       # オブジェクト
+ value: null                    # null
```

### 変更タイプの例

#### 追加
新しいキーや配列要素が追加された場合：

```
+ database.port: 5432
+ servers[2]: {"name": "web-03", "port": 8080}
+ features[0]: "authentication"
```

#### 削除
キーや配列要素が削除された場合：

```
- cache.ttl: 3600
- servers[1]: {"name": "web-02", "port": 8080}
- features[2]: "legacy-api"
```

#### 修正
値が変更されるが同じ型を維持する場合：

```
~ database.host: "localhost" -> "prod-db.example.com"
~ servers[0].port: 8080 -> 9090
~ config.debug: false -> true
```

#### 型変更
値が型を変更する場合（修正の特別ケース）：

```
! port: "8080" -> 8080           # 文字列から数値
! enabled: "true" -> true        # 文字列からブール値
! config: {} -> null             # オブジェクトからnull
```

### 複雑な例

#### ネストしたオブジェクトの変更
```
~ user.profile.settings.theme: "light" -> "dark"
+ user.profile.preferences.notifications: true
- user.profile.cache.lastLogin: "2024-01-01T00:00:00Z"
```

#### 配列の修正
```
+ items[3]: "new-item"
- items[1]: "removed-item"
~ items[0].name: "old-name" -> "new-name"
```

#### 混合変更
```
+ database.port: 5432
~ database.host: "localhost" -> "prod-db.example.com"
- cache.enabled: true
! debug: "false" -> false
```

## 設計原則

### 1. セマンティックな明確さ
diffx形式は、**テキストがどう変更されたか**よりも**何が変更されたか**の理解を優先します：

- 行単位のテキスト差分の代わりに `database.port: 5432 -> 6432` を表示
- セマンティックな意味によって関連する変更をグループ化
- データ構造のコンテキストを維持

### 2. 形式独立性
同じdiffx形式出力が、サポートされるすべてのデータ形式で一貫して変更を表現します：

- JSON、YAML、TOML、XML、INI、CSVがすべて統一されたdiffx形式出力を生成
- ユーザーは形式固有の差分表現ではなく、1つの形式を学習
- ツールはソースデータ形式に関係なくdiffx形式出力を処理可能

### 3. パスの精度
完全なパス記法により、変更場所の曖昧さを排除：

- 曖昧な行番号ではなく `config.database.connection.host`
- 配列インデックスを明確に指定: `users[2].email`
- ネストした変更が完全なコンテキストを維持

### 4. 型安全性
明示的な型変更検出によりデータ破損を防止：

- `"8080" -> 8080`（型変更）と `8080 -> 9090`（値変更）を区別
- 意図しない型変換の識別を支援
- APIスキーマ進化と設定管理にとって重要

## 使用例

### DevOpsと設定管理
```bash
# インフラストラクチャ設定比較
diffx infrastructure.json infrastructure.new.json
# 出力:
# ~ services.database.instance_type: "t3.micro" -> "t3.small"
# + services.cache.enabled: true
# - services.legacy.port: 3000
```

### APIスキーマ進化
```bash
# OpenAPI仕様比較  
diffx api-v1.yaml api-v2.yaml --path "paths"
# 出力:
# + /users.post.responses.201: {"description": "Created"}
# ~ /users/{id}.get.parameters[0].schema.type: "integer" -> "string"
```

### データパイプライン検証
```bash
# ETL出力検証
diffx expected_output.json actual_output.json --array-id-key "id"
# 出力:
# ~ records[id=123].status: "pending" -> "completed"
# + records[id=456]: {"status": "new", "timestamp": "2024-01-01T12:00:00Z"}
```

## 従来のDiffに対する利点

| 従来のDiff | diffx形式 |
|------------------|--------------|
| `- "port": 8080,`<br>`+ "port": 9090,` | `~ port: 8080 -> 9090` |
| 行の変更を表示 | セマンティックな変更を表示 |
| 形式依存出力 | すべての形式で一貫 |
| フォーマッティングに敏感 | 無関係なフォーマッティングを無視 |
| 型の認識なし | 明示的な型変更検出 |
| コンテキストが乏しい | 完全な階層コンテキスト |

## 統合とツール

diffx形式は、人間の消費と機械処理の両方のために設計されています：

### 人間による消費
- 明確で直感的なシンボル
- 階層パスコンテキスト
- 一貫したフォーマットルール

### 機械処理  
- 解析のための予測可能な構文
- 構造化された変更表現
- ツールフレンドリーな出力形式

### コマンドライン統合
```bash
# diffx形式出力を生成
diffx config.json config.new.json > changes.diffx

# 標準ツールでdiffx形式を処理
grep "^+" changes.diffx | wc -l    # 追加をカウント
grep "database\." changes.diffx    # データベース変更を検索
```

## 将来の拡張

diffx形式仕様は以下をサポートするために拡張される可能性があります：

- **信頼度レベル**: 検出された変更の確実性を示す
- **変更メタデータ**: タイムスタンプ、作成者、変更理由を含む
- **セマンティック注釈**: 技術的変更にビジネスコンテキストを追加
- **差分圧縮**: 大きな変更セットのコンパクト表現

## 採用と標準化

diffx形式を業界標準として確立するために：

1. **オープン仕様**: 公開されたバージョン管理された仕様ドキュメント
2. **リファレンス実装**: 正規実装としての `diffx` ツール
3. **ツールエコシステム**: エディタ、CI/CDツール、分析ソフトウェアでのサポート
4. **コミュニティフィードバック**: 実世界の使用に基づく反復改善

目標は、「diffx形式」が開発者エコシステムで「JSON形式」や「YAML形式」と同じくらい認識され有用になることです。

---

*この仕様はdiffx形式バージョン1.0を反映しています。最新の更新とコミュニティディスカッションについては、[diffxプロジェクトリポジトリ](https://github.com/kako-jun/diffx)を参照してください。*
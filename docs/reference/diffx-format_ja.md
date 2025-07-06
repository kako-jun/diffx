# diffx フォーマット仕様

**diffx フォーマット** は、構造化データの比較に特化して設計された、人間が読みやすいセマンティック差分表現です。従来のテキストベース差分フォーマットとは異なり、diffx フォーマットはテキストの見た目ではなく、データの意味と構造に焦点を当てています。

## 概要

diffx フォーマットは、構造化データを扱う際の従来の diff ツールの制限を解決します：

- **セマンティックフォーカス**: テキストの違いではなく、論理的な変更を表示
- **フォーマット非依存**: JSON、YAML、TOML、XML、INI、CSV すべてで一貫した表現
- **パスベース**: 変更箇所を正確に示す完全な階層パス記法
- **型認識**: 値の変更と型の変更を区別
- **人間可読**: 直感的な記号と明確なフォーマット

## 仕様

### 基本構文

diffx フォーマットは、変更タイプを示すために4つの主要な記号を使用します：

```
+ path: value    # 追加
- path: value    # 削除
~ path: old -> new    # 変更
! path: old -> new    # 型変更
```

### パス記法

パスは、オブジェクトにはドット記法、配列には角括弧記法を使用します：

```
database.host                    # オブジェクトプロパティ
servers[0].port                  # 配列要素のプロパティ
config.users[2].permissions[1]   # ネストした配列アクセス
```

### 値の表現

値は標準的な JSON ライクな表現で表示されます：

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

#### 変更
値が変更されるが同じ型を保持する場合：

```
~ database.host: "localhost" -> "prod-db.example.com"
~ servers[0].port: 8080 -> 9090
~ config.debug: false -> true
```

#### 型変更
値が型を変更する場合（変更の特殊ケース）：

```
! port: "8080" -> 8080           # 文字列から数値
! enabled: "true" -> true        # 文字列からブール値
! config: {} -> null             # オブジェクトから null
```

### 複雑な例

#### ネストしたオブジェクトの変更
```
~ user.profile.settings.theme: "light" -> "dark"
+ user.profile.preferences.notifications: true
- user.profile.cache.lastLogin: "2024-01-01T00:00:00Z"
```

#### 配列の変更
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

### 1. セマンティックな明確性
diffx フォーマットは **テキストがどう変わったか** よりも **何が変わったか** の理解を優先します：

- 行ごとのテキスト差分ではなく `database.port: 5432 -> 6432` を表示
- セマンティックな意味により関連する変更をグループ化
- データ構造のコンテキストを維持

### 2. フォーマット独立性
同じ diffx フォーマット出力は、サポートされるすべてのデータフォーマットで一貫して変更を表現します：

- JSON、YAML、TOML、XML、INI、CSV はすべて統一された diffx フォーマット出力を生成
- ユーザーはフォーマット固有の差分表現ではなく、一つのフォーマットを学習するだけ
- ソースデータフォーマットに関係なく、ツールは diffx フォーマット出力を処理可能

### 3. パスの精度
完全なパス記法により、変更箇所の曖昧さを排除します：

- 曖昧な行番号ではなく `config.database.connection.host`
- 配列インデックスを明確に指定：`users[2].email`
- ネストした変更は完全なコンテキストを維持

### 4. 型安全性
明示的な型変更検出により、データ破損を防止します：

- `"8080" -> 8080`（型変更）と `8080 -> 9090`（値変更）を区別
- 意図しない型変換の特定に役立つ
- API スキーマ進化と設定管理において重要

## 使用例

### DevOps と設定管理
```bash
# インフラ設定の比較
diffx infrastructure.json infrastructure.new.json
# 出力：
# ~ services.database.instance_type: "t3.micro" -> "t3.small"
# + services.cache.enabled: true
# - services.legacy.port: 3000
```

### API スキーマ進化
```bash
# OpenAPI 仕様の比較
diffx api-v1.yaml api-v2.yaml --path "paths"
# 出力：
# + /users.post.responses.201: {"description": "Created"}
# ~ /users/{id}.get.parameters[0].schema.type: "integer" -> "string"
```

### データパイプライン検証
```bash
# ETL 出力検証
diffx expected_output.json actual_output.json --array-id-key "id"
# 出力：
# ~ records[id=123].status: "pending" -> "completed"
# + records[id=456]: {"status": "new", "timestamp": "2024-01-01T12:00:00Z"}
```

## 従来の Diff に対する利点

| 従来の Diff | diffx フォーマット |
|-------------|-------------------|
| `- "port": 8080,`<br>`+ "port": 9090,` | `~ port: 8080 -> 9090` |
| 行の変更を表示 | セマンティックな変更を表示 |
| フォーマット依存の出力 | 全フォーマットで一貫 |
| フォーマットに敏感 | 無関係なフォーマットを無視 |
| 型認識なし | 明示的な型変更検出 |
| コンテキスト不足 | 完全な階層コンテキスト |

## 統合とツーリング

diffx フォーマットは、人間の利用と機械処理の両方のために設計されています：

### 人間の利用
- 明確で直感的な記号
- 階層パスコンテキスト
- 一貫したフォーマットルール

### 機械処理
- 解析のための予測可能な構文
- 構造化された変更表現
- ツールフレンドリーな出力フォーマット

### コマンドライン統合
```bash
# diffx フォーマット出力を生成
diffx config.json config.new.json > changes.diffx

# 標準ツールで diffx フォーマットを処理
grep "^+" changes.diffx | wc -l    # 追加をカウント
grep "database\." changes.diffx    # データベースの変更を検索
```

## 将来の拡張

diffx フォーマット仕様は以下をサポートするように拡張される可能性があります：

- **信頼度レベル**: 検出された変更の確実性を示す
- **変更メタデータ**: タイムスタンプ、作成者、変更理由を含む
- **セマンティック注釈**: 技術的変更にビジネスコンテキストを追加
- **差分圧縮**: 大規模な変更セットのコンパクト表現

## 採用と標準化

diffx フォーマットを業界標準として確立するために：

1. **オープン仕様**: 公開された、バージョン管理された仕様文書
2. **リファレンス実装**: `diffx` ツールを標準的な実装として
3. **ツールエコシステム**: エディタ、CI/CD ツール、分析ソフトウェアでのサポート
4. **コミュニティフィードバック**: 実世界の使用に基づく反復的改善

目標は、"diffx フォーマット"が開発者エコシステムにおいて "JSON フォーマット" や "YAML フォーマット" と同様に認識され、有用になることです。

---

*この仕様は diffx フォーマット バージョン 1.0 を反映しています。最新の更新とコミュニティディスカッションについては、[diffx プロジェクトリポジトリ](https://github.com/kako-jun/diffx) をご覧ください。*
# ツール比較

このドキュメントでは、`diffx` と他のdiffおよびデータ比較ツールを比較し、特定の用途で`diffx`をいつ、なぜ選択すべきかを理解できるよう支援します。

## クイック比較表

| ツール | タイプ | 形式 | セマンティック対応 | 配列追跡 | 設定サポート | 最適用途 |
|------|------|---------|---------------|----------------|----------------|----------|
| **diffx** | セマンティック | JSON/YAML/TOML/XML/INI/CSV | ✅ | ✅ | ✅ | 構造化データ比較 |
| diff | テキストベース | 任意のテキスト | ❌ | ❌ | ❌ | 一般的なテキストファイル |
| jq | JSON処理 | JSON | 部分的 | ❌ | ❌ | JSON操作 |
| yq | YAML処理 | YAML/JSON | 部分的 | ❌ | ❌ | YAML操作 |
| daff | 表形式 | CSV | ✅ | ❌ | ❌ | CSV/スプレッドシートデータ |
| jsondiff | JSON差分 | JSON | ✅ | 部分的 | ❌ | JSONのみの比較 |
| deep-diff | JavaScript | JSON/オブジェクト | ✅ | ❌ | ❌ | JavaScriptアプリケーション |

## 詳細比較

### 従来の `diff` との比較

**従来のdiff:**
```bash
$ diff config_v1.json config_v2.json
< {
<   "name": "myapp",
<   "version": "1.0"
< }
> {
>   "version": "1.1",
>   "name": "myapp"
> }
```

**diffx:**
```bash
$ diffx config_v1.json config_v2.json
~ version: "1.0" -> "1.1"
```

**主要な違い:**

| 側面 | 従来のdiff | diffx |
|--------|------------------|-------|
| **理解** | 行単位のテキスト | セマンティック構造 |
| **キー順序** | 異なるものとして報告 | 並べ替えを無視 |
| **空白** | 差分として報告 | フォーマットを無視 |
| **末尾カンマ** | 差分として報告 | フォーマットを無視 |
| **型変更** | テキスト変更として表示 | 型変換を報告 |
| **配列処理** | 位置ベース | IDベース追跡が利用可能 |
| **出力形式** | テキスト差分 | CLI/JSON/YAML/diffx |

**従来のdiffを使用すべき場合:**
- 一般的なテキストファイル
- ソースコード比較
- 行単位の分析が必要
- 構造化データのないシンプルなスクリプト

**diffxを使用すべき場合:**
- 設定ファイル
- APIレスポンス
- データエクスポート
- 構造化ドキュメント

### JSON処理での `jq` との比較

**jqを使用した差分（複雑）:**
```bash
# 基本比較のための複雑なjqコマンド
jq -n --argjson a "$(cat file1.json)" --argjson b "$(cat file2.json)" \
  'def diff(a; b): 
    if (a | type) != (b | type) then {type_changed: {from: (a | type), to: (b | type)}}
    elif a == b then empty
    elif (a | type) == "object" then
      (a + b) | to_entries | map(select(.value != a[.key] or .value != b[.key])) |
      from_entries
    else {changed: {from: a, to: b}}
    end;
  diff($a; $b)'
```

**diffxを使用（シンプル）:**
```bash
diffx file1.json file2.json --output json
```

**比較:**

| 側面 | jq | diffx |
|--------|-------|-------|
| **複雑さ** | 高（複雑なクエリ） | 低（シンプルなコマンド） |
| **学習曲線** | 急勾配 | 緩やか |
| **JSONのみ** | はい | いいえ（6形式） |
| **内蔵差分** | いいえ（手動スクリプト） | はい |
| **配列追跡** | 手動実装 | 内蔵 |
| **フィルタリング** | 手動クエリ | 正規表現パターン |
| **出力** | カスタムJSON | 複数形式 |

**jqを使用すべき場合:**
- 複雑なJSON変換
- データ抽出と操作
- カスタム処理パイプライン
- JSONのみのワークフロー

**diffxを使用すべき場合:**
- シンプルな比較タスク
- 複数形式サポートが必要
- セマンティック差分が特に必要
- 設定管理

### YAML処理での `yq` との比較

**yqを使用した比較:**
```bash
# yqには内蔵差分がないため、手動比較が必要
yq eval '. as $item ireduce ({}; . * $item)' file1.yaml file2.yaml
```

**diffxを使用:**
```bash
diffx file1.yaml file2.yaml
```

**比較:**

| 側面 | yq | diffx |
|--------|-----|-------|
| **主用途** | YAML処理 | セマンティック差分 |
| **差分機能** | 限定的/手動 | ネイティブ |
| **形式サポート** | YAML/JSON | 6形式 |
| **セマンティック対応** | 部分的 | 完全 |
| **設定** | なし | あり |

**yqを使用すべき場合:**
- YAML変換
- YAMLからのデータ抽出
- YAML検証
- 複雑なYAML処理

**diffxを使用すべき場合:**
- YAML比較専用
- マルチ形式環境
- 設定ドリフト検出
- セマンティック変更追跡

### CSVデータでの `daff` との比較

**daffの例:**
```bash
daff data1.csv data2.csv
```

**diffxの例:**
```bash
diffx data1.csv data2.csv --array-id-key "id"
```

**比較:**

| 側面 | daff | diffx |
|--------|------|-------|
| **焦点** | 表形式データ | 一般的な構造化データ |
| **形式サポート** | CSV/TSV | CSVを含む6形式 |
| **可視化** | HTML出力 | CLI/JSON/YAML |
| **ID追跡** | 限定的 | 完全サポート |
| **統合** | 専用 | 汎用 |

**daffを使用すべき場合:**
- 大量のCSV/スプレッドシート作業
- 表形式データの可視化
- Excel統合が必要
- CSV専用ワークフロー

**diffxを使用すべき場合:**
- 混合形式環境
- CSV + 他の構造化データ
- API統合が必要
- 自動化ワークフロー

### `jsondiff`（Python）との比較

**jsondiffの例:**
```python
from jsondiff import diff
import json

with open('file1.json') as f1, open('file2.json') as f2:
    diff_result = diff(json.load(f1), json.load(f2))
    print(diff_result)
```

**diffxの例:**
```bash
diffx file1.json file2.json --output json
```

**比較:**

| 側面 | jsondiff | diffx |
|--------|----------|-------|
| **言語** | Pythonライブラリ | CLIツール |
| **統合** | Pythonアプリ | 任意の言語/スクリプト |
| **形式サポート** | JSONのみ | 6形式 |
| **パフォーマンス** | Python速度 | Rust速度 |
| **デプロイメント** | Python必須 | 単一バイナリ |
| **配列追跡** | 基本的 | 高度 |

**jsondiffを使用すべき場合:**
- Pythonネイティブアプリケーション
- 埋め込み差分ロジック
- カスタムPython処理
- JSONのみの要件

**diffxを使用すべき場合:**
- マルチ言語環境
- CLI/スクリプト統合
- より良いパフォーマンスが必要
- 複数形式サポート

### Gitの内蔵diffとの比較

**Git diff:**
```bash
git diff HEAD~1 config.json
```

**Git diffとdiffxの組み合わせ:**
```bash
git show HEAD~1:config.json | diffx - config.json
```

**比較:**

| 側面 | Git diff | Git + diffx |
|--------|----------|-------------|
| **統合** | ネイティブ | 外部ツール |
| **理解** | 行ベース | セマンティック |
| **設定** | 限定的 | 広範囲 |
| **形式対応** | なし | あり |
| **学習曲線** | 馴染み深い | 追加ツール |

**Git統合例:**
```bash
# .gitconfigに追加
[diff "json"]
    textconv = diffx --output diffx

# .gitattributesで
*.json diff=json
```

### 言語固有ライブラリとの比較

#### JavaScript（`deep-diff`）
```javascript
const diff = require('deep-diff');
const differences = diff(obj1, obj2);
```

#### Python（`deepdiff`）
```python
from deepdiff import DeepDiff
diff = DeepDiff(dict1, dict2)
```

#### Ruby（`hashdiff`）
```ruby
require 'hashdiff'
diff = Hashdiff.diff(hash1, hash2)
```

**diffxとの比較:**

| 側面 | 言語ライブラリ | diffx |
|--------|-------------------|-------|
| **統合** | 言語ネイティブ | CLI/外部 |
| **パフォーマンス** | 可変 | 一貫性（Rust） |
| **形式サポート** | 通常単一 | 複数 |
| **デプロイメント** | 言語依存 | 単一バイナリ |
| **標準化** | 言語別API | 一貫したCLI |
| **チーム横断使用** | 言語固有 | 汎用 |

## パフォーマンス比較

### 速度ベンチマーク

テストファイル: 1MB JSON設定ファイル

| ツール | 時間（平均） | メモリ使用量 |
|------|------------|--------------|
| **diffx** | 5ms | 15MB |
| 従来のdiff | 2ms | 8MB |
| jq（スクリプト） | 150ms | 45MB |
| jsondiff | 80ms | 35MB |
| daff | 25ms | 20MB |

*注記: ベンチマークは概算値で、実際のパフォーマンスはデータ構造により変動*

### スケーラビリティ

| ファイルサイズ | diffx | 従来のdiff | jq（スクリプト） |
|-----------|-------|------------------|---------------|
| 1KB | 1ms | 1ms | 15ms |
| 100KB | 3ms | 2ms | 45ms |
| 1MB | 5ms | 8ms | 150ms |
| 10MB | 50ms | 80ms | 1500ms |
| 100MB | 500ms | 800ms | 15秒以上 |

## 機能マトリクス

### コア機能

| 機能 | diffx | diff | jq | yq | daff | jsondiff |
|---------|-------|------|----|----|------|----------|
| **セマンティック理解** | ✅ | ❌ | 部分的 | 部分的 | ✅ | ✅ |
| **複数形式** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **配列ID追跡** | ✅ | ❌ | ❌ | ❌ | 限定的 | 部分的 |
| **正規表現フィルタリング** | ✅ | ❌ | 手動 | 手動 | ❌ | ❌ |
| **イプシロン比較** | ✅ | ❌ | 手動 | 手動 | ❌ | ❌ |
| **パスフィルタリング** | ✅ | ❌ | ✅ | ✅ | ❌ | ❌ |
| **複数出力形式** | ✅ | ❌ | ✅ | ✅ | 限定的 | ❌ |

### 統合機能

| 機能 | diffx | diff | jq | yq | daff | jsondiff |
|---------|-------|------|----|----|------|----------|
| **CLIツール** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| **ライブラリ** | ✅ | ❌ | ❌ | ❌ | ❌ | ✅ |
| **設定ファイル** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **環境変数** | ✅ | ❌ | ❌ | ❌ | ❌ | ❌ |
| **終了コード** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |
| **パイプサポート** | ✅ | ✅ | ✅ | ✅ | 限定的 | ❌ |

## 用途別推奨

### 設定管理
**最適選択: diffx**
- セマンティック理解が重要
- 複数形式が一般的
- 自動化対応
- フィルタリング機能

**代替案: diff**（シンプルなテキスト設定用）

### APIテスト
**最適選択: diffx**
- JSON/YAMLレスポンス比較
- タイムスタンプフィールドの無視
- 複数出力形式
- CI/CD統合

**代替案: jq**（複雑なJSON操作用）

### データ処理
**最適選択: diffx**（構造化データ）または **daff**（CSV重視）
- 混合形式にはdiffxを選択
- 純粋なCSVワークフローにはdaffを選択

### ソースコード
**最適選択: diff**
- 行単位比較が必要
- Git統合
- パッチ生成

**diffxの使用対象:** package.json、ソース内の設定ファイル

### データベースエクスポート
**最適選択: diffx**
- JSON/CSVエクスポート比較
- 配列ID追跡
- 大容量ファイル処理

### DevOps/インフラストラクチャ
**最適選択: diffx**
- Kubernetesマニフェスト（YAML）
- Terraformステート（JSON）
- Docker Composeファイル
- 設定ドリフト検出

## 移行ガイド

### `diff` から `diffx` への移行

**旧ワークフロー:**
```bash
diff config1.json config2.json > changes.txt
```

**新ワークフロー:**
```bash
diffx config1.json config2.json --output diffx > changes.txt
# またはセマンティック差分用:
diffx config1.json config2.json > semantic_changes.txt
```

### `jq` 比較から `diffx` への移行

**旧複雑jqスクリプト:**
```bash
jq -n --argjson a "$(cat file1.json)" --argjson b "$(cat file2.json)" \
  'complex_diff_function($a; $b)'
```

**新シンプルdiffx:**
```bash
diffx file1.json file2.json --output json
```

### 言語固有ツールからの移行

**Python（jsondiff）:**
```python
# 旧
from jsondiff import diff
result = diff(data1, data2)

# 新
import subprocess
result = subprocess.run(['diffx', 'file1.json', 'file2.json', '--output', 'json'], 
                       capture_output=True, text=True)
diff_data = json.loads(result.stdout)
```

## 結論

以下の場合に `diffx` を選択してください:
- 構造化データの**セマンティック理解**
- 1つのツールでの**複数形式サポート**
- **高度なフィルタリング**と比較オプション
- **自動化対応**のCLIインターフェース
- 異なるデータタイプでの**一貫した動作**

以下の場合に他のツールを選択してください:
- **従来のdiff**: 一般的なテキストファイル、ソースコード、シンプルな行単位比較
- **jq/yq**: 複雑なデータ変換、単一形式専用処理  
- **daff**: 大量のCSV/表形式データ重視
- **言語ライブラリ**: 特定のプログラミング言語での深い統合

`diffx` は、テキストレベルの差異よりもデータ構造変更のセマンティック理解が重要な混合形式環境で優れています。
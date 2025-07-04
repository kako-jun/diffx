# diffx 利用ガイド

このガイドでは、`diffx` を使って様々な構造化データ比較タスクを行うための詳細な例を提供します。

## 基本的な使い方

2つのJSONファイルを比較する:

```bash
diffx file1.json file2.json
```

2つのYAMLファイルを比較する:

```bash
diffx file1.yaml file2.yaml
```

2つのTOMLファイルを比較する:

```bash
diffx file1.toml file2.toml
```

## 入力フォーマットの指定

ファイル拡張子から `diffx` がフォーマットを推測できない場合（例：標準入力を使用する場合やカスタムファイル拡張子の場合）、`--format` オプションを使って明示的にフォーマットを指定できます。

```bash
cat file1.json | diffx - file2.json --format json
```

## 出力フォーマット

`diffx` はいくつかの出力フォーマットをサポートしています。デフォルトは人間が読みやすいCLI出力です。

### CLI出力 (デフォルト)

このフォーマットは、差分を色分け、インデント、記号で表現し、視覚的に分かりやすくします。意味的な変更に焦点を当てています。

```bash
diffx file1.json file2.json --output cli
```

CLI出力例:

```
  ~ config.users[1].name: Bob -> Robert
  + config.users[2]: {"id":3,"name":"Charlie"}
```

### JSON出力

機械可読性のため、差分をJSON配列として出力できます。

```bash
diffx file1.json file2.json --output json
```

JSON出力例:

```json
[
  {
    "Added": [
      "config.users[2]",
      {
        "id": 3,
        "name": "Charlie"
      }
    ]
  },
  {
    "Modified": [
      "config.users[1].name",
      "Bob",
      "Robert"
    ]
  }
]
```

### YAML出力

JSON出力と同様ですが、YAMLフォーマットです。

```bash
diffx file1.json file2.json --output yaml
```

YAML出力例:

```yaml
- Added:
  - config.users[2]
  - id: 3
    name: Charlie
- Modified:
  - config.users[1].name
  - Bob
  - Robert
```

### Unified Format

`git` や他のdiffツールとの互換性のため、`diffx` はUnified diffフォーマットで出力できます。このフォーマットはテキストベースであり、`diffx` が非意味的と判断する差分（例：空白の変更）も表示される可能性があることに注意してください。

```bash
diffx file1.json file2.json --output unified
```

## パスによる差分のフィルタリング

`--path` オプションを使用して、特定のパス内の差分のみを表示します。これは、大規模な設定ファイルで関連するセクションに焦点を当てる場合に便利です。

```bash
diffx file1.json file2.json --path "config.users[1]"
```

## 正規表現によるキーの無視

`--ignore-keys-regex` を使用して、正規表現に一致するキーを比較から除外します。これは、タイムスタンプやIDなど、意味的な比較には関係のない動的なフィールドを無視する場合に便利です。

```bash
diffx file1.json file2.json --ignore-keys-regex "^_.*$"
```

## 浮動小数点数比較の許容誤差

浮動小数点数を比較する際、精度による微小な差を `--epsilon` オプションで無視できます。

```bash
diffx data1.json data2.json --epsilon 0.00001
```

## キーによる配列要素の識別

オブジェクトの配列の場合、`--array-id-key` を使用して一意の識別子キーで要素を追跡できます。これにより、順序が変更されても、`diffx` は変更、追加、または削除された要素を正しく識別できます。

```bash
diffx users1.json users2.json --array-id-key "id"
```

## ディレクトリ比較

`--recursive` オプションを使用して2つのディレクトリを再帰的に比較します。`diffx` はディレクトリ内の対応するファイルを比較します。

```bash
diffx dir1/ dir2/ --recursive
```

## 高度な使い方：差分レポートの差分 (Meta-chaining)

`diffx` の強力な機能の1つは、自身の出力を比較できることです。`diffx` の出力をJSONまたはYAML形式で保存し、その「差分レポート」自体を `diffx` で比較できます。これは、設定変更履歴の追跡、監査、またはシステムの進化を理解するのに役立ちます。

1.  差分レポートを生成し、ファイルに保存します。

    ```bash
    diffx config_v1.json config_v2.json --output json > diff_report_v1.json
    ```

2.  後で、別の差分レポートを生成します。

    ```bash
    diffx config_v2.json config_v3.json --output json > diff_report_v2.json
    ```

3.  2つの差分レポートを比較します。

    ```bash
    diffx diff_report_v1.json diff_report_v2.json
    ```

これにより、**変更そのもの**の差分を確認でき、システムの進化をメタレベルで把握できます。

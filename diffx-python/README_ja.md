# diffx

`diffx` CLIツールのPythonラッパー - 構造化データのセマンティック差分ツール

## インストール

```bash
pip install diffx-python
```

これにより、GitHub Releasesからお使いのシステムに適した `diffx` バイナリが自動的にダウンロードされます。

## 使い方

### モダンAPI（推奨）

```python
import diffx

# 2つのJSONファイルを比較
result = diffx.diff('file1.json', 'file2.json')
print(result)

# 構造化出力をJSONとして取得
json_result = diffx.diff(
    'config1.yaml', 
    'config2.yaml',
    diffx.DiffOptions(format='yaml', output='json')
)

for diff_item in json_result:
    if diff_item.added:
        print(f"追加: {diff_item.added}")
    elif diff_item.modified:
        print(f"変更: {diff_item.modified}")

# ディレクトリツリーを比較
dir_result = diffx.diff(
    'dir1/', 
    'dir2/',
    diffx.DiffOptions(recursive=True, path='config')
)

# 文字列を直接比較
json1 = '{"name": "Alice", "age": 30}'
json2 = '{"name": "Alice", "age": 31}'
string_result = diffx.diff_string(
    json1, json2, 'json',
    diffx.DiffOptions(output='json')
)
```

### レガシーAPI（後方互換性）

```python
from diffx import run_diffx

# 2つのJSONファイルを比較（レガシー）
result = run_diffx(["file1.json", "file2.json"])

if result.returncode == 0:
    print("違いはありません。")
else:
    print("違いが見つかりました：")
    print(result.stdout)
```

## 機能

- **複数フォーマット**: JSON、YAML、TOML、XML、INI、CSV
- **スマート差分**: テキストではなく構造を理解
- **柔軟な出力**: CLI、JSON、YAML、unified diff フォーマット
- **高度なオプション**: 
  - 正規表現ベースのキーフィルタリング
  - 浮動小数点許容誤差
  - 配列要素識別
  - パスベースフィルタリング
- **クロスプラットフォーム**: 適切なバイナリを自動ダウンロード

## 開発

uv を使用して開発モードでインストールするには：

```bash
uv venv
source .venv/bin/activate
uv pip install -e .[dev]
```

## 手動バイナリインストール

自動ダウンロードが失敗した場合：

```bash
diffx-download-binary
```

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。
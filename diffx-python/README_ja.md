# diffx

`diffx` CLIツールのPythonラッパー - 構造化データのセマンティック差分ツール

## インストール

```bash
pip install diffx-python
```

`diffx` バイナリはwheelに自動的に含まれているため、追加のダウンロードは不要です！このパッケージは [maturin](https://github.com/PyO3/maturin) を使用してネイティブバイナリをPython wheelに直接埋め込んでおり、`ruff` などのツールと同様の仕組みです。

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
- **クロスプラットフォーム**: プラットフォーム固有のwheelにネイティブバイナリを埋め込み

## 主な利点

- **🚀 ゼロセットアップ**: 外部ダウンロードやバイナリ管理が不要
- **📦 完全自己完結**: 必要なものはすべてwheelに含まれています
- **⚡ 高速インストール**: `pip install` 後のネットワーク依存なし
- **🔒 セキュア**: 外部ソースからの実行時ダウンロードなし
- **🌐 オフライン対応**: エアギャップ環境でも動作

## 開発

開発モードでインストールするには：

```bash
pip install -e .[dev]
```

## 動作確認

インストールを確認：

```python
import diffx
print("diffx 利用可能:", diffx.is_diffx_available())
print("バージョン:", diffx.__version__)
```

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。
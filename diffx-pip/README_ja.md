# diffx-python

`diffx` CLIツールのPythonラッパー

## インストール

```bash
pip install diffx-py
```

これにより、GitHub Releasesからお使いのシステムに適した `diffx` バイナリが自動的にダウンロードされます。

## 使い方

```python
from diffx_python import run_diffx

# 2つのJSONファイルを比較
result = run_diffx(["file1.json", "file2.json"])

if result.returncode == 0:
    print("違いはありません。")
else:
    print("違いが見つかりました：")
    print(result.stdout)

# diffx CLIでサポートされている任意の引数を渡すことができます
result = run_diffx(["file1.yaml", "file2.yaml", "--output", "json"])
print(result.stdout)
```

## 開発

開発用に編集可能モードでインストールするには：

```bash
pip install -e .
```

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。
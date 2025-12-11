# diffx

[English](README.md)

[![CI](https://github.com/kako-jun/diffx/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/diffx/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/diffx.svg)](https://crates.io/crates/diffx)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

構造化データ（JSON/YAML/TOML/XML/INI/CSV）の意味的差分ツール。キー順序や空白を無視し、本質的な変更のみを表示。

## なぜ diffx？

従来の `diff` は構造を理解しません：

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

キーの順序が変わっただけで全行が差分として表示されます。

`diffx` は意味的な変更だけを表示：

```bash
$ diffx config_v1.json config_v2.json
~ version: "1.0" -> "1.1"
```

## インストール

```bash
# CLIツールとして
cargo install diffx

# ライブラリとして（Cargo.toml）
[dependencies]
diffx-core = "0.6"
```

## 使い方

```bash
# 基本
diffx file1.json file2.json

# 出力例
~ version: "1.0" -> "1.1"
+ features[0]: "new-feature"
- deprecated: "old-value"
```

## 対応フォーマット

JSON, YAML, TOML, XML, INI, CSV（拡張子で自動判定、`--format` で明示指定可）

## 主なオプション

```bash
--output json|yaml       # 機械可読な出力
--quiet                  # 差分有無を終了コードのみで返す（0:同じ, 1:差分あり）
--ignore-keys-regex RE   # 正規表現にマッチするキーを無視
--array-id-key KEY       # 配列要素をKEYで識別して比較
--epsilon N              # 浮動小数点の許容誤差
--ignore-case            # 大文字小文字を無視
--ignore-whitespace      # 空白の違いを無視
-r, --recursive          # ディレクトリを再帰比較
```

## 出力記号

- `+` 追加
- `-` 削除
- `~` 変更
- `!` 型変更

## CI/CDでの活用

```bash
# 設定ファイルの変更検知
if ! diffx config/prod.json config/staging.json --quiet; then
  echo "設定が変更されています"
  diffx config/prod.json config/staging.json --output json > changes.json
fi

# タイムスタンプやメタデータを無視して比較
diffx api_v1.json api_v2.json --ignore-keys-regex "^(timestamp|updated_at)$"
```

## 実行例

[diffx-cli/tests/cmd/](diffx-cli/tests/cmd/) に詳細な例があります：

- [基本的な比較](diffx-cli/tests/cmd/basic.md)
- [対応フォーマット](diffx-cli/tests/cmd/formats.md)
- [オプション](diffx-cli/tests/cmd/options.md)
- [出力形式](diffx-cli/tests/cmd/output.md)
- [ディレクトリ比較](diffx-cli/tests/cmd/directory.md)

## ドキュメント

- [CLI仕様書](docs/specs/cli.md)
- [Core API仕様書](docs/specs/core.md)

## ライセンス

MIT

# diffx

[![build status](https://github.com/kako-jun/diffx/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/diffx/actions)
[![Crates.io](https://img.shields.io/crates/v/diffx)](https://crates.io/crates/diffx)
[![npm version](https://badge.fury.io/js/diffx.svg)](https://badge.fury.io/js/diffx)
[![PyPI version](https://badge.fury.io/py/diffx.svg)](https://badge.fury.io/py/diffx)

意味を理解して差分を検出する、次世代の差分ツール

[English](README.md) | [日本語](README_ja.md) | [中文](README_zh.md)

`diffx` は、JSON、YAML、TOMLなどの構造化データファイル間の意味的な差分を比較するために設計された、高速でパワフルなコマンドラインツールです。フォーマットの変更や些細な違いに惑わされることなく、データの構造と意味を理解し、本質的な変更点のみをハイライトします。

![diffx-demo](https://github.com/user-attachments/assets/b3333184-d375-482e-95a3-431f53857c2c)

## 主な特徴

- **意味的な比較**: ホワイトスペース、コメント、キーの順序の違いを無視し、データの意味的な変更点のみを検出します。
- **多様なフォーマット対応**: JSON, YAML, TOML, XML, INI, CSV に標準で対応しています。
- **高度なフィルタリング**: 正規表現やJSONPathライクな構文を使い、特定のキーやパスを無視したり、比較対象を絞り込んだりできます。
- **柔軟な出力形式**: 人間が読みやすいCLI表示、`unified`形式、JSON、YAMLなど、多彩な出力オプションを提供します。
- **パフォーマンス**: Rustによる高速な処理性能を誇り、巨大なファイルでもメモリ効率の良いストリーミング処理で対応します。
- **配列のIDベース比較**: 配列内の要素の順序変更と、実際の要素の変更を区別できます。
- **数値の許容誤差**: 浮動小数点数を比較する際に、わずかな誤差を許容する設定が可能です。

## なぜ diffx なのか？

従来の `diff` ツールはテキストの行単位で比較するため、構造化データ（特にJSONやYAML）の比較には不向きです。キーの順序が変わっただけ、あるいはフォーマットが変更されただけでも、大量の差分が検出されてしまい、本質的な変更を見逃す原因となります。

`diffx` は、まずファイルを構造として解析し、意味的なレベルで比較します。これにより、開発者はノイズに惑わされることなく、重要な変更に集中できます。

## インストール

### Homebrew (macOS / Linux)

```bash
brew install kako-jun/tap/diffx
```

### Cargo (Rust)

```bash
cargo install diffx
```

### npm (Node.js)

```bash
npm install -g diffx
```

### PyPI (Python)

```bash
pip install diffx
```

### マニュアルダウンロード

[リリースページ](https://github.com/kako-jun/diffx/releases)から、お使いのOSに対応したバイナリをダウンロードできます。

## 基本的な使い方

2つのファイルを比較するのは非常に簡単です。

```bash
diffx file1.json file2.json
```

### サポートされているフォーマット

`diffx` は、ファイル拡張子に基づいてフォーマットを自動で検出します。

- **JSON**: `.json`
- **YAML**: `.yaml`, `.yml`
- **TOML**: `.toml`
- **XML**: `.xml`
- **INI**: `.ini`, `.cfg`, `.conf`
- **CSV**: `.csv`

異なるフォーマット間での比較も可能です。

```bash
diffx config.json config.backup.toml
```

## 高度な使い方

### 出力フォーマットの変更

`--output` または `-o` オプションで、出力形式を `json` や `yaml` に変更できます。

```bash
diffx file1.json file2.json --output json
```

### 特定キーの無視

`--ignore-keys-regex` を使うと、正規表現にマッチするキーを比較から除外できます。

```bash
# "timestamp" または "id" で終わるキーを無視
diffx data1.json data2.json --ignore-keys-regex "(timestamp|id)$"
```

### 特定パスの比較

`--path` オプションで、比較対象を特定のデータパスに限定できます。

```bash
# "database.connections" 配下の差分のみを表示
diffx config1.json config2.json --path "database.connections"
```

### 配列のIDキー指定

`--array-id-key` を使うと、配列内のオブジェクトを、位置ではなく指定したキーの値で比較します。これにより、順序の変更と内容の変更を正しく区別できます。

```bash
# "user_id" をキーとして配列要素を比較
diffx users1.json users2.json --array-id-key "user_id"
```

詳しいオプションについては、[CLIリファレンス](docs/reference/cli-reference_ja.md)を参照してください。

## 貢献

バグ報告、機能提案、プルリクエストを歓迎します。詳しくは [CONTRIBUTING.md](CONTRIBUTING.md) をご覧ください。

## ライセンス

このプロジェクトは [MITライセンス](LICENSE) の下で公開されています。
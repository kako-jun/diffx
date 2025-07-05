# diffx

> **🚀 構造化データの意味的差分 - フォーマットではなく本質に集中**

[English README](README.md) | [日本語版 README](README_ja.md)

[![CI](https://github.com/kako-jun/diffx/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/diffx/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/diffx.svg)](https://crates.io/crates/diffx)
[![Documentation](https://docs.rs/diffx/badge.svg)](https://docs.rs/diffx)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

データの**構造**と**意味**を理解する次世代diffツール。JSON、YAML、TOML、XML、INI、CSVファイルに最適。

```bash
# 従来のdiffはフォーマットのノイズを表示（キー順序、ケツカンマなど）
$ diff config_v1.json config_v2.json
< {
<   "name": "myapp",
<   "version": "1.0"
< }
> {
>   "version": "1.1",
>   "name": "myapp"
> }

# diffxは意味的な変更のみを表示
$ diffx config_v1.json config_v2.json
~ version: "1.0" -> "1.1"
```

## ✨ 主な特徴

- **🎯 意味的認識**: フォーマット、キー順序、空白、ケツカンマを無視
- **🔧 多様なフォーマット**: JSON、YAML、TOML、XML、INI、CSV対応
- **🤖 AI対応**: 自動化やAI分析に最適なクリーンなCLI出力
- **⚡ 高速**: Rustで構築された最大限のパフォーマンス
- **🔗 メタチェイン**: 差分レポートを比較して変更の進化を追跡

## 📊 性能

AMD Ryzen 5 PRO 4650Uでの実測ベンチマーク結果：

```bash
# テストファイル: ネストした設定を含む約600バイトのJSON
$ time diff large_test1.json large_test2.json  # 15行以上のノイズを表示
$ time diffx large_test1.json large_test2.json # 3つの意味的変更を表示

# 結果:
従来のdiff: ~0.002秒（フォーマットノイズあり）
diffx:      ~0.005秒（クリーンな意味的出力）
```

**AI時代にCLIが重要な理由**: AI ツールが開発ワークフローに不可欠になる中、構造化された機械可読な差分出力が重要になります。`diffx`はAIが理解し推論できるクリーンで解析可能な結果を提供し、自動コードレビュー、設定管理、インテリジェントなデプロイメントパイプラインに最適です。

## diffxを使う理由

従来の`diff`は見た目の変更をたくさん表示します。`diffx`は本当に変わった部分だけを教えてくれます。

- **意味に注目**: キーの順序や空白、フォーマットを無視
- **多様な形式**: JSON、YAML、TOML、XML、INI、CSVに対応  
- **きれいな出力**: 人間にもスクリプトにもAIにも優しい

## 仕様（Specification）

### 対応フォーマット
- JSON
- YAML
- TOML
- XML
- INI
- CSV

### 差分の種類
- キーの追加・削除
- 値の変更
- 配列の挿入・削除・変更
- ネスト構造の差分
- 値の型変更

### 出力形式
`diffx`は、構造化データの差分を最も豊かに表現できる独自のCLI表示形式を推奨しますが、特定のユースケースや既存ツールとの連携のために、以下の代替出力形式もサポートします。

- **推奨CLI表示 (デフォルト)**
    *   構造的な差分（追加、変更、削除、型変更など）を人間が理解しやすいように、ユニバーサルデザインに配慮した色分けや記号、インデントを用いて明確に表示する独自形式です。
    *   `+` (追加), `-` (削除), `~` (変更), `!` (型変更) の記号と、青、黄、シアン、マゼンタの色で差分を表現します。
    *   **特徴**: データの意味的な変更に焦点を当て、キーの順序や空白の変更は無視します。これが `diffx` の核となる価値です。

- **JSON形式**
    *   機械可読な形式です。CI/CDや他のプログラムとの連携に利用します。
    *   `diffx` の検出した差分がJSON配列として出力されます。

- **YAML形式**
    *   機械可読な形式です。JSONと同様にプログラムとの連携に利用します。
    *   `diffx` の検出した差分がYAML配列として出力されます。

- **diff互換形式 (Unified Format)**
    *   `--output unified` オプションで提供されます。
    *   `git` や既存のマージツールとの連携を目的としています。
    *   **注意点**: この形式は、`diffx` が内部で検出した「意味的な差分」を、元のファイルの整形済みテキストの行ベースの差分として表現します。そのため、`diffx` が意味的な差分ではないと判断した変更（例：キーの順序変更、空白の変更）も、テキスト表現上変更があれば `+`/`-` で表示される可能性があります。あくまで互換性のための補助的な位置づけであり、**`diffx` の意味的な差分とは異なる**点にご注意ください。

## アーキテクチャ（Architecture）

### 構成案
```
diffx/
├── diffx-core/      # 差分抽出ライブラリ（Crate）
├── diffx-cli/       # CLIラッパー
├── tests/           # すべてのテスト関連ファイル
│   ├── fixtures/    # テスト用入力データ
│   ├── integration/ # CLI統合テスト
│   ├── unit/        # コアライブラリユニットテスト
│   └── output/      # テスト中間ファイル
├── docs/            # ドキュメントと仕様書
└── ...
```

### 技術スタック
- **Rust**（高速・安全・クロスプラットフォーム）
- `serde_json`, `serde_yml`, `toml`, `configparser`, `quick-xml`, `csv` などのパーサー
- `clap`（CLI引数処理）
- `colored`（CLI出力の色付け）
- `similar`（Unified Format出力）

## 🚀 インストールと使い方

### インストール

```bash
# CLIツールをインストール
cargo install diffx
```

### 基本的な使い方

```bash
# JSONファイルを比較
diffx file1.json file2.json

# 異なる出力形式で比較
diffx config.yaml config_new.yaml --output json
diffx data.toml data_updated.toml --output yaml

# 高度なオプション
diffx large.json large_v2.json --ignore-keys-regex "^timestamp$|^_.*"
diffx users.json users_v2.json --array-id-key "id"
diffx metrics.json metrics_v2.json --epsilon 0.001

# ディレクトリ比較
diffx config_dir1/ config_dir2/ --recursive

# メタチェイニング（変更の追跡）
diffx config_v1.json config_v2.json --output json > diff1.json
diffx config_v2.json config_v3.json --output json > diff2.json
diffx diff1.json diff2.json  # 変更の変更を比較！
```

## 将来的な展望

- **インタラクティブTUI (`diffx-tui`)**: `diffx`の力を示すためのサンプル兼高機能ビューア。左右に並べたデータと、それと連動する差分リストを表示し、「フォーマットの揺れに惑わされない、本質的な差分理解」という体験を提供する。
- **他言語向けラッパー**: Node.js/Python環境からdiffx CLIを呼び出すラッパーパッケージ
- GitHub Actionsでの差分チェック
- AIエージェントとの連携（差分要約・説明）
- Web UI版（diffx-web）
- VSCode拡張（diffx-vscode）

## 🤝 コントリビューション

コントリビューションを歓迎します！詳細は [CONTRIBUTING.md](CONTRIBUTING.md) をご確認ください。

## 📄 ライセンス

MIT License - 詳細は [LICENSE](LICENSE) をご確認ください。

# diffx

> **🚀 構造化データの意味的差分 - フォーマットではなく本質に集中**

[![CI](https://github.com/kako-jun/diffx/actions/workflows/ci.yml/badge.svg)](https://github.com/kako-jun/diffx/actions/workflows/ci.yml)
[![Crates.io](https://img.shields.io/crates/v/diffx.svg)](https://crates.io/crates/diffx)
[![Documentation](https://docs.rs/diffx/badge.svg)](https://docs.rs/diffx)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

データの**構造**と**意味**を理解する次世代diffツール。JSON、YAML、TOML、XML、INI、CSVファイルに最適。

```bash
# 従来のdiffはフォーマットのノイズを表示
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

- **🎯 意味的認識**: フォーマット、キー順序、空白を無視
- **🔧 多様なフォーマット**: JSON、YAML、TOML、XML、INI、CSV対応
- **🤖 AI対応**: 自動化やAI分析に最適なクリーンな出力
- **⚡ 高速**: Rustで構築された最大限のパフォーマンス
- **🔗 メタチェイン**: 差分レポートを比較して変更の進化を追跡

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
- *将来的に：XML, INI, CSV*

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

## 将来的な展望
- **差分レポートの差分 (Meta-chaining)**: `diffx`の出力をYAML/TOML形式で保存し、それを再び`diffx`の入力として比較することで、「差分レポートの差分」を検出する。これにより、設定変更の履歴管理や監査、デプロイメントの追跡など、高度な運用が可能になる。
- **インタラクティブTUI (`diffx-tui`)**: `diffx`の力を示すためのサンプル兼高機能ビューア。左右に並べたデータと、それと連動する差分リストを表示し、「フォーマットの揺れに惑わされない、本質的な差分理解」という体験を提供する。
- GitHub Actionsでの差分チェック
- AIエージェントとの連携（差分要約・説明）

## 提供形態（Overall Distribution）

### 1. Rust Crate（diffx-core）
- 構造化差分抽出のロジックをライブラリとして提供
- 他のRustアプリやCLIツールに組み込み可能
- 高速・型安全・拡張性あり

### 2. CLIツール（diffx）
- ユーザーが直接使えるコマンドラインツール
- AIやCI/CDツールからも呼び出しやすい
- `cargo install diffx` で導入可能

### 3. 他言語向けラッパー（npm/pip）
- **npmパッケージ（diffx-bin）**
  - Node.js環境から diffx CLI を呼び出すラッパー
  - `child_process.spawn()` でCLIを実行
- **pipパッケージ（diffx-bin）**
  - Python環境から diffx CLI を呼び出すラッパー
  - `subprocess.run()` でCLIを実行

### なぜこの構成が有効か？
- **AIとの親和性**: CLIがあることで、言語を問わずAIが操作可能
- **開発者の再利用性**: Rust Crateで他ツールに組み込みやすい
- **言語圏の拡張**: npm/pipでJS/Pythonユーザーにも届く
- **メンテナンス性**: CLIが主で、ラッパーは薄く保てる

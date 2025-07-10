# diffx ドキュメント

包括的な `diffx` ドキュメントへようこそ！

`diffx` は構造化データのためのセマンティック差分ツールで、単なる書式ではなく意味を理解します。従来のテキストベース差分ツールとは異なり、`diffx` はデータ構造の実際の変更に焦点を当てます。

## クイックリンク

- **[はじめに](user-guide/getting-started_ja.md)** - 基本概念と核となる考え方を学ぶ
- **[インストールガイド](user-guide/installation_ja.md)** - システムに diffx をセットアップ
- **[CLI リファレンス](reference/cli-reference_ja.md)** - 完全なコマンドライン資料
- **[実用例](user-guide/examples_ja.md)** - 業界横断的な実用例

## ドキュメント構成

### 📚 ユーザーガイド
*はじめに使うための必須ガイドと日常使用方法*

- **[インストール](user-guide/installation_ja.md)** - プラットフォーム別インストール手順
- **[はじめに](user-guide/getting-started_ja.md)** - 基本概念と最初のステップ
- **[設定](user-guide/configuration_ja.md)** - 設定ファイルとオプション
- **[実用例](user-guide/examples_ja.md)** - 8業界カテゴリーの実用例
- **[FAQ](user-guide/faq_ja.md)** - よくある質問とトラブルシューティング

### 📖 リファレンス
*完全な技術リファレンス資料*

- **[CLI リファレンス](reference/cli-reference_ja.md)** - 完全なコマンドラインインターフェース資料
- **[API リファレンス](reference/api-reference_ja.md)** - Rust クレート API 資料
- **[ツール比較](reference/comparison_ja.md)** - diffx と他のツールとの比較

### 🛠️ ガイド
*高度なトピックと統合ガイダンス*

- **[統合ガイド](guides/integrations_ja.md)** - CI/CD、開発ツール、自動化
- **[パフォーマンスガイド](guides/performance_ja.md)** - ベンチマークと最適化戦略

### 📋 プロジェクト情報
*プロジェクトガバナンスと開発情報*

- **[コントリビューションガイド](../CONTRIBUTING.md)** - プロジェクトへの貢献方法

## diffx の特徴

### セマンティック理解
```bash
# 従来の diff は書式ノイズを表示
$ diff config1.json config2.json
< {
<   "name": "myapp",
<   "version": "1.0"
< }
> {
>   "version": "1.1",
>   "name": "myapp"
> }

# diffx は意味のある変更のみを表示
$ diffx config1.json config2.json
~ version: "1.0" -> "1.1"
```

### マルチフォーマット対応
6つの構造化データフォーマットに対応：
- **JSON** - Web API、設定ファイル
- **YAML** - Kubernetes、Docker Compose、CI/CD
- **TOML** - Rust プロジェクト、モダンな設定ファイル
- **XML** - レガシーシステム、SOAP API
- **INI** - 伝統的な設定ファイル
- **CSV** - データエクスポート、表形式データ

### AI・自動化フレンドリー
- **一貫したCLIインターフェース** - 全フォーマット共通
- **機械読み取り可能な出力** (JSON、YAML)
- **柔軟なフィルタリング** - 正規表現パターン
- **ゼロ設定** - 賢いデフォルトで即座に動作

## 言語版

- **[English Documentation](./index.md)** - 英語版
- **[日本語ドキュメント](./index_ja.md)** (現在)

## コミュニティとサポート

- **[GitHub リポジトリ](https://github.com/kako-jun/diffx)** - ソースコードと課題追跡
- **[GitHub ディスカッション](https://github.com/kako-jun/diffx/discussions)** - コミュニティディスカッション
- **[GitHub リリース](https://github.com/kako-jun/diffx/releases)** - 最新版のダウンロード

---

*特定の情報をお探しですか？検索機能を使うか、よくある質問の [FAQ](user-guide/faq_ja.md) をご確認ください。*
# diffx の思想（Philosophy）
「構造化された差分を、誰でも、どこでも、簡単に」
従来の diff はテキストベースで、構造を理解できない。
diffx は JSON/YAML/TOMLなどの構造化データに特化した差分抽出ツール。
人間にもAIにもわかりやすい出力を提供し、設定ファイル・構成ファイル・データの変更を明確に可視化する。

# 🚨 重要な開発ルール (Important Development Rules)

## Claude対応時の必須ルール (Claude Response Rules)
**技術質問への回答では以下を必ず守ること:**
- **完全な仕様を最初から提供**: 条件・制限・例外をすべて含める
- **小出し回答の禁止**: 「確認が必要」「追加質問待ち」の姿勢を取らない
- **具体例を複数提示**: 動作例・制限例・エラー例を網羅
- **背景情報も同時提供**: なぜその仕様なのか、他の選択肢との違い
- **例**: `--context`なら「unified形式専用、CLI/JSON/YAML無効、前後N行表示、具体例3パターン」を一度に報告

## プッシュ前の必須チェック (Pre-Push Requirements)
**必ずプッシュ前に以下を実行すること:**
```bash
./scripts/testing/quick-check.sh
```

- このスクリプトはGitHub Actions CIと完全に同じ環境・パラメータで実行される
- 1つでもエラーが発生したら即座に停止する（`set -e`）
- フォーマット・Clippy・ビルド・テスト・CLI動作確認をすべて実行
- ローカルで成功 → GitHub CIでも成功が保証される
- CI失敗によるプッシュのやり直しを防げる

## コンテキスト効率化ルール (Context Efficiency Rules)
**CLAUDE.mdは目次として使用し、詳細情報は以下の専用ファイルを参照:**

- **📋 タスクリスト**: `.claude/tasks.md` を参照
- **🚀 リリース手順**: `.claude/release-guide.md` を参照
- **📊 プロジェクト状況**: `.claude/project-status.md` を参照  
- **🏗️ アーキテクチャ**: `.claude/architecture.md` を参照
- **🎯 ロードマップ**: `.claude/roadmap.md` を参照

**重要**: 詳細が必要な時のみ該当ファイルを読むこと。CLAUDE.md自体は最小限に保つ。

---

# 📦 現在の状況 (Current Status)

## 🎯 プロジェクト完成度
**diffx は設計・実装・テスト・ドキュメント化・マルチプラットフォーム公開が完了**

- **✅ 全6フォーマット対応**: JSON/YAML/TOML/XML/INI/CSV
- **✅ 高度な差分機能**: 構造認識、配列追跡、正規表現フィルタ
- **✅ UNIX互換CLI**: --context, --ignore-case, --quiet等の標準オプション
- **✅ 3言語エコシステム**: Rust(crates.io), JavaScript(npm), Python(PyPI)
- **✅ 2幕リリースワークフロー**: 安定したCI/CD自動公開システム

## 📦 最新リリース: v0.5.4 (2025-07-15)
- **🔧 CI/CD信頼性向上**: GitHub Actions環境での安定したテスト実行
- **🚀 完全汎用化リリースシステム**: lawkit/diffaiプロジェクトへのコピー可能
- **🛠️ スクリプト自動化強化**: Act1/Act2テストの信頼性とエラー処理改善

## 💻 提供形態
- **🦀 Rust (crates.io)**: ソースベースコンパイルで最高性能
- **📦 npm (diffx-js)**: 全プラットフォームバイナリ同梱のユニバーサルパッケージ
- **🐍 Python (diffx-python)**: maturin製の自己完結型wheel

---

# 🚀 開発ガイド (Development Guide)

## リリース手順
```bash
# 詳細手順は以下を参照
cat .claude/release-guide.md
```

## Python環境管理
```bash
# 必ずuvでvenv作成
uv venv && source .venv/bin/activate
```

# important-instruction-reminders
Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.
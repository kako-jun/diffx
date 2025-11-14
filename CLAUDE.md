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

## コンテキスト効率化ルール (Context Efficiency Rules)
**CLAUDE.mdは目次として使用し、詳細情報は以下の専用ファイルを参照:**

- **📋 タスクリスト**: `.claude/tasks.md` を参照
- **🔄 リブート計画**: `.claude/reboot/` ディレクトリを参照
- **📢 マーケティング**: `.claude/marketing/` ディレクトリを参照

**重要**: 詳細が必要な時のみ該当ファイルを読むこと。CLAUDE.md自体は最小限に保つ。

---

# 📦 現在の状況 (Current Status)

## 🎯 リブート中（Phase 2: 真実の特定）

**現在の優先順位**: 「仕切り直してからテストが通ること」

詳細は `.claude/tasks.md` と `.claude/reboot/` を参照。

## 🚫 信頼できないもの
- 既存のテスト（29 passed ≠ 正しい仕様）
- 既存の実装（動く ≠ 正しく動く）
- 既存のドキュメント（書いてある ≠ 本当にできる）

**合言葉**: 「疑って、確認して、記録する」

---

# important-instruction-reminders
Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.

# diffx の思想（Philosophy）
「構造化された差分を、誰でも、どこでも、簡単に」
従来の diff はテキストベースで、構造を理解できない。
diffx は JSON/YAML/TOMLなどの構造化データに特化した差分抽出ツール。
人間にもAIにもわかりやすい出力を提供し、設定ファイル・構成ファイル・データの変更を明確に可視化する。

# 📦 現在の状況

**diffx-core / diffx-cli リブート完了**

- 仕様書: `docs/specs/cli.md`, `docs/specs/core.md`
- タスク: `.claude/tasks.md`
- リブートノウハウ: `.claude/reboot-knowhow.md`

# 🚨 開発ルール

## Claude対応時の必須ルール
- **完全な仕様を最初から提供**: 条件・制限・例外をすべて含める
- **小出し回答の禁止**: 「確認が必要」「追加質問待ち」の姿勢を取らない
- **具体例を複数提示**: 動作例・制限例・エラー例を網羅

## コンテキスト効率化
CLAUDE.mdは目次。詳細は専用ファイルを参照。

---

# important-instruction-reminders
Do what has been asked; nothing more, nothing less.
NEVER create files unless they're absolutely necessary for achieving your goal.
ALWAYS prefer editing an existing file to creating a new one.
NEVER proactively create documentation files (*.md) or README files. Only create documentation files if explicitly requested by the User.

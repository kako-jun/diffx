# diffx タスクリスト

最終更新: 2025-11-14

## 🎯 現在の優先順位

**「仕切り直してからテストが通ること」**

マーケティングは後。まず正しい仕様・実装・テストを確立する。

## 📋 Phase 2: 真実の特定（今週）

### 仕様の確認
- [x] README_ja.md を読む
- [x] 書いてある機能をリストアップ
- [x] 実際に動くか1つずつ確認（基本機能）
- [x] 動いたものだけを記録

### 検証項目（基本機能）
- [x] JSON差分 ✅
- [x] YAML差分 ✅
- [x] TOML差分 ✅
- [x] XML差分 ✅
- [x] INI差分 ✅
- [x] CSV差分 ✅

### 検証済みオプション
- [x] `--output json` ✅
- [x] `--output yaml` ✅
- [x] `--quiet` ✅
- [x] `--ignore-keys-regex` ✅
- [x] `--epsilon` ✅
- [x] `--array-id-key` ✅

### 残りの検証項目
- [ ] `--ignore-case` の詳細（値のみ？キーも？）
- [ ] `--ignore-whitespace`
- [ ] `--brief`
- [ ] ディレクトリ比較
- [ ] メタチェイン

### ドキュメント作成
- [x] `.claude/reboot/features-claimed.md` 作成 ✅
- [x] `.claude/reboot/verified-features.md` 作成 ✅
- [ ] `docs/specs/core-spec.md` 作成（検証完了後）
- [ ] `docs/specs/cli-spec.md` 作成（検証完了後）
- [ ] 確認済み機能のみ記載

## 🔧 Phase 3: GitHub Actions簡素化（来週）

### 目標
- [ ] Rust専用のCI/CD
- [ ] 分岐なし、シンプルに
- [ ] `rust-ci.yml` 作成
- [ ] `rust-release.yml` 作成（後で）

### quick-check.sh
- [ ] シンプルなチェックスクリプト作成
- [ ] cargo fmt --check
- [ ] cargo clippy
- [ ] cargo build --release
- [ ] cargo test

## 📝 Phase 4: 新しいREADME（来週）

### 作成方針
- [ ] README_ja.md のみ（英語は後）
- [ ] 確認済み機能のみ記載
- [ ] 嘘なし、過剰な宣伝なし
- [ ] シンプルで明確に

## ⏳ 後回し（diffx安定後）

### マーケティング
- [ ] Product Hunt投稿
- [ ] Hacker News投稿
- [ ] Reddit投稿
（`.claude/marketing/plan.md` 参照）

### 他言語対応
- [ ] diffx-js 本格開発
- [ ] diffx-python 本格開発
（diffxが安定してから）

### エコシステム拡張
- [ ] Homebrew Formula
- [ ] Docker Hub公開
- [ ] VS Code拡張
（ユーザーが増えてから）

## 🚫 やらないこと

- ❌ lawkit/diffai移植（まだ早い）
- ❌ 3言語ドキュメント同時管理（_jaのみ）
- ❌ 完璧なCI/CD（シンプルで十分）
- ❌ ベンチマーク整備（本質的でない）
- ❌ 過度な機能追加（安定が先）

## 📊 完了済み

### Day 1（2025-11-14）

#### 朝〜昼
- [x] 問題分析
- [x] リブート計画
- [x] 別リポジトリ化
- [x] 大掃除（109ファイル削除）
- [x] マーケティングフォルダ整理

#### 夜
- [x] README.md整理（.claude配下の重複削除）
- [x] CLAUDE.md簡素化（93行→48行）
- [x] ベンチマーク削除（`diffx-core/benches/`）
- [x] README_ja.md から機能リスト抽出
- [x] 6フォーマット動作検証（JSON/YAML/TOML/XML/INI/CSV）
- [x] 3出力形式検証（CLI/JSON/YAML）
- [x] 4オプション検証（quiet/ignore-keys-regex/epsilon/array-id-key）
- [x] 検証結果ドキュメント作成

---

**合言葉**: 「疑って、確認して、記録する」
**次のステップ**: README_ja.md を読んで機能を確認

# diffx タスクリスト

最終更新: 2025-12-11

## 🎯 現在の優先順位

**「仕切り直してからテストが通ること」** → 達成！

次: 仕様の精査とテストの信頼性向上

## 📋 Phase 2.6: テスト正常化（完了）

### 完了項目
- [x] ハリボテオプション削除（--memory-optimization, --batch-size, --show-unchanged, --show-types）
- [x] --recursive (-r) 復活
- [x] テストのfixtureパス修正（../tests/fixtures/ → tests/fixtures/）
- [x] 全469テスト通過

### 課題（未解決）
- [ ] テスト自体が正しい仕様を反映しているか未検証
- [ ] 7月作成・11月移動のテストの信頼性

## 📋 Phase 2.5: リファクタリング（完了）

### diffx-core のリファクタリング
- [x] types.rs を抽出（DiffResult, DiffOptions など）
- [x] parser/ モジュールを作成（6フォーマット + format.rs）
- [x] lib.rs を更新（モジュールインポート、既存コード削除）
- [x] diff/ モジュールを作成（差分検出ロジック）
- [x] io/ モジュールを作成（ファイル・ディレクトリ操作）
- [x] コンパイル確認
- [x] 基本動作確認（6フォーマット）

### diffx-cli のリファクタリング（後）
- [ ] main.rs を分割（cli/, input/, output/, run.rs）

## 📋 Phase 2: 真実の特定（完了）

### 検証済みオプション
- [x] `--format` ✅
- [x] `--output json/yaml` ✅
- [x] `--recursive` ✅（復活）
- [x] `--path` ✅
- [x] `--ignore-keys-regex` ✅
- [x] `--epsilon` ✅
- [x] `--array-id-key` ✅
- [x] `--ignore-whitespace` ✅
- [x] `--ignore-case` ✅
- [x] `--quiet` ✅
- [x] `--brief` ✅
- [x] `--verbose` ✅
- [x] `--no-color` ✅
- [x] `--completions` ✅

### 削除済み（ハリボテだった）
- ~~--memory-optimization~~
- ~~--batch-size~~
- ~~--show-unchanged~~
- ~~--show-types~~
- ~~--context~~

## 🔧 Phase 3: 仕様精査（次）

### 目標
- [ ] 各オプションの正確な動作を文書化
- [ ] テストが仕様を正しく反映しているか検証
- [ ] 不正確なテストの修正

## ⏳ 後回し

### GitHub Actions簡素化
- [ ] Rust専用のCI/CD
- [ ] `rust-ci.yml` 作成

### 新しいREADME
- [ ] README_ja.md のみ（英語は後）
- [ ] 確認済み機能のみ記載

### マーケティング
- [ ] Product Hunt投稿
- [ ] Hacker News投稿
（`.claude/marketing/plan.md` 参照）

## 📊 完了済み

### 2025-12-11
- [x] ハリボテオプション4つ削除
- [x] --recursive 復活
- [x] テストパス修正
- [x] 全テスト通過（469 passed, 2 ignored）

### 2025-11-14
- [x] Phase 2.5 リファクタリング完了
- [x] diffx-core モジュール化（100%）

---

**合言葉**: 「疑って、確認して、記録する」
**次のステップ**: 仕様精査、テストの信頼性向上

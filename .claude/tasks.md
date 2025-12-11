# diffx タスクリスト

最終更新: 2025-12-11

## 🎯 現在の状態

**diffx-core / diffx-cli リブート完了**

- 仕様書作成済み（docs/specs/cli.md, docs/specs/core.md）
- spec-based テスト作成済み（tests/spec/ - 69テスト）
- trycmd ドキュメントテスト作成済み（tests/cmd/ - 19テスト）
- 不要ファイルクリーンアップ済み

## 📋 残タスク

### diffx-cli
- [ ] main.rs 分割（cli/, input/, output/, run.rs）- 任意

### npm/pip パッケージ
- [ ] diffx-js リブート
- [ ] diffx-python リブート

## 📊 完了済み

### 2025-12-11（リブート完了）
- [x] 仕様書作成（docs/specs/cli.md, docs/specs/core.md）
- [x] 古いテスト削除（436テスト、8022行）
- [x] spec-based テスト作成（69テスト）
- [x] trycmd ドキュメントテスト作成（19テスト）
- [x] docs/examples/ 削除（嘘だらけ）
- [x] README_ja.md 修正
- [x] Cargo.lock をバージョン管理に追加
- [x] .claude/reboot/ 削除
- [x] .claude/marketing/ 削除
- [x] リブートノウハウ記録（.claude/reboot-knowhow.md）

### 2025-11-14
- [x] Phase 2.5 リファクタリング完了
- [x] diffx-core モジュール化（100%）

---

**次のプロジェクト**: diffx-js, diffx-python, lawkit, diffai
**参考**: `.claude/reboot-knowhow.md`

# TODOリスト

## 🚀 次期リリース向け改善 (Next Release)

### 🧹 コードクリーンアップ
- [ ] `diffx-python/examples.py`の`temp_feature`フラグを適切な名前に変更
- [ ] `ci-local.sh`のTODO/FIXME警告処理の見直し（厳格化 or 削除）
- [ ] `monitor-release.sh`のハードコードされたリトライ回数を設定可能に

### 📦 パッケージ改善
- [x] npmパッケージのpostinstall関連コード削除（v0.5.3で完了）
- [ ] `--skip-existing`フラグの必要性再検討

## 🌍 コミュニティ展開 (Community)

### 📢 プロモーション
- [ ] GitHub Pages公式サイト開設
- [ ] Docker Hub公開
- [ ] Reddit r/rust紹介投稿準備

### 🔧 エコシステム拡張
- [ ] Homebrew Formula作成
- [ ] TUIモード実装検討
- [ ] VS Code拡張検討

## 💡 長期的機能 (Long-term Features)

- [ ] 3者間差分（`--three-way`）
- [ ] AI要約機能
- [ ] Web API（`diffx serve`）

---

## ✅ 完了済み (Completed)

### v0.5.3 (2025-07-13)
- [x] npmユニバーサルパッケージ実装
- [x] 全プラットフォームバイナリ同梱
- [x] スクリプトパス問題修正
- [x] 3言語ドキュメント更新
- [x] obsoleteなpostinstallコード削除

### v0.5.2 以前
- [x] 2幕リリースワークフロー実装
- [x] 組織レベルGitHubテンプレート作成
- [x] APIドキュメント改善（レガシーAPI削除）
- [x] 統合リリーススクリプト作成
- [x] maturin-based Python wheel実装
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

### 📢 戦略的マーケティングキャンペーン
- [ ] プロモーション戦略策定・原稿作成
- [ ] グローバル同時投稿キャンペーン実行
- [ ] 投稿効果測定・改善サイクル構築
- [ ] Docker Hub公開（DevOpsコミュニティ向け）

### 🎯 マーケティング管理（コードベース）
- [ ] `.claude/marketing/` ディレクトリ作成
- [ ] プラットフォーム別投稿戦略・原稿管理
- [ ] 効果測定スクリプト作成
- [ ] 多言語・多地域対応戦略

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

### v0.5.3 (2025-07-13) - 完璧なリリース達成 🎉
- [x] ブランチ同期確認とmain最新化
- [x] バージョン番号確認（全パッケージ0.5.2→0.5.3）
- [x] 環境チェック（Python venv、maturin、認証情報）
- [x] pre-releaseチェック実行（全事前チェック通過）
- [x] CIローカルテスト（全テスト成功）
- [x] スクリプトパス修正（"could not open directory"エラー解決）
- [x] リリース実行（v0.5.3タグ作成・プッシュ）
- [x] GitHub Actions監視（Act1、Act2完全成功）
- [x] リリースノート作成（包括的で詳細な内容）
- [x] パッケージ公開確認（Rust、npm、PyPI全て成功）
- [x] npmユニバーサルパッケージ実装
- [x] 全プラットフォームバイナリ同梱でオフライン対応
- [x] 3言語ドキュメント更新（英・日・中）
- [x] obsoleteなpostinstallコード削除
- [x] CLAUDE.md現代化（コンテキスト効率化）
- [x] todo.mdベース管理への移行

### v0.5.2 以前
- [x] 2幕リリースワークフロー実装
- [x] 組織レベルGitHubテンプレート作成
- [x] APIドキュメント改善（レガシーAPI削除）
- [x] 統合リリーススクリプト作成
- [x] maturin-based Python wheel実装
- [x] Reddit r/rust紹介投稿（反応限定的）
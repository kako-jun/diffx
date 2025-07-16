# TODOリスト

## 🚀 優先度: マーケティング (Product is Ready - Focus on Awareness)

### 📢 戦略的マーケティングキャンペーン (最優先)
- [x] プロモーション戦略策定・原稿作成
- [ ] グローバル同時投稿キャンペーン実行
- [ ] 投稿効果測定・改善サイクル構築
- [ ] Docker Hub公開（DevOpsコミュニティ向け）

### 🎯 マーケティング管理（コードベース）
- [x] `.claude/marketing/` ディレクトリ作成
- [x] プラットフォーム別投稿戦略・原稿管理
- [ ] 効果測定スクリプト作成
- [ ] 多言語・多地域対応戦略

## 🔧 低優先度: 機能改善 (After Market Success)

### 🧹 コードクリーンアップ (後回し)
- [ ] `diffx-python/examples.py`の`temp_feature`フラグを適切な名前に変更
- [ ] `monitor-release.sh`のハードコードされたリトライ回数を設定可能に

### 📦 パッケージ改善 (後回し)
- [x] npmパッケージのpostinstall関連コード削除（v0.5.3で完了）
- [ ] `--skip-existing`フラグの必要性再検討

### 🔧 エコシステム拡張
- [ ] Homebrew Formula作成
- [ ] TUIモード実装検討
- [ ] VS Code拡張検討

## 🚀 汎用化プロジェクト (Universal Release System)

### ✅ 完了済み - diffxでの汎用化
- [x] **100%汎用化達成**: 全20ファイルがプロジェクト横断対応
- [x] **リリーススクリプト完全汎用化**: 7ファイル、PROJECT_NAME変数使用
- [x] **GitHub Actionsワークフロー汎用化**: 動的リポジトリ名抽出
- [x] **テストスクリプト汎用化**: パッケージ名・コマンド名の動的生成
- [x] **ドキュメント汎用化**: プロジェクト名ハードコード除去

### ✅ 完了 - 共有CI/CDシステム実装
- [x] **共有リポジトリ構造作成**: `.github/rust-cli-kiln/` ディレクトリ配下にscripts/workflows移動
- [x] **シンボリックリンク作成**: `github-shared -> ../.github` でローカル開発対応（パス名統一）
- [x] **workflow_call対応**: reusable workflowsでプロジェクト間共有
- [x] **動的プロジェクト名取得**: `${{ github.event.repository.name }}` 使用
- [x] **common.sh共通化**: プロジェクトルート検出とBASH_SOURCE処理改善
- [x] **CI workflow修正**: プロジェクトcheckout追加、共有リポジトリcheckout対応
- [x] **CIワークフロー動作確認**: GitHub Actions実行テスト完了
- [x] **benchmark/release workflow有効化**: 全ワークフロー有効化・権限修正
- [x] **パス問題解決**: `.github-shared`→`github-shared`でENOENTエラー解決
- [x] **スクリプトパス修正**: Act1/Act2でgithub-sharedパス使用
- [x] **汎用スクリプト移動**: setup-github-workflow.sh, check-docs-consistency.sh共有化
- [x] **JSON設定連携**: labels.json, branch-protection.json自動適用機能追加

### ✅ 完了 - v0.5.6 Release workflow最適化
- [x] **Release Act1実行成功**: v0.5.6で全プラットフォームビルド成功
- [x] **Act1の問題発見・修正**: npm/Pythonテストの二重実行を発見・削除
- [x] **npmパスエラー修正**: download-all-binaries.jsのパス問題解決
- [x] **Act1/Act2動作確認**: 修正後のAct1/Act2完全テスト成功
- [x] **v0.5.6リリース完全成功**: 全パッケージ（Rust/npm/PyPI）公開成功
- [x] **プラットフォーム統一**: 全パッケージで5プラットフォーム対応
- [x] **ベンチマークワークフロー簡素化**: 複雑な性能回帰テストを削除

### 🔧 進行中 - lawkit/diffai移植
- [x] **プラットフォーム統一作業**: 全プロジェクトでLinux ARM64サポート追加
- [x] **npmパッケージ修正**: lawkit/diffaiのindex.js, download-all-binaries.js修正
- [ ] **🔄 現在のタスク**: 共有CI/CDワークフロー移植
- [ ] **移植後動作確認**: 各プロジェクトでのquick-check.sh実行

### 🐛 発見・修正した問題
- **ENOENTエラー**: `.github-shared`ドット始まりパスがGitHub Actionsで失敗
- **シンボリックリンクコミット**: `github-shared`がGitにコミットされて競合
- **workflow_call不足**: Act2にworkflow_callトリガーが不足
- **権限不足**: release workflowsに`contents: write`権限が不足
- **二重テスト実行**: Act1でnpm/Pythonテスト、Act2でも同様テスト実行
- **npmパスエラー**: download-all-binaries.jsのパス参照エラー（v0.5.6で修正）
- **プラットフォーム不統一**: PyPIのみLinux ARM64対応、統一化完了
- **プロジェクト検出失敗**: 共通化スクリプトでcommon.sh使用漏れ

### 📋 最終的なディレクトリ構造
```bash
# 共有リポジトリ構造
/home/kako-jun/repos/2025/.github/
├── .github/
│   ├── workflows/
│   │   ├── rust-cli-kiln-ci.yml (workflow_call対応)
│   │   ├── rust-cli-kiln-benchmark.yml (workflow_call対応)
│   │   ├── rust-cli-kiln-release-act1.yml (workflow_call対応)
│   │   └── rust-cli-kiln-release-act2.yml (workflow_call対応)
│   ├── labels.json (共有ラベル設定)
│   └── branch-protection.json (共有ブランチ保護設定)
└── rust-cli-kiln/
    ├── scripts/
    │   ├── setup/
    │   │   └── setup-github-workflow.sh (JSON設定自動適用)
    │   ├── docs/
    │   │   └── check-docs-consistency.sh (3言語整合性)
    │   ├── testing/
    │   │   ├── quick-check.sh (CI完全対応)
    │   │   ├── 04-pre-release-test-act1.sh (npm/Python削除予定)
    │   │   └── 05-pre-release-test-act2.sh (公開準備テスト)
    │   └── utils/
    │       └── common.sh (汎用プロジェクト検出)
    └── release-guide.md

# 各プロジェクト構造
/home/kako-jun/repos/2025/diffx/
├── github-shared -> ../.github (symlink, GitHub Actionsと統一)
├── .github/workflows/
│   ├── ci.yml (workflow_call薄ラッパー)
│   ├── benchmark.yml (workflow_call薄ラッパー)  
│   ├── release-act1.yml (workflow_call薄ラッパー)
│   └── release-act2.yml (workflow_call薄ラッパー)
└── scripts/utils/
    └── create-github-shared-symlink.sh (プロジェクト固有)
```

### 🚨 次のステップ（現在進行中）
1. **lawkit/diffai移植**: 共有CI/CDワークフローの移植実行
2. **シンボリックリンク作成**: github-sharedリンクの各プロジェクトでの作成
3. **ワークフロー移植**: ci.yml, benchmark.yml, release-act1.yml, release-act2.yml移植
4. **動作確認**: 各プロジェクトでquick-check.sh実行
5. **テストリリース**: 小バージョンアップでの動作確認

### ✅ 達成された成果
- **🏗️ 完全な共有CI/CDシステム**: workflow_call方式で美しく構築
- **⚡ 高度な問題解決**: 8つの複雑な技術問題を段階的に解決
- **🔧 汎用化の達成**: プロジェクト名動的取得、JSON設定連携
- **📊 設計の最適化**: サブモジュール方式よりも論理的な構造を実現
- **🚀 v0.5.6リリース成功**: 全プラットフォーム統一、全パッケージ公開成功
- **📦 プラットフォーム統一**: 5プラットフォーム対応（Linux x86_64/ARM64, Windows x86_64, macOS x86_64/ARM64）
- **🔄 CI/CD信頼性向上**: パス問題解決、テスト最適化、プロジェクト検出改善

## 💡 長期的機能 (Long-term Features)

- [ ] 3者間差分（`--three-way`）
- [ ] AI要約機能
- [ ] Web API（`diffx serve`）

---

## ✅ 完了済み (Completed)

### v0.5.4 (2025-07-15) - 汎用化とCI/CD信頼性向上 🚀
- [x] v0.5.4リリース完全成功（Act1、Act2両方成功）
- [x] GitHub Actions CI環境でのテスト信頼性向上
- [x] Act1テストスクリプト強化（ビルドアーティファクト除外、Cargo.lock自動処理）
- [x] 終了コード処理改善（diff特有の戻り値1の適切な処理）
- [x] **完全汎用化達成**: 全スクリプト・ワークフローのプロジェクト横断対応
- [x] 残り6ファイルの汎用化（08-test-published-packages.sh、GitHub Actionsワークフロー等）
- [x] リリースノート詳細化とCHANGELOG.md更新
- [x] **lawkit/diffaiへの移植準備完了**: ファイルコピーのみで利用可能

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
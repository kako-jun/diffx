# Release Guide

完全自動化されたリリースプロセス - 人間もAIも同じ手順で実行

> 🔄 **汎用リリースガイド**: このガイドは複数のRustプロジェクトで使い回し可能です

## 🎯 リリース手順（9ステップ）

### ステップ0: 事前チェック（推奨）
```bash
./scripts/release/00-pre-release-check.sh
```
- git状態、バージョン整合性、認証、依存関係の包括的確認
- 問題発見時は早期修正可能

### ステップ1: 公開済みバージョンの確認
```bash
./scripts/release/01-check-published-versions.sh
```
- crates.io, PyPI, npmの現在の公開バージョンを確認
- 次にリリースすべきバージョンを判断

### ステップ2: バージョン更新
```bash
./scripts/release/02-update-version.sh X.Y.Z
```
- 全製品コンポーネントのバージョンを一括更新
- 例: `./scripts/release/02-update-version.sh 1.2.3`

### ステップ3: 更新後の整合性確認
```bash
./scripts/release/03-check-local-versions.sh
```
- 更新後のローカルファイル内バージョンの整合性チェック
- 抜けや不一致がないか確認

### ステップ4: Act1テスト（コア機能）
```bash
./scripts/testing/04-pre-release-test-act1.sh
```
- GitHub Actions Release Act1と同等のテスト実行
- Rustビルド・テスト・crates.io公開準備確認（dry runのみ）
- **実際の公開はGitHub Actionsでのみ実行**
- **失敗時はここで停止**
- **Claude実行時の注意**: 5分以上かかる場合があるため、タイムアウトを10分に設定して実行

### ステップ5: Act2テスト（言語ラッパー）
```bash
./scripts/testing/05-pre-release-test-act2.sh
```
- GitHub Actions Release Act2と同等のテスト実行
- npm・PyPI公開準備確認（dry runのみ）
- **実際の公開はGitHub Actionsでのみ実行**
- **Act1成功後のみ実行**

### ステップ6: リリースタグ作成
```bash
./scripts/release/06-create-release-tag.sh
```
- Gitタグ作成・プッシュ
- GitHubリリースページ作成
- GitHub Actionsトリガー

### ステップ7: リリース監視
```bash
./scripts/release/07-monitor-release.sh vX.Y.Z
```
- GitHub Actionsの実行状況監視
- 全プラットフォームでの公開完了確認

### ステップ8: 公開パッケージ検証（オプション）
```bash
./scripts/testing/08-test-published-packages.sh
```
- npm、PyPI公開パッケージの動作確認
- 実環境でのインストール・動作テスト
- クロスプラットフォーム互換性確認
- **失敗してもリリースは有効（品質保証目的）**

## 📋 完全な実行例

```bash
# ステップ0: 事前チェック（推奨）
./scripts/release/00-pre-release-check.sh

# ステップ1: 現在の状況確認
./scripts/release/01-check-published-versions.sh

# ステップ2: 新バージョンに更新
./scripts/release/02-update-version.sh X.Y.Z

# ステップ3: 整合性確認
./scripts/release/03-check-local-versions.sh

# ステップ4: Act1テスト
./scripts/testing/04-pre-release-test-act1.sh

# ステップ5: Act2テスト  
./scripts/testing/05-pre-release-test-act2.sh

# ステップ6: リリース実行
./scripts/release/06-create-release-tag.sh

# ステップ7: 監視
./scripts/release/07-monitor-release.sh vX.Y.Z

# ステップ8: 公開パッケージ検証（オプション）
./scripts/testing/08-test-published-packages.sh
```

## 🛠️ 日常開発用クイックチェック

リリース以外の日常開発では以下を使用：

```bash
# 通常のプッシュ前チェック（5-10秒）
./scripts/testing/quick-check.sh
```

- GitHub Actions CIと完全同等
- 軽量・高速実行
- フォーマット・Clippy・ビルド・テスト

## 🚨 重要な原則

### 2層テスト体系
- **日常開発**: `quick-check.sh` → GitHub Actions CI
- **リリース**: `pre-release-test-*.sh` → GitHub Actions Release

### 失敗時の安全性
- **Act1失敗** → Act2実行されない（GitHub Actions仕様）
- **ローカルテスト失敗** → リリース作成しない
- **全テスト成功** → のみリリース実行

### 責任分離
- **各スクリプトは単一責任**
- **事前チェック**: `00-pre-release-check.sh`のみ
- **バージョン更新**: `02-update-version.sh`のみ
- **テスト実行**: `quick-check.sh` / `pre-release-test-*.sh`のみ
- **リリース作成**: `06-create-release-tag.sh`のみ
- **失敗時清理**: `cleanup-failed-release.sh`のみ

### 手動更新が必要なファイル
以下は製品コンポーネントでないため、手動更新：
- プロジェクト管理文書（`CLAUDE.md`, `README.md`等）
- `CHANGELOG.md` (リリース内容記述)
- ドキュメント類（`docs/`配下等）

## 🤖 AI使用時の注意

AIが「リリースして」と言われた時：
1. このガイドを参照
2. 手順を順次実行
3. 失敗時は即座に停止
4. 成功時のみ次のステップへ

**AIは人間の代替作業のみ実行。特別な処理なし。**

## 🔧 トラブルシューティング

### よくある問題
1. **事前チェック失敗**: git状態・認証・依存関係 → 修正後再実行
2. **Act1失敗**: コードの問題 → 修正後再実行
3. **Act2失敗**: 依存関係の問題 → 環境修正後再実行
4. **バージョン不整合**: `03-check-local-versions.sh`で詳細確認

### 緊急時の対応
- **途中で止める**: 現在のステップで停止（Git操作前なら安全）
- **失敗したリリース**: `scripts/release/cleanup-failed-release.sh`で清理

### 失敗したリリースの完全清理
```bash
./scripts/release/cleanup-failed-release.sh vX.Y.Z
```
- GitHub release削除
- Git tag削除（local/remote）
- 各パッケージレジストリからの削除（オプション）
  - Rust: crates.io yank
  - npm: unpublish（24時間以内のみ）
  - Python: PyPI削除指示（手動）

## 🎯 AI向け自動実行ルール

### GitHub操作について
**AIは以下のいずれかの方法でGitHub操作を実行：**
- **GitHub CLI (`gh`)**: コマンドライン経由
- **GitHub MCP**: MCP (Model Context Protocol) 経由
- どちらも同等の機能を提供、AIの環境に応じて選択

### リリース完了後の必須作業

**ステップ7（監視）完了後、以下を必ず実行：**

#### 1. リリースノート詳細化の自動判断
```bash
# 現在のリリースの本文をチェック
gh release view v<version>
# または MCPを使用
# GitHub MCPを使ってリリース情報を取得
```

**実行条件**（すべて満たす場合）：
- ✅ GitHub Actions（Act1/Act2）が完全成功した
- ✅ 新しいリリースページが作成された
- ✅ リリース本文が簡素（200文字未満、または「**Full Changelog**」のみ）
- ✅ パッケージレジストリへの公開が成功した

#### 2. 実質的な前バージョンの特定
```bash
# 一般ユーザーに見える前の実質的なリリースを特定
gh release list --limit 10
# または MCPを使用
# GitHub MCPを使ってリリース一覧を取得
```

**判断基準**：
- 充実した本文があるリリース = 「実質的な前バージョン」
- 簡素な本文のリリース（「ゴミリリース」）は飛ばす
- 実質的な前バージョンから現在までの変更を収集

#### 3. リリースノートとCHANGELOG同時更新
```bash
# 実質的な前バージョンから現在までの変更を収集
git log v<前の実質バージョン>..v<現在バージョン> --oneline

# 詳細なリリースノートで上書き
gh release edit v<version> --notes "$(cat <<'EOF'
[詳細な本文]
EOF
)"
# または MCPを使用
# GitHub MCPを使ってリリースノートを更新
```

**リリースノート必須要素**：
- 主要機能・改善点の要約（ユーザー目線）
- 技術的改善点（開発者・上級ユーザー向け）
- 破壊的変更・移行ガイド（該当する場合）
- パッケージ提供状況（Rust/npm/Python）
- インストール・使用方法の変更（該当する場合）
- 次のマイルストーンへの布石

**CHANGELOG.md更新**：
- 新機能・改善・バグ修正を整理
- 破壊的変更があれば明記
- リンクやマイグレーション情報を追加

#### 4. 失敗バージョンの考慮
**重要**: 失敗したリリースで番号が飛んだ場合は、1つ前の成功バージョンからの差分をまとめて記載

例: v1.4.2が成功、v1.5.0が失敗、v1.5.1が成功の場合
→ v1.5.1には v1.4.2からv1.5.1までの全変更を記載

### 新機能確認の必須手順

#### Rust機能の動的確認
```bash
# ソースコードから最新のCLIオプションを確認
# 各プロジェクトのCLIクレート内のmain.rsのArgs構造体をチェック
find . -name "main.rs" -path "*/cli/*" -exec grep -l "struct.*Args" {} \;
```

**注意**: システムにインストール済みの古いバイナリで`--help`を確認してはいけない

#### 実装済みオプション確認方法
```bash
# 動的にCLIオプションを確認
# 1. ソースコードから確認
rg "struct.*Args" --type rust -A 20

# 2. ビルド後のヘルプで確認
cargo run --bin <プロジェクト名> -- --help

# 3. 新オプションがあれば各言語ラッパーへの対応確認
```

## 📊 成功の指標

全ステップ完了時：
- ✅ GitHub Release作成済み
- ✅ Rust crates公開済み（該当する場合）
- ✅ Python package公開済み（該当する場合）  
- ✅ npm package公開済み（該当する場合）
- ✅ 全プラットフォームバイナリ配布済み
- ✅ リリースノート詳細化完了
- ✅ CHANGELOG.md更新完了

## 🔄 Python環境管理の必須ルール

**Pythonを使う際の絶対ルール**：
- **システムPython禁止**: `pip install`でシステムレベルにインストールしない
- **uv必須**: 必ず`uv`を使って仮想環境を管理する
- **venv自動作成**: `.venv`がなければ`uv venv`で作成
- **アクティベート必須**: 作業前に`source .venv/bin/activate`を実行

```bash
# Python環境セットアップ手順
# 1. uvのインストール（一度だけ）
pipx install uv

# 2. プロジェクトごとにvenv作成
cd /path/to/project
uv venv

# 3. 環境アクティベート（作業のたびに）
source .venv/bin/activate

# 4. 必要なツールインストール
uv pip install maturin wheel build twine
```

## 🚀 緊急時の個別リリース

### Rust cratesのみ緊急修正
```bash
# プロジェクトのcrateディレクトリを確認
find . -name "Cargo.toml" -not -path "./target/*" | head -5

# 各crateで個別公開
cd <crate-directory> && cargo publish
```

### npmのみ緊急修正
```bash
# npmパッケージディレクトリを確認
find . -name "package.json" -not -path "./node_modules/*"

cd <npm-package-dir>
npm version patch --no-git-tag-version
npm publish
```

### Python packageのみ緊急修正
```bash
# Python パッケージディレクトリを確認
find . -name "pyproject.toml"

cd <python-package-dir>
# pyproject.tomlのバージョンを手動更新
maturin build --release
maturin publish
```

**このガイドに従えば、確実にリリースできます。**
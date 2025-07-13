# diffx成功パターン完全移植計画書

## 📋 **移植対象ツール**
- `../diffai` - AI-powered diff analysis
- `../lawkit` - Legal document toolkit

## 🎯 **diffxで確立された成功要素**

### 1. **ディレクトリ構造の現代化**

#### `.claude/` ディレクトリ (Claude Code最適化)
```
.claude/
├── release-guide.md      # AI向けリリース指示
├── tasks.md              # タスク管理
└── marketing/            # マーケティング戦略
    ├── strategy.md
    ├── content-templates.md
    └── execution-plan.md
```

#### `scripts/` ディレクトリ (機能別分類)
```
scripts/
├── release/              # リリース関連
│   ├── release.sh        # 統合リリーススクリプト
│   ├── pre-release-check.sh
│   ├── monitor-release.sh
│   ├── cleanup-failed-release.sh
│   └── validate-dynamic-versions.sh
├── testing/              # テスト関連
│   └── ci-local.sh       # ローカルCI実行
└── utils/                # ユーティリティ
    └── check-versions.sh
```

### 2. **CLAUDE.md の目次化**

#### 現在のCLAUDE.md構造
```markdown
# {TOOL_NAME} の思想（Philosophy）
{ツール固有の説明}

# 🚨 重要な開発ルール (Important Development Rules)

## Claude対応時の必須ルール (Claude Response Rules)
## プッシュ前の必須チェック (Pre-Push Requirements)
## コンテキスト効率化ルール (Context Efficiency Rules)

# 📦 現在の状況 (Current Status)
# 🚀 開発ガイド (Development Guide)
```

#### コンテキスト効率化ルール
```markdown
**CLAUDE.mdは目次として使用し、詳細情報は以下の専用ファイルを参照:**

- **📋 タスクリスト**: `.claude/tasks.md` を参照
- **🚀 リリース手順**: `.claude/release-guide.md` を参照
- **📊 プロジェクト状況**: `.claude/project-status.md` を参照  
- **🏗️ アーキテクチャ**: `.claude/architecture.md` を参照
- **🎯 ロードマップ**: `.claude/roadmap.md` を参照

**重要**: 詳細が必要な時のみ該当ファイルを読むこと。CLAUDE.md自体は最小限に保つ。
```

### 3. **パッケージング現代化**

#### Python (maturin化)
```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "{tool-name}-python"
version = "X.Y.Z"
```

#### npm (ユニバーサルパッケージ)
```javascript
// index.js - プラットフォーム自動検出
function getPlatformInfo() {
  const platform = process.platform;
  const arch = process.arch;
  // 全プラットフォーム対応
}
```

### 4. **GitHub Actions 2幕構成**

#### Act1: Core (Release Act 1)
- Rustバイナリビルド (全プラットフォーム)
- GitHub Release作成
- crates.io公開

#### Act2: Language Wrappers (Release Act 2)
- npm公開 (全バイナリ同梱)
- PyPI公開 (maturin wheels)

### 5. **開発フロー改善**

#### GitHub Flow緩和
- mainへの直接コミット許可
- 小さな変更のPR不要
- 迅速な反復開発

#### CI/CD 最適化
- プッシュ前: `./scripts/testing/ci-local.sh`
- 動的バージョン管理
- 失敗時自動クリーンアップ

## 🔧 **移植時の注意点・実際の問題**

### スクリプト実行時の実際の問題
1. **パス問題**: スクリプトが相対パスで失敗
   - **解決**: 全スクリプトにプロジェクトルート検出を追加
   
2. **権限問題**: GitHub Actions権限不足
   - **解決**: repo設定でActions権限を緩和
   
3. **認証問題**: npm/PyPI/cargo認証
   - **解決**: 事前認証確認スクリプト

4. **バージョン不整合**: ハードコードされたバージョン
   - **解決**: 動的バージョン取得に変更

### 臨機応変に実行したコマンド例
```bash
# スクリプト失敗時の直接実行
git add . && git commit -m "fix: resolve issue"
uv venv && source .venv/bin/activate
npm audit fix
cargo update --workspace
```

## 📝 **完全移植チェックリスト**

### Phase 1: 基本構造 (必須)
- [ ] `.claude/` ディレクトリ作成
- [ ] `CLAUDE.md` 目次化
- [ ] `scripts/` 分類・整理 (release/testing/utils)
- [ ] ディレクトリ構造標準化
- [ ] `.gitignore` 更新

### Phase 2: Claude Code最適化
- [ ] `.claude/release-guide.md` 作成
- [ ] `.claude/tasks.md` 作成
- [ ] コンテキスト効率化実装
- [ ] `.claude/marketing/` 構造作成

### Phase 3: ドキュメント品質標準化
- [ ] README_ja.md, README_zh.md 作成
- [ ] 3言語同期ドキュメント確立
- [ ] 使用例のテストケース作成
- [ ] docs/ ディレクトリ整備

### Phase 4: 開発環境標準化
- [ ] uv仮想環境必須化
- [ ] Python開発環境統一
- [ ] 依存関係管理改善
- [ ] 開発ワークフロー統一

### Phase 5: スクリプト移植・修正
- [ ] リリーススクリプト移植
- [ ] パス問題修正 (プロジェクトルート検出)
- [ ] 動的バージョン管理実装
- [ ] エラーハンドリング強化
- [ ] プッシュ前チェック実装

### Phase 6: パッケージング現代化
- [ ] maturin Python実装
- [ ] ユニバーサルnpm実装
- [ ] マルチプラットフォーム対応
- [ ] パッケージメタデータ統一

### Phase 7: CI/CD 2幕構成
- [ ] Release Act1 実装
- [ ] Release Act2 実装
- [ ] GitHub設定調整
- [ ] 権限問題解決

### Phase 8: 品質保証標準実装
- [ ] プッシュ前ci-local必須化
- [ ] リリース前チェック必須化
- [ ] テストカバレッジ確保
- [ ] ドキュメント例のテスト化
- [ ] Clippy warnings厳格対応
- [ ] cargo fmt --check 導入

### Phase 9: ライセンス・法的対応
- [ ] ライセンス表記統一
- [ ] THIRD_PARTY_LICENSES生成
- [ ] 第三者ライセンス確認
- [ ] 著作権表記統一

### Phase 10: 運用・監視標準実装  
- [ ] 適切なログ出力実装
- [ ] エラーハンドリング改善
- [ ] Graceful shutdown実装
- [ ] メトリクス計測（必要時）

### Phase 11: ドキュメント品質向上
- [ ] API documentation充実
- [ ] Examples実行可能性確認
- [ ] CHANGELOG.md作成・更新
- [ ] cargo test --doc 成功確認

### Phase 12: パフォーマンス・最適化
- [ ] cargo bench 実装
- [ ] メモリ使用量テスト
- [ ] 大きなファイル処理テスト
- [ ] プロファイリング（必要時）

### Phase 13: セキュリティ・監査
- [ ] cargo audit 成功確認
- [ ] npm audit 成功確認
- [ ] 依存関係脆弱性チェック
- [ ] 秘密情報除外確認

### Phase 14: 最終検証
- [ ] 全スクリプト動作確認
- [ ] 3言語ドキュメント同期確認
- [ ] リリースプロセス完全テスト
- [ ] 全品質基準遵守確認
- [ ] CI/CD完全成功確認

## 🎯 **ツール固有の差分**

### diffai 固有要素
```markdown
# diffai の思想
「AI駆動の差分解析で、意味のある変更を自動検出」

# 提供形態
- Rust Crate (diffai-core)
- CLI Tool (diffai)
- Python/npm wrappers
```

### lawkit 固有要素
```markdown
# lawkit の思想  
「法的文書の変更追跡と分析の専門ツール」

# 提供形態
- Rust Crate (lawkit-core)
- CLI Tool (lawkit)
- Python/npm wrappers
```

## 📐 **diffxで確立された品質基準**

### 開発環境標準
```bash
# Python環境は必ずuvで仮想環境
uv venv && source .venv/bin/activate
# システムpipは絶対に使わない
```

### ディレクトリ構造標準
```
{tool-name}/
├── .claude/              # Claude Code最適化
├── scripts/
│   ├── release/         # リリース関連
│   ├── testing/         # テスト関連
│   └── utils/           # ユーティリティ
├── {tool-name}-core/    # コアライブラリ
├── {tool-name}-cli/     # CLI実装
├── {tool-name}-python/  # Python wrapper
├── {tool-name}-npm/     # npm wrapper
├── tests/               # テスト
├── docs/                # ドキュメント
├── README.md            # 英語メイン
├── README_ja.md         # 日本語
└── README_zh.md         # 中国語
```

### ドキュメント管理標準
```markdown
# 3言語同期更新（必須）
README.md      (英語 - プライマリ)
README_ja.md   (日本語)
README_zh.md   (中国語)

# 更新時のルール
1. 英語で内容を確定
2. 日本語・中国語に翻訳
3. 3ファイル同時コミット
```

### 品質保証標準
```bash
# ドキュメント使用例 → テストケース必須
# README.mdに書いた例
diffx config.yaml config_new.yaml --output json

# 対応するテストケース必須
#[test]
fn test_yaml_json_output() {
    // README例と同じコマンドをテスト
}
```

### リリース品質標準
```bash
# プッシュ前必須チェック
./scripts/testing/ci-local.sh

# リリース前必須確認
./scripts/release/pre-release-check.sh

# バージョン管理
- ハードコード禁止
- 動的取得必須
- 全パッケージ同期必須
```

### マーケティング標準
```bash
# .claude/marketing/ 構造
├── strategy.md           # 戦略
├── content-templates.md  # 原稿テンプレート
└── execution-plan.md     # 実行計画

# 多地域対応
- 米国・インド・中国市場
- 時差考慮投稿
- プラットフォーム別最適化
```

### Git管理標準
```bash
# コミットメッセージ品質
feat: add semantic analysis for YAML files
fix: resolve path issues in release scripts
docs: update all README files (en/ja/zh)

# 必須要素
- 英語での簡潔な説明
- 影響範囲の明記
- 3言語更新時は明記
```

### テスト品質標準
```rust
// 統合テスト必須項目
#[test] 
fn test_readme_examples() {
    // README.mdの全使用例をテスト
}

#[test]
fn test_error_messages() {
    // エラーメッセージの親切さ確認
}

#[test] 
fn test_performance_benchmarks() {
    // パフォーマンス基準確認
}
```

### エラーハンドリング標準
```rust
// 親切なエラーメッセージ
anyhow::bail!("Failed to parse {}: {}\nTry: diffx --help", filename, err);

// 建設的な提案を含む
// ファイル存在確認
// 権限確認
// フォーマット提案
```

### パフォーマンス標準
```bash
# ベンチマーク必須
cargo bench

# 大きなファイルでのテスト
# メモリ使用量チェック  
# 処理時間の合理性確認
```

### セキュリティ標準
```bash
# 依存関係脆弱性チェック
cargo audit

# npm脆弱性チェック  
npm audit

# 秘密情報の除外確認
# ログに秘密情報を出力しない
```

### CLI設計標準
```bash
# 引数命名規則
--output (short: -o)          # 統一された略記法
--verbose (short: -v)         # 標準的なフラグ
--help (short: -h)            # 必須ヘルプ
--version (short: -V)         # バージョン表示

# ヘルプメッセージ品質
- 簡潔で分かりやすい説明
- 使用例を含む
- エラー時に建設的な提案
```

### ファイル管理標準
```bash
# 文字エンコーディング統一
UTF-8 without BOM

# 改行コード統一  
LF (Unix style)

# 一時ファイル管理
- 適切なクリーンアップ
- /tmp 使用時の権限考慮
- プロセス終了時の自動削除
```

### 依存関係管理標準
```toml
# 最小限の依存関係
- 必要最小限のクレート使用
- メジャーバージョン固定
- セキュリティ重視の選択

# バージョン指定方針
serde = "1.0"           # メジャー固定
clap = "4.5"            # マイナー固定
anyhow = "1.0.98"       # パッチまで固定（必要時）
```

### 国際化・アクセシビリティ標準
```rust
// エラーメッセージの多言語対応考慮
// 色覚対応（色だけに頼らない表示）
// スクリーンリーダー対応
// 高コントラスト対応
```

### パフォーマンス・メモリ管理標準
```rust
// メモリリーク対策
#[test]
fn test_memory_usage() {
    // 大きなファイル処理時のメモリ使用量確認
}

// 並行処理の適切な使用
use rayon::prelude::*;  // 必要時のみ

// ストリーミング処理
// 大きなファイルは一度にメモリに読み込まない
```

### プラットフォーム対応標準
```rust
// プラットフォーム固有処理の分離
#[cfg(target_os = "windows")]
fn platform_specific_function() { }

#[cfg(unix)]
fn platform_specific_function() { }

// パス処理の統一
use std::path::PathBuf;  // OS固有パス処理
```

### 設定・環境管理標準
```bash
# 設定ファイル場所統一
~/.config/{tool-name}/config.toml    # Linux/macOS
%APPDATA%\{tool-name}\config.toml    # Windows

# 環境変数命名
{TOOL_NAME}_CONFIG_PATH
{TOOL_NAME}_LOG_LEVEL
{TOOL_NAME}_NO_COLOR
```

### ログ・デバッグ標準
```rust
// ログレベル統一
trace!("Detailed debugging info");
debug!("Development info");  
info!("General info");
warn!("Warning condition");
error!("Error condition");

// デバッグ情報の適切な管理
#[cfg(debug_assertions)]
eprintln!("Debug: {}", debug_info);
```

### バックワード互換性標準
```rust
// 破壊的変更の回避
// 非推奨機能の適切な警告
#[deprecated(since = "1.2.0", note = "Use new_function instead")]
fn old_function() { }

// 設定ファイル形式の互換性維持
// CLI引数の後方互換性
```

### UX・ユーザビリティ標準
```bash
# プログレスバー（長時間処理）
Processing files... [████████████████████] 100%

# 人間に優しい出力
Found 3 changes in config.yaml (took 0.5s)

# カラー出力の配慮
--no-color フラグ必須
NO_COLOR 環境変数対応
```

### 開発体験標準
```rust
// 開発者向けドキュメント
/// # Examples
/// ```
/// let result = function_name("input");
/// assert_eq!(result, expected);
/// ```

// 型安全性の重視
// パニックの回避（Result型使用）
// 適切なエラー型の定義
```

### コード品質標準
```bash
# Clippy warnings厳格対応
cargo clippy -- -D warnings

# 未使用コード除去
#[allow(dead_code)]  # 例外時のみ明示的に許可

# 命名規則統一
struct ConfigParser;     # PascalCase
fn parse_config();       # snake_case
const MAX_SIZE: usize;   # SCREAMING_SNAKE_CASE
```

### ドキュメント品質標準
```rust
// API documentation必須
/// Parse configuration file and return structured data
/// 
/// # Arguments
/// * `path` - Path to configuration file
/// 
/// # Returns
/// * `Ok(Config)` - Successfully parsed configuration
/// * `Err(Error)` - Parse error with detailed message
/// 
/// # Examples
/// ```
/// let config = parse_config("config.yaml")?;
/// assert_eq!(config.database.host, "localhost");
/// ```

// Examples の実行可能性保証
cargo test --doc
```

### CHANGELOG管理標準
```markdown
# CHANGELOG.md 必須更新
## [X.Y.Z] - YYYY-MM-DD
### Added
- 新機能の説明

### Changed  
- 既存機能の変更

### Deprecated
- 非推奨機能

### Removed
- 削除された機能

### Fixed
- バグ修正

### Security
- セキュリティ修正
```

### ライセンス管理標準
```bash
# ライセンス表記統一
// Copyright (c) 2025 kako-jun
// SPDX-License-Identifier: MIT

# 第三者ライセンス管理
cargo about generate about.hbs > THIRD_PARTY_LICENSES

# ライセンス互換性確認
cargo deny check licenses
```

### CI/CD品質標準
```yaml
# 必須チェック項目
- name: Format check
  run: cargo fmt --all -- --check

- name: Clippy check  
  run: cargo clippy -- -D warnings

- name: Test coverage
  run: cargo tarpaulin --out Xml --output-dir coverage

- name: Security audit
  run: cargo audit

- name: License check
  run: cargo deny check
```

### 運用・監視標準
```rust
// 適切なログ出力
use tracing::{info, warn, error};

info!("Processing {} files", file_count);
warn!("Large file detected: {} bytes", size);
error!("Failed to process {}: {}", filename, err);

// メトリクス計測（必要時）
let start = std::time::Instant::now();
// 処理実行
let duration = start.elapsed();
debug!("Processing took {:?}", duration);
```

### 異常系処理標準
```rust
// 適切なエラーハンドリング
match result {
    Ok(data) => process_data(data),
    Err(e) => {
        error!("Processing failed: {}", e);
        eprintln!("Error: {}", e);
        // 建設的な解決提案
        eprintln!("Try: {} --help", env!("CARGO_PKG_NAME"));
        std::process::exit(1);
    }
}

// Graceful shutdown
signal::ctrl_c().await?;
info!("Shutting down gracefully...");
cleanup().await?;
```

### プロファイリング・最適化標準
```bash
# パフォーマンス測定
cargo bench

# メモリプロファイリング（必要時）
valgrind --tool=massif ./target/release/diffx

# プロファイルガイド最適化（必要時）
RUSTFLAGS="-Cprofile-generate" cargo build --release
# サンプル実行後
RUSTFLAGS="-Cprofile-use=merged.profdata" cargo build --release
```

## 🚨 **移植時の落とし穴**

### 1. **ツール名の置換漏れ**
- ファイル内容
- スクリプト内のパス
- GitHub Actions設定

### 2. **認証情報の相違**
- crates.io: 同じアカウント可能
- npm: パッケージ名が異なる
- PyPI: パッケージ名が異なる

### 3. **依存関係の相違**
- Cargo.toml の dependencies
- 各ツール固有のライブラリ

### 4. **テストの相違**
- 各ツール固有のテストケース
- CLI引数・オプションの違い

## 📊 **成功指標**

### 移植完了の判定基準
1. **ローカルビルド成功**: `cargo build --release`
2. **テスト成功**: `cargo test` + `cargo test --doc`
3. **品質チェック成功**: `cargo clippy -- -D warnings`
4. **フォーマットチェック成功**: `cargo fmt --check`
5. **セキュリティチェック成功**: `cargo audit`
6. **CI成功**: `./scripts/testing/ci-local.sh`
7. **リリース成功**: スクリプトでのリリース実行
8. **パッケージ公開**: 3プラットフォーム全て

### 品質指標
- **コード品質**: Clippy warnings = 0
- **フォーマット**: cargo fmt 準拠 100%
- **テストカバレッジ**: 主要機能 80%+
- **ドキュメント**: API documentation 100%
- **セキュリティ**: 脆弱性 0件
- **スクリプト実行**: エラー 0件
- **リリースプロセス**: マニュアル介入 0回
- **多言語対応**: 3言語同期更新 100%
- **ライセンス**: 第三者ライセンス適切管理

## 🎬 **実行段取り**

### diffai移植 (推定4-5時間)
1. **新Claude Codeセッション開始**
2. **この計画書を参照**
3. **Phase 1-14を順次実行**
   - Phase 1-4: 基本構造・ドキュメント (1時間)
   - Phase 5-7: スクリプト・パッケージング・CI/CD (2時間)  
   - Phase 8-13: 品質基準・ライセンス・最適化 (1.5時間)
   - Phase 14: 最終検証 (0.5時間)

### lawkit移植 (推定2-3時間)
1. **diffaiパターンを適用** (1時間)
2. **lawkit固有部分の調整** (1時間)
3. **全品質基準の最終検証** (1時間)

---

**この計画書に従えば、diffxの成功パターンを100%再現できます。**
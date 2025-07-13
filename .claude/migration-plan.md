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
- [ ] `scripts/` 分類・整理
- [ ] `.gitignore` 更新

### Phase 2: Claude Code最適化
- [ ] `.claude/release-guide.md` 作成
- [ ] `.claude/tasks.md` 作成
- [ ] コンテキスト効率化実装

### Phase 3: スクリプト移植・修正
- [ ] リリーススクリプト移植
- [ ] パス問題修正 (プロジェクトルート検出)
- [ ] 動的バージョン管理実装
- [ ] エラーハンドリング強化

### Phase 4: パッケージング現代化
- [ ] maturin Python実装
- [ ] ユニバーサルnpm実装
- [ ] マルチプラットフォーム対応

### Phase 5: CI/CD 2幕構成
- [ ] Release Act1 実装
- [ ] Release Act2 実装
- [ ] GitHub設定調整

### Phase 6: 開発フロー改善
- [ ] GitHub Flow緩和
- [ ] 認証設定確認
- [ ] 権限問題解決

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
2. **テスト成功**: `cargo test`
3. **CI成功**: `./scripts/testing/ci-local.sh`
4. **リリース成功**: スクリプトでのリリース実行
5. **パッケージ公開**: 3プラットフォーム全て

### 品質指標
- スクリプト実行でエラー0
- マニュアル介入0でリリース完了
- 全プラットフォームでパッケージ利用可能

## 🎬 **実行段取り**

### diffai移植 (推定2-3時間)
1. **新Claude Codeセッション開始**
2. **この計画書を参照**
3. **Phase 1-6を順次実行**
4. **テスト・検証**

### lawkit移植 (推定1-2時間)
1. **diffaiパターンを適用**
2. **lawkit固有部分の調整**
3. **最終検証**

---

**この計画書に従えば、diffxの成功パターンを100%再現できます。**
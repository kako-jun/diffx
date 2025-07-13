# diffx実際の成功パターン移植計画書

## 🎯 **diffxで実際に達成済みの要素のみ**

### 1. **ディレクトリ構造**
```
{tool-name}/
├── .claude/              # Claude Code最適化
│   ├── release-guide.md  # AI向けリリース指示
│   ├── tasks.md          # タスク管理
│   └── marketing/        # マーケティング戦略
├── scripts/              # 機能別分類
│   ├── release/         # リリース関連スクリプト
│   ├── testing/         # テスト関連スクリプト
│   └── utils/           # ユーティリティスクリプト
├── {tool-name}-core/    # コアライブラリ
├── {tool-name}-cli/     # CLI実装
├── {tool-name}-python/  # Python wrapper (maturin)
├── {tool-name}-npm/     # npm wrapper (universal)
├── tests/               # テスト
├── docs/                # ドキュメント
├── README.md            # 英語
├── README_ja.md         # 日本語
├── README_zh.md         # 中国語
├── CHANGELOG.md         # 変更履歴
└── CLAUDE.md            # 目次化済み
```

### 2. **CLAUDE.md目次化**
```markdown
# {TOOL_NAME} の思想（Philosophy）
{ツール固有の説明}

# 🚨 重要な開発ルール
## Claude対応時の必須ルール
## プッシュ前の必須チェック
## コンテキスト効率化ルール

**CLAUDE.mdは目次として使用し、詳細情報は以下の専用ファイルを参照:**
- **📋 タスクリスト**: `.claude/tasks.md` を参照
- **🚀 リリース手順**: `.claude/release-guide.md` を参照

**重要**: 詳細が必要な時のみ該当ファイルを読むこと。CLAUDE.md自体は最小限に保つ。
```

### 3. **3言語ドキュメント**
- README.md (英語プライマリ)
- README_ja.md (日本語)
- README_zh.md (中国語)
- 3ファイル同時更新

### 4. **Python maturin化**
```toml
# pyproject.toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"
```

### 5. **npm ユニバーサルパッケージ**
```javascript
// 全プラットフォームバイナリ同梱
bin/
├── linux-x64/diffx
├── darwin-x64/diffx
├── darwin-arm64/diffx
└── win32-x64/diffx.exe
```

### 6. **GitHub Actions 2幕構成**
- **Act1**: Rustビルド + GitHub Release + crates.io
- **Act2**: npm公開 + PyPI公開

### 7. **スクリプト分類・修正**
- パス問題修正（プロジェクトルート検出）
- 動的バージョン管理
- 失敗時クリーンアップ

### 8. **uv仮想環境使用**
```bash
uv venv && source .venv/bin/activate
```

## 📝 **実用的移植チェックリスト**

### Phase 1: 基本構造移植
- [ ] `.claude/` ディレクトリ作成
- [ ] `CLAUDE.md` 目次化
- [ ] `scripts/` 分類（release/testing/utils）
- [ ] ディレクトリ構造整備

### Phase 2: ドキュメント移植
- [ ] `README_ja.md`, `README_zh.md` 作成
- [ ] `.claude/tasks.md` 作成
- [ ] `.claude/release-guide.md` 作成
- [ ] `.claude/marketing/` 作成

### Phase 3: スクリプト移植
- [ ] リリーススクリプト移植
- [ ] テストスクリプト移植
- [ ] パス問題修正適用
- [ ] 動的バージョン管理適用

### Phase 4: パッケージング現代化
- [ ] maturin Python実装
- [ ] ユニバーサルnpm実装
- [ ] 依存関係整理

### Phase 5: CI/CD移植
- [ ] GitHub Actions 2幕構成実装
- [ ] リリースワークフロー移植
- [ ] 認証設定

### Phase 6: 最終確認
- [ ] 基本ビルド成功
- [ ] スクリプト動作確認
- [ ] リリースプロセステスト

## 🎯 **ツール固有の調整点**

### diffai 固有
```markdown
# diffai の思想
「AI駆動の差分解析で、意味のある変更を自動検出」
```

### lawkit 固有
```markdown
# lawkit の思想  
「法的文書の変更追跡と分析の専門ツール」
```

## 📊 **移植完了基準**

### 必須達成項目
1. **ディレクトリ構造**: diffxと同じ構造
2. **ドキュメント**: 3言語README完備
3. **スクリプト**: 基本的なリリース・テストスクリプト動作
4. **パッケージング**: maturin + universal npm
5. **CI/CD**: 2幕構成で動作

### 品質指標
- **ローカルビルド**: `cargo build --release` 成功
- **基本テスト**: `cargo test` 成功
- **スクリプト**: エラーなく実行
- **リリース**: 自動化で完了

## 🎬 **実行段取り**

### diffai移植 (推定2-3時間)
1. **新Claude Codeセッション開始**
2. **この計画書参照**
3. **Phase 1-6順次実行**

### lawkit移植 (推定1-2時間)
1. **diffaiパターン適用**
2. **固有部分調整**

---

**この計画書はdiffxの実際の達成済み要素のみを記録しています。**
**diffxに適用しても「やることがない」レベルの現実的な基準です。**
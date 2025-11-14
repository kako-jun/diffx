# 移行後のセットアップガイド

## 📍 リポジトリ構成

```
/home/d131/repos/2025/
├── diffx/          # Rust専用（このリポジトリ）
├── diffx-js/       # JavaScript/npm
└── diffx-python/   # Python/PyPI
```

## 🎯 移行の確認

### 移行スクリプト実行

```bash
cd /home/d131/repos/2025/diffx
chmod +x .claude/reboot/migrate-to-separate-repos.sh
./.claude/reboot/migrate-to-separate-repos.sh
```

## 🦀 diffx (Rust専用) のセットアップ

### 現在の状態確認

```bash
cd /home/d131/repos/2025/diffx

# ディレクトリ構造確認
ls -la
# 期待: diffx-core/, diffx-cli/, Cargo.toml

# ビルド確認
cargo build --release

# テスト確認
cargo test --workspace

# CLI確認
./target/release/diffx --version
```

### README.md 作成

```bash
cd /home/d131/repos/2025/diffx
```

`README.md` を以下の内容で作成:

```markdown
# diffx

高速な構造化データ差分抽出ツール（Rust製）

## 特徴

- JSON, YAML, TOML, XML, INI, CSV 対応
- 構造を理解した意味のある差分
- 高速・軽量
- クロスプラットフォーム

## インストール

### Cargo経由
```bash
cargo install diffx
```

### ソースから
```bash
git clone https://github.com/kako-jun/diffx
cd diffx
cargo build --release
```

## 基本的な使い方

```bash
# JSON ファイルの差分
diffx config.json config.new.json

# 出力フォーマット指定
diffx file1.yaml file2.yaml --output json

# ヘルプ
diffx --help
```

## ドキュメント

詳細なドキュメント: 準備中

## 他言語サポート

- JavaScript/npm: [diffx-js](https://github.com/kako-jun/diffx-js)
- Python/PyPI: [diffx-python](https://github.com/kako-jun/diffx-python)

## ライセンス

MIT License

## 開発

```bash
# ビルド
cargo build --release

# テスト
cargo test --workspace

# フォーマット
cargo fmt --all

# Lint
cargo clippy --workspace
```
```

### .github/workflows/ の整理

```bash
cd /home/d131/repos/2025/diffx

# 既存のワークフローを確認
ls -la .github/workflows/

# 新しいシンプルなワークフローを作成（別途提供）
```

### コミット

```bash
cd /home/d131/repos/2025/diffx

git add .
git commit -m "refactor: migrate to Rust-only repository

- Remove diffx-js and diffx-python (moved to separate repos)
- Update Cargo.toml workspace members
- Simplify to Rust-only structure
- Update README.md for Rust-focused documentation

Related repositories:
- JavaScript: https://github.com/kako-jun/diffx-js
- Python: https://github.com/kako-jun/diffx-python
"
git push origin main
```

## 📦 diffx-js のセットアップ

### 初期確認

```bash
cd /home/d131/repos/2025/../diffx-js

# ファイル確認
ls -la

# 期待されるファイル
# - Cargo.toml (Rust部分)
# - package.json
# - index.js
# - lib/ または src/
```

### Cargo.toml の修正

`Cargo.toml` を編集して、diffx-core の依存を crates.io に変更:

```toml
[package]
name = "diffx-js"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
# ローカルパスから crates.io に変更
diffx-core = "0.6.0"  # ← これを変更

# その他の依存関係はそのまま
napi = "2"
napi-derive = "2"
```

### README.md 作成

```markdown
# diffx-js

[diffx](https://github.com/kako-jun/diffx) の JavaScript/Node.js バインディング

## インストール

```bash
npm install diffx-js
```

## 使い方

```javascript
const diffx = require('diffx-js');

// 基本的な使い方
const result = diffx.diff(obj1, obj2);
console.log(result);
```

## ドキュメント

準備中

## Rust版

本家Rustツール: [diffx](https://github.com/kako-jun/diffx)

## ライセンス

MIT License
```

### .gitignore 作成

```gitignore
# Node
node_modules/
npm-debug.log
yarn-error.log

# Rust
target/
Cargo.lock

# Build
dist/
*.node

# OS
.DS_Store
Thumbs.db

# IDE
.vscode/
.idea/
```

### 初期コミット

```bash
cd /home/d131/repos/2025/../diffx-js

git add .
git commit -m "Initial commit: diffx JavaScript bindings

Migrated from kako-jun/diffx monorepo.
Updated dependencies to use diffx-core from crates.io.
"
git push origin main
```

## 🐍 diffx-python のセットアップ

### 初期確認

```bash
cd /home/d131/repos/2025/../diffx-python

# ファイル確認
ls -la

# 期待されるファイル
# - Cargo.toml (Rust部分)
# - pyproject.toml
# - src/lib.rs
# - python/diffx_python/
```

### Cargo.toml の修正

`Cargo.toml` を編集:

```toml
[package]
name = "diffx-python"
version = "0.1.0"
edition = "2021"

[lib]
name = "diffx_python"
crate-type = ["cdylib"]

[dependencies]
# ローカルパスから crates.io に変更
diffx-core = "0.6.0"  # ← これを変更

# PyO3関連はそのまま
pyo3 = { version = "0.22", features = ["extension-module"] }
```

### pyproject.toml の確認

```toml
[build-system]
requires = ["maturin>=1.0,<2.0"]
build-backend = "maturin"

[project]
name = "diffx-python"
description = "Python bindings for diffx"
readme = "README.md"
requires-python = ">=3.8"
license = { text = "MIT" }
keywords = ["diff", "json", "yaml"]
classifiers = [
    "Programming Language :: Python",
    "Programming Language :: Rust",
]
dynamic = ["version"]

[tool.maturin]
features = ["pyo3/extension-module"]
```

### README.md 作成

```markdown
# diffx-python

[diffx](https://github.com/kako-jun/diffx) の Python バインディング

## インストール

```bash
pip install diffx-python
```

## 使い方

```python
import diffx_python

# 基本的な使い方
result = diffx_python.diff(obj1, obj2)
print(result)
```

## ドキュメント

準備中

## Rust版

本家Rustツール: [diffx](https://github.com/kako-jun/diffx)

## ライセンス

MIT License
```

### .gitignore 作成

```gitignore
# Python
__pycache__/
*.py[cod]
*$py.class
*.so
.Python
env/
venv/
.venv/
*.egg-info/
dist/
build/

# Rust
target/
Cargo.lock

# OS
.DS_Store
Thumbs.db

# IDE
.vscode/
.idea/
*.swp
```

### 初期コミット

```bash
cd /home/d131/repos/2025/../diffx-python

git add .
git commit -m "Initial commit: diffx Python bindings

Migrated from kako-jun/diffx monorepo.
Updated dependencies to use diffx-core from crates.io.
"
git push origin main
```

## 📋 チェックリスト

### diffx (Rust)
- [ ] 移行スクリプト実行完了
- [ ] ビルド成功: `cargo build --release`
- [ ] テスト成功: `cargo test --workspace`
- [ ] README.md 作成
- [ ] コミット・プッシュ完了

### diffx-js
- [ ] ファイル移行確認
- [ ] Cargo.toml 修正（crates.io依存）
- [ ] README.md 作成
- [ ] .gitignore 作成
- [ ] 初期コミット・プッシュ完了

### diffx-python
- [ ] ファイル移行確認
- [ ] Cargo.toml 修正（crates.io依存）
- [ ] pyproject.toml 確認
- [ ] README.md 作成
- [ ] .gitignore 作成
- [ ] 初期コミット・プッシュ完了

## 🚨 注意事項

### crates.io公開が必要

diffx-js と diffx-python は diffx-core に依存するため、
**diffx-core を crates.io に公開してから**
JS/Python版のビルドを試してください。

```bash
# diffx-core の公開（diffxリポジトリで実行）
cd /home/d131/repos/2025/diffx/diffx-core
cargo publish
```

### 公開前の開発

crates.io公開前に開発したい場合は、一時的にローカルパスを使用:

```toml
# 開発時のみ
[dependencies]
diffx-core = { path = "../diffx/diffx-core" }

# 公開時
[dependencies]
diffx-core = "0.6.0"
```

## 📊 移行後のワークフロー

### 通常の開発サイクル

1. **diffx (Rust) の開発**
   ```bash
   cd /home/d131/repos/2025/diffx
   # コード変更
   cargo test
   cargo build --release
   git commit && git push
   ```

2. **diffx-core の公開**（必要時）
   ```bash
   cd diffx-core
   # バージョンアップ
   cargo publish
   ```

3. **diffx-js の更新**（必要時）
   ```bash
   cd /home/d131/repos/2025/../diffx-js
   # Cargo.toml の diffx-core バージョン更新
   cargo build
   npm test
   git commit && git push
   ```

4. **diffx-python の更新**（必要時）
   ```bash
   cd /home/d131/repos/2025/../diffx-python
   # Cargo.toml の diffx-core バージョン更新
   maturin develop
   pytest
   git commit && git push
   ```

## 🎯 成功の確認

### すべて完了したら

```bash
# diffx
cd /home/d131/repos/2025/diffx
cargo build --release
./target/release/diffx --version

# diffx-js
cd /home/d131/repos/2025/../diffx-js
cargo build  # Rust部分
npm install  # Node部分
npm test

# diffx-python
cd /home/d131/repos/2025/../diffx-python
maturin develop
python -c "import diffx_python; print('OK')"
```

すべて成功すれば、移行完了！

## 💡 次のステップ

1. **diffx のリブート継続**
   - `.claude/reboot/clean-slate-plan.md` に従う
   - 仕様書作成
   - GitHub Actions整理

2. **diffx-js, diffx-python は後回し**
   - diffx が安定してから本格開発
   - 当面は最小限の動作確認のみ

---

**作成日**: 2025-11-14
**目的**: モノレポから別リポジトリへの移行ガイド

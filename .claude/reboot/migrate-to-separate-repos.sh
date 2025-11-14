#!/usr/bin/env bash
# diffx-js, diffx-python を別リポジトリに移行するスクリプト

set -e

echo "🚀 別リポジトリへの移行スクリプト"
echo "================================"
echo ""
echo "このスクリプトは："
echo "  - diffx/diffx-js/ → ../diffx-js/ に移動"
echo "  - diffx/diffx-python/ → ../diffx-python/ に移動"
echo "  - diffx を Rust専用にクリーンアップ"
echo ""
read -p "続行しますか？ (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "中止しました。"
    exit 0
fi

# プロジェクトルートに移動
cd "$(dirname "$0")/../.."
PROJECT_ROOT=$(pwd)
echo ""
echo "📍 プロジェクトルート: $PROJECT_ROOT"

# 移動先の確認
echo ""
echo "🔍 移動先リポジトリの確認..."

if [ ! -d "../diffx-js/.git" ]; then
    echo "❌ エラー: ../diffx-js/ が Git リポジトリではありません"
    echo "   先に新規リポジトリを作成してクローンしてください"
    exit 1
fi

if [ ! -d "../diffx-python/.git" ]; then
    echo "❌ エラー: ../diffx-python/ が Git リポジトリではありません"
    echo "   先に新規リポジトリを作成してクローンしてください"
    exit 1
fi

echo "✅ ../diffx-js/ - OK"
echo "✅ ../diffx-python/ - OK"

# 移動元の確認
echo ""
echo "🔍 移動元の確認..."

if [ ! -d "diffx-js" ]; then
    echo "⚠️  警告: diffx-js/ が存在しません"
    SKIP_JS=true
else
    echo "✅ diffx-js/ - 存在"
    SKIP_JS=false
fi

if [ ! -d "diffx-python" ]; then
    echo "⚠️  警告: diffx-python/ が存在しません"
    SKIP_PYTHON=true
else
    echo "✅ diffx-python/ - 存在"
    SKIP_PYTHON=false
fi

# バックアップ作成
echo ""
echo "💾 バックアップを作成..."
mkdir -p _old/backup_before_migration

if [ "$SKIP_JS" = false ]; then
    echo "  - diffx-js/ → _old/backup_before_migration/"
    cp -r diffx-js _old/backup_before_migration/
fi

if [ "$SKIP_PYTHON" = false ]; then
    echo "  - diffx-python/ → _old/backup_before_migration/"
    cp -r diffx-python _old/backup_before_migration/
fi

# diffx-js の移動
if [ "$SKIP_JS" = false ]; then
    echo ""
    echo "📦 diffx-js を移行中..."

    # 内容をコピー（.gitを除く）
    echo "  - ファイルをコピー..."
    rsync -av --exclude='.git' --exclude='target' --exclude='node_modules' \
        diffx-js/ ../diffx-js/

    echo "  - 完了"
fi

# diffx-python の移動
if [ "$SKIP_PYTHON" = false ]; then
    echo ""
    echo "📦 diffx-python を移行中..."

    # 内容をコピー（.gitを除く）
    echo "  - ファイルをコピー..."
    rsync -av --exclude='.git' --exclude='target' --exclude='__pycache__' \
        --exclude='.venv' --exclude='*.egg-info' \
        diffx-python/ ../diffx-python/

    echo "  - 完了"
fi

# 元のディレクトリを削除
echo ""
echo "🗑️  元のディレクトリを削除..."

if [ "$SKIP_JS" = false ]; then
    echo "  - diffx-js/ を削除"
    rm -rf diffx-js
fi

if [ "$SKIP_PYTHON" = false ]; then
    echo "  - diffx-python/ を削除"
    rm -rf diffx-python
fi

# Cargo.toml のクリーンアップ
echo ""
echo "📝 Cargo.toml をクリーンアップ..."

# バックアップ
cp Cargo.toml Cargo.toml.backup

# workspace メンバーから削除
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = [
    "diffx-core",
    "diffx-cli",
]

[workspace.package]
version = "0.6.0"
edition = "2021"
authors = ["kako-jun"]
license = "MIT"
description = "Blazing fast semantic diff for JSON/YAML/TOML/XML/INI/CSV. Features: array tracking, regex filters, float tolerance, directory compare, UNIX-compatible options"
homepage = "https://github.com/kako-jun/diffx"
repository = "https://github.com/kako-jun/diffx"
documentation = "https://docs.rs/diffx"
readme = "README.md"
keywords = ["diff", "json", "yaml", "toml", "semantic-diff"]
categories = ["command-line-utilities", "development-tools", "text-processing", "parsing", "data-structures"]
exclude = [
    ".github/",
    "target/",
    "_old/"
]
rust-version = "1.75"

[workspace.metadata.docs.rs.badges]
maintenance = { status = "actively-developed" }

[workspace.metadata.docs.rs]
all-features = true
rustdoc-args = ["--cfg", "docsrs"]

[workspace.dependencies]
diffx-core = { version = "0.6.0", path = "diffx-core" }
anyhow = "1.0"
clap = { version = "4.0", features = ["derive", "cargo"] }
colored = "3.0"
csv = "1.3"
configparser = "3.0"
quick-xml = { version = "0.31", features = ["serialize"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"
serde_yml = "0.0.12"
toml = "0.8"
walkdir = "2.5"
dirs = "5.0"
regex = "1.0"
EOF

echo "  - Cargo.toml 更新完了"
echo "  - バックアップ: Cargo.toml.backup"

# .gitignore の更新
echo ""
echo "📝 .gitignore を更新..."

if ! grep -q "^_old/" .gitignore 2>/dev/null; then
    echo "_old/" >> .gitignore
    echo "  - _old/ を追加"
fi

if ! grep -q "^Cargo.toml.backup$" .gitignore 2>/dev/null; then
    echo "Cargo.toml.backup" >> .gitignore
    echo "  - Cargo.toml.backup を追加"
fi

# ビルド確認
echo ""
echo "🔨 ビルド確認..."
if cargo build --release 2>&1 | tee _old/build-after-migration.txt; then
    echo "✅ ビルド成功"
else
    echo "❌ ビルド失敗"
    echo "   詳細: _old/build-after-migration.txt"
    exit 1
fi

# テスト確認
echo ""
echo "🧪 テスト確認..."
if cargo test --workspace 2>&1 | tee _old/test-after-migration.txt; then
    echo "✅ テスト成功"
else
    echo "⚠️  テスト失敗"
    echo "   詳細: _old/test-after-migration.txt"
fi

# 移行先の確認
echo ""
echo "🔍 移行先リポジトリの確認..."

echo ""
echo "📦 ../diffx-js/ の状態:"
ls -la ../diffx-js/
echo ""
echo "📦 ../diffx-python/ の状態:"
ls -la ../diffx-python/

# 完了メッセージ
echo ""
echo "✅ 移行完了！"
echo ""
echo "📋 次のステップ:"
echo ""
echo "1. diffx (Rust専用) の確認:"
echo "   cd $PROJECT_ROOT"
echo "   cargo build --release"
echo "   cargo test"
echo ""
echo "2. diffx-js リポジトリのセットアップ:"
echo "   cd ../diffx-js"
echo "   git status"
echo "   # 必要に応じて初期設定"
echo "   # - README.md 作成"
echo "   # - .gitignore 作成"
echo "   # - Cargo.toml の依存関係を crates.io に変更"
echo ""
echo "3. diffx-python リポジトリのセットアップ:"
echo "   cd ../diffx-python"
echo "   git status"
echo "   # 必要に応じて初期設定"
echo "   # - README.md 作成"
echo "   # - .gitignore 作成"
echo "   # - Cargo.toml の依存関係を crates.io に変更"
echo ""
echo "💡 バックアップ:"
echo "   _old/backup_before_migration/ に元のファイルを保存済み"
echo ""
echo "🎉 完了！"

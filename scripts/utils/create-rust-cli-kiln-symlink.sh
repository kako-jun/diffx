#!/bin/bash
set -euo pipefail

# プロジェクトルートに移動
cd "$(dirname "$0")/../.."

# mntディレクトリが存在しない場合は作成
mkdir -p mnt

# 既存のシンボリックリンクがある場合は削除
if [ -L mnt/rust-cli-kiln ]; then
    rm mnt/rust-cli-kiln
    echo "🔄 既存のシンボリックリンクを削除"
elif [ -e mnt/rust-cli-kiln ]; then
    echo "❌ エラー: mnt/rust-cli-kiln が既に存在します（シンボリックリンクではありません）"
    exit 1
fi

# mnt以下にシンボリックリンクを作成
ln -s ../../.github/rust-cli-kiln mnt/rust-cli-kiln

echo "✅ シンボリックリンク作成完了: mnt/rust-cli-kiln -> ../../.github/rust-cli-kiln"
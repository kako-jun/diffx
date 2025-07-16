#!/bin/bash
set -euo pipefail

# プロジェクトルートに移動
cd "$(dirname "$0")/../.."

# 古い .github-shared シンボリックリンクがある場合は削除
if [ -L .github-shared ]; then
    rm .github-shared
    echo "🔄 古い .github-shared シンボリックリンクを削除"
fi

# 既存の github-shared がある場合は削除
if [ -L github-shared ]; then
    rm github-shared
    echo "🔄 既存の github-shared シンボリックリンクを削除"
elif [ -e github-shared ]; then
    echo "❌ エラー: github-shared が既に存在します（シンボリックリンクではありません）"
    exit 1
fi

# プロジェクトルート直下にシンボリックリンクを作成
ln -s ../.github github-shared

echo "✅ シンボリックリンク作成完了: github-shared -> ../.github"
echo "📝 GitHub Actionsと同じパス名で統一されました"
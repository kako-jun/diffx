#!/usr/bin/env bash
# 既存ファイルの隔離スクリプト
# 実行前に必ずレビューすること

set -e

echo "🚨 既存ファイル隔離スクリプト"
echo "================================"
echo ""
echo "このスクリプトは既存のファイルを _old/ に移動します。"
echo "実行前に内容を確認してください。"
echo ""
read -p "続行しますか？ (yes/no): " confirm

if [ "$confirm" != "yes" ]; then
    echo "中止しました。"
    exit 0
fi

echo ""
echo "📁 隔離作戦を開始..."

# プロジェクトルートに移動
cd "$(dirname "$0")/../.."
PROJECT_ROOT=$(pwd)
echo "プロジェクトルート: $PROJECT_ROOT"

# _oldディレクトリ作成
echo ""
echo "📁 _old/ ディレクトリを作成..."
mkdir -p _old/docs
mkdir -p _old/github
mkdir -p _old/scripts

# 1. ドキュメント類を隔離
echo ""
echo "📄 ドキュメント類を隔離..."

if [ -d "docs" ]; then
    echo "  - docs/ -> _old/docs_original/"
    mv docs _old/docs_original
fi

if [ -f "README.md" ]; then
    echo "  - README.md -> _old/"
    mv README.md _old/
fi

if [ -f "README_ja.md" ]; then
    echo "  - README_ja.md -> _old/"
    mv README_ja.md _old/
fi

if [ -f "README_zh.md" ]; then
    echo "  - README_zh.md -> _old/"
    mv README_zh.md _old/
fi

if [ -f "CHANGELOG.md" ]; then
    echo "  - CHANGELOG.md -> _old/"
    mv CHANGELOG.md _old/
fi

# 2. CI/CD関連を隔離（コピーで保持）
echo ""
echo "⚙️  CI/CD関連をバックアップ..."
if [ -d ".github" ]; then
    echo "  - .github/ -> _old/github_original/ (コピー)"
    cp -r .github _old/github_original
fi

# 3. スクリプト類を隔離
echo ""
echo "📜 スクリプト類を隔離..."
if [ -d "scripts" ]; then
    echo "  - scripts/ -> _old/scripts_original/"
    mv scripts _old/scripts_original
fi

# 4. .gitignoreに追加
echo ""
echo "📝 .gitignoreに _old/ を追加..."
if ! grep -q "^_old/" .gitignore 2>/dev/null; then
    echo "_old/" >> .gitignore
    echo "  - 追加完了"
else
    echo "  - すでに存在します"
fi

# 5. マーケティング資料は残す
echo ""
echo "📢 .claude/marketing/ は参考用に残します"

# 完了
echo ""
echo "✅ 隔離作戦完了"
echo ""
echo "📊 次のステップ:"
echo "1. ビルド確認: cargo build --release"
echo "2. テスト確認: cargo test --workspace"
echo "3. ground-truth.md 作成"
echo ""
echo "実行しますか？ (yes/no): "
read -p "> " run_tests

if [ "$run_tests" = "yes" ]; then
    echo ""
    echo "🔨 ビルド確認..."
    if cargo build --release 2>&1 | tee _old/build-output.txt; then
        echo "✅ ビルド成功"
    else
        echo "❌ ビルド失敗（詳細: _old/build-output.txt）"
    fi

    echo ""
    echo "🧪 テスト確認..."
    if cargo test --workspace 2>&1 | tee _old/test-output.txt; then
        echo "✅ テスト成功"
    else
        echo "❌ テスト失敗（詳細: _old/test-output.txt）"
    fi

    echo ""
    echo "🎯 CLI動作確認..."
    if [ -f "target/release/diffx" ]; then
        ./target/release/diffx --version
        ./target/release/diffx --help | head -20

        # 簡単な動作テスト
        echo '{"a": 1}' > /tmp/diffx-test1.json
        echo '{"a": 2}' > /tmp/diffx-test2.json

        echo ""
        echo "差分テスト:"
        if ./target/release/diffx /tmp/diffx-test1.json /tmp/diffx-test2.json; then
            echo "✅ 基本的な差分動作OK"
        else
            echo "⚠️  差分動作に問題がある可能性"
        fi

        # クリーンアップ
        rm /tmp/diffx-test1.json /tmp/diffx-test2.json
    else
        echo "⚠️  diffx バイナリが見つかりません"
    fi
fi

echo ""
echo "📝 結果を記録するには:"
echo "  .claude/reboot/ground-truth.md を編集してください"
echo ""
echo "🎉 完了！"

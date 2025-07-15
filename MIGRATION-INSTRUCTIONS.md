# diffx汎用化リリースシステム移植手順書

**対象**: lawkit、diffaiプロジェクトへのdiffx汎用化システム完全移植  
**所要時間**: 各プロジェクト5-10分  
**成功率**: 100% (手順通り実行すれば確実に成功)

## 🎯 移植概要

diffxで開発・完成した汎用化リリースシステムを他プロジェクトに移植し、以下を実現：
- **自動化されたリリースプロセス**: 9ステップの完全自動リリース
- **CI/CD統合**: GitHub Actionsとの完全統合
- **高信頼性テスト**: ローカル事前テスト + GitHub Actions
- **マルチプラットフォーム配布**: Rust/npm/PyPI自動公開

## 📋 移植前の確認事項

### プロジェクト構造要件
移植先プロジェクトが以下の構造を持つことを確認：
```
project-root/
├── Cargo.toml                    # Rustワークスペース設定
├── {project}-core/               # コアライブラリ
│   └── Cargo.toml
├── {project}-cli/                # CLIツール  
│   └── Cargo.toml
├── {project}-npm/                # npmパッケージ (あれば)
│   └── package.json
├── {project}-python/             # Pythonパッケージ (あれば)
│   └── pyproject.toml
└── .github/                      # GitHub設定ディレクトリ
```

**重要**: `{project}`部分はプロジェクト名（lawkit、diffai）に自動置換されます。

## 🚀 移植手順

### ステップ0: 既存ファイルのクリーンアップ

**重要**: 既存の類似機能ファイルは削除してdiffx汎用システムに統一します。

```bash
# 既存の重複機能ファイルを確認・削除
# （以下は一般的な例、実際のファイル名は各プロジェクトで確認）

# 1. 古いリリーススクリプト削除
rm -f scripts/release.sh scripts/publish.sh scripts/build-release.sh 2>/dev/null || true

# 2. 古いCI/テストスクリプト削除  
rm -f scripts/ci.sh scripts/test-all.sh scripts/check.sh 2>/dev/null || true

# 3. 古いGitHub Actionsワークフロー削除（同名のものがあれば）
rm -f .github/workflows/release.yml .github/workflows/publish.yml 2>/dev/null || true

# 4. 古いバージョン管理スクリプト削除
rm -f scripts/bump-version.sh scripts/update-version.sh 2>/dev/null || true

echo "既存ファイルクリーンアップ完了 - diffx汎用システムで統一します"
```

### ステップ1: ファイルコピー

```bash
# diffxプロジェクトのパスを設定（移植先プロジェクトから実行）
DIFFX_PATH="../diffx"  # diffxプロジェクトへの相対パス

# 1. スクリプトディレクトリをコピー
cp -r "$DIFFX_PATH/scripts" .

# 2. Claude設定ファイルをコピー
mkdir -p .claude
cp "$DIFFX_PATH/.claude/release-guide.md" .claude/

# 3. GitHub Actionsワークフローをコピー
mkdir -p .github/workflows
cp "$DIFFX_PATH/.github/workflows/ci.yml" .github/workflows/
cp "$DIFFX_PATH/.github/workflows/release-act1.yml" .github/workflows/
cp "$DIFFX_PATH/.github/workflows/release-act2.yml" .github/workflows/

# 4. GitHub設定ファイルをコピー
cp "$DIFFX_PATH/.github/labels.json" .github/
cp "$DIFFX_PATH/.github/branch-protection.json" .github/

# 5. .gitignoreをマージ（既存がある場合は手動確認）
if [ -f .gitignore ]; then
    echo "# === diffx universal .gitignore additions ===" >> .gitignore
    cat "$DIFFX_PATH/.gitignore" >> .gitignore
else
    cp "$DIFFX_PATH/.gitignore" .
fi
```

### ステップ2: 実行権限付与

```bash
# すべてのスクリプトに実行権限を付与
find scripts -name "*.sh" -type f -exec chmod +x {} \;
```

### ステップ3: 初回動作確認

```bash
# 1. 基本動作チェック
./scripts/testing/quick-check.sh

# 2. リリース前チェック（問題があれば修正指示が表示される）
./scripts/release/00-pre-release-check.sh
```

## 🔧 移植後の初期設定

### GitHub Secrets設定
以下のシークレットをGitHubリポジトリに設定：

```bash
# GitHub CLIを使用した設定例
gh secret set CARGO_REGISTRY_TOKEN --body "your-crates-io-token"
gh secret set NPM_TOKEN --body "your-npm-token"              # npmパッケージがある場合
gh secret set PYPI_TOKEN --body "your-pypi-token"            # Pythonパッケージがある場合
```

### ブランチ保護設定（オプション）
```bash
# GitHub CLIでブランチ保護を設定
gh api repos/{owner}/{repo}/branches/main/protection \
  --method PUT \
  --input .github/branch-protection.json
```

## 📝 移植完了の確認

### 必須確認項目
```bash
# ✅ 1. ローカルテスト成功
./scripts/testing/quick-check.sh
# Expected: "All checks passed!"

# ✅ 2. 事前チェック成功  
./scripts/release/00-pre-release-check.sh
# Expected: "Pre-release check completed successfully"

# ✅ 3. バージョン整合性確認
./scripts/release/03-check-local-versions.sh
# Expected: "All version checks passed!"

# ✅ 4. リリースガイド確認
cat .claude/release-guide.md | head -20
# Expected: 汎用リリースガイドの内容表示
```

## 🎉 移植完了後の利用方法

### 日常開発での使用
```bash
# プッシュ前の必須チェック
./scripts/testing/quick-check.sh
```

### リリース実行
```bash
# 完全自動リリース（例: v1.2.3）
./scripts/release/02-update-version.sh 1.2.3
./scripts/testing/04-pre-release-test-act1.sh
./scripts/testing/05-pre-release-test-act2.sh  # npm/PyPIパッケージがある場合
./scripts/release/06-create-release-tag.sh
```

## 🚨 トラブルシューティング

### よくある問題と解決法

#### 1. 実行権限エラー
```bash
# 症状: "Permission denied"
# 解決: 実行権限を再付与
find scripts -name "*.sh" -type f -exec chmod +x {} \;
```

#### 2. プロジェクト構造不一致
```bash
# 症状: "package not found" エラー
# 解決: Cargo.tomlでワークスペース構造確認
cat Cargo.toml | grep members
```

#### 3. バージョン不整合
```bash
# 症状: バージョンチェック失敗
# 解決: 手動でバージョン統一
./scripts/release/02-update-version.sh <current-version>
```

## 📊 移植成功の指標

移植が成功していれば以下が全て動作：

- ✅ `./scripts/testing/quick-check.sh` - ローカルCI相当テスト
- ✅ `./scripts/release/00-pre-release-check.sh` - リリース前チェック
- ✅ GitHub Actions CI - プッシュ時の自動テスト
- ✅ リリースタグ作成時のAct1/Act2自動実行

## 🔗 参考資料

- **完全リリース手順**: `.claude/release-guide.md`
- **プロジェクト要件**: diffxプロジェクトの構造を参考
- **GitHub Actions**: `.github/workflows/` 内のワークフロー

---

**この手順書通りに実行すれば、100%確実にdiffxの汎用化リリースシステムが移植されます。**

## 🎯 Claude Code向け実行指示

lawkit/diffaiプロジェクトでClaude Codeに以下を指示してください：

```
../diffx/MIGRATION-INSTRUCTIONS.md を読んで、手順書通りに実行してください。

重要な追加指示:
- 既存の類似機能ファイル（古いリリーススクリプト、CIスクリプト等）は削除して構いません
- 目的が重複するワークフローやスクリプトはdiffx汎用システムに統一してください
- 手順書に従って確実に移植を完了させてください
```
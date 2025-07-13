# 内部リリースガイド（開発者専用）

## リリース時チェックリスト

### 1. ブランチとマージ状況確認
```bash
# 現在のブランチがmainであることを確認
git branch --show-current

# リモートからの最新情報を取得
git fetch origin

# mainブランチが最新であることを確認
git status
git log --oneline -10  # 最新10コミットを確認

# 作業ブランチがmainにマージ済みかを確認
# 例: feature/new-option ブランチがマージ済みかチェック
git branch --merged main | grep -v main  # mainにマージ済みのブランチ一覧
git branch --no-merged main  # まだマージされていないブランチ一覧
```

### 2. Issue・PR状況確認
```bash
# GitHubのIssue・PR状況を確認
gh issue list --state open
gh pr list --state open

# 古いIssue・PRがないかWebで確認
# https://github.com/kako-jun/diffx/issues
# https://github.com/kako-jun/diffx/pulls

# 必要に応じて古いIssue・PRを閉じる
gh issue close <issue-number>
gh pr close <pr-number>
```

### 3. 現在のバージョン確認
```bash
# 各プラットフォームの最新バージョンを確認
cargo search diffx-core
npm view diffx-js version
pip show diffx-python

# GitHubリリースページも確認
# https://github.com/kako-jun/diffx/releases
```

### 4. 新バージョン番号決定
- セマンティックバージョニング（X.Y.Z）に従う
- 破壊的変更: X をインクリメント
- 新機能追加: Y をインクリメント  
- バグ修正: Z をインクリメント

### 5. Rustの最新機能/オプション確認
```bash
# ソースコードから最新のCLIオプションを確認
# diffx-cli/src/main.rs の Args 構造体をチェック

# 現在の実装済みオプション（v0.5.1基準）：
# - input1, input2: 入力ファイル/ディレクトリ
# - --format: 入力フォーマット (json/yaml/toml/ini/xml/csv)
# - --output: 出力フォーマット (cli/json/yaml/unified)
# - --recursive: ディレクトリ再帰比較
# - --path: 特定パスのフィルタ
# - --ignore-keys-regex: キー除外正規表現
# - --epsilon: 浮動小数点許容誤差
# - --array-id-key: 配列要素識別キー
# - --context: unified diff形式のコンテキスト行数
# - --ignore-whitespace: 空白文字無視
# - --ignore-case: 大文字小文字無視
# - --quiet: 静音モード（終了コードのみ）
# - --brief: ファイル名のみ表示
# - --verbose: 詳細情報表示

# 新しいオプションがあればnpm/python側に反映
```

### 6. npm/pipパッケージの機能同期確認
- RustのCLIオプションがnpm/pythonラッパーで利用可能か確認
- 新しいオプションの引数や動作をテスト
- READMEの使用例を更新

### 7. バージョン更新箇所
以下のファイルを統一バージョンに更新：
- `Cargo.toml`
- `diffx-core/Cargo.toml`
- `diffx-cli/Cargo.toml`
- `diffx-python/pyproject.toml`
- `diffx-python/Cargo.toml`
- `diffx-npm/package.json`

### 8. 機能同期チェック
```bash
# Rustの最新ヘルプ
cargo run -- --help

# npmラッパーの対応確認
cd diffx-npm && node examples.js

# Pythonラッパーの対応確認
cd diffx-python && python examples.py
```

### 9. リリース前テスト
```bash
# 1. 高速チェック
./scripts/release/quick-release-check.sh

# 2. 包括的リリース前チェック（環境・バージョン・認証確認含む）
./scripts/release/pre-release-check.sh

# 3. CI環境での最終確認
./scripts/testing/ci-local.sh
```

### 10. リリース実行と監視
```bash
# 統合リリース（監視機能付き）
./scripts/release/release.sh

# または個別に監視
./scripts/release/monitor-release.sh v<version>
```

### 11. リリース後の徹底監視・確認
**重要**: タグプッシュ後は全てのアクションが正常完了するまで監視を継続する

#### GitHub Actions監視
```bash
# リアルタイムでアクション状況を監視
gh run list --limit 10
gh run watch  # 現在実行中のワークフローを監視

# 特定のワークフローの詳細確認
gh run view <run-id>
gh run view <run-id> --log  # ログ詳細表示
```

#### リリース成果物の確認
```bash
# 1. GitHubリリースページの確認
gh release list
gh release view v<version>

# リリースページが作成され、バイナリがアタッチされているかを確認
# - Linux x86_64バイナリ
# - Windows x86_64バイナリ 
# - macOS x86_64バイナリ
# - macOS arm64バイナリ
# - ソースコードアーカイブ

# 2. crates.ioの確認
cargo search diffx-core
cargo search diffx-cli
# または直接ブラウザで確認:
# https://crates.io/crates/diffx-core
# https://crates.io/crates/diffx-cli

# 3. PyPIの確認
pip index versions diffx-python
# または直接ブラウザで確認:
# https://pypi.org/project/diffx-python/

# 4. npmの確認
npm view diffx-js versions --json
# または直接ブラウザで確認:
# https://www.npmjs.com/package/diffx-js
```

#### エラー時の対処
```bash
# アクションが失敗した場合
gh run list --status failure
gh run view <failed-run-id> --log

# 失敗したアクションの再実行
gh run rerun <run-id>

# 特定のジョブのみ再実行
gh run rerun <run-id> --job <job-name>
```

#### 失敗リリースの掃除
```bash
# 失敗したリリースとタグの削除
./scripts/release/cleanup-failed-release.sh v<failed-version>

# 手動での削除手順
# 1. GitHubリリースページから削除
gh release delete v<failed-version> --yes

# 2. ローカルタグ削除
git tag -d v<failed-version>

# 3. リモートタグ削除
git push --delete origin v<failed-version>

# 4. 必要に応じてパッケージレジストリからも削除
# crates.io: cargo yank --vers <version> diffx-core
# npm: npm unpublish diffx-js@<version> (24時間以内のみ可能)
# PyPI: サポートに連絡が必要
```

## 重要な注意事項

### GitHubリリースの仕組み
- タグをプッシュすると自動的にAct1, Act2のGitHub Actionsが実行される
- Act1: Rustクレート・バイナリのビルドとリリース
- Act2: npm/Pythonパッケージのラッパー更新とリリース
- これによりGitHubリリースページが自動生成される

### リリースページの本文について
- **必須作業**: GitHub Actionsによる自動生成は簡素なため、必ず詳細な本文を追記する
- **重要**: 失敗したリリースで番号が飛んだ場合は、1つ前の成功バージョンからの差分をまとめて記載
- 例: v0.4.2が成功、v0.5.0が失敗、v0.5.1が成功の場合 → v0.5.1には v0.4.2からv0.5.1までの全変更を記載

#### **Claude向け自動判断ルール**
**リリース完了後は以下を必ず実行する：**

1. **本文充実度チェック**
   ```bash
   # 現在のリリースの本文をチェック
   gh release view v<version>
   ```
   - 本文が「**Full Changelog**: https://...」のみ → **即座に詳細化が必要**
   - 本文に具体的な機能説明がない → **詳細化が必要**

2. **実質的な前バージョン特定**
   ```bash
   # 一般ユーザーに見える前の実質的なリリースを特定
   gh release list --limit 10
   ```
   - **判断基準**: 充実した本文があるリリースが「実質的な前バージョン」
   - 簡素な本文のリリース（「ゴミリリース」）は飛ばして、その前の充実したリリースから差分を取る

3. **変更内容の収集と分析**
   ```bash
   # 実質的な前バージョンから現在までの変更を収集
   git log v<前の実質バージョン>..v<現在バージョン> --oneline
   ```

4. **リリースノートとCHANGELOG同時更新**
   ```bash
   # 詳細なリリースノートで上書き
   gh release edit v<version> --notes "$(cat <<'EOF'
   [詳細な本文]
   EOF
   )"
   
   # 同時にCHANGELOG.mdも更新
   # - 新機能・改善・バグ修正を整理
   # - 破壊的変更があれば明記
   # - リンクやマイグレーション情報を追加
   ```

#### **リリースノート必須要素**
- **主要機能・改善点の要約**（ユーザー目線）
- **技術的改善点**（開発者・上級ユーザー向け）
- **破壊的変更・移行ガイド**（該当する場合）
- **パッケージ提供状況**（Rust/npm/Python）
- **インストール・使用方法の変更**（該当する場合）
- **次のマイルストーンへの布石**

#### **自動実行トリガー**
**以下の条件時は必ずリリースノート詳細化を実行：**
- ✅ GitHub Actions（Act1/Act2）が完全成功した
- ✅ 新しいリリースページが作成された
- ✅ リリース本文が簡素（200文字未満、または「**Full Changelog**」のみ）
- ✅ パッケージレジストリへの公開が成功した

**実行タイミング**: リリース監視完了の直後、成功確認と同時に実行

### バージョン番号の決定方法
- 各プラットフォームの現在の最新バージョンを必ず確認
- セマンティックバージョニングに従って適切に増分
- 失敗したリリースの番号は再利用しない（混乱を避けるため）

### ソースコードベースでの機能確認
- **注意**: システムにインストール済みの古いdiffxバイナリで`--help`を確認してはいけない
- 必ずソースコード（diffx-cli/src/main.rs）の Args 構造体から最新オプションを確認
- 過去に古い実行ファイルで確認して、古いオプション情報に基づいたミスが発生済み

### ハードコーディングされたバージョンチェックの根絶
**重要**: リリースでエラーが頻発する最大の原因がハードコーディングされたバージョン番号

#### 修正済み項目（v0.5.1+）
- **diffx-python/src/diffx/__init__.py**: `__version__`を動的取得に変更
- **check-versions.sh**: 動的バージョン取得を使用（問題なし）
- **test-published-packages.sh**: 動的バージョン取得を使用（問題なし）

#### バージョンチェックの原則
1. **絶対禁止**: ハードコーディングされたバージョン番号での確認
2. **必須**: 動的バージョン取得（`cargo search`, `npm view`, `pip index`など）
3. **check-versions.shの活用**: リリース前にバージョン一貫性を確認済みなら、個別確認は不要
4. **テストデータは除外**: アプリケーション例のバージョン（1.0.0→1.1.0など）は問題なし

## リリースでミスが起きないための追加対策

### ドライラン・段階的実行
```bash
# 1. ドライランモードでテスト（実際のリリースは行わない）
git tag v0.6.0-test
git push origin v0.6.0-test
# アクションが正常動作するか確認後削除
git tag -d v0.6.0-test
git push --delete origin v0.6.0-test

# 2. 段階的リリース（小さな変更から）
# 重要な機能追加の前に、小さなパッチリリースで動作確認
```

### 環境・依存関係の統一
```bash
# 1. Node.jsバージョン固定
echo "Node.js version: $(node --version)"
echo "npm version: $(npm --version)"

# 2. Rustツールチェーン確認  
rustc --version
cargo --version

# 3. Python環境とmaturin確認（必ずuvでvenv使用）
# **重要**: システムのPythonは使わず、必ずuvでvenvを作成する
if ! command -v uv &> /dev/null; then
    echo "❌ uvがインストールされていません"
    echo "pipx install uv でインストールしてください"
    exit 1
fi

# venv環境の確認・作成
if [ ! -d ".venv" ]; then
    echo "🔧 Python仮想環境を作成中..."
    uv venv
fi

# venv環境をアクティベート
source .venv/bin/activate

# 必要なツールをvenv環境にインストール
if ! command -v maturin &> /dev/null; then
    echo "🔧 maturinをインストール中..."
    uv pip install maturin
fi

echo "Python version: $(python --version)"
echo "maturin version: $(maturin --version)"

# 4. 認証情報の事前確認
cargo login --dry-run
npm whoami
echo $MATURIN_PYPI_TOKEN | head -c 10  # 先頭10文字のみ表示
```

### Python環境管理の必須ルール
**Pythonを使う際の絶対ルール:**
- **システムPython禁止**: `pip install`でシステムレベルにインストールしない
- **uv必須**: 必ず`uv`を使って仮想環境を管理する
- **venv自動作成**: `.venv`がなければ`uv venv`で作成
- **アクティベート必須**: 作業前に`source .venv/bin/activate`を実行
- **依存関係管理**: `uv pip install`でパッケージをインストール

```bash
# Python環境セットアップ手順
# 1. uvのインストール（一度だけ）
pipx install uv

# 2. プロジェクトごとにvenv作成
cd /path/to/diffx
uv venv

# 3. 環境アクティベート（作業のたびに）
source .venv/bin/activate

# 4. 必要なツールインストール
uv pip install maturin wheel build twine

# 5. 作業完了後（任意）
deactivate
```

### リリース直前の最終確認
```bash
# 1. 他のブランチでの作業状況確認
git branch -r --no-merged main  # 未マージリモートブランチ
gh pr list --state open  # オープンPR

# 2. 最新のCI状況確認
gh run list --limit 5  # 最近のワークフロー実行状況

# 3. 依存関係の脆弱性チェック
cargo audit
npm audit
pip-audit  # pip install pip-audit が必要
```

### リリース後の軽量確認
```bash
# 1. パッケージレジストリでの公開確認（APIベース、軽量）
# crates.io: 通常5-10分で反映
# PyPI: 通常1-5分で反映
# npm: 通常1-3分で反映

# 2. GitHubリリースページとアセットの確認
gh release view v<version>

# monitor-release.sh は以下を自動実行:
# - GitHub Actions完了監視
# - パッケージレジストリAPI確認（cargo/npm/pip コマンド不要）
# - リリースアセット存在確認
```

### 重要な方針変更
**事前確認 vs Act1/Act2実行の完全分離**
- **事前確認**: ツール動作、認証、バージョン整合性を徹底チェック
- **Act1/Act2**: 最小限の必要なことのみ（エラー終了リスクゼロ）
- **リリース後確認**: 公開状態のみを軽量APIで確認

### Act1/Act2で絶対にやってはいけないこと
- ❌ バージョン番号の検証・チェック（事前確認済み）
- ❌ ツールの動作確認（事前確認済み）
- ❌ 認証状態の確認（事前確認済み）
- ❌ 外部APIへの接続テスト（不安定）
- ❌ ファイル存在確認以外のファイルシステム操作
- ❌ 複雑な条件分岐やエラーハンドリング
- ❌ 環境依存の処理

### Act1/Act2で行うべき最小限のこと
- ✅ ビルド・コンパイル（確実に成功する前提）
- ✅ パッケージング（確実に成功する前提）
- ✅ アップロード・公開（認証済み前提）
- ✅ リリースページ作成（権限確認済み前提）
- ✅ 単純なファイルコピー・移動

### Act1/Act2エラー終了の根絶方針
**「すべての不確実要素は事前確認で排除、Act1/Act2は機械的実行のみ」**

1. **事前確認での徹底検証**
   - 全ツールの動作確認とバージョン検証
   - 全認証情報の有効性確認
   - 全外部サービスの接続確認
   - ビルド・テストの事前実行（`ci-local.sh`）

2. **Act1/Act2での機械的実行**
   - 条件分岐なし
   - エラーハンドリング最小限
   - 外部依存最小限
   - 既に検証済みの処理のみ実行

3. **失敗時の対応**
   - Act1/Act2失敗 = 予期せぬ外的要因
   - 即座にクリーンアップ実行
   - 原因調査は別途実施

### 緊急時対応準備
```bash
# 1. リバート用ブランチ準備
git checkout -b emergency-revert-v0.6.0

# 2. 前回成功バージョンの記録
echo "Last successful release: $(git tag --sort=-version:refname | head -2 | tail -1)"

# 3. ロールバック手順の確認
./scripts/cleanup-failed-release.sh --dry-run v0.6.0
```

### 平日・作業時間での実行
- **推奨時間**: 平日10:00-15:00（問題発生時の対応時間確保）
- **避けるべき時間**: 金曜夕方、祝日前、深夜早朝
- **GitHub Actions制限**: 同時実行数やレート制限に注意

### ドキュメント更新の同期
```bash
# 1. CHANGELOG.mdの更新（リリースノート充実化と同時実施）
# 2. README.mdの新機能説明
# 3. ドキュメントサイトの更新準備
# 4. リリースノートのテンプレート準備
```

### 外部サービス依存の最小化
- **GitHub Actions**: 外部アクションの最新バージョンピン留め
- **パッケージレジストリ**: 各サービスのステータスページ事前確認
- **ネットワーク**: VPN使用時は解除（接続安定性のため）

## 新機能追加時の手順

### Rustに新オプション追加時
1. diffx-cli/src/main.rs でオプション追加
2. diffx-npm/lib.js でオプション対応追加
3. diffx-python/src/diffx/diffx.py でオプション対応追加
4. 各パッケージのREADME更新
5. 各パッケージのexamples更新

### テストケース追加
- 新オプションの動作テスト
- 既存機能の回帰テスト
- エラーハンドリングテスト

## 失敗リリースの掃除

### 自動掃除スクリプト
```bash
# 失敗したリリースを完全にクリーンアップ
./scripts/cleanup-failed-release.sh v<failed-version>
```

### 掃除される内容
- GitHubリリースページとアセット
- ローカル・リモートのGitタグ
- crates.ioからのyank（オプション）
- npmパッケージの削除（24時間以内のみ可能）
- PyPI削除の手順案内（手動対応が必要）

### 掃除のタイミング
- GitHub Actionsが失敗した場合
- パッケージ公開が部分的に失敗した場合
- リリース番号を変更して再リリースする前

## 緊急時の個別リリース

### crates.ioのみ緊急修正
```bash
cd diffx-core && cargo publish
cd ../diffx-cli && cargo publish
```

### npmのみ緊急修正
```bash
cd diffx-npm
npm version patch --no-git-tag-version
npm publish
```

### pipのみ緊急修正
```bash
cd diffx-python
# pyproject.tomlのバージョンを手動更新
maturin build --release
maturin publish
```
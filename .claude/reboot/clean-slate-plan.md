# Clean Slate Plan - 白紙からのやり直し

作成日: 2025年11月14日

## 🎯 基本方針

### 大原則
```
reboot以外の既存ファイルは信じない。
中途半端である前提で考える。
ドキュメントも嘘だと疑う。
どこまでが真実かを考える。
```

### やり直しの哲学
1. **仕様を固めることから着実に**
2. **言語ごとに独立・シンプルに**
3. **複雑なフロー・分岐なし**
4. **oldは参考程度**
5. **動作確認を徹底**

## 📁 Phase 1: 隔離作戦（今日実行）

### 既存ファイルの_old化

```bash
# 実行コマンド
cd /home/d131/repos/2025/diffx

# 1. ドキュメント類を隔離
mkdir -p _old/docs
mv docs _old/docs_original
mv README.md _old/
mv README_ja.md _old/  # もし存在すれば
mv README_zh.md _old/  # もし存在すれば
mv CHANGELOG.md _old/

# 2. CI/CD関連を隔離
mkdir -p _old/github
cp -r .github _old/github_original

# 3. スクリプト類を隔離
mkdir -p _old/scripts
mv scripts _old/scripts_original  # もし存在すれば

# 4. マーケティング資料を隔離（参考用に残す）
# .claude/marketing/ は残す（まだ使える）

# 5. 旧計画書はすでにreboot/deprecatedへ移動済み

# 6. .gitignoreに追加
echo "_old/" >> .gitignore
```

### 隔離後の確認
```bash
# ビルドできるか確認（コアコードは残すので）
cargo build --release

# テストできるか確認
cargo test

# もし失敗したら、それも記録する
```

## 📋 Phase 2: 真実の特定（1-2日）

### 何が真実か？

```bash
# 検証項目リスト
1. Cargo.tomlの内容は正しいか？
2. 実際に動くコードはどこか？
3. テストは何が通るか？
4. 依存関係は正しいか？
5. ビルド成果物は期待通りか？
```

### 検証方法

```bash
# 1. バージョン確認
cat Cargo.toml | grep version
cat diffx-core/Cargo.toml | grep version
cat diffx-cli/Cargo.toml | grep version
cat diffx-python/Cargo.toml | grep version
cat diffx-js/Cargo.toml | grep version

# 2. ビルド確認
cargo build --release 2>&1 | tee _old/build-output.txt

# 3. テスト実行
cargo test --all 2>&1 | tee _old/test-output.txt

# 4. 実際のCLI動作確認
./target/release/diffx --version
./target/release/diffx --help

# 5. 簡単な動作テスト
echo '{"a": 1}' > /tmp/test1.json
echo '{"a": 2}' > /tmp/test2.json
./target/release/diffx /tmp/test1.json /tmp/test2.json
```

### 真実を記録

```bash
# .claude/reboot/ground-truth.md に記録
cat > .claude/reboot/ground-truth.md << 'EOF'
# Ground Truth - 確認された真実

## 検証日時
2025-11-14

## ビルド状況
- [ ] cargo build --release: 成功/失敗
- [ ] cargo test --all: 成功/失敗
- [ ] diffx --version: 成功/失敗

## バージョン情報
- Cargo.toml: X.Y.Z
- 実際のCLI: X.Y.Z
- 一致: Yes/No

## 動作確認
- [ ] JSON diff: 動作する/しない
- [ ] YAML diff: 動作する/しない
- [ ] TOML diff: 動作する/しない

## 問題点
（ここに発見した問題を記録）

## 使える機能
（ここに確実に動く機能を記録）
EOF
```

## 📐 Phase 3: 仕様の再定義（2-3日）

### 仕様書の作成

```bash
mkdir -p docs/specs
```

#### 1. コア仕様書
```markdown
# docs/specs/core-spec.md

## diffx-core の仕様

### 目的
構造化データの差分抽出

### サポートフォーマット
（実際に動作確認したもののみ記載）
- [ ] JSON
- [ ] YAML
- [ ] TOML
- [ ] XML
- [ ] INI
- [ ] CSV

### API仕様
（実際に動くAPIのみ記載）

### テストケース
（通るテストのみ記載）
```

#### 2. CLI仕様書
```markdown
# docs/specs/cli-spec.md

## diffx CLI の仕様

### コマンド形式
```
diffx <FILE1> <FILE2> [OPTIONS]
```

### オプション
（実際に動くオプションのみ記載）

### 出力形式
（実際の出力を記録）

### エラー処理
（実際のエラーメッセージを記録）
```

#### 3. Python仕様書
```markdown
# docs/specs/python-spec.md

## diffx-python の仕様

### インストール
（実際に試して成功した方法のみ）

### API
（実際に動くAPIのみ）

### 使用例
（実際に動作確認した例のみ）
```

#### 4. JavaScript仕様書
```markdown
# docs/specs/js-spec.md

## diffx-js の仕様

### インストール
（実際に試して成功した方法のみ）

### API
（実際に動くAPIのみ）

### 使用例
（実際に動作確認した例のみ）
```

### 仕様策定の原則

```
1. 実装を見る前に、「あるべき姿」を書く
2. 実装を確認する
3. 動く部分だけを仕様に記載
4. 動かない部分は「TODO」として別記
5. 嘘は絶対に書かない
```

## 🔧 Phase 4: GitHub Actions 再設計（1-2日）

### 設計原則

```
1. Rust、Python、npmで完全に独立
2. 分岐なし
3. 複雑なフローなし
4. 理解可能なシンプルさ
5. デバッグ可能
```

### 新しいワークフロー構成

```yaml
.github/workflows/
├── rust-ci.yml           # Rustのみ
├── rust-release.yml      # Rustのみ
├── python-ci.yml         # Pythonのみ
├── python-release.yml    # Pythonのみ
├── npm-ci.yml            # npmのみ
└── npm-release.yml       # npmのみ
```

### Rust CI (rust-ci.yml)

```yaml
name: Rust CI

on:
  push:
    branches: [ main ]
    paths:
      - 'diffx-core/**'
      - 'diffx-cli/**'
      - 'Cargo.*'
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: Format check
        run: cargo fmt --all -- --check

      - name: Clippy
        run: cargo clippy --workspace -- -D warnings

      - name: Build
        run: cargo build --release --workspace

      - name: Test
        run: cargo test --workspace

      - name: CLI smoke test
        run: |
          echo '{"a": 1}' > test1.json
          echo '{"a": 2}' > test2.json
          ./target/release/diffx test1.json test2.json
```

### Rust Release (rust-release.yml)

```yaml
name: Rust Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    runs-on: ${{ matrix.os }}

    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: Build
        run: cargo build --release

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: diffx-${{ matrix.os }}
          path: target/release/diffx*

  publish:
    needs: build
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable

      - name: Publish to crates.io
        run: cargo publish --token ${{ secrets.CARGO_TOKEN }}
```

### Python CI (python-ci.yml)

```yaml
name: Python CI

on:
  push:
    branches: [ main ]
    paths:
      - 'diffx-python/**'
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v5
        with:
          python-version: '3.11'

      - name: Install dependencies
        run: |
          pip install maturin pytest
          cd diffx-python
          maturin develop

      - name: Run tests
        run: |
          cd diffx-python
          pytest
```

### npm CI (npm-ci.yml)

```yaml
name: npm CI

on:
  push:
    branches: [ main ]
    paths:
      - 'diffx-js/**'
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
        with:
          node-version: '20'

      - name: Install dependencies
        run: |
          cd diffx-js
          npm install

      - name: Run tests
        run: |
          cd diffx-js
          npm test
```

### 旧ワークフローの扱い

```bash
# すでに _old/github_original に移動済み
# 新しいワークフローを .github/workflows/ に作成
# 段階的に移行（まずrust-ci.ymlから）
```

## 📝 Phase 5: ドキュメント新規作成（3-5日）

### 日本語のみから開始

```bash
docs/
├── specs/              # Phase 3で作成済み
│   ├── core-spec.md
│   ├── cli-spec.md
│   ├── python-spec.md
│   └── js-spec.md
├── README_ja.md        # 新規作成（日本語のみ）
└── guides/
    ├── installation_ja.md
    ├── quickstart_ja.md
    └── examples_ja.md
```

### README_ja.md の内容

```markdown
# diffx

構造化データの差分抽出ツール

## インストール

（実際に動作確認した方法のみ）

## 基本的な使い方

（実際に動作する例のみ）

## ドキュメント

- [インストールガイド](guides/installation_ja.md)
- [クイックスタート](guides/quickstart_ja.md)
- [使用例](guides/examples_ja.md)
- [仕様書](specs/)

## ライセンス

MIT
```

### ルート README.md

```markdown
# diffx

**日本語ドキュメント**: [docs/README_ja.md](docs/README_ja.md)

Structured data diff tool for JSON/YAML/TOML/XML/INI/CSV.

## Documentation

See [docs/README_ja.md](docs/README_ja.md) (Japanese)

English documentation: Coming soon

## License

MIT
```

## 🎯 Phase 6: 段階的な実装修正（継続）

### 修正の原則

```
1. 仕様書に書いたことを実装
2. 実装したらテスト
3. テストが通ったら仕様書に✓
4. 通らなかったら仕様書からTODOへ移動
5. 嘘をつかない
```

### 優先順位

```
Priority 1: Rustコアライブラリ
- 基本的な差分抽出
- JSON/YAML/TOML対応
- テスト100%通過

Priority 2: CLI
- 基本的なコマンド
- オプション
- エラー処理

Priority 3: Python
- maturinビルド
- 基本API
- テスト

Priority 4: npm
- ビルド
- 基本API
- テスト
```

## 📊 実行スケジュール

### Day 1（今日）
```
- [x] clean-slate-plan.md 作成
- [ ] 隔離作戦実行
- [ ] ビルド・テスト確認
- [ ] ground-truth.md 作成
```

### Day 2-3
```
- [ ] 仕様書作成（core, CLI）
- [ ] 実装確認
- [ ] 動作テスト
```

### Day 4-5
```
- [ ] 仕様書作成（Python, npm）
- [ ] GitHub Actions設計
- [ ] rust-ci.yml 作成・テスト
```

### Day 6-7
```
- [ ] ドキュメント作成（日本語のみ）
- [ ] README_ja.md
- [ ] インストールガイド
```

### Week 2
```
- [ ] 残りのGitHub Actions
- [ ] Python CI
- [ ] npm CI
- [ ] 統合テスト
```

## 🚨 重要な約束

### やってはいけないこと

```
❌ 既存ドキュメントを信じる
❌ AIの「実装した」を確認なしで信じる
❌ 複雑なワークフローを作る
❌ 3言語同時にドキュメント作成
❌ 動作確認なしで仕様書に書く
❌ テストが通らないのに「完成」と言う
```

### 必ずやること

```
✅ 実装を確認する
✅ 実際に動かす
✅ テストを通す
✅ 仕様書に正直に書く
✅ わからないことは「TODO」に
✅ 小さく確実に進む
```

## 📋 チェックリスト

### 毎日のルーチン

```markdown
朝:
- [ ] 今日のゴール（1つ）を決める
- [ ] コンテキスト残量確認
- [ ] 前提ドキュメント読み込み

作業中:
- [ ] 小さな単位で実装
- [ ] 実装したら即テスト
- [ ] テスト通過を確認

終了時:
- [ ] 今日の成果を記録
- [ ] 次回の開始点を記録
- [ ] 嘘を書いていないか確認
```

## 🎯 成功の定義

### Week 1終了時
```
✅ _old/ に既存ファイル隔離済み
✅ ground-truth.md に真実を記録
✅ 仕様書（core, CLI）完成
✅ rust-ci.yml 動作確認
✅ README_ja.md 完成
```

### Week 2終了時
```
✅ 全仕様書完成
✅ 全GitHub Actions動作
✅ ドキュメント完成（日本語）
✅ quick-check.sh 動作
✅ マーケティング準備完了
```

## 💡 この計画の意義

### なぜ白紙から？

```
既存システムの問題:
1. 何が真実か不明
2. 複雑すぎて理解不能
3. ドキュメントと実装の乖離
4. テストの信頼性低下
5. 修正より作り直しが早い
```

### 期待される効果

```
1. 確実に動くシステム
2. 理解可能な構造
3. 信頼できるドキュメント
4. デバッグ可能なCI/CD
5. 自信を持って開発できる
```

---

## 次のアクション

1. 隔離作戦の実行（今日）
2. ground-truth.mdの作成（今日）
3. 仕様書の作成開始（明日）

**合言葉: 「疑って、確認して、記録する」**

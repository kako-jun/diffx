# Quick Wins - 今週実行できること

## 🎯 今週の目標
**CI/CDを動かす、マーケティング資料を準備する**

所要時間: 合計10-15時間（1日2-3時間 × 5日）

## Day 1: 大掃除（2-3時間）✓

### ✅ 完了
- [x] 問題分析: `problem-analysis.md`
- [x] リブート計画: `reboot-plan.md`
- [x] クイック勝利: `quick-wins.md`（このファイル）

### 次のステップ
- [ ] deprecated フォルダ作成
- [ ] 不要ファイル移動

## Day 2: 不要ファイル整理（1-2時間）

### 実行コマンド
```bash
# deprecatedフォルダ作成
mkdir -p .claude/reboot/deprecated

# 移動するファイル
mv .claude/migration-plan.md .claude/reboot/deprecated/
mv .claude/migration-plan-realistic.md .claude/reboot/deprecated/

# tasks.mdのバックアップ
cp .claude/tasks.md .claude/reboot/deprecated/tasks-old.md
```

### tasks.md簡素化
```markdown
# 新しいtasks.md

## 🎯 今週（Week 1）
- [ ] CI/CDシンプル化方針決定
- [ ] quick-check.sh作成
- [ ] 動作確認

## 📢 来週（Week 2）
- [ ] マーケティング資料準備
- [ ] Product Hunt投稿準備
- [ ] Hacker News投稿準備

## 🚀 継続タスク
- [ ] 週次リリースサイクル確立
- [ ] ユーザーフィードバック収集

## ✅ 完了（アーカイブ）
過去の膨大なタスクは .claude/reboot/deprecated/tasks-old.md へ移動
```

## Day 3: CI/CD方針決定（2-3時間）

### 調査タスク
```bash
# 1. 現在の.github/workflows/を確認
ls -la .github/workflows/
cat .github/workflows/ci.yml | head -50

# 2. 共有リポジトリを確認
ls -la /home/d131/repos/2025/.github/
ls -la /home/d131/repos/2025/.github/rust-cli-kiln/

# 3. 必要なスクリプトを特定
find /home/d131/repos/2025/.github/rust-cli-kiln/scripts/ -name "*.sh"
```

### 決定事項
```markdown
選択肢を選ぶ:

A. シンボリックリンク修復
- 既存システム維持
- 共有リポジトリへの依存継続
- 複雑さは残る

B. スクリプトローカル化（推奨）
- diffx内に必要なスクリプトをコピー
- 共有リポジトリから独立
- シンプルで理解しやすい

C. GitHub Actions標準化
- rust-cache, cargo-actionなど使用
- カスタムスクリプト最小化
- 最もシンプル

推奨順: C > B > A
```

### 選択の記録
```bash
# 決定を記録
cat > .claude/reboot/ci-cd-decision.md << 'EOF'
# CI/CD方針

## 選択: [ここに選択肢を記入]

## 理由:
- [理由1]
- [理由2]

## 実装計画:
1. [ステップ1]
2. [ステップ2]

## 期待される結果:
- quick-check.shが動作する
- プッシュ前チェックが可能
- CI/CDが理解できる
EOF
```

## Day 4: CI/CD実装（3-4時間）

### 選択肢Cの場合（推奨）
```yaml
# .github/workflows/ci.yml（簡素版）
name: CI

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: Check format
        run: cargo fmt --all -- --check

      - name: Clippy
        run: cargo clippy --all-targets --all-features -- -D warnings

      - name: Build
        run: cargo build --release

      - name: Test
        run: cargo test --all-features

      - name: Test Rust examples
        run: cargo test --examples
```

### quick-check.sh作成
```bash
#!/usr/bin/env bash
set -e

# diffx quick check script
echo "🔍 Running quick checks..."

echo "📝 Checking format..."
cargo fmt --all -- --check

echo "📎 Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings

echo "🔨 Building..."
cargo build --release

echo "🧪 Running tests..."
cargo test --all-features

echo "✅ All checks passed!"
```

### 実行
```bash
# scripts/testing/ディレクトリ作成
mkdir -p scripts/testing

# quick-check.sh作成
cat > scripts/testing/quick-check.sh << 'EOF'
#!/usr/bin/env bash
set -e
echo "🔍 Running quick checks..."
echo "📝 Checking format..."
cargo fmt --all -- --check
echo "📎 Running Clippy..."
cargo clippy --all-targets --all-features -- -D warnings
echo "🔨 Building..."
cargo build --release
echo "🧪 Running tests..."
cargo test --all-features
echo "✅ All checks passed!"
EOF

# 実行権限付与
chmod +x scripts/testing/quick-check.sh

# テスト実行
./scripts/testing/quick-check.sh
```

### 選択肢Bの場合
```bash
# 共有リポジトリからコピー
cp /home/d131/repos/2025/.github/rust-cli-kiln/scripts/testing/quick-check.sh \
   scripts/testing/

# パス修正（必要なら）
sed -i 's|github-shared|scripts|g' scripts/testing/quick-check.sh

# テスト
./scripts/testing/quick-check.sh
```

## Day 5: 動作確認とコミット（2時間）

### 確認項目
```bash
# 1. quick-check.shが動く
./scripts/testing/quick-check.sh

# 2. CIが動く（プッシュして確認）
git add .
git commit -m "chore: simplify CI/CD and restore quick-check script"
git push origin main

# 3. GitHub Actionsで確認
# https://github.com/kako-jun/diffx/actions
```

### CLAUDE.md更新
```markdown
# CLAUDE.md の「プッシュ前の必須チェック」セクション更新

## プッシュ前の必須チェック (Pre-Push Requirements)
**必ずプッシュ前に以下を実行すること:**
```bash
./scripts/testing/quick-check.sh
```

- フォーマット・Clippy・ビルド・テスト・CLI動作確認をすべて実行
- 1つでもエラーが発生したら即座に停止する（`set -e`）
- ローカルで成功 → GitHub CIでも成功が保証される
```

### コミット
```bash
git add .
git commit -m "docs: update CI/CD documentation to reflect simplified structure"
git push origin main
```

## 📊 Week 1の成果物

### 完成するファイル
```
.claude/reboot/
├── problem-analysis.md          ✓
├── reboot-plan.md              ✓
├── quick-wins.md               ✓
├── ci-cd-decision.md           (Day 3)
└── deprecated/
    ├── migration-plan.md
    ├── migration-plan-realistic.md
    └── tasks-old.md

scripts/testing/
└── quick-check.sh              (Day 4)

.github/workflows/
└── ci.yml                      (Day 4, simplified)

.claude/
└── tasks.md                    (Day 2, simplified)
```

### 達成目標
- ✅ 問題を明確に理解した
- ✅ 現実的な計画を立てた
- [ ] 不要なファイルを整理した
- [ ] CI/CDがシンプルに動く
- [ ] quick-check.shが動作する
- [ ] 自信を持って開発を再開できる

## 🎉 Week 1完了後の状態

### できるようになること
1. プッシュ前に`./scripts/testing/quick-check.sh`実行
2. CI/CDが理解できる（複雑性が排除された）
3. 安心してコード変更できる
4. 次週のマーケティングに集中できる

### 心理的効果
- ✅ プロジェクトを理解した
- ✅ 複雑性を排除した
- ✅ 動くシステムを手に入れた
- ✅ 前進している実感
- ✅ マーケティングへの自信

## 🚀 Week 2への準備

### 次週のタスク（軽く準備）
```bash
# マーケティング資料を読む（金曜夕方）
cat .claude/marketing/strategy.md
cat .claude/marketing/content-templates.md
cat .claude/marketing/execution-plan.md

# 投稿先を調査
- Product Hunt: 投稿方法確認
- Hacker News: Show HNの例を確認
- Reddit r/rust: 前回の投稿を振り返り
```

### 心の準備
- 技術は完成している ✓
- CI/CDが動く ✓
- マーケティングに集中する時期が来た

---

## 💡 Important Reminders

### 避けるべき誘惑
- ❌ 「もっと機能を追加してから」
- ❌ 「完璧なCI/CDを作ってから」
- ❌ 「lawkit/diffaiも一緒に」
- ❌ 「3プロジェクト統一を」

### 心に留めること
- ✅ シンプルが最高
- ✅ 動けば十分
- ✅ マーケティングが最優先
- ✅ 小さく始める
- ✅ 継続が力

### 今週の成功基準
```
「./scripts/testing/quick-check.shが動作する」

これだけ。シンプルに。
```

## 📞 困ったときのチェックリスト

### CI/CD実装で困ったら
1. 選択肢Cを選ぶ（最もシンプル）
2. 既存の.github/workflows/を見る
3. 動いてる部分を残す
4. quick-check.shを単純にする

### モチベーション低下したら
1. problem-analysis.mdを読み直す
2. 「技術は完成している」を思い出す
3. 「ユーザーが必要」を思い出す
4. 小さな一歩でOK

### 完璧主義が頭をもたげたら
1. 「740行の計画書は捨てた」を思い出す
2. 「80%で良い」を思い出す
3. 「動けば十分」を思い出す
4. reboot-plan.mdを読み直す

---

**今週の合言葉: 「シンプルに、動かす、前へ」**

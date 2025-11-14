# diffx Reboot Project（更新版）

最終更新: 2025-11-14

## 📌 重要な決定: 別リポジトリ化

diffx-js と diffx-python を別リポジトリに移行することが決定しました。

```
/home/d131/repos/2025/
├── diffx/          # Rust専用（このリポジトリ）✓
├── diffx-js/       # JavaScript/npm（新規作成済み）✓
└── diffx-python/   # Python/PyPI（新規作成済み）✓
```

## 📚 ドキュメント構成（最新版）

### 1. [problem-analysis.md](./problem-analysis.md)
何が問題だったのかの徹底分析。

### 2. [user-reflection.md](./user-reflection.md) - 最重要
ユーザー視点での失敗原因11項目と改善策。

特に重要:
- AIの実装嘘問題
- コンテキスト管理の失敗
- モノレポの失敗

### 3. [reboot-plan.md](./reboot-plan.md)
全体的な再起動計画（4つのPhase）。

### 4. [clean-slate-plan.md](./clean-slate-plan.md) - 実行計画
白紙からのやり直し計画。

**重要な方針**:
- reboot以外は信じない
- ドキュメントも嘘だと疑う
- 仕様を固めることから着実に
- 言語ごとに独立・シンプルに

### 5. [migrate-to-separate-repos.sh](./migrate-to-separate-repos.sh) - 実行スクリプト ← NEW!
diffx-js, diffx-python を別リポジトリに移行するスクリプト。

### 6. [post-migration-guide.md](./post-migration-guide.md) - セットアップガイド ← NEW!
移行後の各リポジトリのセットアップ方法。

### 7. [ground-truth.md](./ground-truth.md)
確認された真実を記録するテンプレート。

### 8. [quick-wins.md](./quick-wins.md)
今週実行できる具体的タスク（移行後に更新予定）。

## 🚀 今すぐ実行するステップ

### Step 1: 別リポジトリへの移行

```bash
cd /home/d131/repos/2025/diffx

# 移行スクリプト実行
./.claude/reboot/migrate-to-separate-repos.sh
```

このスクリプトは:
1. diffx/diffx-js/ → ../diffx-js/ に移動
2. diffx/diffx-python/ → ../diffx-python/ に移動
3. Cargo.toml をRust専用に更新
4. ビルド・テストを確認

### Step 2: 各リポジトリのセットアップ

[post-migration-guide.md](./post-migration-guide.md) を参照:

1. **diffx (Rust専用)**
   - README.md 作成
   - コミット

2. **diffx-js**
   - Cargo.toml 修正（crates.io依存）
   - README.md, .gitignore 作成
   - 初期コミット

3. **diffx-python**
   - Cargo.toml 修正（crates.io依存）
   - README.md, .gitignore 作成
   - 初期コミット

### Step 3: 真実の確認

```bash
cd /home/d131/repos/2025/diffx

# ビルド確認
cargo build --release

# テスト確認
cargo test --workspace

# 結果を記録
# .claude/reboot/ground-truth.md を編集
```

## 🎯 リブートの核心（変更なし）

### 3つの気づき

1. **技術は完成している**
2. **複雑性が開発を麻痺させた**
3. **マーケティングが未実行**

### リブートの哲学

```
「完璧を目指すな、実行せよ」

- 技術は完成している
- 各言語は独立・シンプルに
- マーケティングが最優先
- 小さく始めて継続する
- ユーザーが全てを教えてくれる
```

## 📅 更新されたタイムライン

### Day 1（今日）- 別リポジトリ化
- [x] 問題分析 ✓
- [x] ユーザー自己分析 ✓
- [x] クリーンスレート計画 ✓
- [x] 移行スクリプト作成 ✓
- [x] 移行後ガイド作成 ✓
- [ ] 移行スクリプト実行
- [ ] 各リポジトリセットアップ

### Day 2-3 - 真実の特定
- [ ] ground-truth.md 記録
- [ ] 動作確認（各フォーマット）
- [ ] 仕様書作成開始

### Day 4-5 - 仕様策定
- [ ] core-spec.md
- [ ] cli-spec.md
- [ ] GitHub Actions設計

### Week 2 - ドキュメント・CI/CD
- [ ] README_ja.md 完成
- [ ] rust-ci.yml 作成・テスト
- [ ] quick-check.sh 復活

### Week 3+ - マーケティング
- [ ] Product Hunt準備
- [ ] Hacker News準備
- [ ] 実行

## 🚨 重要な約束（更新版）

### やってはいけないこと

```
❌ 既存ドキュメントを信じる
❌ AIの「実装した」を確認なしで信じる
❌ 複雑なワークフローを作る
❌ モノレポに戻す
❌ diffx-js, diffx-python を同時開発
❌ 仕様確定前に実装
```

### 必ずやること

```
✅ 実装を確認する
✅ 実際に動かす
✅ テストを通す
✅ 仕様書に正直に書く
✅ diffx (Rust) に集中
✅ 小さく確実に進む
```

## 📊 今日の成果物

### 完成ファイル

```
.claude/reboot/
├── README-UPDATED.md           ✓ （このファイル）
├── problem-analysis.md          ✓
├── user-reflection.md           ✓ （最重要）
├── reboot-plan.md              ✓
├── clean-slate-plan.md         ✓
├── migrate-to-separate-repos.sh ✓ （実行準備完了）
├── post-migration-guide.md     ✓
├── ground-truth.md             ✓ （テンプレート）
└── quick-wins.md               ✓ （移行後に更新）
```

## 🎯 成功の定義（更新版）

### 今日終了時
```
✅ 移行スクリプト実行完了
✅ 各リポジトリ分離完了
✅ diffx (Rust) ビルド成功
✅ 初期コミット完了
```

### Week 1終了時
```
✅ ground-truth.md に真実を記録
✅ 仕様書（core, CLI）完成
✅ README_ja.md 完成
✅ rust-ci.yml 動作確認
```

### Week 2終了時
```
✅ 全仕様書完成
✅ quick-check.sh 動作
✅ マーケティング準備完了
```

## 💪 今日の目標

**「3つのリポジトリを独立させる」**

これだけ。シンプルに。

## 📞 困ったときのチェックリスト

### 移行スクリプトで困ったら
1. バックアップは自動作成される（_old/backup_before_migration/）
2. Cargo.toml.backup も作成される
3. 問題があれば元に戻せる

### 移行後のビルドで困ったら
1. cargo clean
2. cargo build --release
3. エラーメッセージを確認
4. _old/build-after-migration.txt を確認

### 次のステップがわからなくなったら
1. post-migration-guide.md を読む
2. clean-slate-plan.md を読む
3. user-reflection.md の教訓を思い出す

## 🌟 最後に

今日の決断（別リポジトリ化）は、user-reflection.md の教訓を実践する第一歩です。

**「モノレポで運用したのがまずかった」**
→ 今日、これを修正します。

**「RustのときはRustのことだけやる」**
→ diffxはRust専用になります。

**「シンプルに、動かす、前へ」**
→ 複雑性を排除します。

一緒に、diffxを立て直しましょう。

---

**更新日**: 2025年11月14日
**重要度**: 最高
**次のアクション**: `./migrate-to-separate-repos.sh` を実行

**合言葉: 「疑って、確認して、記録する」**

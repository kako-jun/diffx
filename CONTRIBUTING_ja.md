# diffx への貢献

`diffx` プロジェクトへの貢献にご関心をお寄せいただき、ありがとうございます！このドキュメントでは、プロジェクトへの貢献に関する包括的なガイドラインを提供します。

## 🎯 プロジェクトビジョン

`diffx` は構造化データの決定版となるセマンティック差分ツールを目指しています。私たちが重視するのは：

- **セマンティックな正確性**: テキストフォーマットではなく、データの意味を理解すること
- **パフォーマンス**: 大きなファイルの高速処理
- **ユーザビリティ**: 人間と自動化の両方に優しいクリーンなCLIインターフェース
- **信頼性**: 包括的なテストと型安全性

## 🚀 クイックスタート

### 開発環境のセットアップ

1. **前提条件**
   ```bash
   # Rust をインストール（まだインストールしていない場合）
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source ~/.cargo/env
   
   # インストールを確認
   rustc --version
   cargo --version
   ```

2. **クローンとビルド**
   ```bash
   git clone https://github.com/your-username/diffx.git
   cd diffx
   cargo build --workspace
   ```

3. **テストの実行**
   ```bash
   # すべてのテストを実行（計58テスト）
   cargo test --workspace
   
   # 特定のテストカテゴリを実行
   cargo test --package diffx-core     # コアライブラリテスト（29テスト）
   cargo test integration              # CLI統合テスト（29テスト）
   ```

4. **開発ツール**
   ```bash
   # コードフォーマット（CIに必須）
   cargo fmt --all
   
   # リンターの実行（CIに必須）
   cargo clippy --workspace --all-targets --all-features -- -D warnings
   
   # ベンチマークの実行
   cargo bench --package diffx-core
   ```

## 🔧 開発ワークフロー

### 1. フォークとブランチ作成
```bash
git checkout -b feature/your-feature-name
# または
git checkout -b fix/your-bug-fix-name
```

### 2. 変更の実装
- Rustの慣例とプロジェクトパターンに従う
- 新機能には包括的なテストを追加
- 関連するドキュメントを更新

### 3. ローカルテスト
```bash
# すべてのCIチェック（必須通過）
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo build --workspace
cargo test --workspace
```

### 4. コミットとプッシュ
```bash
git add .
git commit -m "feat(scope): 変更の説明"
git push origin feature/your-feature-name
```

### 5. プルリクエストの作成
- わかりやすいタイトルと説明を使用
- 関連するイシューを参照
- すべてのCIチェックが通過することを確認

## 📁 プロジェクト構造

```
diffx/
├── diffx-core/           # コア差分ロジックライブラリ
│   ├── src/lib.rs       # メイン差分アルゴリズム
│   ├── benches/         # パフォーマンスベンチマーク
│   └── Cargo.toml
├── diffx-cli/           # コマンドラインインターフェース
│   ├── src/main.rs      # CLI実装
│   └── Cargo.toml
├── tests/               # 包括的テストスイート
│   ├── fixtures/        # テストデータファイル
│   ├── integration/     # CLI統合テスト（29テスト）
│   └── unit/           # ユニットテスト（29テスト）
├── docs/               # ドキュメント
│   ├── user-guide/     # ユーザードキュメント
│   ├── reference/      # 技術リファレンス
│   ├── guides/         # 統合ガイド
│   └── project/        # プロジェクト情報
└── .github/            # CI/CDワークフロー
```

## 🧪 テストガイドライン

### テストカテゴリ

1. **ユニットテスト** (`tests/unit/core_tests.rs`): コア差分ロジック
2. **統合テスト** (`tests/integration/cli_tests.rs`): CLI動作
3. **ベンチマーク** (`diffx-core/benches/`): パフォーマンス検証

### テストの実行

```bash
# すべてのテスト（計58個）
cargo test --workspace

# 特定のカテゴリ
cargo test core_tests::     # ユニットテスト
cargo test cli_tests::      # 統合テスト

# 出力付き
cargo test -- --nocapture

# 特定のテスト
cargo test test_diff_json_objects
```

### テストの追加

**ユニットテストの例:**
```rust
#[test]
fn test_your_feature() {
    let v1 = json!({"key": "value1"});
    let v2 = json!({"key": "value2"});
    let result = diff(&v1, &v2, None, None, None);
    assert_eq!(result.len(), 1);
}
```

**統合テストの例:**
```rust
#[test]
fn test_cli_feature() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = diffx_cmd();
    cmd.arg("file1.json").arg("file2.json").arg("--flag");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("expected"));
    Ok(())
}
```

## 📝 コーディング規約

### 必須事項（CI強制）
- **フォーマット**: `cargo fmt --all`（警告ゼロ）
- **リント**: `cargo clippy`（警告ゼロ）
- **テスト**: すべてのテストが通過
- **ビルド**: クリーンワークスペースビルド

### 規約
- **エラーハンドリング**: エラー伝播には `anyhow::Result` を使用
- **型安全性**: Rustの型システムを活用
- **ドキュメント**: パブリックAPIには `///` コメント
- **命名**: Rustの命名規約に従う

### コミット形式
[Conventional Commits](https://www.conventionalcommits.org/) に従う:

```
feat(core): XMLの配列比較サポートを追加
fix(cli): 設定ファイル解析エラーを解決
docs(readme): インストール手順を更新
test(integration): CSVフォーマットのテストケースを追加
```

## 🎨 アーキテクチャガイドライン

### コア原則

1. **関心の分離**
   - `diffx-core`: 純粋な差分ロジック、I/Oなし
   - `diffx-cli`: CLIインターフェース、ファイル処理、出力フォーマット

2. **パフォーマンス第一**
   - `cargo bench` でクリティカルパスをベンチマーク
   - 大きなファイルに効率的なデータ構造
   - メモリ効率を意識したアルゴリズム

3. **型安全性**
   - すべてのパブリックAPIで強い型付け
   - 本番コードで `unwrap()` を避ける
   - 包括的なエラーハンドリング

### 新機能の追加

**新しいデータフォーマットサポート:**
1. `Format` に enum バリアントを追加
2. `parse_value()` でパーサーを実装
3. CLI統合を追加
4. 包括的なテストカバレッジ

**新しいCLIオプション:**
1. clap アノテーションで `Args` 構造体に追加
2. メインロジックと統合
3. ヘルプドキュメントを更新
4. 統合テストを追加

## 🐛 バグレポート

バグレポートには以下を含めてください:

1. **環境**
   - OS とバージョン
   - Rust バージョン (`rustc --version`)
   - diffx バージョン (`cargo run -- --version`)

2. **再現方法**
   - 最小限の例
   - 入力ファイル（共有しても安全な場合）
   - 使用したコマンドライン
   - 期待される出力 vs 実際の出力

3. **コンテキスト**
   - エラーメッセージ
   - スタックトレース
   - 関連するイシュー

## 💡 機能リクエスト

機能をリクエストする際は:

1. **ユースケース**: 解決しようとしている問題を説明
2. **提案**: 具体的な実装アイデア
3. **例**: 具体的な使用例
4. **代替案**: 検討した他のアプローチ

## 📈 パフォーマンスガイドライン

パフォーマンスを最適化する際は:

1. **最初にベンチマーク**: `cargo bench` でベースラインを確立
2. **プロファイル**: 実際のボトルネックを特定
3. **測定**: 改善を検証
4. **文書化**: 最適化アプローチを説明

## 🌍 ドキュメント

### 種類
- **ユーザードキュメント** (`docs/user-guide/`): diffx の使用方法
- **API ドキュメント** (`docs/reference/`): 技術リファレンス
- **統合** (`docs/guides/`): 実世界での使用法
- **プロジェクト** (`docs/project/`): 開発情報

### ガイドライン
- 明確で簡潔な記述
- 実用的な例
- 関連セクションへの相互参照
- コードと例を最新に保つ

## 🚀 リリースプロセス

リリースはセマンティックバージョニングに従います:

1. **CI/CD**: すべてのチェックが通過する必要がある
2. **テスト**: 包括的なテストカバレッジ
3. **ドキュメント**: 新機能について更新
4. **変更ログ**: 破壊的変更を文書化

## 📄 ライセンス

貢献することにより、あなたの貢献がMITライセンスの下でライセンスされることに同意したことになります。

## 🙏 貢献者の認知

貢献者は以下で認知されます:
- 重要な貢献に対するリリースノート
- GitHub 貢献者リスト
- 主要機能に対する `CHANGELOG.md`

---

**質問がありますか？** GitHub Discussion を開くかイシューを作成してください。私たちがお手伝いします！🎉
# diffx への貢献

`diffx` への貢献を歓迎します！以下のガイドラインに従うことで、高品質で一貫性のあるコードベースを維持するのに役立ちます。

## 貢献方法

1.  **リポジトリをフォークする**: まず、GitHubで `diffx` リポジトリをフォークします。
2.  **フォークをクローンする**: フォークしたリポジトリをローカルマシンにクローンします。

    ```bash
    git clone https://github.com/your-username/diffx.git
    cd diffx
    ```

3.  **新しいブランチを作成する**: 機能追加やバグ修正のための新しいブランチを作成します。

    ```bash
    git checkout -b feature/your-feature-name
    # または
    git checkout -b bugfix/your-bug-fix-name
    ```

4.  **変更を加える**: コーディングスタイルとガイドライン（下記参照）に従って変更を実装します。

5.  **変更をテストする**: テストを実行して、変更が既存の機能に影響を与えていないこと、および新しい機能が適切にカバーされていることを確認します。

6.  **変更をコミットする**: 明確で簡潔なコミットメッセージを作成します。[Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/) の仕様に従ってください（例：`feat: add new feature`、`fix: resolve bug`）。

7.  **フォークにプッシュする**: 新しいブランチをGitHubのフォークにプッシュします。

    ```bash
    git push origin feature/your-feature-name
    ```

8.  **プルリクエストを作成する**: フォークしたリポジトリから `diffx` リポジトリの `main` ブランチにプルリクエストを開きます。変更内容を明確に記述してください。

## 開発環境のセットアップ

`diffx` はRustで書かれています。開始するには、Rustツールチェインをインストールする必要があります。

1.  **Rustのインストール**: Rustがインストールされていない場合は、`rustup` の使用をお勧めします。

    ```bash
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
    ```

    画面の指示に従ってください。インストール後、ターミナルを再起動するか、`source $HOME/.cargo/env` を実行します。

2.  **プロジェクトのビルド**: `diffx` リポジトリのルートに移動し、プロジェクトをビルドします。

    ```bash
    cargo build
    ```

3.  **テストの実行**: すべてが正しくセットアップされていることを確認し、変更を検証するためにテストを実行します。

    ```bash
    cargo test
    ```

## コーディングスタイルとガイドライン

*   **Rustfmt**: コードのフォーマットには `rustfmt` を使用します。以下のコマンドを実行して、コードがフォーマットされていることを確認してください。

    ```bash
    cargo fmt
    ```

*   **Clippy**: リントには `clippy` を使用します。以下のコマンドを実行して、コードが `clippy` チェックをパスすることを確認してください。

    ```bash
    cargo clippy
    ```

*   **エラーハンドリング**: 堅牢なエラーハンドリングのために `anyhow` と `thiserror` を使用します。
*   **テスト**: すべての新しい機能とバグ修正には、適切なユニットテストおよび/または統合テストを伴う必要があります。
*   **ドキュメント**: 複雑なコードセクションにはコメントを追加し、新しい機能や重要な変更については関連するドキュメント（例：`README.md`、`docs/`）を更新してください。

## バグの報告

バグを見つけた場合は、[GitHub Issuesページ](https://github.com/your-org/diffx/issues) でissueを開いてください。バグの明確な説明、再現手順、期待される動作を提供してください。

## 機能リクエスト

新しい機能のアイデアがある場合は、[GitHub Issuesページ](https://github.com/your-org/diffx/issues) でissueを開いて議論してください。ご提案をお待ちしております！

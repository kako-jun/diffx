# インストールガイド

このガイドでは、様々なプラットフォームと環境での `diffx` のインストール方法を説明します。

## 目次

- [システム要件](#システム要件)
- [プラットフォーム別インストール](#プラットフォーム別インストール)
- [パッケージマネージャー](#パッケージマネージャー)
- [ソースからのビルド](#ソースからのビルド)
- [Dockerを使用したインストール](#dockerを使用したインストール)
- [インストールの確認](#インストールの確認)
- [アップデート](#アップデート)
- [アンインストール](#アンインストール)
- [トラブルシューティング](#トラブルシューティング)

## システム要件

### 最小要件
- **OS**: Linux (glibc 2.17+), macOS 10.12+, Windows 10+
- **アーキテクチャ**: x86_64 (AMD64), ARM64 (Apple Silicon対応)
- **RAM**: 最小 128MB、推奨 512MB以上
- **ディスク容量**: 15MB (バイナリ単体)

### 推奨環境
- **OS**: 最新の安定版
- **RAM**: 1GB以上（大きなファイル処理時）
- **CPU**: マルチコア（並列処理時）

## プラットフォーム別インストール

### Linux

#### Cargo経由（推奨）
```bash
# Rustがインストール済みの場合
cargo install diffx

# パスの確認
which diffx
# /home/username/.cargo/bin/diffx
```

#### バイナリリリース
```bash
# 最新版のダウンロード（x86_64）
curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz

# バイナリをシステムパスに移動
sudo mv diffx /usr/local/bin/
chmod +x /usr/local/bin/diffx

# インストール確認
diffx --version
```

#### ARM64 Linux
```bash
# ARM64版のダウンロード
curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-aarch64-unknown-linux-gnu.tar.gz" | tar -xz

sudo mv diffx /usr/local/bin/
chmod +x /usr/local/bin/diffx
```

#### Linux ディストリビューション別

**Ubuntu/Debian:**
```bash
# システム更新
sudo apt update

# Rustインストール（まだの場合）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# diffxインストール
cargo install diffx

# またはバイナリ使用
wget https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz
tar -xzf diffx-x86_64-unknown-linux-gnu.tar.gz
sudo mv diffx /usr/local/bin/
```

**CentOS/RHEL/Fedora:**
```bash
# Rustインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# diffxインストール
cargo install diffx

# システムワイドインストール
sudo cp ~/.cargo/bin/diffx /usr/local/bin/
```

**Arch Linux:**
```bash
# AUR経由（コミュニティパッケージ）
yay -S diffx

# または手動
cargo install diffx
```

### macOS

#### Homebrew（予定）
```bash
# 将来的にHomebrew対応予定
# brew install diffx
```

#### Cargo経由
```bash
# Rustインストール（まだの場合）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# diffxインストール
cargo install diffx
```

#### バイナリリリース（Intel Mac）
```bash
# Intel Mac用
curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-apple-darwin.tar.gz" | tar -xz

sudo mv diffx /usr/local/bin/
chmod +x /usr/local/bin/diffx
```

#### バイナリリリース（Apple Silicon）
```bash
# Apple Silicon (M1/M2) Mac用
curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-aarch64-apple-darwin.tar.gz" | tar -xz

sudo mv diffx /usr/local/bin/
chmod +x /usr/local/bin/diffx
```

#### macOS固有の注意事項
```bash
# Gatekeeperの警告が出る場合
sudo xattr -rd com.apple.quarantine /usr/local/bin/diffx

# または、システム環境設定 > セキュリティとプライバシー で許可
```

### Windows

#### Cargo経由
```powershell
# Rustインストール（まだの場合）
# https://rustup.rs/ からrustup-init.exeをダウンロード・実行

# diffxインストール
cargo install diffx

# パスの確認
where diffx
# C:\Users\username\.cargo\bin\diffx.exe
```

#### バイナリリリース
```powershell
# PowerShellで実行
Invoke-WebRequest -Uri "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-pc-windows-msvc.zip" -OutFile "diffx.zip"
Expand-Archive -Path "diffx.zip" -DestinationPath "."

# PATHに追加（例：C:\Tools\diffx\）
# システム環境変数のPATHに追加
```

#### Windows固有の設定
```powershell
# 実行ポリシーの設定（必要に応じて）
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# Windows Defenderの除外設定（誤検知される場合）
Add-MpPreference -ExclusionPath "C:\Users\%USERNAME%\.cargo\bin\diffx.exe"
```

## パッケージマネージャー

### Cargo（Rust公式）
```bash
# 標準インストール
cargo install diffx

# 特定バージョンのインストール
cargo install diffx --version 0.3.0

# 強制再インストール
cargo install diffx --force

# 開発版（最新のmainブランチ）
cargo install --git https://github.com/kako-jun/diffx.git
```

### Scoop（Windows）
```powershell
# Scoopインストール（まだの場合）
iwr -useb get.scoop.sh | iex

# diffx追加（将来的に対応予定）
# scoop install diffx
```

### Chocolatey（Windows）
```powershell
# Chocolateyインストール（まだの場合）
Set-ExecutionPolicy Bypass -Scope Process -Force; iex ((New-Object System.Net.WebClient).DownloadString('https://chocolatey.org/install.ps1'))

# diffx追加（将来的に対応予定）
# choco install diffx
```

### Node.jsエコシステム

```bash
# 近日対応予定
npm install diffx-js
npx diffx-js file1.json file2.json
```

### Pythonエコシステム

```bash
# 🆕 バイナリ埋め込み完全自己完結型wheel（v0.5.1+）
pip install diffx-python

# Pythonプロジェクトで使用
import diffx
result = diffx.diff('file1.json', 'file2.json')
print(result)

# インストール確認
import diffx
print("diffx 利用可能:", diffx.is_diffx_available())
print("バージョン:", diffx.__version__)
```

**Pythonパッケージの主な利点（v0.5.1+）:**
- **🚀 ゼロセットアップ**: 外部ダウンロードやバイナリ管理が不要
- **📦 完全自己完結**: 必要なものはすべてwheelに含まれています
- **⚡ 高速インストール**: `pip install` 後のネットワーク依存なし
- **🔒 セキュア**: 外部ソースからの実行時ダウンロードなし
- **🌐 オフライン対応**: エアギャップ環境でも動作

Pythonパッケージは [maturin](https://github.com/PyO3/maturin) を使用してネイティブ `diffx` バイナリをPython wheelに直接埋め込んでおり、`ruff` などのツールと同様の仕組みです。

## ソースからのビルド

### 前提条件
```bash
# Rustツールチェーンのインストール
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# 必要コンポーネントの確認
rustc --version  # 1.70.0以上が必要
cargo --version
```

### ソースコードの取得とビルド
```bash
# リポジトリのクローン
git clone https://github.com/kako-jun/diffx.git
cd diffx

# リリースビルド
cargo build --release

# バイナリの場所
ls -la target/release/diffx

# システムにインストール
cargo install --path .

# または手動コピー
sudo cp target/release/diffx /usr/local/bin/
```

### 開発者向けビルド
```bash
# デバッグビルド
cargo build

# テスト実行
cargo test

# フォーマットチェック
cargo fmt --check

# リントチェック
cargo clippy

# ベンチマーク実行
cargo bench
```

### カスタムビルド
```bash
# 特定の機能のみ有効化
cargo build --release --no-default-features --features "json,yaml"

# 全機能有効化
cargo build --release --features "all-formats"

# 最適化レベル指定
RUSTFLAGS="-C target-cpu=native" cargo build --release
```

## Dockerを使用したインストール

### 公式Dockerイメージ（将来予定）
```bash
# Docker Hubから実行（将来的に提供予定）
# docker run --rm -v $(pwd):/data kako-jun/diffx:latest file1.json file2.json
```

### 自作Dockerイメージ
```dockerfile
# Dockerfile
FROM rust:1.75 as builder

WORKDIR /usr/src/diffx
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/src/diffx/target/release/diffx /usr/local/bin/diffx

ENTRYPOINT ["diffx"]
```

```bash
# ビルドと実行
docker build -t my-diffx .
docker run --rm -v $(pwd):/data my-diffx /data/file1.json /data/file2.json
```

### Docker Compose
```yaml
# docker-compose.yml
version: '3.8'
services:
  diffx:
    build: .
    volumes:
      - ./configs:/data
    command: ["/data/config1.json", "/data/config2.json"]
```

## インストールの確認

### 基本確認
```bash
# バージョン確認
diffx --version
# diffx 0.2.0

# ヘルプ表示
diffx --help

# インストール場所確認
which diffx
type diffx
```

### 機能確認
```bash
# サンプルファイル作成
echo '{"name": "test", "version": "1.0"}' > test1.json
echo '{"name": "test", "version": "1.1"}' > test2.json

# 基本テスト
diffx test1.json test2.json
# 期待される出力: ~ version: "1.0" -> "1.1"

# 全フォーマットテスト
echo -e "name=test\nversion=1.0" > test1.ini
echo -e "name=test\nversion=1.1" > test2.ini
diffx test1.ini test2.ini

# クリーンアップ
rm test*.json test*.ini
```

### パフォーマンステスト
```bash
# 大きなファイルでのテスト
curl -s https://api.github.com/repos/rust-lang/rust > large1.json
cp large1.json large2.json

# 実行時間測定
time diffx large1.json large2.json

# メモリ使用量確認（Linux）
/usr/bin/time -v diffx large1.json large2.json
```

## アップデート

### Cargo経由でのアップデート
```bash
# 現在のバージョン確認
diffx --version

# 最新版へアップデート
cargo install diffx --force

# 特定バージョンへアップデート
cargo install diffx --version 0.2.1 --force
```

### バイナリの手動アップデート
```bash
# 現在のインストール場所確認
which diffx

# 新しいバイナリのダウンロード
curl -L "https://github.com/kako-jun/diffx/releases/latest/download/diffx-x86_64-unknown-linux-gnu.tar.gz" | tar -xz

# 既存のバイナリをバックアップ
sudo cp /usr/local/bin/diffx /usr/local/bin/diffx.backup

# 新しいバイナリで置換
sudo mv diffx /usr/local/bin/
sudo chmod +x /usr/local/bin/diffx

# アップデート確認
diffx --version
```

### 自動アップデート（将来予定）
```bash
# 自動アップデート機能（将来実装予定）
# diffx self-update
```

## アンインストール

### Cargo経由でインストールした場合
```bash
# cargoでアンインストール
cargo uninstall diffx

# 設定ファイルも削除する場合
rm -rf ~/.config/diffx/
```

### 手動インストールの場合
```bash
# バイナリ削除
sudo rm /usr/local/bin/diffx

# 設定ファイル削除
rm -rf ~/.config/diffx/

# シェル設定の確認（必要に応じて）
# ~/.bashrc, ~/.zshrc等から diffx 関連の設定を削除
```

### Windows
```powershell
# Cargo経由の場合
cargo uninstall diffx

# 手動インストールの場合
Remove-Item "C:\Tools\diffx\diffx.exe"

# 環境変数PATHからパスを削除
```

## トラブルシューティング

### 一般的な問題

#### 1. コマンドが見つからない
```bash
# 問題: diffx: command not found

# 解決策1: パスの確認
echo $PATH
which diffx

# 解決策2: パスを手動追加
export PATH="$HOME/.cargo/bin:$PATH"
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bashrc

# 解決策3: シンボリックリンク作成
sudo ln -s ~/.cargo/bin/diffx /usr/local/bin/diffx
```

#### 2. 権限エラー
```bash
# 問題: Permission denied

# 解決策1: 実行権限付与
chmod +x /path/to/diffx

# 解決策2: 所有者変更
sudo chown $USER:$USER /path/to/diffx

# 解決策3: sudoで実行
sudo diffx file1.json file2.json
```

#### 3. ライブラリエラー（Linux）
```bash
# 問題: error while loading shared libraries

# 解決策1: システム更新
sudo apt update && sudo apt upgrade

# 解決策2: 必要ライブラリインストール
sudo apt install libc6-dev

# 解決策3: 静的リンクビルド
RUSTFLAGS="-C target-feature=+crt-static" cargo build --release
```

### プラットフォーム固有の問題

#### macOS
```bash
# 問題: "diffx" cannot be opened because the developer cannot be verified

# 解決策1: Gatekeeperの無効化
sudo spctl --master-disable

# 解決策2: 個別許可
sudo xattr -rd com.apple.quarantine /usr/local/bin/diffx

# 解決策3: システム設定で手動許可
# システム環境設定 > セキュリティとプライバシー > 一般
```

#### Windows
```powershell
# 問題: 実行ポリシーエラー

# 解決策1: 実行ポリシー変更
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser

# 解決策2: 一時的な実行許可
PowerShell -ExecutionPolicy Bypass -File script.ps1

# 問題: Windows Defenderの誤検知
# 解決策: 除外設定
Add-MpPreference -ExclusionPath "C:\path\to\diffx.exe"
```

### パフォーマンスの問題

#### 1. 実行が遅い
```bash
# 診断: 詳細モードで実行
diffx file1.json file2.json --help

# 解決策1: パスフィルタリング使用
diffx large1.json large2.json --path "specific.section"

# 解決策2: 不要フィールド除外
diffx file1.json file2.json --ignore-keys-regex "^(timestamp|logs)"
```

#### 2. メモリ不足
```bash
# 診断: メモリ使用量確認
/usr/bin/time -v diffx large1.json large2.json

# 解決策1: スワップ増加
sudo swapon --show
sudo fallocate -l 2G /swapfile
sudo chmod 600 /swapfile
sudo mkswap /swapfile
sudo swapon /swapfile

# 解決策2: ファイル分割処理
split -l 1000 large_file.json chunk_
for chunk in chunk_*; do diffx "$chunk" "${chunk}.new"; done
```

### サポート情報

#### ログ収集
```bash
# デバッグ情報収集
diffx --version > debug_info.txt
echo "--- System Info ---" >> debug_info.txt
uname -a >> debug_info.txt
echo "--- Error Output ---" >> debug_info.txt
diffx file1.json file2.json 2>> debug_info.txt
```

#### 問題報告
```bash
# GitHub Issueテンプレート用情報
echo "OS: $(uname -a)"
echo "diffx version: $(diffx --version)"
echo "Cargo version: $(cargo --version)"
echo "Rust version: $(rustc --version)"
```

### ヘルプリソース

- **GitHub Issues**: [https://github.com/kako-jun/diffx/issues](https://github.com/kako-jun/diffx/issues)
- **GitHub Discussions**: [https://github.com/kako-jun/diffx/discussions](https://github.com/kako-jun/diffx/discussions)
- **ドキュメント**: [https://github.com/kako-jun/diffx/tree/main/docs](https://github.com/kako-jun/diffx/tree/main/docs)

困った時は、上記のリソースで検索するか、新しいIssueを作成してください。詳細な環境情報とエラーメッセージを含めると、より迅速なサポートを受けられます。
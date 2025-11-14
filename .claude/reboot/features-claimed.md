# README_ja.md で主張されている機能リスト

作成日: 2025-11-14
参照元: `/home/d131/repos/2025/diffx/README_ja.md`

## 📋 対応フォーマット

README_ja.mdで対応していると書かれているフォーマット:

- [ ] JSON
- [ ] YAML
- [ ] TOML
- [ ] XML
- [ ] INI
- [ ] CSV

## 🔧 差分の種類

README_ja.mdで検出できると書かれている差分:

- [ ] キーの追加・削除
- [ ] 値の変更
- [ ] 配列の挿入・削除・変更
- [ ] ネスト構造の差分
- [ ] 値の型変更

## 📤 出力形式

README_ja.mdでサポートしていると書かれている出力形式:

- [ ] CLI表示（デフォルト）- 色分け、記号（+, -, ~, !）、インデント
- [ ] JSON形式（`--output json`）
- [ ] YAML形式（`--output yaml`）

## ⚙️ CLIオプション

README_ja.mdで使えると書かれているオプション:

### 基本オプション
- [ ] `--output json` - JSON形式で出力
- [ ] `--output yaml` - YAML形式で出力

### フィルタリングオプション
- [ ] `--ignore-keys-regex` - 正規表現でキーを無視
- [ ] `--array-id-key` - 配列要素の識別キー指定
- [ ] `--epsilon` - 数値比較の誤差許容

### 実用的オプション
- [ ] `--ignore-case` - 大文字小文字の違いを無視
- [ ] `--ignore-whitespace` - 空白の変更を無視
- [ ] `--quiet` - 差分がない場合はexit 0、ある場合はexit 1
- [ ] `--brief` - 高速ディレクトリ変更チェック

### その他の機能
- [ ] ディレクトリ比較（自動再帰検出）
- [ ] メタチェイン（差分の差分）

## 🎯 主張されている特徴

README_ja.mdで謳っている特徴:

- [ ] **意味的認識**: フォーマット、キー順序、空白、ケツカンマを無視
- [ ] **AI対応**: クリーンなCLI出力
- [ ] **高速**: Rustで構築

## 🔗 統合例

README_ja.mdで示されている統合方法:

- [ ] CI/CDパイプライン統合
- [ ] Gitフック統合
- [ ] JSON出力でのパイプライン処理

## 📦 インストール方法

README_ja.mdで提示されているインストール方法:

- [ ] `cargo install diffx`
- [ ] `npm install diffx-js`
- [ ] `pip install diffx-python`
- [ ] GitHub Releasesからバイナリダウンロード

---

## 🚨 次のステップ

このリストの各項目を実際に試して、動作を確認する。

確認方法:
1. 各フォーマットでテストファイルを作成
2. diffxで差分を取得
3. 期待通りの動作か確認
4. 動いたら✓、動かなかったら問題を記録

結果は `verified-features.md` に記録する。

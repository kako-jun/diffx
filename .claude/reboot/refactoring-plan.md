# リファクタリング計画

作成日: 2025-11-14

## 🚨 現在の問題

### 現状
```
diffx-core/src/lib.rs: 1153行（1ファイルのみ）
diffx-cli/src/main.rs:  714行（1ファイルのみ）
```

**問題点**:
- すべてのロジックが1ファイルに集中
- モジュール化されていない
- 保守性が低い
- テストがしにくい
- 責任が分離されていない

## 🎯 目指す構造

### diffx-core の理想的な構造

```
diffx-core/src/
├── lib.rs              # 公開API、再エクスポート（50-100行程度）
├── types.rs            # 型定義（DiffResult, DiffOptions など）
├── parser/
│   ├── mod.rs          # パーサーモジュール
│   ├── json.rs         # JSON パーサー
│   ├── yaml.rs         # YAML パーサー
│   ├── toml.rs         # TOML パーサー
│   ├── xml.rs          # XML パーサー
│   ├── ini.rs          # INI パーサー
│   ├── csv.rs          # CSV パーサー
│   └── format.rs       # フォーマット検出
├── diff/
│   ├── mod.rs          # 差分検出モジュール
│   ├── core.rs         # コア差分ロジック
│   ├── arrays.rs       # 配列差分
│   ├── objects.rs      # オブジェクト差分
│   └── recursive.rs    # 再帰的差分
├── io/
│   ├── mod.rs          # 入出力モジュール
│   ├── files.rs        # ファイル操作
│   └── directories.rs  # ディレクトリ操作
└── utils.rs            # ユーティリティ関数
```

**各モジュールの責任**:

1. **lib.rs** (50-100行)
   - 公開APIの定義
   - モジュールの再エクスポート
   - ドキュメントコメント

2. **types.rs** (150-200行)
   - `DiffResult` 型
   - `DiffOptions` 型
   - その他の公開型

3. **parser/** (各100-150行)
   - フォーマット別パーサー
   - フォーマット検出ロジック
   - エラーハンドリング

4. **diff/** (各150-250行)
   - 差分検出のコアロジック
   - 配列差分（ID追跡、インデックス）
   - オブジェクト差分
   - 再帰的な差分検出

5. **io/** (各100-200行)
   - ファイル読み込み
   - ディレクトリ走査
   - パス処理

### diffx-cli の理想的な構造

```
diffx-cli/src/
├── main.rs             # エントリーポイント（50-100行程度）
├── cli/
│   ├── mod.rs          # CLIモジュール
│   ├── args.rs         # 引数定義（clap）
│   └── parser.rs       # 引数パース・検証
├── input/
│   ├── mod.rs          # 入力モジュール
│   ├── file.rs         # ファイル入力
│   ├── stdin.rs        # 標準入力
│   └── format.rs       # フォーマット推論
├── output/
│   ├── mod.rs          # 出力モジュール
│   ├── cli.rs          # CLI表示形式
│   ├── json.rs         # JSON出力
│   └── yaml.rs         # YAML出力
└── run.rs              # メイン実行ロジック
```

**各モジュールの責任**:

1. **main.rs** (50-100行)
   - エントリーポイント
   - エラーハンドリング
   - 終了コード設定

2. **cli/** (各100-150行)
   - コマンドライン引数定義
   - 引数パース
   - バリデーション

3. **input/** (各100-150行)
   - ファイル読み込み
   - 標準入力処理
   - フォーマット推論

4. **output/** (各100-200行)
   - 各種出力形式
   - カラー表示
   - フォーマット変換

5. **run.rs** (150-250行)
   - メイン実行ロジック
   - オプション構築
   - 入出力の調整

## 📋 リファクタリング手順

### Phase 1: diffx-core のリファクタリング（優先度: 高）

1. **types.rs の抽出**
   - `DiffResult` enum
   - `DiffOptions` struct
   - `OutputFormat` enum
   - その他の公開型

2. **parser/ モジュールの作成**
   - `parser/mod.rs` 作成
   - `parser/json.rs` - `parse_json()` を移動
   - `parser/yaml.rs` - `parse_yaml()` を移動
   - `parser/toml.rs` - `parse_toml()` を移動
   - `parser/xml.rs` - `parse_xml()` を移動
   - `parser/ini.rs` - `parse_ini()` を移動
   - `parser/csv.rs` - `parse_csv()` を移動
   - `parser/format.rs` - フォーマット検出を移動

3. **diff/ モジュールの作成**
   - `diff/mod.rs` 作成
   - `diff/core.rs` - `diff()` 関数を移動
   - `diff/arrays.rs` - 配列差分関数を移動
   - `diff/objects.rs` - オブジェクト差分を移動
   - `diff/recursive.rs` - 再帰的差分を移動

4. **io/ モジュールの作成**
   - `io/mod.rs` 作成
   - `io/files.rs` - `diff_files()` を移動
   - `io/directories.rs` - `diff_directories()` を移動

5. **lib.rs の整理**
   - モジュール宣言
   - 公開APIの再エクスポート
   - ドキュメント整備

### Phase 2: diffx-cli のリファクタリング（優先度: 中）

1. **cli/ モジュールの作成**
   - `cli/mod.rs` 作成
   - `cli/args.rs` - Args struct を移動
   - `cli/parser.rs` - `build_diff_options()` を移動

2. **input/ モジュールの作成**
   - `input/mod.rs` 作成
   - `input/file.rs` - `read_input()` を移動
   - `input/stdin.rs` - 標準入力処理を移動
   - `input/format.rs` - `infer_format_from_path()` を移動

3. **output/ モジュールの作成**
   - `output/mod.rs` 作成
   - `output/cli.rs` - `print_cli_output()` を移動
   - `output/json.rs` - JSON出力を移動
   - `output/yaml.rs` - YAML出力を移動

4. **run.rs の作成**
   - `run()` 関数を移動
   - メイン実行ロジック

5. **main.rs の簡素化**
   - エントリーポイントのみ
   - エラーハンドリング

### Phase 3: テストの整理（優先度: 低）

1. **ユニットテストの追加**
   - 各モジュールに `#[cfg(test)]` を追加
   - モジュール単位でテスト

2. **統合テストの見直し**
   - 既存のテストを検証
   - 新しい構造に合わせて修正

## ⏰ 実行タイミング

**今すぐやるべきか？**

**NO - まだ早い**

理由：
1. **Phase 2（真実の特定）が未完了**
   - まだ全機能の動作確認ができていない
   - どの機能が本当に動作するか不明確

2. **仕様が不明確**
   - `--ignore-case` の挙動が不明
   - 未検証のオプションが多数存在
   - ドキュメントと実装の乖離がある

3. **リファクタリングのリスク**
   - 大規模な変更になる
   - 既存のテストが多数失敗している（86 passed; 83 failed）
   - 何が壊れたか判断できない

**正しい順序**:

```
1. Phase 2 完了: 真実の特定 ← 今ここ
   - すべての機能を検証
   - 動作する機能を明確化
   - 新しい仕様書作成

2. Phase 2.5: テストの整理
   - 動作する機能のテストを修正
   - テストが通る状態にする
   - リファクタリングの基準を作る

3. Phase 3: リファクタリング ← ここで実行
   - テストが通る状態から開始
   - モジュール化
   - テストで検証しながら進める

4. Phase 4: GitHub Actions簡素化
   - 新しい構造でCI/CD

5. Phase 5: README更新
   - 検証済み機能のみ記載
```

## 📝 次のアクション

**今やること**:
1. ❌ リファクタリング開始（まだ早い）
2. ✅ Phase 2 を完了させる
   - 残りのオプション検証
   - 新しい仕様書作成
3. ✅ テストを修正して通す
4. ✅ その後、リファクタリング計画を実行

**このドキュメントの位置づけ**:
- リファクタリングの青写真
- Phase 3 で参照する設計書
- 今すぐ実行はしない
- Phase 2 完了後に再検討

## 🎯 リファクタリングの価値

**メリット**:
- 保守性向上
- テストしやすい
- 責任が明確
- 新機能追加が容易
- バグの局所化

**デメリット**:
- 時間がかかる
- 一時的に不安定になる可能性
- 大規模な変更

**結論**:
価値はあるが、**Phase 2 完了後に実施すべき**。
今は「真実の特定」に集中する。

---

**作成日**: 2025-11-14
**優先度**: Phase 2 完了後
**合言葉**: 「疑って、確認して、記録する」→「リファクタリングは確認の後」

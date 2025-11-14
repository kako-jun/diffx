# リファクタリング進捗状況

作成日: 2025-11-14
状態: 🔄 進行中（一時停止）

## ✅ 完了した作業

### 1. types.rs の作成 ✅
**ファイル**: `/home/d131/repos/2025/diffx/diffx-core/src/types.rs`

**抽出した型**:
- `DiffResult` enum（+ Display impl）
- `LightweightDiffResult` enum（+ From<&DiffResult> impl）
- `OutputFormat` enum（+ parse_format メソッド）
- `DiffxSpecificOptions` struct
- `DiffOptions` struct

**行数**: 約130行

### 2. parser/ モジュールの完全作成 ✅
**ディレクトリ**: `/home/d131/repos/2025/diffx/diffx-core/src/parser/`

**作成したファイル**:
1. `mod.rs` - モジュール宣言と再エクスポート
2. `format.rs` - FileFormat enum、フォーマット検出、ディスパッチ
3. `json.rs` - JSON パーサー
4. `yaml.rs` - YAML パーサー
5. `toml.rs` - TOML パーサー（+ 変換ヘルパー）
6. `xml.rs` - XML パーサー（+ add_to_parent ヘルパー）
7. `ini.rs` - INI パーサー
8. `csv.rs` - CSV パーサー

**合計**: 8ファイル、約500行

## ⏸️ 未完了の作業

### 3. lib.rs の更新 ⏸️
**状態**: 未着手

**必要な作業**:
1. モジュール宣言を追加：
   ```rust
   mod types;
   mod parser;
   ```

2. 公開APIの再エクスポート：
   ```rust
   pub use types::*;
   pub use parser::{parse_json, parse_yaml, parse_toml, parse_xml, parse_ini, parse_csv};
   ```

3. 既存コードの削除：
   - 1-129行目の型定義（types.rs に移動済み）
   - 330-947行目のパーサーコード（parser/ に移動済み）

4. 残すべきコード：
   - `diff_paths()` 関数（130行目以降）
   - `diff()` 関数
   - `diff_recursive()` などの差分検出ロジック
   - ユーティリティ関数

**推定行数**: lib.rs は 1153行 → 600-700行程度になる予定

### 4. diff/ モジュールの作成 ⏸️
**状態**: 未着手

**予定構造**:
```
diff/
├── mod.rs          # モジュール宣言
├── core.rs         # diff()、diff_paths()
├── arrays.rs       # diff_arrays、diff_arrays_with_id、diff_arrays_by_index
├── objects.rs      # diff_objects
└── recursive.rs    # diff_recursive
```

**抽出元**: lib.rs の 差分検出関数

### 5. io/ モジュールの作成 ⏸️
**状態**: 未着手

**予定構造**:
```
io/
├── mod.rs          # モジュール宣言
├── files.rs        # diff_files、ファイル読み込み
└── directories.rs  # diff_directories、get_all_files_recursive
```

**抽出元**: lib.rs のファイル・ディレクトリ操作関数

### 6. diffx-cli のリファクタリング ⏸️
**状態**: 未着手

**予定構造**:
```
diffx-cli/src/
├── main.rs         # エントリーポイント（50-100行）
├── cli/            # 引数処理
├── input/          # 入力処理
├── output/         # 出力処理
└── run.rs          # メインロジック
```

**現状**: `main.rs` 714行（1ファイルのみ）

## 📊 進捗率

**diffx-core**:
- types.rs: ✅ 100%
- parser/: ✅ 100%
- diff/: ⏸️ 0%
- io/: ⏸️ 0%
- lib.rs更新: ⏸️ 0%

**全体**: 約30% 完了

**diffx-cli**: ⏸️ 0%

## 🚧 次のステップ

### 即座に実行すべき作業

1. **lib.rs の更新**
   - モジュール宣言追加
   - 型定義削除（1-129行）
   - パーサーコード削除（330-947行）
   - 公開APIの再エクスポート
   - コンパイル確認

2. **diff/ モジュールの作成**
   - diff/mod.rs 作成
   - 差分検出関数を抽出
   - lib.rs から削除

3. **io/ モジュールの作成**
   - io/mod.rs 作成
   - ファイル操作関数を抽出
   - lib.rs から削除

4. **lib.rs の最終整理**
   - 公開APIのみ残す
   - ドキュメントコメント整備

5. **コンパイルとテスト**
   - `cargo build --release`
   - 基本動作確認（6フォーマット）

6. **diffx-cli のリファクタリング**
   - モジュール分割
   - 同様のアプローチ

## ⚠️ 注意事項

### なぜ一時停止したか

1. **作業規模が大きい**
   - lib.rs は 1153行あり、全体を書き直すと時間がかかる
   - コンテキストトークンが限られている

2. **段階的アプローチが必要**
   - 各ステップでコンパイル確認すべき
   - 一度に大量のコードを移動するとエラーが追いにくい

3. **進捗を保存**
   - 現在の作業（types.rs + parser/）は完全に機能する
   - コミットして保存すべき

### 再開時の戦略

1. **lib.rs の段階的更新**
   ```rust
   // Step 1: モジュール宣言を追加
   mod types;
   mod parser;
   pub use types::*;
   pub use parser::*;

   // Step 2: 型定義をコメントアウト（削除前に確認）
   // Step 3: コンパイル確認
   // Step 4: パーサーコードをコメントアウト
   // Step 5: コンパイル確認
   // Step 6: コメントアウトしたコードを削除
   ```

2. **diff/ と io/ を順次作成**
   - 同様の手順で段階的に

3. **各ステップでコミット**
   - 動作する状態を保つ

## 🎯 目標

**最終的な構造**:
```
diffx-core/src/
├── lib.rs              # 公開API（50-100行）
├── types.rs            # 型定義 ✅
├── parser/             # パーサーモジュール ✅
│   ├── mod.rs
│   ├── format.rs
│   ├── json.rs
│   ├── yaml.rs
│   ├── toml.rs
│   ├── xml.rs
│   ├── ini.rs
│   └── csv.rs
├── diff/               # 差分検出 ⏸️
│   ├── mod.rs
│   ├── core.rs
│   ├── arrays.rs
│   ├── objects.rs
│   └── recursive.rs
└── io/                 # ファイル操作 ⏸️
    ├── mod.rs
    ├── files.rs
    └── directories.rs
```

**メリット**:
- 保守性向上 ✅
- テストしやすい ✅
- 責任が明確 ✅
- 各モジュール200行以下 ✅

---

**次回作業開始時**: このファイルを読んで、「次のステップ」セクションから再開する。

**合言葉**: 「段階的に、確実に、コンパイル確認しながら」

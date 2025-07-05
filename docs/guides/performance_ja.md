# パフォーマンスガイド

このガイドでは、`diffx`のパフォーマンス特性、ベンチマーク、最適化戦略について説明します。

## 目次

- [パフォーマンス概要](#パフォーマンス概要)
- [ベンチマーク](#ベンチマーク)
- [最適化戦略](#最適化戦略)
- [メモリ管理](#メモリ管理)
- [大きなファイルの処理](#大きなファイルの処理)
- [バッチ処理](#バッチ処理)
- [パフォーマンス監視](#パフォーマンス監視)
- [トラブルシューティング](#トラブルシューティング)

## パフォーマンス概要

`diffx`は高性能を追求して設計されており、Rustのゼロコスト抽象化とメモリ安全性を活用しています。パフォーマンス特性は以下の要因によって変化します：

- **ファイルサイズ**: リーズナブルなメモリ使用量でリニアにスケール
- **データ構造の複雑さ**: ネストしたオブジェクトはより多くの処理が必要
- **比較オプション**: 正規表現フィルタリングと配列ID追跡はオーバーヘッドを追加
- **出力形式**: JSON/YAML出力は追加のシリアライゼーションが必要

### 主要なパフォーマンス機能

- **可能な限りゼロコピー解析**
- **大きなファイルに対応したストリーミング親和性アーキテクチャ**
- **Rustの所有権システムを使用した効率的なメモリレイアウト**
- **構造化データ用に最適化された差分アルゴリズム**
- **ディレクトリ比較の並列処理サポート**

## ベンチマーク

すべてのベンチマークは以下の環境で実行：
- **CPU**: AMD Ryzen 7 5800X (8コア、16スレッド、3.8GHzベース)
- **メモリ**: 32GB DDR4-3600
- **ストレージ**: NVMe SSD
- **OS**: Linux 6.15.4-arch2-1

### ファイルサイズパフォーマンス

| ファイルサイズ | diffx | GNU diff | jq (スクリプト) | メモリ使用量 |
|-----------|-------|----------|---------------|--------------|
| 1KB | 0.8ms | 1.2ms | 12ms | 8MB |
| 10KB | 1.5ms | 2.1ms | 28ms | 12MB |
| 100KB | 2.8ms | 4.5ms | 85ms | 18MB |
| 1MB | 8.2ms | 15ms | 320ms | 35MB |
| 10MB | 65ms | 120ms | 2.8s | 180MB |
| 100MB | 580ms | 1.2s | 28s | 1.2GB |

### フォーマット別パフォーマンス

| フォーマット | 1MBファイル | 10MBファイル | メモリオーバーヘッド |
|--------|----------|-----------|-----------------|
| **JSON** | 8.2ms | 65ms | ベースライン |
| **YAML** | 12.1ms | 95ms | +15% |
| **TOML** | 9.8ms | 78ms | +8% |
| **XML** | 18.5ms | 145ms | +35% |
| **INI** | 6.9ms | 52ms | -12% |
| **CSV** | 15.2ms | 118ms | +25% |

### 操作別ベンチマーク

#### 基本比較 (1MB JSON)
```bash
# ベースライン比較
time diffx file1.json file2.json
# 平均: 8.2ms ± 0.8ms
```

#### 正規表現フィルタリング使用時
```bash
# シンプルな正規表現パターン
time diffx file1.json file2.json --ignore-keys-regex "^timestamp$"
# 平均: 9.1ms ± 0.9ms (+11% オーバーヘッド)

# 複雑な正規表現パターン
time diffx file1.json file2.json --ignore-keys-regex "^(timestamp|_.*|temp_.*)$"
# 平均: 10.8ms ± 1.1ms (+32% オーバーヘッド)
```

#### 配列ID追跡
```bash
# ID追跡なし（位置ベース）
time diffx users1.json users2.json
# 平均: 12.3ms ± 1.2ms

# ID追跡あり
time diffx users1.json users2.json --array-id-key "id"
# 平均: 15.7ms ± 1.5ms (+28% オーバーヘッド)
```

#### 出力形式の影響
```bash
# CLI出力（デフォルト）
time diffx file1.json file2.json
# 平均: 8.2ms ± 0.8ms

# JSON出力
time diffx file1.json file2.json --output json
# 平均: 9.8ms ± 0.9ms (+19% オーバーヘッド)

# YAML出力
time diffx file1.json file2.json --output yaml
# 平均: 11.2ms ± 1.1ms (+37% オーバーヘッド)
```

### ディレクトリ比較ベンチマーク

| ディレクトリサイズ | ファイル数 | 合計サイズ | 時間 | メモリ |
|----------------|-------|------------|------|--------|
| 小 | 10ファイル | 1MB | 45ms | 25MB |
| 中 | 100ファイル | 50MB | 890ms | 180MB |
| 大 | 1000ファイル | 500MB | 8.2s | 1.5GB |

## 最適化戦略

### 1. パスフィルタリングの使用

大きなファイルの特定セクションに比較を集中：

```bash
# 大きなconfig全体を比較する代わりに
diffx large_config.json large_config.new.json

# 特定のセクションに集中
diffx large_config.json large_config.new.json --path "database.connections"
# パフォーマンス改善: 大きなconfigで60-80%
```

### 2. 正規表現パターンの最適化

キーフィルタリング用の効率的な正規表現パターンを使用：

```bash
# 非効率: バックトラッキングのある複雑なパターン
--ignore-keys-regex ".*_temp.*|.*_cache.*|.*_debug.*"

# 効率的: アンカーされたパターン
--ignore-keys-regex "^(.*_temp|.*_cache|.*_debug)$"

# 最も効率的: シンプルな選択肢
--ignore-keys-regex "^(_temp|_cache|_debug)_.*$"
```

### 3. 適切な出力形式の選択

用途に基づいて出力形式を選択：

```bash
# 人間が読む用 - 最速
diffx file1.json file2.json

# 自動処理用 - 中程度
diffx file1.json file2.json --output json

# レガシーツール統合用 - 最遅
diffx file1.json file2.json --output unified
```

### 4. バッチ処理の最適化

複数ファイルを効率的に処理：

```bash
# 順次処理（遅い）
for file in *.json; do
  diffx "$file" "${file}.backup"
done

# 並列処理（速い）
find . -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup || echo "Diff in {}"'
```

### 5. メモリ効率的な大ファイル処理

非常に大きなファイルには、ストリーミング親和性アプローチを使用：

```bash
# ファイルをセクションで処理
diffx huge1.json huge2.json --path "section1" > diff_section1.json &
diffx huge1.json huge2.json --path "section2" > diff_section2.json &
diffx huge1.json huge2.json --path "section3" > diff_section3.json &
wait
```

## メモリ管理

### メモリ使用パターン

`diffx`のメモリ使用は以下のパターンに従います：

1. **ファイルサイズとのリニアスケーリング**
2. **解析フェーズでのピーク使用量**
3. **比較フェーズでの使用量減少**
4. **出力シリアライゼーションによる二次ピーク**

### メモリ最適化のヒント

#### 1. ファイルを個別に処理
```bash
# 高メモリ使用 - 両方のファイルを同時にロード
diffx very_large1.json very_large2.json

# 低メモリ使用 - チャンクで処理
diffx very_large1.json very_large2.json --path "chunk1"
diffx very_large1.json very_large2.json --path "chunk2"
```

#### 2. 適切なデータ型の使用
```bash
# 数値比較により効率的
diffx data1.json data2.json --epsilon 0.001

# 効率的でない - 数値の厳密文字列比較
diffx data1.json data2.json
```

#### 3. 出力サイズの最小化
```bash
# 大きな出力 - すべてのコンテキストを含む
diffx file1.json file2.json --output json

# 小さな出力 - CLI形式はよりコンパクト
diffx file1.json file2.json
```

### メモリ監視

大きな操作中にメモリ使用量を監視：

```bash
# メモリ使用量を監視
/usr/bin/time -v diffx large1.json large2.json

# valgrindでのメモリプロファイリング（デバッグ用）
valgrind --tool=massif diffx file1.json file2.json
```

## 大きなファイルの処理

### 大ファイル用戦略

#### 1. 構造化細分化
```bash
# 100MBファイル全体を比較する代わりに
diffx huge1.json huge2.json

# 論理セクションで分割
diffx huge1.json huge2.json --path "users"
diffx huge1.json huge2.json --path "products"
diffx huge1.json huge2.json --path "orders"
```

#### 2. プログレッシブフィルタリング
```bash
# ステップ1: 変更されたセクションを特定
diffx config1.json config2.json --output json | jq '.[] | .Added[0] // .Modified[0] // .Removed[0]' | cut -d. -f1 | sort -u

# ステップ2: 変更されたセクションを詳しく調査
diffx config1.json config2.json --path "database"
diffx config1.json config2.json --path "services"
```

#### 3. サンプリング戦略
```bash
# 非常に大きなデータセットでは、最初にサンプルを比較
head -n 1000 large1.jsonl > sample1.json
head -n 1000 large2.jsonl > sample2.json
diffx sample1.json sample2.json --array-id-key "id"
```

### 大ファイルのベストプラクティス

1. **パスフィルタリングを使用**して関連セクションに集中
2. **正規表現フィルタリングを早期に適用**してデータサイズを削減
3. **可能な限り並列処理**
4. **処理中のメモリ使用量を監視**
5. **極端に大きなデータセットではファイル分割を検討**

## バッチ処理

### 並列ディレクトリ処理

ディレクトリ比較を最適化：

```bash
# 効率的な並列処理
find dir1/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} bash -c '
    file2="dir2/${1#dir1/}"
    if [[ -f "$file2" ]]; then
      diffx "$1" "$file2" --output json > "diff_$(basename "$1" .json).json"
    fi
  ' bash {}
```

### バッチ設定管理

複数環境設定の処理：

```bash
#!/bin/bash
# batch_config_compare.sh

ENVIRONMENTS=("dev" "staging" "prod")
BASE="prod"

for env in "${ENVIRONMENTS[@]}"; do
  if [[ "$env" != "$BASE" ]]; then
    echo "$envと$BASEを比較中..."
    
    # 異なる設定タイプを並列処理
    {
      diffx "configs/$env/app.json" "configs/$BASE/app.json" \
        --ignore-keys-regex "^(host|port|password)" \
        --output json > "diff_${env}_app.json"
    } &
    
    {
      diffx "configs/$env/db.json" "configs/$BASE/db.json" \
        --ignore-keys-regex "^(connection_string|credentials)" \
        --output json > "diff_${env}_db.json"
    } &
    
    wait  # 並列プロセスの完了を待機
  fi
done
```

### パイプライン統合

CI/CDパイプライン使用を最適化：

```bash
#!/bin/bash
# 最適化されたCIパイプライン差分チェック

# よく使用されるベース設定をキャッシュ
if [[ ! -f "baseline_config.json" ]] || [[ $(find baseline_config.json -mtime +1) ]]; then
  curl -s "$CONFIG_SOURCE" > baseline_config.json
fi

# クイックチェック - 変更が検出された場合のみ詳細な差分
if ! diffx baseline_config.json current_config.json >/dev/null 2>&1; then
  # 必要な場合のみ詳細解析
  diffx baseline_config.json current_config.json \
    --ignore-keys-regex "^(timestamp|build_id|deployment_time)" \
    --output json > detailed_diff.json
fi
```

## パフォーマンス監視

### 組み込みパフォーマンスメトリクス

アプリケーションでdiffxパフォーマンスを監視：

```bash
# 時間測定
time diffx file1.json file2.json

# 詳細なシステムメトリクス
/usr/bin/time -v diffx file1.json file2.json
```

### ベンチマークスクリプト

ユースケース用のカスタムベンチマークを作成：

```bash
#!/bin/bash
# benchmark_diffx.sh

ITERATIONS=10
FILES=("small.json" "medium.json" "large.json")

for file in "${FILES[@]}"; do
  echo "$fileをベンチマーク中..."
  
  total_time=0
  for i in $(seq 1 $ITERATIONS); do
    start_time=$(date +%s%3N)
    diffx "$file" "${file}.backup" >/dev/null
    end_time=$(date +%s%3N)
    
    duration=$((end_time - start_time))
    total_time=$((total_time + duration))
  done
  
  avg_time=$((total_time / ITERATIONS))
  echo "$fileの平均時間: ${avg_time}ms"
done
```

### パフォーマンス回帰テスト

CIにパフォーマンステストを含める：

```bash
# performance_test.sh
#!/bin/bash

BASELINE_TIME=100  # ミリ秒
CURRENT_TIME=$(time diffx test_file.json test_file.backup 2>&1 | grep real | cut -d' ' -f2)

if [[ $(echo "$CURRENT_TIME > $BASELINE_TIME * 1.5" | bc) -eq 1 ]]; then
  echo "パフォーマンス回帰が検出されました！"
  echo "現在: ${CURRENT_TIME}ms, ベースライン: ${BASELINE_TIME}ms"
  exit 1
fi
```

## トラブルシューティング

### 一般的なパフォーマンス問題

#### 1. 遅い正規表現処理
**問題**: 複雑な正規表現パターンによる速度低下
```bash
# 問題のあるパターン
--ignore-keys-regex ".*_(temp|cache|debug).*"
```

**解決策**: アンカーされた特定パターンを使用
```bash
# 最適化されたパターン
--ignore-keys-regex "^[^_]*_(temp|cache|debug)_[^_]*$"
```

#### 2. メモリ不足
**問題**: 大きなファイルでメモリ不足
```bash
# エラー: メモリ割り当て失敗
diffx huge1.json huge2.json
```

**解決策**: パスフィルタリングを使用またはチャンクで処理
```bash
# 管理可能なチャンクで処理
diffx huge1.json huge2.json --path "section1"
diffx huge1.json huge2.json --path "section2"
```

#### 3. 遅い配列処理
**問題**: 大きな配列の処理に時間がかかりすぎる
```bash
# IDなしの大きな配列で遅い
diffx users1.json users2.json
```

**解決策**: 利用可能な場合は配列IDキーを使用
```bash
# ID追跡でずっと高速
diffx users1.json users2.json --array-id-key "id"
```

### パフォーマンスデバッグ

#### 詳細出力を有効化
```bash
# diffxが何を処理しているかチェック
DIFFX_VERBOSE=true diffx file1.json file2.json
```

#### メモリ使用量をプロファイル
```bash
# メモリパターンを監視
valgrind --tool=massif diffx large1.json large2.json
ms_print massif.out.<pid>
```

#### CPUプロファイリング
```bash
# CPU使用量をプロファイル
perf record diffx large1.json large2.json
perf report
```

### 最適化チェックリスト

パフォーマンス問題を報告する前に：

- [ ] **パスフィルタリング**: 必要なセクションのみを比較していますか？
- [ ] **正規表現最適化**: 正規表現パターンはアンカーされ、特定ですか？
- [ ] **ファイルサイズ**: ファイルサイズに対して適切な比較ですか？
- [ ] **メモリ可用性**: 十分なRAMがありますか？
- [ ] **出力形式**: 最も効率的な出力形式を使用していますか？
- [ ] **配列処理**: 該当する場合に配列IDキーを使用していますか？
- [ ] **並列処理**: 利用可能なCPUコアを活用していますか？

## パフォーマンスベストプラクティス要約

1. **早期かつ頻繁にフィルタリング** - `--path`と`--ignore-keys-regex`を使用
2. **適切な出力形式を選択** - 表示はCLI、処理はJSON
3. **並列処理を活用** - バッチ操作で複数コアを使用
4. **リソース使用量を監視** - メモリとCPU使用率を監視
5. **定期的にプロファイル** - 特定のユースケースをベンチマーク
6. **正規表現パターンを最適化** - アンカーされた特定パターンを使用
7. **ファイル構造を考慮** - 効率的なアクセスのためにデータを構造化
8. **配列IDキーを使用** - 効率的な配列要素追跡を有効化

これらの最適化戦略は、様々なユースケースとデータサイズで`diffx`の最適なパフォーマンスを達成するのに役立ちます。
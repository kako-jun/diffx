# diffx 動作実証

v0.5.3の実際の動作を示す簡潔な例集

## 基本機能

**JSON比較** [`basic-json-diff.txt`](outputs/basic-json-diff.txt)
```bash
diffx config_v1.json config_v2.json
```

**詳細モード** [`verbose-mode.txt`](outputs/verbose-mode.txt) 
```bash
diffx config_v1.json config_v2.json --verbose
```

**JSON出力** [`output-json.txt`](outputs/output-json.txt)
```bash
diffx config_v1.json config_v2.json --output json
```

## フォーマット対応

**YAML比較** [`yaml-diff.txt`](outputs/yaml-diff.txt)
```bash
diffx config.yaml config_new.yaml
```

**TOML比較** [`toml-diff.txt`](outputs/toml-diff.txt)
```bash
diffx Cargo.toml Cargo_new.toml
```

**XML比較** [`xml-diff.txt`](outputs/xml-diff.txt)
```bash
diffx config.xml config_new.xml
```

**CSV比較** [`csv-diff.txt`](outputs/csv-diff.txt)
```bash
diffx data.csv data_new.csv
```

## 高度な機能

**コンテキスト表示** [`context-mode.txt`](outputs/context-mode.txt)
```bash
diffx config.json config_new.json --context 3
```

**大文字小文字無視** [`ignore-case.txt`](outputs/ignore-case.txt)
```bash
diffx config.json config_new.json --ignore-case
```

**サイレントモード** [`quiet-mode.txt`](outputs/quiet-mode.txt)
```bash
diffx config.json config_new.json --quiet
```

## システム情報

**ヘルプ** [`help-output.txt`](outputs/help-output.txt)
**バージョン** [`version-info.txt`](outputs/version-info.txt)

---

全12例、すべて動作確認済み。diffxは文書通りに機能します。
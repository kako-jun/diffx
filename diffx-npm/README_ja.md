# diffx-npm

`diffx` CLIツールのNode.jsラッパー

## インストール

```bash
npm install diffx-js
```

これにより、GitHub Releasesからお使いのシステムに適した `diffx` バイナリが自動的にダウンロードされます。

## 使い方

```javascript
const { runDiffx } = require('diffx-npm');

async function main() {
  // 2つのJSONファイルを比較
  let result = await runDiffx(["file1.json", "file2.json"]);

  if (result.code === 0) {
    console.log("違いはありません。");
  } else {
    console.log("違いが見つかりました：");
    console.log(result.stdout);
  }

  // diffx CLIでサポートされている任意の引数を渡すことができます
  result = await runDiffx(["file1.yaml", "file2.yaml", "--output", "json"]);
  console.log(result.stdout);
}

main();
```

## 開発

ローカル開発用にリンクするには：

```bash
npm link
```

## ライセンス

このプロジェクトはMITライセンスの下でライセンスされています。
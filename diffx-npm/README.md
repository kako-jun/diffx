# diffx-js

A Node.js wrapper for the `diffx` CLI tool.

## Installation

```bash
npm install diffx-js
```

This will automatically download the appropriate `diffx` binary for your system from GitHub Releases.

## Usage

```javascript
const { runDiffx } = require('diffx-js');

async function main() {
  // Compare two JSON files
  let result = await runDiffx(["file1.json", "file2.json"]);

  if (result.code === 0) {
    console.log("No differences found.");
  } else {
    console.log("Differences found:");
    console.log(result.stdout);
  }

  // You can pass any arguments supported by the diffx CLI
  result = await runDiffx(["file1.yaml", "file2.yaml", "--output", "json"]);
  console.log(result.stdout);
}

main();
```

## Development

To link for local development:

```bash
npm link
```

## License

This project is licensed under the MIT License.

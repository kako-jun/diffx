# diffx-js

A Node.js wrapper for the `diffx` CLI tool.

## Installation

```bash
npm install diffx-js
```

This will automatically download the appropriate `diffx` binary for your system from GitHub Releases.

## Usage

```javascript
const { diff, diffString } = require('diffx-js');

async function main() {
  // Compare two files
  const result = await diff('file1.json', 'file2.json');
  
  if (result.length === 0) {
    console.log("No differences found.");
  } else {
    console.log("Differences found:");
    for (const change of result) {
      console.log(`${change.type}: ${change.path} = ${change.new_value}`);
    }
  }

  // Compare with options
  const jsonResult = await diff('config1.yaml', 'config2.yaml', {
    output: 'json',
    ignoreKeysRegex: 'timestamp'
  });

  // Compare directory structures
  const dirResult = await diff('dir1/', 'dir2/', {
    recursive: true,
    output: 'json'
  });

  // Compare strings directly
  const stringResult = await diffString(
    '{"a": 1}', 
    '{"a": 2}', 
    'json',
    { output: 'json' }
  );
}

main();
```


### API Reference

#### `diff(input1, input2, options?)`
- **input1, input2**: File paths or directory paths to compare
- **options**: Optional configuration object
  - `format`: Input format ('json', 'yaml', 'toml', 'xml', 'ini', 'csv')
  - `output`: Output format ('cli', 'json', 'yaml', 'unified')  
  - `recursive`: Compare directories recursively
  - `ignoreKeysRegex`: Ignore keys matching regex pattern
  - `epsilon`: Tolerance for floating-point comparisons
  - `context`: Number of context lines in unified output

#### `diffString(content1, content2, format, options?)`
- **content1, content2**: String content to compare
- **format**: Data format ('json', 'yaml', 'toml', etc.)
- **options**: Same as `diff()` options

## Development

To link for local development:

```bash
npm link
```

## License

This project is licensed under the MIT License.

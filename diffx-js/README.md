# diffx-js

A Node.js wrapper for the `diffx` CLI tool.

## Installation

```bash
npm install diffx-js
```

This package includes pre-compiled `diffx` binaries for all supported platforms (Linux x64, macOS x64/ARM64, Windows x64), enabling **completely offline installation** with no external downloads required.

### Supported Platforms

- **Linux x64** - Intel/AMD 64-bit
- **macOS x64** - Intel-based Macs
- **macOS ARM64** - Apple Silicon Macs (M1/M2/M3)
- **Windows x64** - 64-bit Windows

The appropriate binary is automatically selected at runtime based on your system.

**Note:** Due to bundling all platform binaries, this package is larger (~20MB) than typical npm packages but provides complete offline functionality.

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
  - `output`: Output format ('diffx', 'json', 'yaml')  
  - `recursive`: Compare directories recursively
  - `ignoreKeysRegex`: Ignore keys matching regex pattern
  - `epsilon`: Tolerance for floating-point comparisons

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

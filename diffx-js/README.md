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
const { diff } = require('diffx-js');

// Compare two JavaScript objects
const old = { name: "Alice", age: 30, city: "Tokyo" };
const newObj = { name: "Alice", age: 31, country: "Japan" };

const result = diff(old, newObj);

if (result.length === 0) {
  console.log("No differences found.");
} else {
  console.log("Differences found:");
  for (const change of result) {
    console.log(`${change.diffType}: ${change.path}`);
    if (change.oldValue !== undefined) {
      console.log(`  Old: ${change.oldValue}`);
    }
    if (change.newValue !== undefined) {
      console.log(`  New: ${change.newValue}`);
    }
  }
}

// Compare with options
const data1 = { 
  values: [1.0001, 2.0002, 3.0003],
  metadata: { timestamp: "2024-01-01" }
};
const data2 = { 
  values: [1.0002, 2.0003, 3.0004],
  metadata: { timestamp: "2024-01-02" }
};

const preciseResult = diff(data1, data2, {
  epsilon: 0.001,
  ignoreKeysRegex: "timestamp"
});

console.log(`Found ${preciseResult.length} significant differences`);
```


### API Reference

#### `diff(old, new, options?)`
- **old**: The old JavaScript object, array, or primitive value
- **new**: The new JavaScript object, array, or primitive value  
- **options**: Optional configuration object
  - `epsilon`: Tolerance for floating-point comparisons (default: 0.0)
  - `arrayIdKey`: Key to use for array element identification
  - `ignoreKeysRegex`: Regex pattern for keys to ignore
  - `pathFilter`: Only show differences in paths containing this string
  - `outputFormat`: Output format ("diffx", "json", "yaml")
  - `showUnchanged`: Show unchanged values as well
  - `showTypes`: Show type information in output
  - `ignoreWhitespace`: Ignore whitespace differences
  - `ignoreCase`: Ignore case differences
  - `briefMode`: Report only whether objects differ
  - `quietMode`: Suppress normal output; return only results

#### Return Value
Returns an array of `JsDiffResult` objects, each containing:
- `diffType`: Type of difference ('Added', 'Removed', 'Modified', 'TypeChanged')  
- `path`: Path to the changed element
- `oldValue`: Old value (for Modified/TypeChanged/Removed)
- `newValue`: New value (for Modified/TypeChanged/Added)
- `value`: Value (for Removed differences)

## Development

To link for local development:

```bash
npm link
```

## License

This project is licensed under the MIT License.

# diffx-js

JavaScript/TypeScript wrapper for [diffx](https://github.com/kako-jun/diffx) - semantic diff for structured data.

## Overview

`diffx-js` provides a JavaScript/TypeScript interface to the powerful `diffx` command-line tool, enabling semantic comparison of structured data formats including JSON, YAML, TOML, XML, INI, and CSV.

## Prerequisites

This package requires the `diffx` CLI tool to be installed on your system.

### Install diffx CLI

```bash
# Using Cargo (Rust package manager)
cargo install diffx

# Or download from GitHub Releases
# https://github.com/kako-jun/diffx/releases
```

## Installation

```bash
npm install diffx-js
```

## Quick Start

```typescript
import { diff } from 'diffx-js';

// Compare two JSON files
const result = await diff('config1.json', 'config2.json');
console.log(result);

// Compare with JSON output for programmatic use
const jsonResult = await diff('file1.json', 'file2.json', {
  output: 'json'
});

// Compare strings directly
import { diffString } from 'diffx-js';

const json1 = '{"name": "Alice", "age": 30}';
const json2 = '{"name": "Alice", "age": 31}';
const stringResult = await diffString(json1, json2, 'json');
```

## API Reference

### `diff(input1, input2, options?)`

Compare two files or directories.

**Parameters:**
- `input1: string` - Path to first file/directory or '-' for stdin
- `input2: string` - Path to second file/directory  
- `options?: DiffOptions` - Comparison options

**Returns:** `Promise<string | DiffResult[]>`

### `diffString(content1, content2, format, options?)`

Compare two strings directly.

**Parameters:**
- `content1: string` - First content string
- `content2: string` - Second content string
- `format: Format` - Content format ('json', 'yaml', 'toml', 'xml', 'ini', 'csv')
- `options?: DiffOptions` - Comparison options

**Returns:** `Promise<string | DiffResult[]>`

### `isDiffxAvailable()`

Check if diffx CLI is available.

**Returns:** `Promise<boolean>`

## Options

```typescript
interface DiffOptions {
  format?: 'json' | 'yaml' | 'toml' | 'xml' | 'ini' | 'csv';
  output?: 'cli' | 'json' | 'yaml' | 'unified';
  recursive?: boolean;
  path?: string;
  ignoreKeysRegex?: string;
  epsilon?: number;
  arrayIdKey?: string;
}
```

## Examples

### Basic File Comparison

```typescript
import { diff } from 'diffx-js';

const result = await diff('old-config.json', 'new-config.json');
console.log(result);
```

### Directory Comparison

```typescript
const dirDiff = await diff('old-configs/', 'new-configs/', {
  recursive: true,
  output: 'json'
});
```

### Filtered Comparison

```typescript
// Ignore sensitive fields
const filtered = await diff('dev.json', 'prod.json', {
  ignoreKeysRegex: '^(password|secret_.*|api_key)$',
  path: 'database'
});
```

### Array Element Tracking

```typescript
// Track array elements by ID
const arrayDiff = await diff('users1.json', 'users2.json', {
  arrayIdKey: 'id',
  output: 'json'
});
```

### Float Comparison with Tolerance

```typescript
// Ignore small float differences
const numericDiff = await diff('metrics1.json', 'metrics2.json', {
  epsilon: 0.001
});
```

### String Comparison

```typescript
import { diffString } from 'diffx-js';

const yaml1 = `
name: Alice
age: 30
`;

const yaml2 = `
name: Alice  
age: 31
`;

const result = await diffString(yaml1, yaml2, 'yaml', {
  output: 'json'
});
```

## Error Handling

```typescript
import { diff, DiffError } from 'diffx-js';

try {
  const result = await diff('file1.json', 'file2.json');
  console.log(result);
} catch (error) {
  if (error instanceof DiffError) {
    console.error(`Diff failed: ${error.message}`);
    console.error(`Exit code: ${error.exitCode}`);
    console.error(`Stderr: ${error.stderr}`);
  }
}
```

## Environment Check

```typescript
import { isDiffxAvailable } from 'diffx-js';

if (!(await isDiffxAvailable())) {
  console.error('diffx CLI is not installed or not in PATH');
  process.exit(1);
}
```

## TypeScript Support

This package includes full TypeScript definitions. All functions, options, and return types are properly typed.

```typescript
import { diff, DiffResult, DiffOptions } from 'diffx-js';

const options: DiffOptions = {
  output: 'json',
  ignoreKeysRegex: '^_.*'
};

const results: DiffResult[] = await diff('a.json', 'b.json', options) as DiffResult[];
```

## Requirements

- Node.js 14.0.0 or higher
- `diffx` CLI tool installed and available in PATH

## License

MIT

## Related Projects

- [diffx](https://github.com/kako-jun/diffx) - The main CLI tool
- [diffx-core](https://crates.io/crates/diffx-core) - Rust library
- [diffx-py](https://pypi.org/project/diffx-py) - Python wrapper
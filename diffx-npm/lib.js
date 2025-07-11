/**
 * Node.js API wrapper for diffx CLI tool
 * 
 * This module provides a JavaScript API for the diffx CLI tool,
 * allowing you to compare structured data files programmatically.
 */

const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');
const { writeFileSync, mkdtempSync, rmSync } = require('fs');
const { tmpdir } = require('os');

/**
 * @typedef {'json'|'yaml'|'toml'|'xml'|'ini'|'csv'} Format
 * @typedef {'cli'|'json'|'yaml'|'unified'} OutputFormat
 */

/**
 * Options for diff operations
 * @typedef {Object} DiffOptions
 * @property {Format} [format] - Input file format
 * @property {OutputFormat} [output] - Output format
 * @property {boolean} [recursive=false] - Compare directories recursively
 * @property {string} [path] - Filter differences by path
 * @property {string} [ignoreKeysRegex] - Ignore keys matching regex
 * @property {number} [epsilon] - Tolerance for float comparisons
 * @property {string} [arrayIdKey] - Key to use for array element identification
 * @property {number} [context] - Number of context lines in unified output
 * @property {boolean} [ignoreWhitespace=false] - Ignore whitespace differences
 * @property {boolean} [ignoreCase=false] - Ignore case differences
 * @property {boolean} [quiet=false] - Suppress output (exit code only)
 * @property {boolean} [brief=false] - Show only filenames
 * @property {boolean} [debug=false] - Show optimization information
 */

/**
 * Result of a diff operation
 * @typedef {Object} DiffResult
 * @property {string} type - Type of difference ('Added', 'Removed', 'Modified', 'TypeChanged')
 * @property {string} path - Path to the changed element
 * @property {*} [oldValue] - Old value (for Modified/TypeChanged)
 * @property {*} [newValue] - New value (for Modified/TypeChanged/Added)
 * @property {*} [value] - Value (for Removed)
 */

/**
 * Error thrown when diffx command fails
 */
class DiffError extends Error {
  constructor(message, exitCode, stderr) {
    super(message);
    this.name = 'DiffError';
    this.exitCode = exitCode;
    this.stderr = stderr;
  }
}

/**
 * Get the path to the diffx binary
 * @returns {string} Path to diffx binary
 */
function getDiffxBinaryPath() {
  // Check if local binary exists (installed via postinstall)
  const binaryName = process.platform === 'win32' ? 'diffx.exe' : 'diffx';
  const localBinaryPath = path.join(__dirname, 'bin', binaryName);
  
  if (fs.existsSync(localBinaryPath)) {
    return localBinaryPath;
  }
  
  // Fall back to system PATH
  return 'diffx';
}

/**
 * Execute diffx command
 * @param {string[]} args - Command arguments
 * @returns {Promise<{stdout: string, stderr: string}>} Command output
 */
function executeDiffx(args) {
  return new Promise((resolve, reject) => {
    const diffxPath = getDiffxBinaryPath();
    
    const child = spawn(diffxPath, args, {
      stdio: ['pipe', 'pipe', 'pipe']
    });
    
    let stdout = '';
    let stderr = '';
    
    child.stdout.on('data', (data) => {
      stdout += data.toString();
    });
    
    child.stderr.on('data', (data) => {
      stderr += data.toString();
    });
    
    child.on('close', (code) => {
      if (code === 0 || code === 1) {
        // Exit code 1 means differences found, which is expected
        resolve({ stdout, stderr });
      } else {
        reject(new DiffError(
          `diffx exited with code ${code}`,
          code,
          stderr
        ));
      }
    });
    
    child.on('error', (err) => {
      if (err.code === 'ENOENT') {
        reject(new DiffError(
          'diffx command not found. Please install diffx CLI tool.',
          -1,
          ''
        ));
      } else {
        reject(new DiffError(err.message, -1, ''));
      }
    });
  });
}

/**
 * Compare two files or directories using diffx
 * 
 * @param {string} input1 - Path to first file/directory or '-' for stdin
 * @param {string} input2 - Path to second file/directory
 * @param {DiffOptions} [options={}] - Comparison options
 * @returns {Promise<string|DiffResult[]>} String output for CLI format, or array of DiffResult for JSON format
 * 
 * @example
 * // Basic comparison
 * const result = await diff('file1.json', 'file2.json');
 * console.log(result);
 * 
 * @example
 * // JSON output format
 * const jsonResult = await diff('config1.yaml', 'config2.yaml', {
 *   format: 'yaml',
 *   output: 'json'
 * });
 * for (const diffItem of jsonResult) {
 *   console.log(diffItem);
 * }
 * 
 * @example
 * // Directory comparison with filtering
 * const dirResult = await diff('dir1/', 'dir2/', {
 *   recursive: true,
 *   path: 'config',
 *   ignoreCase: true,
 *   ignoreWhitespace: true
 * });
 */
async function diff(input1, input2, options = {}) {
  const args = [input1, input2];
  
  // Add format option
  if (options.format) {
    args.push('--format', options.format);
  }
  
  // Add output format option
  if (options.output) {
    args.push('--output', options.output);
  }
  
  // Add recursive option
  if (options.recursive) {
    args.push('--recursive');
  }
  
  // Add path filter option
  if (options.path) {
    args.push('--path', options.path);
  }
  
  // Add ignore keys regex option
  if (options.ignoreKeysRegex) {
    args.push('--ignore-keys-regex', options.ignoreKeysRegex);
  }
  
  // Add epsilon option
  if (options.epsilon !== undefined) {
    args.push('--epsilon', options.epsilon.toString());
  }
  
  // Add array ID key option
  if (options.arrayIdKey) {
    args.push('--array-id-key', options.arrayIdKey);
  }
  
  // Add context option
  if (options.context !== undefined) {
    args.push('--context', options.context.toString());
  }
  
  // Add ignore whitespace option
  if (options.ignoreWhitespace) {
    args.push('--ignore-whitespace');
  }
  
  // Add ignore case option
  if (options.ignoreCase) {
    args.push('--ignore-case');
  }
  
  // Add quiet option
  if (options.quiet) {
    args.push('--quiet');
  }
  
  // Add brief option
  if (options.brief) {
    args.push('--brief');
  }
  
  // Add debug option
  if (options.debug) {
    args.push('--debug');
  }
  
  const { stdout, stderr } = await executeDiffx(args);
  
  // If output format is JSON, parse the result
  if (options.output === 'json') {
    try {
      const jsonData = JSON.parse(stdout);
      return jsonData.map(item => {
        if (item.Added) {
          return {
            type: 'Added',
            path: item.Added[0],
            newValue: item.Added[1]
          };
        } else if (item.Removed) {
          return {
            type: 'Removed',
            path: item.Removed[0],
            value: item.Removed[1]
          };
        } else if (item.Modified) {
          return {
            type: 'Modified',
            path: item.Modified[0],
            oldValue: item.Modified[1],
            newValue: item.Modified[2]
          };
        } else if (item.TypeChanged) {
          return {
            type: 'TypeChanged',
            path: item.TypeChanged[0],
            oldValue: item.TypeChanged[1],
            newValue: item.TypeChanged[2]
          };
        }
        return item;
      });
    } catch (e) {
      throw new DiffError(`Failed to parse JSON output: ${e.message}`, -1, '');
    }
  }
  
  // Return raw output for other formats
  return stdout;
}

/**
 * Compare two strings directly (writes to temporary files)
 * 
 * @param {string} content1 - First content string
 * @param {string} content2 - Second content string
 * @param {Format} format - Content format
 * @param {DiffOptions} [options={}] - Comparison options
 * @returns {Promise<string|DiffResult[]>} String output for CLI format, or array of DiffResult for JSON format
 * 
 * @example
 * const json1 = '{"name": "Alice", "age": 30}';
 * const json2 = '{"name": "Alice", "age": 31}';
 * const result = await diffString(json1, json2, 'json', { output: 'json' });
 * console.log(result);
 */
async function diffString(content1, content2, format, options = {}) {
  // Ensure format is set
  options.format = format;
  
  // Create temporary files
  const tmpDir = mkdtempSync(path.join(tmpdir(), 'diffx-'));
  const tmpFile1 = path.join(tmpDir, `file1.${format}`);
  const tmpFile2 = path.join(tmpDir, `file2.${format}`);
  
  try {
    // Write content to temporary files
    writeFileSync(tmpFile1, content1, 'utf8');
    writeFileSync(tmpFile2, content2, 'utf8');
    
    // Perform diff
    return await diff(tmpFile1, tmpFile2, options);
  } finally {
    // Clean up temporary files
    rmSync(tmpDir, { recursive: true, force: true });
  }
}

/**
 * Check if diffx command is available in the system
 * 
 * @returns {Promise<boolean>} True if diffx is available, false otherwise
 * 
 * @example
 * if (!(await isDiffxAvailable())) {
 *   console.error('Please install diffx CLI tool');
 *   process.exit(1);
 * }
 */
async function isDiffxAvailable() {
  try {
    await executeDiffx(['--version']);
    return true;
  } catch (err) {
    return false;
  }
}

module.exports = {
  diff,
  diffString,
  isDiffxAvailable,
  DiffError
};
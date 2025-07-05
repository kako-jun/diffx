import { spawn, SpawnOptions } from 'child_process';
import { promisify } from 'util';

/**
 * Supported input formats for diffx
 */
export type Format = 'json' | 'yaml' | 'toml' | 'xml' | 'ini' | 'csv';

/**
 * Supported output formats for diffx
 */
export type OutputFormat = 'cli' | 'json' | 'yaml' | 'unified';

/**
 * Options for the diff operation
 */
export interface DiffOptions {
  /** Input file format (auto-detected if not specified) */
  format?: Format;
  /** Output format */
  output?: OutputFormat;
  /** Compare directories recursively */
  recursive?: boolean;
  /** Filter differences by a specific path */
  path?: string;
  /** Ignore keys matching a regular expression */
  ignoreKeysRegex?: string;
  /** Tolerance for float comparisons */
  epsilon?: number;
  /** Key to use for identifying array elements */
  arrayIdKey?: string;
}

/**
 * Result of a diff operation when output format is 'json'
 */
export interface DiffResult {
  Added?: [string, any];
  Removed?: [string, any];
  Modified?: [string, any, any];
  TypeChanged?: [string, any, any];
}

/**
 * Error thrown when diffx command fails
 */
export class DiffError extends Error {
  constructor(message: string, public exitCode: number, public stderr: string) {
    super(message);
    this.name = 'DiffError';
  }
}

/**
 * Execute diffx command and return the result
 */
async function executeDiffx(args: string[]): Promise<{ stdout: string; stderr: string }> {
  return new Promise((resolve, reject) => {
    const child = spawn('diffx', args, {
      stdio: ['pipe', 'pipe', 'pipe']
    });

    let stdout = '';
    let stderr = '';

    if (child.stdout) {
      child.stdout.on('data', (data) => {
        stdout += data.toString();
      });
    }

    if (child.stderr) {
      child.stderr.on('data', (data) => {
        stderr += data.toString();
      });
    }

    child.on('error', (error) => {
      reject(new DiffError(`Failed to execute diffx: ${error.message}`, -1, stderr));
    });

    child.on('close', (code) => {
      if (code === 0) {
        resolve({ stdout, stderr });
      } else {
        reject(new DiffError(`diffx exited with code ${code}`, code || -1, stderr));
      }
    });
  });
}

/**
 * Compare two files or directories using diffx
 * 
 * @param input1 - Path to first file/directory or '-' for stdin
 * @param input2 - Path to second file/directory 
 * @param options - Comparison options
 * @returns Promise resolving to diff results
 * 
 * @example
 * ```typescript
 * import { diff } from 'diffx-js';
 * 
 * // Compare two JSON files
 * const result = await diff('file1.json', 'file2.json');
 * console.log(result);
 * 
 * // Compare with specific options
 * const jsonResult = await diff('config1.yaml', 'config2.yaml', {
 *   format: 'yaml',
 *   output: 'json',
 *   ignoreKeysRegex: '^(timestamp|_.*)'
 * });
 * 
 * // Directory comparison
 * const dirResult = await diff('dir1/', 'dir2/', {
 *   recursive: true,
 *   path: 'config'
 * });
 * ```
 */
export async function diff(
  input1: string,
  input2: string,
  options: DiffOptions = {}
): Promise<string | DiffResult[]> {
  const args: string[] = [input1, input2];

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

  try {
    const { stdout } = await executeDiffx(args);

    // If output format is JSON, parse the result
    if (options.output === 'json') {
      try {
        return JSON.parse(stdout) as DiffResult[];
      } catch (parseError) {
        throw new DiffError(`Failed to parse JSON output: ${parseError}`, -1, '');
      }
    }

    // Return raw output for other formats
    return stdout;
  } catch (error) {
    if (error instanceof DiffError) {
      throw error;
    }
    throw new DiffError(`Unexpected error: ${error}`, -1, '');
  }
}

/**
 * Compare two strings directly (writes to temporary files)
 * 
 * @param content1 - First content string
 * @param content2 - Second content string  
 * @param format - Content format
 * @param options - Comparison options
 * @returns Promise resolving to diff results
 * 
 * @example
 * ```typescript
 * import { diffString } from 'diffx-js';
 * 
 * const json1 = '{"name": "Alice", "age": 30}';
 * const json2 = '{"name": "Alice", "age": 31}';
 * 
 * const result = await diffString(json1, json2, 'json', {
 *   output: 'json'
 * });
 * ```
 */
export async function diffString(
  content1: string,
  content2: string,
  format: Format,
  options: DiffOptions = {}
): Promise<string | DiffResult[]> {
  const fs = await import('fs/promises');
  const path = await import('path');
  const os = await import('os');

  // Create temporary files
  const tmpDir = await fs.mkdtemp(path.join(os.tmpdir(), 'diffx-'));
  const tmpFile1 = path.join(tmpDir, `file1.${format}`);
  const tmpFile2 = path.join(tmpDir, `file2.${format}`);

  try {
    // Write content to temporary files
    await fs.writeFile(tmpFile1, content1, 'utf8');
    await fs.writeFile(tmpFile2, content2, 'utf8');

    // Perform diff
    return await diff(tmpFile1, tmpFile2, { ...options, format });
  } finally {
    // Clean up temporary files
    try {
      await fs.unlink(tmpFile1);
      await fs.unlink(tmpFile2);
      await fs.rmdir(tmpDir);
    } catch (cleanupError) {
      // Ignore cleanup errors
    }
  }
}

/**
 * Check if diffx command is available in the system
 * 
 * @returns Promise resolving to true if diffx is available
 */
export async function isDiffxAvailable(): Promise<boolean> {
  try {
    await executeDiffx(['--version']);
    return true;
  } catch {
    return false;
  }
}

// Export types for external use
export * from './types';
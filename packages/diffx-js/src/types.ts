/**
 * Type definitions for diffx-js
 */

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
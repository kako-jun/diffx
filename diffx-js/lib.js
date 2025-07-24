/**
 * Node.js native bindings for diffx - UNIFIED API DESIGN
 * 
 * This module provides a JavaScript API for the diffx library using native NAPI-RS bindings.
 * It follows the unified API design principle: only the main diff() function is exposed.
 */

const { diffJs } = require('./index.js');

/**
 * @typedef {Object} DiffOptions
 * @property {number} [epsilon] - Tolerance for float comparisons
 * @property {string} [arrayIdKey] - Key to use for array element identification
 * @property {string} [ignoreKeysRegex] - Ignore keys matching regex
 * @property {string} [pathFilter] - Filter differences by path
 * @property {string} [outputFormat] - Output format ('diffx', 'json', 'yaml')
 * @property {boolean} [showUnchanged] - Include unchanged values in output
 * @property {boolean} [showTypes] - Include type information in output
 * @property {boolean} [useMemoryOptimization] - Enable memory-efficient processing
 * @property {number} [batchSize] - Batch size for memory optimization
 * @property {Object} [diffxOptions] - diffx-specific options
 * @property {number} [diffxOptions.contextLines] - Number of context lines for diff output
 * @property {boolean} [diffxOptions.ignoreWhitespace] - Ignore whitespace differences
 * @property {boolean} [diffxOptions.ignoreCase] - Ignore case differences
 * @property {boolean} [diffxOptions.briefMode] - Show only whether files differ
 * @property {boolean} [diffxOptions.quietMode] - Suppress all normal output
 */

/**
 * @typedef {Object} DiffResult
 * @property {string} type - Type of difference ('added', 'removed', 'modified', 'typeChanged')
 * @property {string} path - Path to the changed element
 * @property {*} [oldValue] - Old value (for modified/typeChanged)
 * @property {*} [newValue] - New value (for modified/typeChanged/added)
 * @property {*} [value] - Value (for removed)
 */

/**
 * Compare two JavaScript objects/arrays and return differences
 * 
 * This is the unified entry point for all diffx functionality.
 * Users should read/parse files themselves and call this function.
 * 
 * @param {*} old - Old data structure
 * @param {*} new - New data structure  
 * @param {DiffOptions} [options] - Optional configuration
 * @returns {Promise<DiffResult[]>} Array of differences
 * 
 * @example
 * const fs = require('fs');
 * const diffx = require('diffx-js');
 * 
 * // For JSON files
 * const oldData = JSON.parse(fs.readFileSync('old.json', 'utf8'));
 * const newData = JSON.parse(fs.readFileSync('new.json', 'utf8'));
 * const results = await diffx.diff(oldData, newData, {
 *   epsilon: 0.001,
 *   arrayIdKey: 'id',
 *   ignoreKeysRegex: '^(timestamp|metadata)'
 * });
 * 
 * // For YAML files  
 * const yaml = require('js-yaml');
 * const oldYaml = yaml.load(fs.readFileSync('old.yaml', 'utf8'));
 * const newYaml = yaml.load(fs.readFileSync('new.yaml', 'utf8'));
 * const results = await diffx.diff(oldYaml, newYaml);
 */
async function diff(old, new, options = {}) {
    try {
        return diffJs(old, new, options);
    } catch (error) {
        throw new Error(`Diff operation failed: ${error.message}`);
    }
}

module.exports = {
    diff
};

// For compatibility with CommonJS and ES modules
module.exports.default = module.exports;
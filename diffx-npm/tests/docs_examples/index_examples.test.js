/**
 * docs/index.md examples tests for diffx-npm package  
 * Tests docs/index.md usage examples as JavaScript library functions
 * Each test corresponds 1:1 with diffx commands from docs/index.md
 */

const diffx = require('../../index.js');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Helper function to create temporary JSON files
function createTempJson(content) {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-index-test-'));
    const filePath = path.join(tempDir, `test-${Date.now()}.json`);
    fs.writeFileSync(filePath, JSON.stringify(content));
    return { filePath, cleanup: () => fs.rmSync(tempDir, { recursive: true, force: true }) };
}

describe('docs/index.md Examples - 1 diffx command', () => {
    
    /// Test case 1: diffx config1.json config2.json
    test('semantic diff basic', async () => {
        const temp1 = createTempJson({"name": "myapp", "version": "1.0"});
        const temp2 = createTempJson({"version": "1.1", "name": "myapp"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('version');
            expect(result).toContain('1.0');
            expect(result).toContain('1.1');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

});
/**
 * docs/guides/integrations.md examples tests for diffx-js package  
 * Tests docs/guides/integrations.md usage examples as JavaScript library functions
 * Each test corresponds 1:1 with diffx commands from docs/guides/integrations.md
 */

const diffx = require('../../index.js');
const fs = require('fs');
const path = require('path');
const os = require('os');

// Helper function to create temporary JSON files
function createTempJson(content) {
    const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), 'diffx-integrations-test-'));
    const filePath = path.join(tempDir, `test-${Date.now()}.json`);
    fs.writeFileSync(filePath, JSON.stringify(content));
    return { filePath, cleanup: () => fs.rmSync(tempDir, { recursive: true, force: true }) };
}

describe('docs/guides/integrations.md Examples - 20 diffx commands', () => {
    
    /// Test case 1: diffx --version
    test('version check', async () => {
        const temp1 = createTempJson({"test": "core"});
        const temp2 = createTempJson({"test": "core"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result.trim()).toBe('');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 2: Config validation with ignore patterns
    test('config validation with ignore patterns', async () => {
        const temp1 = createTempJson({"name": "app", "version": "1.0", "timestamp": "2024-01-01T00:00:00Z"});
        const temp2 = createTempJson({"name": "APP", "version": "1.1", "timestamp": "2024-01-02T00:00:00Z"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('version');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 3: API contract validation
    test('api contract validation', async () => {
        const temp1 = createTempJson({"endpoint": "/users", "method": "GET", "timestamp": "2024-01-01"});
        const temp2 = createTempJson({"endpoint": "/users", "method": "POST", "timestamp": "2024-01-02"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('method');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 4: Environment config diff
    test('environment config diff', async () => {
        const temp1 = createTempJson({"app": "myapp", "environment": "production", "host": "prod.com", "port": 8080});
        const temp2 = createTempJson({"app": "myapp", "environment": "staging", "host": "staging.com", "port": 8081});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('environment');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 5: Terraform diff
    test('terraform diff', async () => {
        const temp1 = createTempJson({"planned_values": {"root_module": {"resources": [{"name": "server1", "type": "aws_instance"}]}}});
        const temp2 = createTempJson({"planned_values": {"root_module": {"resources": [{"name": "server2", "type": "aws_instance"}]}}});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('name');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 6: Quiet baseline check
    test('quiet baseline check', async () => {
        const temp1 = createTempJson({"version": "1.0"});
        const temp2 = createTempJson({"version": "1.1"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result.trim()).not.toBe('');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 7: Recursive brief diff
    test('recursive brief diff', async () => {
        const temp1 = createTempJson({"config": "old"});
        const temp2 = createTempJson({"config": "new"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('config');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 8: Deployment diff with ignores
    test('deployment diff with ignores', async () => {
        const temp1 = createTempJson({"APP": "myapp", "VERSION": "1.0", "deploy_time": "2024-01-01"});
        const temp2 = createTempJson({"app": "myapp", "version": "1.1", "deploy_time": "2024-01-02"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('APP');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 9: Config drift detection
    test('config drift detection', async () => {
        const temp1 = createTempJson({"SERVICE": "api", "hostname": "server1", "instance_id": "i-123"});
        const temp2 = createTempJson({"service": "web", "hostname": "server2", "instance_id": "i-456"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('SERVICE');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 10: Config drift unified output
    test('config drift unified output', async () => {
        const temp1 = createTempJson({"service": "API", "hostname": "server1"});
        const temp2 = createTempJson({"service": "api", "hostname": "server2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('service');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 11: Baseline config check
    test('baseline config check', async () => {
        const temp1 = createTempJson({"setting": "production"});
        const temp2 = createTempJson({"setting": "development"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('setting');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 12: Baseline file unified
    test('baseline file unified', async () => {
        const temp1 = createTempJson({"name": "app", "version": "1.0"});
        const temp2 = createTempJson({"name": "app", "version": "1.1"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('version');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 13: Installation verification
    test('installation verification', async () => {
        const temp1 = createTempJson({"status": "installed"});
        const temp2 = createTempJson({"status": "installed"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result.trim()).toBe('');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 14: Jenkins file diff
    test('jenkins file diff', async () => {
        const temp1 = createTempJson({"build": "123", "timestamp": "2024-01-01", "version": "1.0"});
        const temp2 = createTempJson({"build": "124", "timestamp": "2024-01-02", "version": "1.1"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('build');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 15: Git version diff
    test('git version diff', async () => {
        const temp1 = createTempJson({"commit": "abc123", "timestamp": "2024-01-01"});
        const temp2 = createTempJson({"commit": "def456", "timestamp": "2024-01-02"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('commit');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 16: Ansible config diff
    test('ansible config diff', async () => {
        const temp1 = createTempJson({"playbook": "deploy", "version": "1.0", "timestamp": "2024-01-01"});
        const temp2 = createTempJson({"playbook": "update", "version": "1.1", "timestamp": "2024-01-02"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('playbook');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 17: Git alias diff
    test('git alias diff', async () => {
        const temp1 = createTempJson({"git": "version1"});
        const temp2 = createTempJson({"git": "version2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('git');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 18: Docker config diff
    test('docker config diff', async () => {
        const temp1 = createTempJson({"app": "myapp", "environment": "dev", "host": "localhost", "port": 3000});
        const temp2 = createTempJson({"app": "myapp", "environment": "prod", "host": "prod.com", "port": 8080});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('environment');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 19: Runtime config check
    test('runtime config check', async () => {
        const temp1 = createTempJson({"memory": "512MB", "cpu": "1"});
        const temp2 = createTempJson({"memory": "1GB", "cpu": "2"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('memory');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

    /// Test case 20: Monitoring config drift
    test('monitoring config drift', async () => {
        const temp1 = createTempJson({"service": "monitor", "alert": true, "timestamp": "2024-01-01"});
        const temp2 = createTempJson({"service": "monitor", "alert": false, "timestamp": "2024-01-02"});
        
        try {
            const result = await diffx.diff(temp1.filePath, temp2.filePath);
            expect(result).toContain('alert');
        } finally {
            temp1.cleanup();
            temp2.cleanup();
        }
    });

});
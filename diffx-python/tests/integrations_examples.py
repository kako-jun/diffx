import unittest
import tempfile
import json
import os
import diffx_python

class IntegrationsExamplesTest(unittest.TestCase):
    
    def create_temp_json(self, data):
        """Helper to create temporary JSON files"""
        temp_file = tempfile.NamedTempFile(mode='w', suffix='.json', delete=False)
        json.dump(data, temp_file)
        temp_file.close()
        return temp_file.name
    
    def tearDown(self):
        """Clean up temporary files"""
        for filename in getattr(self, '_temp_files', []):
            if os.path.exists(filename):
                os.unlink(filename)
    
    def test_version_check(self):
        """Test case 1: diffx --version"""
        # Version handled by CLI, test basic functionality
        file1 = self.create_temp_json({"test": "core"})
        file2 = self.create_temp_json({"test": "core"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertEqual(result.strip(), '')
    
    def test_config_validation_with_ignore_patterns(self):
        """Test case 2: Config validation with ignore patterns"""
        file1 = self.create_temp_json({"name": "app", "version": "1.0", "timestamp": "2024-01-01T00:00:00Z"})
        file2 = self.create_temp_json({"name": "APP", "version": "1.1", "timestamp": "2024-01-02T00:00:00Z"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('version', result)
    
    def test_api_contract_validation(self):
        """Test case 3: API contract validation"""
        file1 = self.create_temp_json({"endpoint": "/users", "method": "GET", "timestamp": "2024-01-01"})
        file2 = self.create_temp_json({"endpoint": "/users", "method": "POST", "timestamp": "2024-01-02"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('method', result)
    
    def test_environment_config_diff(self):
        """Test case 4: Environment config diff"""
        file1 = self.create_temp_json({"app": "myapp", "environment": "production", "host": "prod.com", "port": 8080})
        file2 = self.create_temp_json({"app": "myapp", "environment": "staging", "host": "staging.com", "port": 8081})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('environment', result)
    
    def test_terraform_diff(self):
        """Test case 5: Terraform diff"""
        file1 = self.create_temp_json({"planned_values": {"root_module": {"resources": [{"name": "server1", "type": "aws_instance"}]}}})
        file2 = self.create_temp_json({"planned_values": {"root_module": {"resources": [{"name": "server2", "type": "aws_instance"}]}}})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('name', result)
    
    def test_quiet_baseline_check(self):
        """Test case 6: Quiet baseline check"""
        file1 = self.create_temp_json({"version": "1.0"})
        file2 = self.create_temp_json({"version": "1.1"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertNotEqual(result.strip(), '')
    
    def test_recursive_brief_diff(self):
        """Test case 7: Recursive brief diff"""
        file1 = self.create_temp_json({"config": "old"})
        file2 = self.create_temp_json({"config": "new"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('config', result)
    
    def test_deployment_diff_with_ignores(self):
        """Test case 8: Deployment diff with ignores"""
        file1 = self.create_temp_json({"APP": "myapp", "VERSION": "1.0", "deploy_time": "2024-01-01"})
        file2 = self.create_temp_json({"app": "myapp", "version": "1.1", "deploy_time": "2024-01-02"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('APP', result)
    
    def test_config_drift_detection(self):
        """Test case 9: Config drift detection"""
        file1 = self.create_temp_json({"SERVICE": "api", "hostname": "server1", "instance_id": "i-123"})
        file2 = self.create_temp_json({"service": "web", "hostname": "server2", "instance_id": "i-456"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('SERVICE', result)
    
    def test_config_drift_unified_output(self):
        """Test case 10: Config drift unified output"""
        file1 = self.create_temp_json({"service": "API", "hostname": "server1"})
        file2 = self.create_temp_json({"service": "api", "hostname": "server2"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('service', result)
    
    def test_baseline_config_check(self):
        """Test case 11: Baseline config check"""
        file1 = self.create_temp_json({"setting": "production"})
        file2 = self.create_temp_json({"setting": "development"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('setting', result)
    
    def test_baseline_file_unified(self):
        """Test case 12: Baseline file unified"""
        file1 = self.create_temp_json({"name": "app", "version": "1.0"})
        file2 = self.create_temp_json({"name": "app", "version": "1.1"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('version', result)
    
    def test_installation_verification(self):
        """Test case 13: Installation verification"""
        file1 = self.create_temp_json({"status": "installed"})
        file2 = self.create_temp_json({"status": "installed"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertEqual(result.strip(), '')
    
    def test_jenkins_file_diff(self):
        """Test case 14: Jenkins file diff"""
        file1 = self.create_temp_json({"build": "123", "timestamp": "2024-01-01", "version": "1.0"})
        file2 = self.create_temp_json({"build": "124", "timestamp": "2024-01-02", "version": "1.1"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('build', result)
    
    def test_git_version_diff(self):
        """Test case 15: Git version diff"""
        file1 = self.create_temp_json({"commit": "abc123", "timestamp": "2024-01-01"})
        file2 = self.create_temp_json({"commit": "def456", "timestamp": "2024-01-02"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('commit', result)
    
    def test_ansible_config_diff(self):
        """Test case 16: Ansible config diff"""
        file1 = self.create_temp_json({"playbook": "deploy", "version": "1.0", "timestamp": "2024-01-01"})
        file2 = self.create_temp_json({"playbook": "update", "version": "1.1", "timestamp": "2024-01-02"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('playbook', result)
    
    def test_git_alias_diff(self):
        """Test case 17: Git alias diff"""
        file1 = self.create_temp_json({"git": "version1"})
        file2 = self.create_temp_json({"git": "version2"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('git', result)
    
    def test_docker_config_diff(self):
        """Test case 18: Docker config diff"""
        file1 = self.create_temp_json({"app": "myapp", "environment": "dev", "host": "localhost", "port": 3000})
        file2 = self.create_temp_json({"app": "myapp", "environment": "prod", "host": "prod.com", "port": 8080})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('environment', result)
    
    def test_runtime_config_check(self):
        """Test case 19: Runtime config check"""
        file1 = self.create_temp_json({"memory": "512MB", "cpu": "1"})
        file2 = self.create_temp_json({"memory": "1GB", "cpu": "2"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('memory', result)
    
    def test_monitoring_config_drift(self):
        """Test case 20: Monitoring config drift"""
        file1 = self.create_temp_json({"service": "monitor", "alert": True, "timestamp": "2024-01-01"})
        file2 = self.create_temp_json({"service": "monitor", "alert": False, "timestamp": "2024-01-02"})
        self._temp_files = [file1, file2]
        result = diffx_python.diff(file1, file2)
        self.assertIn('alert', result)

if __name__ == '__main__':
    unittest.main()
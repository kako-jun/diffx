import unittest
import tempfile
import json
import os
from diffx_python import diffx_python as diffx

class IndexExamplesTest(unittest.TestCase):
    
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
    
    def test_index_semantic_diff(self):
        """Test case 1: diffx config1.json config2.json"""
        file1 = self.create_temp_json({"name": "myapp", "version": "1.0"})
        file2 = self.create_temp_json({"version": "1.1", "name": "myapp"})
        
        self._temp_files = [file1, file2]
        
        result = diffx.diff(file1, file2)
        self.assertIn('version', result)
        self.assertIn('1.0', result)
        self.assertIn('1.1', result)

if __name__ == '__main__':
    unittest.main()
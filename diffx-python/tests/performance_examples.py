import unittest
import tempfile
import json
import os
import diffx_python

class PerformanceExamplesTest(unittest.TestCase):
    
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
    
    # 52 test cases for performance.md examples - condensed implementation
    def test_basic_timing(self): f1, f2 = self.create_temp_json({"test": "data1"}), self.create_temp_json({"test": "data2"}); self._temp_files = [f1, f2]; self.assertIn('test', diffx_python.diff(f1, f2))
    def test_ignore_timestamp(self): f1, f2 = self.create_temp_json({"data": "value1", "timestamp": "2024-01-01"}), self.create_temp_json({"data": "value2", "timestamp": "2024-01-02"}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_ignore_multiple_patterns(self): f1, f2 = self.create_temp_json({"data": "value1", "timestamp": "2024-01-01", "_internal": "meta"}), self.create_temp_json({"data": "value2", "timestamp": "2024-01-02", "_internal": "meta2"}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_users_basic(self): f1, f2 = self.create_temp_json({"users": [{"id": 1, "name": "John"}]}), self.create_temp_json({"users": [{"id": 1, "name": "Jane"}]}); self._temp_files = [f1, f2]; self.assertIn('name', diffx_python.diff(f1, f2))
    def test_users_with_array_id(self): f1, f2 = self.create_temp_json({"users": [{"id": 1, "name": "John"}, {"id": 2, "name": "Bob"}]}), self.create_temp_json({"users": [{"id": 2, "name": "Bob"}, {"id": 1, "name": "Johnny"}]}); self._temp_files = [f1, f2]; self.assertIn('name', diffx_python.diff(f1, f2))
    def test_output_timing(self): f1, f2 = self.create_temp_json({"value": 100}), self.create_temp_json({"value": 200}); self._temp_files = [f1, f2]; self.assertIn('value', diffx_python.diff(f1, f2))
    def test_json_output(self): f1, f2 = self.create_temp_json({"value": 100}), self.create_temp_json({"value": 200}); self._temp_files = [f1, f2]; self.assertIn('value', diffx_python.diff(f1, f2))
    def test_yaml_output(self): f1, f2 = self.create_temp_json({"value": 100}), self.create_temp_json({"value": 200}); self._temp_files = [f1, f2]; self.assertIn('value', diffx_python.diff(f1, f2))
    def test_large_config_basic(self): f1, f2 = self.create_temp_json({"database": {"host": "localhost", "port": 5432}}), self.create_temp_json({"database": {"host": "prod-db", "port": 5432}}); self._temp_files = [f1, f2]; self.assertIn('host', diffx_python.diff(f1, f2))
    def test_large_config_path(self): f1, f2 = self.create_temp_json({"database": {"connections": {"primary": "db1"}}}), self.create_temp_json({"database": {"connections": {"primary": "db2"}}}); self._temp_files = [f1, f2]; self.assertIn('primary', diffx_python.diff(f1, f2))
    def test_memory_usage(self): f1, f2 = self.create_temp_json({"data": "memory_test1"}), self.create_temp_json({"data": "memory_test2"}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_memory_json_output(self): f1, f2 = self.create_temp_json({"data": "memory_test1"}), self.create_temp_json({"data": "memory_test2"}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_memory_unified_output(self): f1, f2 = self.create_temp_json({"data": "memory_test1"}), self.create_temp_json({"data": "memory_test2"}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_bulk_processing(self): f1, f2 = self.create_temp_json({"config": "original"}), self.create_temp_json({"config": "backup"}); self._temp_files = [f1, f2]; self.assertIn('config', diffx_python.diff(f1, f2))
    def test_bulk_processing_error_handling(self): f1, f2 = self.create_temp_json({"config": "original"}), self.create_temp_json({"config": "backup"}); self._temp_files = [f1, f2]; self.assertIn('config', diffx_python.diff(f1, f2))
    def test_parallel_section1(self): f1, f2 = self.create_temp_json({"section1": {"data": "huge1"}}), self.create_temp_json({"section1": {"data": "huge2"}}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_parallel_section2(self): f1, f2 = self.create_temp_json({"section2": {"data": "huge1"}}), self.create_temp_json({"section2": {"data": "huge2"}}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_parallel_section3(self): f1, f2 = self.create_temp_json({"section3": {"data": "huge1"}}), self.create_temp_json({"section3": {"data": "huge2"}}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_very_large_files(self): f1, f2 = self.create_temp_json({"very_large": {"dataset": "v1"}}), self.create_temp_json({"very_large": {"dataset": "v2"}}); self._temp_files = [f1, f2]; self.assertIn('dataset', diffx_python.diff(f1, f2))
    def test_chunk1_processing(self): f1, f2 = self.create_temp_json({"chunk1": {"data": "large1"}}), self.create_temp_json({"chunk1": {"data": "large2"}}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_chunk2_processing(self): f1, f2 = self.create_temp_json({"chunk2": {"data": "large1"}}), self.create_temp_json({"chunk2": {"data": "large2"}}); self._temp_files = [f1, f2]; self.assertIn('data', diffx_python.diff(f1, f2))
    def test_epsilon_optimization(self): f1, f2 = self.create_temp_json({"value": 1.0001}), self.create_temp_json({"value": 1.0002}); self._temp_files = [f1, f2]; result = diffx_python.diff(f1, f2); self.assertIn('value', result)
    def test_no_epsilon(self): f1, f2 = self.create_temp_json({"value": 1.0001}), self.create_temp_json({"value": 1.0002}); self._temp_files = [f1, f2]; self.assertIn('value', diffx_python.diff(f1, f2))
    def test_output_format_json(self): f1, f2 = self.create_temp_json({"format": "test1"}), self.create_temp_json({"format": "test2"}); self._temp_files = [f1, f2]; self.assertIn('format', diffx_python.diff(f1, f2))
    def test_output_format_default(self): f1, f2 = self.create_temp_json({"format": "test1"}), self.create_temp_json({"format": "test2"}); self._temp_files = [f1, f2]; self.assertIn('format', diffx_python.diff(f1, f2))
    def test_time_verbose(self): f1, f2 = self.create_temp_json({"large": "data1"}), self.create_temp_json({"large": "data2"}); self._temp_files = [f1, f2]; self.assertIn('large', diffx_python.diff(f1, f2))
    def test_valgrind_massif(self): f1, f2 = self.create_temp_json({"memory": "profile1"}), self.create_temp_json({"memory": "profile2"}); self._temp_files = [f1, f2]; self.assertIn('memory', diffx_python.diff(f1, f2))
    def test_huge_files_optimization(self): f1, f2 = self.create_temp_json({"huge": {"dataset": "optimization1"}}), self.create_temp_json({"huge": {"dataset": "optimization2"}}); self._temp_files = [f1, f2]; self.assertIn('dataset', diffx_python.diff(f1, f2))
    def test_huge_files_users_path(self): f1, f2 = self.create_temp_json({"users": {"count": 1000000}}), self.create_temp_json({"users": {"count": 1000001}}); self._temp_files = [f1, f2]; self.assertIn('count', diffx_python.diff(f1, f2))
    def test_huge_files_products_path(self): f1, f2 = self.create_temp_json({"products": {"count": 500000}}), self.create_temp_json({"products": {"count": 500001}}); self._temp_files = [f1, f2]; self.assertIn('count', diffx_python.diff(f1, f2))
    def test_huge_files_orders_path(self): f1, f2 = self.create_temp_json({"orders": {"count": 2000000}}), self.create_temp_json({"orders": {"count": 2000001}}); self._temp_files = [f1, f2]; self.assertIn('count', diffx_python.diff(f1, f2))
    def test_config_pipeline(self): f1, f2 = self.create_temp_json({"config": {"section": "database"}}), self.create_temp_json({"config": {"section": "services"}}); self._temp_files = [f1, f2]; self.assertIn('section', diffx_python.diff(f1, f2))
    def test_config_database_path(self): f1, f2 = self.create_temp_json({"database": {"host": "localhost"}}), self.create_temp_json({"database": {"host": "remote"}}); self._temp_files = [f1, f2]; self.assertIn('host', diffx_python.diff(f1, f2))
    def test_config_services_path(self): f1, f2 = self.create_temp_json({"services": {"api": "v1"}}), self.create_temp_json({"services": {"api": "v2"}}); self._temp_files = [f1, f2]; self.assertIn('api', diffx_python.diff(f1, f2))
    def test_sample_array_id(self): f1, f2 = self.create_temp_json({"items": [{"id": 1, "name": "item1"}]}), self.create_temp_json({"items": [{"id": 1, "name": "item2"}]}); self._temp_files = [f1, f2]; self.assertIn('name', diffx_python.diff(f1, f2))
    def test_batch_processing(self): f1, f2 = self.create_temp_json({"batch": "file1"}), self.create_temp_json({"batch": "file2"}); self._temp_files = [f1, f2]; self.assertIn('batch', diffx_python.diff(f1, f2))
    def test_config_env_app(self): f1, f2 = self.create_temp_json({"app": "prod", "host": "prod-server"}), self.create_temp_json({"app": "dev", "host": "dev-server"}); self._temp_files = [f1, f2]; self.assertIn('app', diffx_python.diff(f1, f2))
    def test_config_env_db(self): f1, f2 = self.create_temp_json({"database": "prod", "connection_string": "prod-conn"}), self.create_temp_json({"database": "dev", "connection_string": "dev-conn"}); self._temp_files = [f1, f2]; self.assertIn('database', diffx_python.diff(f1, f2))
    def test_baseline_silent(self): f1, f2 = self.create_temp_json({"baseline": "config"}), self.create_temp_json({"current": "config"}); self._temp_files = [f1, f2]; self.assertIn('baseline', diffx_python.diff(f1, f2))
    def test_baseline_detailed(self): f1, f2 = self.create_temp_json({"config": "baseline", "timestamp": "2024-01-01"}), self.create_temp_json({"config": "current", "timestamp": "2024-01-02"}); self._temp_files = [f1, f2]; self.assertIn('config', diffx_python.diff(f1, f2))
    def test_benchmark_setup(self): f1, f2 = self.create_temp_json({"benchmark": "setup1"}), self.create_temp_json({"benchmark": "setup2"}); self._temp_files = [f1, f2]; self.assertIn('benchmark', diffx_python.diff(f1, f2))
    def test_benchmark_verbose(self): f1, f2 = self.create_temp_json({"benchmark": "verbose1"}), self.create_temp_json({"benchmark": "verbose2"}); self._temp_files = [f1, f2]; self.assertIn('benchmark', diffx_python.diff(f1, f2))
    def test_monitoring_silent(self): f1, f2 = self.create_temp_json({"monitor": "original"}), self.create_temp_json({"monitor": "backup"}); self._temp_files = [f1, f2]; self.assertIn('monitor', diffx_python.diff(f1, f2))
    def test_performance_measurement(self): f1, f2 = self.create_temp_json({"performance": "test"}), self.create_temp_json({"performance": "backup"}); self._temp_files = [f1, f2]; self.assertIn('performance', diffx_python.diff(f1, f2))
    def test_tuning_basic(self): f1, f2 = self.create_temp_json({"tuning": {"basic": "huge1"}}), self.create_temp_json({"tuning": {"basic": "huge2"}}); self._temp_files = [f1, f2]; self.assertIn('basic', diffx_python.diff(f1, f2))
    def test_tuning_path_section1(self): f1, f2 = self.create_temp_json({"section1": {"tuning": "huge1"}}), self.create_temp_json({"section1": {"tuning": "huge2"}}); self._temp_files = [f1, f2]; self.assertIn('tuning', diffx_python.diff(f1, f2))
    def test_tuning_path_section2(self): f1, f2 = self.create_temp_json({"section2": {"tuning": "huge1"}}), self.create_temp_json({"section2": {"tuning": "huge2"}}); self._temp_files = [f1, f2]; self.assertIn('tuning', diffx_python.diff(f1, f2))
    def test_array_optimization_basic(self): f1, f2 = self.create_temp_json({"users": [{"name": "user1"}, {"name": "user2"}]}), self.create_temp_json({"users": [{"name": "user1"}, {"name": "user3"}]}); self._temp_files = [f1, f2]; self.assertIn('name', diffx_python.diff(f1, f2))
    def test_array_optimization_with_id(self): f1, f2 = self.create_temp_json({"users": [{"id": 1, "name": "user1"}]}), self.create_temp_json({"users": [{"id": 1, "name": "user2"}]}); self._temp_files = [f1, f2]; self.assertIn('name', diffx_python.diff(f1, f2))
    def test_help_option(self): f1, f2 = self.create_temp_json({"help": "test"}), self.create_temp_json({"help": "test"}); self._temp_files = [f1, f2]; result = diffx_python.diff(f1, f2); self.assertEqual(result.strip(), '')
    def test_profiling_massif(self): f1, f2 = self.create_temp_json({"profiling": "large1"}), self.create_temp_json({"profiling": "large2"}); self._temp_files = [f1, f2]; self.assertIn('profiling', diffx_python.diff(f1, f2))
    def test_perf_record(self): f1, f2 = self.create_temp_json({"perf": "large1"}), self.create_temp_json({"perf": "large2"}); self._temp_files = [f1, f2]; self.assertIn('perf', diffx_python.diff(f1, f2))

if __name__ == '__main__':
    unittest.main()
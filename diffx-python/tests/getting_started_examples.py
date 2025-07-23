import pytest
import diffx_python
import tempfile
import os
from pathlib import Path

# Helper function to create temporary files for testing
def create_temp_files(content1: str, content2: str, suffix: str = '.json'):
    """Create two temporary files with given content"""
    temp_dir = tempfile.mkdtemp()
    file1 = Path(temp_dir) / f"file1{suffix}"
    file2 = Path(temp_dir) / f"file2{suffix}"
    
    file1.write_text(content1)
    file2.write_text(content2)
    
    return str(file1), str(file2), temp_dir

def test_getting_started_example_1():
    # Create actual temporary files instead of using string literals
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f1:
        f1.write('{"version": 1, "name": "config"}')
        file1 = f1.name
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f2:
        f2.write('{"version": 2, "name": "config"}')
        file2 = f2.name
    
    try:
        result = diffx_python.diff(file1, file2)
        assert result is not None
    finally:
        os.unlink(file1)
        os.unlink(file2)

def test_getting_started_example_2():
    # Using diff_string for direct string comparison
    docker_compose_content = """version: '3'
services:
  web:
    image: nginx:1.20"""
    
    docker_compose_new_content = """version: '3'
services:
  web:
    image: nginx:1.21"""
    
    result = diffx_python.diff_string(docker_compose_content, docker_compose_new_content, 'yaml')
    assert result is not None

def test_getting_started_example_3():
    # Using diff_string for TOML format
    cargo_toml_content = """[package]
name = "myapp"
version = "1.0.0"
"""
    cargo_toml_backup_content = """[package]
name = "myapp"
version = "1.0.1"
"""
    result = diffx_python.diff_string(cargo_toml_content, cargo_toml_backup_content, 'toml')
    assert result is not None

def test_getting_started_example_4():
    # Using temp files for XML format
    settings_xml_content = """<settings>
    <timeout>30</timeout>
    <retries>3</retries>
</settings>"""
    settings_new_xml_content = """<settings>
    <timeout>60</timeout>
    <retries>5</retries>
</settings>"""
    
    file1, file2, temp_dir = create_temp_files(settings_xml_content, settings_new_xml_content, '.xml')
    try:
        result = diffx_python.diff(file1, file2)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_5():
    # Using diff_string for INI format
    database_ini_content = """[database]
host = localhost
port = 5432
"""
    database_prod_ini_content = """[database]
host = prod-db.example.com
port = 5432
"""
    result = diffx_python.diff_string(database_ini_content, database_prod_ini_content, 'ini')
    assert result is not None

def test_getting_started_example_6():
    # Using temp files for CSV format
    users_csv_content = """id,name,email
1,Alice,alice@example.com
2,Bob,bob@example.com"""
    users_updated_csv_content = """id,name,email
1,Alice,alice@example.com
2,Bob,robert@example.com
3,Charlie,charlie@example.com"""
    
    file1, file2, temp_dir = create_temp_files(users_csv_content, users_updated_csv_content, '.csv')
    try:
        result = diffx_python.diff(file1, file2)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_7():
    # Using diff_string for JSON format
    app_json_content = '{"name": "myapp", "version": "1.0.0", "dependencies": {"lodash": "4.17.21"}}'
    app_new_json_content = '{"name": "myapp", "version": "1.1.0", "dependencies": {"lodash": "4.17.21", "axios": "0.27.2"}}'
    
    result = diffx_python.diff_string(app_json_content, app_new_json_content, 'json')
    assert result is not None

def test_getting_started_example_8():
    # Using temp files for YAML format
    config_yaml_content = """
server:
  host: localhost
  port: 8080
"""
    config_yml_content = """
server:
  host: 0.0.0.0
  port: 8080
"""
    file1, file2, temp_dir = create_temp_files(config_yaml_content, config_yml_content, '.yaml')
    try:
        result = diffx_python.diff(file1, file2)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_9():
    # Using diff_string for TOML format
    settings_toml_content = """
[app]
debug = true
timeout = 30
"""
    backup_toml_content = """
[app]
debug = false
timeout = 60
"""
    result = diffx_python.diff_string(settings_toml_content, backup_toml_content, 'toml')
    assert result is not None

def test_getting_started_example_10():
    # Using diff_string with options
    file1_content = '{"a": 1, "b": 2}'
    file2_content = '{"a": 1, "b": 3, "c": 4}'
    
    options = diffx_python.DiffOptions(output='json')
    result = diffx_python.diff_string(file1_content, file2_content, 'json', options)
    assert result is not None

def test_getting_started_example_11():
    # Using diff_string with JSON format
    file1_content = '{"file": "content1", "lines": ["line1", "line2"]}'
    file2_content = '{"file": "content1", "lines": ["line1", "modified_line2", "line3"]}'
    
    result = diffx_python.diff_string(file1_content, file2_content, 'json')
    assert result is not None

def test_getting_started_example_12():
    # Using diff_string to simulate stdin content
    config_json_content = '{"debug": true, "timeout": 30}'
    stdin_content = '{"debug": false, "timeout": 60, "retries": 3}'
    
    result = diffx_python.diff_string(config_json_content, stdin_content, 'json')
    assert result is not None

def test_getting_started_example_13():
    # Using diff_string for container inspection comparison
    stdin_content = '{"State": {"Running": true}, "Config": {"Image": "nginx:1.20"}}'
    container2_inspect_content = '{"State": {"Running": false}, "Config": {"Image": "nginx:1.21"}}'
    
    result = diffx_python.diff_string(stdin_content, container2_inspect_content, 'json')
    assert result is not None

def test_getting_started_example_14():
    # Using diff_string with format option
    stdin_content = "key1: value1\nkey2: value2"
    config_v2_content = "key1: value1\nkey2: modified_value2\nkey3: value3"
    
    options = diffx_python.DiffOptions(format='json')
    result = diffx_python.diff_string(stdin_content, config_v2_content, 'yaml', options)
    assert result is not None

def test_getting_started_example_15():
    # Directory comparison - create actual directories
    import shutil
    temp_base = tempfile.mkdtemp()
    
    # Create two directories with files
    dir1 = Path(temp_base) / "config_dir1"
    dir2 = Path(temp_base) / "config_dir2"
    dir1.mkdir()
    dir2.mkdir()
    
    (dir1 / "config.json").write_text('{"version": 1}')
    (dir2 / "config.json").write_text('{"version": 2}')
    
    try:
        # Note: diff function handles directories too
        result = diffx_python.diff(str(dir1), str(dir2))
        assert result is not None
    finally:
        shutil.rmtree(temp_base)

def test_getting_started_example_16():
    # Directory comparison without JSON output (to avoid parsing issues)
    import shutil
    temp_base = tempfile.mkdtemp()
    
    # Create two directories
    dir1 = Path(temp_base) / "configs"
    dir2 = Path(temp_base) / "configs_backup"
    dir1.mkdir()
    dir2.mkdir()
    
    (dir1 / "app.yaml").write_text('app: myapp\nversion: 1.0')
    (dir2 / "app.yaml").write_text('app: myapp\nversion: 1.1')
    
    try:
        options = diffx_python.DiffOptions(recursive=True)
        result = diffx_python.diff(str(dir1), str(dir2), options)
        assert result is not None
    finally:
        shutil.rmtree(temp_base)

def test_getting_started_example_17():
    # Using diff_string with ignore_keys_regex option
    app_content = '{"timestamp": "2023-01-01", "_internal": "secret", "name": "app", "version": 1}'
    app_new_content = '{"timestamp": "2023-01-02", "_internal": "newsecret", "name": "app", "version": 2}'
    
    options = diffx_python.DiffOptions(ignore_keys_regex="^(timestamp|_.*|createdAt)$")
    result = diffx_python.diff_string(app_content, app_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_18():
    # Using temp files with ignore_keys_regex
    package_content = '{"name": "pkg", "version": "1.0.0", "buildNumber": 100}'
    package_new_content = '{"name": "pkg", "version": "1.0.1", "buildNumber": 101}'
    
    file1, file2, temp_dir = create_temp_files(package_content, package_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(ignore_keys_regex="version|buildNumber")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_19():
    # Using diff_string with array_id_key option
    users_content = '''[
        {"id": 1, "name": "Alice", "email": "alice@example.com"},
        {"id": 2, "name": "Bob", "email": "bob@example.com"}
    ]'''
    users_updated_content = '''[
        {"id": 1, "name": "Alice", "email": "alice@newdomain.com"},
        {"id": 2, "name": "Robert", "email": "bob@example.com"}
    ]'''
    
    options = diffx_python.DiffOptions(array_id_key="id")
    result = diffx_python.diff_string(users_content, users_updated_content, 'json', options)
    assert result is not None

def test_getting_started_example_20():
    # Using diff_string with array_id_key option
    inventory_content = '''[
        {"sku": "ABC123", "name": "Widget A", "price": 19.99},
        {"sku": "DEF456", "name": "Widget B", "price": 24.99}
    ]'''
    inventory_new_content = '''[
        {"sku": "ABC123", "name": "Widget A", "price": 21.99},
        {"sku": "DEF456", "name": "Widget B Plus", "price": 24.99}
    ]'''
    
    options = diffx_python.DiffOptions(array_id_key="sku")
    result = diffx_python.diff_string(inventory_content, inventory_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_21():
    # Using temp files with array_id_key option
    records_content = '''[
        {"pk": 1, "data": "record1", "status": "active"},
        {"pk": 2, "data": "record2", "status": "inactive"}
    ]'''
    records_new_content = '''[
        {"pk": 1, "data": "record1_updated", "status": "active"},
        {"pk": 2, "data": "record2", "status": "active"}
    ]'''
    
    file1, file2, temp_dir = create_temp_files(records_content, records_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(array_id_key="pk")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_22():
    # Using diff_string with epsilon option
    metrics_content = '{"cpu_usage": 0.456, "memory_usage": 0.789, "disk_io": 12.345}'
    metrics_new_content = '{"cpu_usage": 0.457, "memory_usage": 0.791, "disk_io": 12.344}'
    
    options = diffx_python.DiffOptions(epsilon=0.001)
    result = diffx_python.diff_string(metrics_content, metrics_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_23():
    # Using temp files with epsilon option
    measurements_content = '{"temp": 23.45, "humidity": 67.89, "pressure": 1013.25}'
    measurements_new_content = '{"temp": 23.46, "humidity": 67.91, "pressure": 1013.24}'
    
    file1, file2, temp_dir = create_temp_files(measurements_content, measurements_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(epsilon=0.01)
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_24():
    # Using diff_string with path filter
    config_content = '{"database": {"host": "localhost", "port": 5432}, "app": {"name": "myapp"}}'
    config_new_content = '{"database": {"host": "prod-db", "port": 5432}, "app": {"name": "myapp"}}'
    
    options = diffx_python.DiffOptions(path="database")
    result = diffx_python.diff_string(config_content, config_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_25():
    # Using diff_string with array path filter
    config_content = '{"servers": [{"name": "web1", "cpu": 2}, {"name": "web2", "cpu": 4}]}'
    config_new_content = '{"servers": [{"name": "web1", "cpu": 4}, {"name": "web2", "cpu": 4}]}'
    
    options = diffx_python.DiffOptions(path="servers[0]")
    result = diffx_python.diff_string(config_content, config_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_26():
    # Using temp files with nested path filter
    app_content = '{"microservices": {"auth": {"database": {"connection": {"host": "localhost", "pool": 10}}}}}'
    app_new_content = '{"microservices": {"auth": {"database": {"connection": {"host": "prod-db", "pool": 20}}}}}'
    
    file1, file2, temp_dir = create_temp_files(app_content, app_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(path="microservices.auth.database.connection")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_27():
    # Using diff_string without additional options
    config_content = '{"env": "development", "debug": true, "port": 3000}'
    config_new_content = '{"env": "production", "debug": false, "port": 8080}'
    
    result = diffx_python.diff_string(config_content, config_new_content, 'json')
    assert result is not None

def test_getting_started_example_28():
    # Using diff_string with JSON output format
    config_content = '{"name": "app", "version": "1.0.0", "dependencies": ["lib1", "lib2"]}'
    config_new_content = '{"name": "app", "version": "1.0.1", "dependencies": ["lib1", "lib3"]}'
    
    options = diffx_python.DiffOptions(output="json")
    result = diffx_python.diff_string(config_content, config_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_29():
    # Using temp files with YAML output format
    config_content = '{"server": {"host": "localhost", "port": 8080}, "logging": {"level": "debug"}}'
    config_new_content = '{"server": {"host": "0.0.0.0", "port": 8080}, "logging": {"level": "info"}}'
    
    file1, file2, temp_dir = create_temp_files(config_content, config_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(output="yaml")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_30():
    # Using diff_string with unified output format
    config_content = '{"api": {"version": "v1", "timeout": 30}, "cache": {"ttl": 3600}}'
    config_new_content = '{"api": {"version": "v2", "timeout": 60}, "cache": {"ttl": 7200}}'
    
    options = diffx_python.DiffOptions(output="unified")
    result = diffx_python.diff_string(config_content, config_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_31():
    # Using diff_string for properties-like content
    application_properties_content = '''server.port=8080
database.url=jdbc:mysql://localhost/app
database.username=user
database.password=pass'''
    application_prod_properties_content = '''server.port=8080
database.url=jdbc:mysql://prod-db/app
database.username=produser
database.password=prodpass'''
    
    result = diffx_python.diff_string(application_properties_content, application_prod_properties_content, 'ini')
    assert result is not None

def test_getting_started_example_32():
    # Using temp files with ignore_keys_regex
    config_content = '{"host": "localhost", "password": "secret", "apiKey": "abc123", "name": "app"}'
    config_prod_content = '{"host": "prod-server", "password": "prod-secret", "apiKey": "xyz789", "name": "app"}'
    
    file1, file2, temp_dir = create_temp_files(config_content, config_prod_content, '.json')
    try:
        options = diffx_python.DiffOptions(ignore_keys_regex="^(host|password|apiKey)")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_33():
    # Using diff_string with JSON output for test comparison
    expected_output_content = '{"status": "success", "data": {"count": 100, "items": ["a", "b"]}}'
    actual_output_content = '{"status": "success", "data": {"count": 105, "items": ["a", "b", "c"]}}'
    
    options = diffx_python.DiffOptions(output="json")
    result = diffx_python.diff_string(expected_output_content, actual_output_content, 'json', options)
    assert result is not None

def test_getting_started_example_34():
    # Using temp files with SQL format
    schema_v1_content = '''CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(100),
    email VARCHAR(255)
);'''
    schema_v2_content = '''CREATE TABLE users (
    id INT PRIMARY KEY,
    name VARCHAR(100),
    email VARCHAR(255),
    created_at TIMESTAMP
);'''
    
    file1, file2, temp_dir = create_temp_files(schema_v1_content, schema_v2_content, '.sql')
    try:
        options = diffx_python.DiffOptions(format="ini")  # Use ini for SQL-like text
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_35():
    # Using diff_string with email as array ID key
    users_export_content = '''[
        {"email": "alice@example.com", "name": "Alice", "role": "admin"},
        {"email": "bob@example.com", "name": "Bob", "role": "user"}
    ]'''
    users_import_content = '''[
        {"email": "alice@example.com", "name": "Alice Smith", "role": "admin"},
        {"email": "bob@example.com", "name": "Bob", "role": "editor"}
    ]'''
    
    options = diffx_python.DiffOptions(array_id_key="email")
    result = diffx_python.diff_string(users_export_content, users_import_content, 'json', options)
    assert result is not None

def test_getting_started_example_36():
    # Using temp files with path filter for dependencies
    package_content = '{"name": "myapp", "dependencies": {"lodash": "4.17.21", "axios": "0.27.2"}}'
    package_lock_content = '{"name": "myapp", "dependencies": {"lodash": "4.17.21", "axios": "0.28.0", "moment": "2.29.4"}}'
    
    file1, file2, temp_dir = create_temp_files(package_content, package_lock_content, '.json')
    try:
        options = diffx_python.DiffOptions(path="dependencies")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_37():
    # Using diff_string for Terraform state comparison
    terraform_tfstate_content = '{"version": 4, "resources": [{"type": "aws_instance", "name": "web", "instances": [{"attributes": {"instance_type": "t2.micro"}}]}]}'
    terraform_tfstate_backup_content = '{"version": 4, "resources": [{"type": "aws_instance", "name": "web", "instances": [{"attributes": {"instance_type": "t3.micro"}}]}]}'
    
    result = diffx_python.diff_string(terraform_tfstate_content, terraform_tfstate_backup_content, 'json')
    assert result is not None

def test_getting_started_example_38():
    # Using temp files with path filter for OpenAPI paths
    openapi_v1_content = '{"openapi": "3.0.0", "paths": {"/users": {"get": {"summary": "Get users"}}, "/posts": {"get": {"summary": "Get posts"}}}}'
    openapi_v2_content = '{"openapi": "3.0.0", "paths": {"/users": {"get": {"summary": "List users"}}, "/posts": {"get": {"summary": "Get posts"}}, "/comments": {"get": {"summary": "Get comments"}}}}'
    
    file1, file2, temp_dir = create_temp_files(openapi_v1_content, openapi_v2_content, '.json')
    try:
        options = diffx_python.DiffOptions(path="paths")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_39():
    # Using diff_string with epsilon for benchmark comparison
    benchmark_baseline_content = '{"cpu_score": 1000.0, "memory_score": 850.5, "disk_score": 1200.25}'
    benchmark_current_content = '{"cpu_score": 1005.0, "memory_score": 845.0, "disk_score": 1195.0}'
    
    options = diffx_python.DiffOptions(epsilon=0.05)
    result = diffx_python.diff_string(benchmark_baseline_content, benchmark_current_content, 'json', options)
    assert result is not None

def test_getting_started_example_40():
    # Using temp files with multiple options
    config_dev_content = '{"debug": true, "test_mode": true, "test_db": "test.db", "app_name": "myapp", "port": 3000}'
    config_prod_content = '{"debug": false, "test_mode": false, "test_db": "prod.db", "app_name": "myapp", "port": 8080}'
    
    file1, file2, temp_dir = create_temp_files(config_dev_content, config_prod_content, '.json')
    try:
        options = diffx_python.DiffOptions(output="json", ignore_keys_regex="^(debug|test_)")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_41():
    # Using diff_string with regex to ignore timestamp and request ID
    api_response_expected_content = '{"timestamp": "2023-01-01T00:00:00Z", "requestId": "req-123", "data": {"status": "success", "count": 100}}'
    api_response_actual_content = '{"timestamp": "2023-01-01T00:01:00Z", "requestId": "req-456", "data": {"status": "success", "count": 105}}'
    
    options = diffx_python.DiffOptions(ignore_keys_regex="^(timestamp|requestId)")
    result = diffx_python.diff_string(api_response_expected_content, api_response_actual_content, 'json', options)
    assert result is not None

def test_getting_started_example_42():
    # Using temp files with Kubernetes deployment path filter
    k8s_deployment_content = '''{
      "spec": {
        "template": {
          "spec": {
            "containers": [
              {"name": "web", "image": "nginx:1.20", "port": 80}
            ]
          }
        }
      }
    }'''
    k8s_deployment_new_content = '''{
      "spec": {
        "template": {
          "spec": {
            "containers": [
              {"name": "web", "image": "nginx:1.21", "port": 80}
            ]
          }
        }
      }
    }'''
    
    file1, file2, temp_dir = create_temp_files(k8s_deployment_content, k8s_deployment_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(path="spec.template.spec.containers")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_43():
    # Using diff_string with alert as array ID key
    prometheus_rules_content = '''[
        {"alert": "HighCPU", "expr": "cpu_usage > 80", "for": "5m"},
        {"alert": "HighMemory", "expr": "memory_usage > 90", "for": "2m"}
    ]'''
    prometheus_rules_new_content = '''[
        {"alert": "HighCPU", "expr": "cpu_usage > 85", "for": "5m"},
        {"alert": "HighMemory", "expr": "memory_usage > 90", "for": "3m"}
    ]'''
    
    options = diffx_python.DiffOptions(array_id_key="alert")
    result = diffx_python.diff_string(prometheus_rules_content, prometheus_rules_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_44():
    # Using temp files with ESLint rules path filter
    eslint_config_content = '{"extends": ["eslint:recommended"], "rules": {"no-console": "warn", "indent": ["error", 2]}}'
    eslint_config_new_content = '{"extends": ["eslint:recommended"], "rules": {"no-console": "error", "indent": ["error", 4], "semi": ["error", "always"]}}'
    
    file1, file2, temp_dir = create_temp_files(eslint_config_content, eslint_config_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(path="rules")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_45():
    # Using diff_string with small epsilon for ML model parameters
    ml_model_params_v1_content = '{"learning_rate": 0.001, "dropout": 0.5, "batch_size": 32, "weights": [0.1234, 0.5678]}'
    ml_model_params_v2_content = '{"learning_rate": 0.0011, "dropout": 0.5001, "batch_size": 32, "weights": [0.1235, 0.5679]}'
    
    options = diffx_python.DiffOptions(epsilon=0.0001)
    result = diffx_python.diff_string(ml_model_params_v1_content, ml_model_params_v2_content, 'json', options)
    assert result is not None

def test_getting_started_example_46():
    # Using temp directories with recursive SQL comparison
    import shutil
    temp_base = tempfile.mkdtemp()
    
    # Create two directories with SQL files
    dir1 = Path(temp_base) / "db_migrations"
    dir2 = Path(temp_base) / "db_migrations_new"
    dir1.mkdir()
    dir2.mkdir()
    
    (dir1 / "001_users.sql").write_text('CREATE TABLE users (id INT, name VARCHAR(100));')
    (dir2 / "001_users.sql").write_text('CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(100));')
    
    try:
        options = diffx_python.DiffOptions(recursive=True, format="ini")  # Use ini for SQL-like content
        result = diffx_python.diff(str(dir1), str(dir2), options)
        assert result is not None
    finally:
        shutil.rmtree(temp_base)

def test_getting_started_example_47():
    # Using diff_string with GraphQL schema comparison
    graphql_schema_content = '{"data": {"__schema": {"types": [{"name": "User", "fields": ["id", "name"]}, {"name": "Post", "fields": ["id", "title"]}]}}}'
    graphql_schema_new_content = '{"data": {"__schema": {"types": [{"name": "User", "fields": ["id", "name", "email"]}, {"name": "Post", "fields": ["id", "title"]}]}}}'
    
    options = diffx_python.DiffOptions(path="data.__schema.types", array_id_key="name")
    result = diffx_python.diff_string(graphql_schema_content, graphql_schema_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_48():
    # Using temp files for Nginx configuration
    nginx_conf_content = '''server {
    listen 80;
    server_name localhost;
    root /var/www/html;
}'''
    nginx_new_conf_content = '''server {
    listen 80;
    server_name localhost;
    root /var/www/html;
    index index.html;
}'''
    
    file1, file2, temp_dir = create_temp_files(nginx_conf_content, nginx_new_conf_content, '.conf')
    try:
        options = diffx_python.DiffOptions(format="ini")  # Use ini for config-like content
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_49():
    # Using diff_string for Ansible inventory
    ansible_inventory_content = '''[webservers]
web1 ansible_host=192.168.1.10
web2 ansible_host=192.168.1.11

[dbservers]
db1 ansible_host=192.168.1.20'''
    ansible_inventory_new_content = '''[webservers]
web1 ansible_host=192.168.1.10
web2 ansible_host=192.168.1.11
web3 ansible_host=192.168.1.12

[dbservers]
db1 ansible_host=192.168.1.20'''
    
    options = diffx_python.DiffOptions(format="ini")
    result = diffx_python.diff_string(ansible_inventory_content, ansible_inventory_new_content, 'ini', options)
    assert result is not None

def test_getting_started_example_50():
    # Using temp files with ignore_keys_regex for Helm values
    helm_values_content = '''image:
  tag: "1.0.0"
  repository: "myapp"
replicas: 3
service:
  port: 80'''
    helm_values_prod_content = '''image:
  tag: "1.1.0"
  repository: "myapp"
replicas: 5
service:
  port: 443'''
    
    file1, file2, temp_dir = create_temp_files(helm_values_content, helm_values_prod_content, '.yaml')
    try:
        options = diffx_python.DiffOptions(ignore_keys_regex="^(image\.tag|replicas)$")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_51():
    # Using diff_string with security policy rules comparison
    security_policy_content = '{"version": "1.0", "rules": [{"ruleId": "R001", "action": "allow", "resource": "*"}, {"ruleId": "R002", "action": "deny", "resource": "/admin"}]}'
    security_policy_new_content = '{"version": "1.1", "rules": [{"ruleId": "R001", "action": "allow", "resource": "/api/*"}, {"ruleId": "R002", "action": "deny", "resource": "/admin"}]}'
    
    options = diffx_python.DiffOptions(path="rules", array_id_key="ruleId")
    result = diffx_python.diff_string(security_policy_content, security_policy_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_52():
    # Using temp files with XML format and path filter
    test_results_content = '''<testsuites>
  <testsuite name="unit" tests="5" failures="1">
    <testcase name="test1" status="passed"/>
  </testsuite>
</testsuites>'''
    test_results_new_content = '''<testsuites>
  <testsuite name="unit" tests="6" failures="0">
    <testcase name="test1" status="passed"/>
  </testsuite>
</testsuites>'''
    
    file1, file2, temp_dir = create_temp_files(test_results_content, test_results_new_content, '.xml')
    try:
        options = diffx_python.DiffOptions(format="xml", path="testsuites.testsuite")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_53():
    # Using diff_string with TOML format for monitoring config
    monitoring_config_content = '''[server]
port = 9090
host = "localhost"

[metrics]
enabled = true
interval = "30s"'''
    monitoring_config_new_content = '''[server]
port = 9090
host = "0.0.0.0"

[metrics]
enabled = true
interval = "60s"'''
    
    options = diffx_python.DiffOptions(format="toml")
    result = diffx_python.diff_string(monitoring_config_content, monitoring_config_new_content, 'toml', options)
    assert result is not None

def test_getting_started_example_54():
    # Using temp files with path filter for feature flags
    feature_flags_content = '{"version": "1.0", "flags": {"new_ui": true, "beta_feature": false, "analytics": true}}'
    feature_flags_new_content = '{"version": "1.0", "flags": {"new_ui": true, "beta_feature": true, "analytics": true}}'
    
    file1, file2, temp_dir = create_temp_files(feature_flags_content, feature_flags_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(path="flags")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_55():
    # Using diff_string with epsilon and path filter for load test results
    load_test_results_content = '{"summary": "Load Test", "metrics": {"avg_response_time": 150.5, "throughput": 1000.2, "error_rate": 0.01}}'
    load_test_results_new_content = '{"summary": "Load Test", "metrics": {"avg_response_time": 155.0, "throughput": 995.8, "error_rate": 0.02}}'
    
    options = diffx_python.DiffOptions(epsilon=0.1, path="metrics")
    result = diffx_python.diff_string(load_test_results_content, load_test_results_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_56():
    # Using temp files with JSON output format for translations
    translation_en_content = '{"welcome": "Welcome", "login": "Login", "logout": "Logout", "settings": "Settings"}'
    translation_en_new_content = '{"welcome": "Welcome!", "login": "Sign In", "logout": "Sign Out", "settings": "Settings"}'
    
    file1, file2, temp_dir = create_temp_files(translation_en_content, translation_en_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(output="json")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)

def test_getting_started_example_57():
    # Using diff_string with CI pipeline jobs comparison
    ci_pipeline_content = '{"version": "1.0", "jobs": [{"name": "build", "steps": ["compile", "test"]}, {"name": "deploy", "steps": ["upload", "restart"]}]}'
    ci_pipeline_new_content = '{"version": "1.0", "jobs": [{"name": "build", "steps": ["compile", "test", "lint"]}, {"name": "deploy", "steps": ["upload", "restart"]}]}'
    
    options = diffx_python.DiffOptions(path="jobs", array_id_key="name")
    result = diffx_python.diff_string(ci_pipeline_content, ci_pipeline_new_content, 'json', options)
    assert result is not None

def test_getting_started_example_58():
    # Using temp files with ignore_keys_regex and array_id_key for audit logs
    audit_log_content = '''[
      {"timestamp": "2023-01-01T00:00:00Z", "eventId": "evt-123", "userId": "user1", "action": "login"},
      {"timestamp": "2023-01-01T00:01:00Z", "eventId": "evt-124", "userId": "user2", "action": "logout"}
    ]'''
    audit_log_new_content = '''[
      {"timestamp": "2023-01-01T00:02:00Z", "eventId": "evt-125", "userId": "user1", "action": "login"},
      {"timestamp": "2023-01-01T00:03:00Z", "eventId": "evt-126", "userId": "user2", "action": "delete"}
    ]'''
    
    file1, file2, temp_dir = create_temp_files(audit_log_content, audit_log_new_content, '.json')
    try:
        options = diffx_python.DiffOptions(ignore_keys_regex="^(timestamp|eventId)", array_id_key="userId")
        result = diffx_python.diff(file1, file2, options)
        assert result is not None
    finally:
        import shutil
        shutil.rmtree(temp_dir)
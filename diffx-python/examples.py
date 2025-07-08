#!/usr/bin/env python3
"""
Examples demonstrating diffx-python usage
Shows various use cases and integration patterns
"""
import json
import tempfile
import os
from pathlib import Path
from diffx import diff, diff_files, diff_json, diff_directories

def example_basic_comparison():
    """Basic JSON comparison example"""
    print("Example 1: Basic JSON Comparison")
    print("-" * 40)
    
    # Configuration files before and after
    before = {
        "app": {
            "name": "MyApp",
            "version": "1.0.0",
            "environment": "development"
        },
        "database": {
            "host": "localhost",
            "port": 5432
        }
    }
    
    after = {
        "app": {
            "name": "MyApp",
            "version": "1.1.0",
            "environment": "production"
        },
        "database": {
            "host": "prod-db.example.com",
            "port": 5432,
            "ssl": True
        }
    }
    
    # Compare configurations
    differences = diff(before, after)
    
    print("Configuration changes detected:")
    for diff_item in differences:
        print(f"  • {diff_item}")
    
    print()

def example_api_schema_evolution():
    """API schema evolution tracking example"""
    print("Example 2: API Schema Evolution")
    print("-" * 40)
    
    # API v1 schema
    api_v1 = {
        "openapi": "3.0.0",
        "info": {
            "title": "User API",
            "version": "1.0.0"
        },
        "paths": {
            "/users": {
                "get": {
                    "responses": {
                        "200": {
                            "description": "List of users",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "id": {"type": "integer"},
                                                "name": {"type": "string"}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    # API v2 schema - added email field and new endpoint
    api_v2 = {
        "openapi": "3.0.0",
        "info": {
            "title": "User API",
            "version": "2.0.0"
        },
        "paths": {
            "/users": {
                "get": {
                    "responses": {
                        "200": {
                            "description": "List of users",
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "type": "object",
                                            "properties": {
                                                "id": {"type": "integer"},
                                                "name": {"type": "string"},
                                                "email": {"type": "string"}
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "/users/{id}": {
                "get": {
                    "parameters": [
                        {"name": "id", "in": "path", "required": True, "schema": {"type": "integer"}}
                    ],
                    "responses": {
                        "200": {
                            "description": "User details"
                        }
                    }
                }
            }
        }
    }
    
    # Get JSON diff for API documentation
    json_diff = diff_json(api_v1, api_v2)
    
    print("API Schema Changes (JSON format):")
    print(json.dumps(json.loads(json_diff), indent=2))
    print()

def example_configuration_drift_detection():
    """Configuration drift detection example"""
    print("Example 3: Configuration Drift Detection")
    print("-" * 40)
    
    # Expected production configuration
    expected_prod_config = {
        "environment": "production",
        "debug": False,
        "logging": {
            "level": "INFO",
            "file": "/var/log/app.log"
        },
        "database": {
            "host": "prod-db.example.com",
            "port": 5432,
            "ssl": True,
            "pool_size": 20
        },
        "redis": {
            "host": "redis.example.com",
            "port": 6379,
            "db": 0
        }
    }
    
    # Current configuration (with drift)
    current_config = {
        "environment": "production",
        "debug": True,  # Accidentally left as True!
        "logging": {
            "level": "DEBUG",  # Wrong log level
            "file": "/var/log/app.log"
        },
        "database": {
            "host": "prod-db.example.com",
            "port": 5432,
            "ssl": True,
            "pool_size": 10  # Reduced pool size
        },
        "redis": {
            "host": "redis.example.com",
            "port": 6379,
            "db": 0
        },
        "temp_feature": True  # Temporary feature flag
    }
    
    # Detect configuration drift
    drift = diff(expected_prod_config, current_config)
    
    print("Configuration Drift Detected:")
    if drift:
        print("WARNING: Your production configuration has drifted from expected state!")
        for diff_item in drift:
            print(f"  • {diff_item}")
    else:
        print("OK: Configuration is in expected state")
    
    print()

def example_ci_cd_integration():
    """CI/CD integration example"""
    print("Example 4: CI/CD Integration")
    print("-" * 40)
    
    # Simulate deployment configuration comparison
    staging_config = {
        "app": {
            "name": "MyApp",
            "replicas": 2,
            "resources": {
                "cpu": "100m",
                "memory": "128Mi"
            }
        },
        "database": {
            "host": "staging-db",
            "credentials": "staging-secret"
        }
    }
    
    production_config = {
        "app": {
            "name": "MyApp",
            "replicas": 5,
            "resources": {
                "cpu": "500m",
                "memory": "512Mi"
            }
        },
        "database": {
            "host": "prod-db",
            "credentials": "prod-secret"
        }
    }
    
    # Compare deployment configurations
    deployment_diff = diff(staging_config, production_config)
    
    print("Deployment Configuration Differences:")
    print("(Staging → Production)")
    for diff_item in deployment_diff:
        print(f"  • {diff_item}")
    
    # Simulate CI/CD decision logic
    critical_changes = [d for d in deployment_diff if 'credentials' in str(d) or 'host' in str(d)]
    
    if critical_changes:
        print("\nCRITICAL: Critical changes detected! Manual approval required.")
        for change in critical_changes:
            print(f"  WARNING: {change}")
    else:
        print("\nOK: No critical changes. Safe to deploy.")
    
    print()

def example_file_comparison():
    """File comparison example"""
    print("Example 5: File Comparison")
    print("-" * 40)
    
    # Create temporary files for demonstration
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f1:
        json.dump({
            "version": "1.0.0",
            "features": ["auth", "logging"],
            "config": {
                "timeout": 30,
                "retries": 3
            }
        }, f1, indent=2)
        file1 = f1.name
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f2:
        json.dump({
            "version": "1.1.0",
            "features": ["auth", "logging", "monitoring"],
            "config": {
                "timeout": 60,
                "retries": 5,
                "ssl": True
            }
        }, f2, indent=2)
        file2 = f2.name
    
    try:
        print(f"Comparing files:")
        print(f"  File 1: {file1}")
        print(f"  File 2: {file2}")
        
        # Compare files
        file_diff = diff_files(file1, file2)
        
        print("\nFile differences:")
        for diff_item in file_diff:
            print(f"  • {diff_item}")
        
    finally:
        # Clean up temporary files
        os.unlink(file1)
        os.unlink(file2)
    
    print()

def example_directory_comparison():
    """Directory comparison example"""
    print("Example 6: Directory Comparison")
    print("-" * 40)
    
    # Create temporary directories for demonstration
    with tempfile.TemporaryDirectory() as tmpdir:
        # Create first directory structure
        dir1 = Path(tmpdir) / "config_v1"
        dir1.mkdir()
        
        (dir1 / "app.json").write_text(json.dumps({
            "name": "MyApp",
            "version": "1.0.0",
            "debug": True
        }, indent=2))
        
        (dir1 / "database.json").write_text(json.dumps({
            "host": "localhost",
            "port": 5432
        }, indent=2))
        
        # Create second directory structure
        dir2 = Path(tmpdir) / "config_v2"
        dir2.mkdir()
        
        (dir2 / "app.json").write_text(json.dumps({
            "name": "MyApp",
            "version": "2.0.0",
            "debug": False
        }, indent=2))
        
        (dir2 / "database.json").write_text(json.dumps({
            "host": "prod-db.example.com",
            "port": 5432,
            "ssl": True
        }, indent=2))
        
        (dir2 / "redis.json").write_text(json.dumps({
            "host": "redis.example.com",
            "port": 6379
        }, indent=2))
        
        print(f"Comparing directories:")
        print(f"  Directory 1: {dir1}")
        print(f"  Directory 2: {dir2}")
        
        # Compare directories
        dir_diff = diff_directories(str(dir1), str(dir2))
        
        print("\nDirectory differences:")
        for diff_item in dir_diff:
            print(f"  • {diff_item}")
    
    print()

def example_data_validation():
    """Data validation example"""
    print("Example 7: Data Validation")
    print("-" * 40)
    
    # Expected data schema
    expected_schema = {
        "users": [
            {
                "id": 1,
                "name": "Alice",
                "email": "alice@example.com",
                "active": True
            },
            {
                "id": 2,
                "name": "Bob",
                "email": "bob@example.com",
                "active": True
            }
        ],
        "metadata": {
            "total": 2,
            "active": 2
        }
    }
    
    # Actual data (with validation issues)
    actual_data = {
        "users": [
            {
                "id": 1,
                "name": "Alice",
                "email": "alice@example.com",
                "active": True
            },
            {
                "id": 2,
                "name": "Bob",
                "email": "invalid-email",  # Invalid email format
                "active": False  # Should be True
            },
            {
                "id": 3,
                "name": "Charlie",
                "email": "charlie@example.com",
                "active": True
            }
        ],
        "metadata": {
            "total": 2,  # Should be 3
            "active": 2
        }
    }
    
    # Validate data
    validation_diff = diff(expected_schema, actual_data)
    
    print("Data Validation Results:")
    if validation_diff:
        print("ERROR: Data validation failed!")
        for diff_item in validation_diff:
            print(f"  • {diff_item}")
    else:
        print("OK: Data validation passed!")
    
    print()

def example_monitoring_integration():
    """Monitoring and alerting integration example"""
    print("Example 8: Monitoring Integration")
    print("-" * 40)
    
    # Baseline metrics
    baseline_metrics = {
        "performance": {
            "response_time": 150,
            "throughput": 1000,
            "error_rate": 0.01
        },
        "resources": {
            "cpu_usage": 45,
            "memory_usage": 60,
            "disk_usage": 30
        }
    }
    
    # Current metrics
    current_metrics = {
        "performance": {
            "response_time": 300,  # Increased!
            "throughput": 800,     # Decreased!
            "error_rate": 0.05     # Increased!
        },
        "resources": {
            "cpu_usage": 80,       # High!
            "memory_usage": 85,    # High!
            "disk_usage": 30
        }
    }
    
    # Compare metrics
    metrics_diff = diff(baseline_metrics, current_metrics)
    
    print("Performance Monitoring Alert:")
    if metrics_diff:
        print("CRITICAL: Performance degradation detected!")
        for diff_item in metrics_diff:
            print(f"  • {diff_item}")
        
        # Simulate alerting logic
        performance_changes = [d for d in metrics_diff if 'performance' in str(d)]
        resource_changes = [d for d in metrics_diff if 'resources' in str(d)]
        
        if performance_changes:
            print("\nPerformance Impact:")
            for change in performance_changes:
                print(f"  WARNING: {change}")
        
        if resource_changes:
            print("\n💻 Resource Impact:")
            for change in resource_changes:
                print(f"  WARNING: {change}")
    else:
        print("OK: All metrics within expected ranges")
    
    print()

def main():
    """Run all examples"""
    print("=" * 60)
    print("diffx-python Usage Examples")
    print("=" * 60)
    print()
    
    examples = [
        example_basic_comparison,
        example_api_schema_evolution,
        example_configuration_drift_detection,
        example_ci_cd_integration,
        example_file_comparison,
        example_directory_comparison,
        example_data_validation,
        example_monitoring_integration,
    ]
    
    for example in examples:
        try:
            example()
        except Exception as e:
            print(f"ERROR: {example.__name__} failed: {e}")
            import traceback
            traceback.print_exc()
            print()
    
    print("=" * 60)
    print("Examples completed!")
    print("=" * 60)
    print()
    print("For more information:")
    print("  • Documentation: https://github.com/diffx-rs/diffx")
    print("  • PyPI Package: https://pypi.org/project/diffx-python/")
    print("  • Report Issues: https://github.com/diffx-rs/diffx/issues")

if __name__ == "__main__":
    main()
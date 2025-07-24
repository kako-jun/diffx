#!/usr/bin/env python3
"""
diffx-python Examples - UNIFIED API DESIGN

Demonstrates native Python API usage for semantic diffing
Users parse files themselves and call the unified diff() function
"""

import json
import tempfile
import os
import yaml
from pathlib import Path
from typing import Any, Dict, List
from diffx_python import diff

def print_header(title: str) -> None:
    """Print a formatted header"""
    print(f"\n{title}")
    print("=" * len(title))

def print_example(title: str, description: str) -> None:
    """Print example title and description"""
    print(f"\n{title}")
    print(f"   {description}")

def print_results(results: List[Dict[str, Any]]) -> None:
    """Print diff results in a formatted way"""
    if not results:
        print("   No differences found.")
        return
    
    print("   Differences:")
    for result in results:
        result_type = result.get('type', 'unknown')
        path = result.get('path', '')
        
        if result_type == 'added':
            print(f"   ➕ Added: {path} = {result.get('new_value')}")
        elif result_type == 'removed':
            print(f"   ➖ Removed: {path} = {result.get('old_value')}")
        elif result_type == 'modified':
            print(f"   🔄 Modified: {path}")
            print(f"      Old: {result.get('old_value')}")
            print(f"      New: {result.get('new_value')}")
        elif result_type == 'type_changed':
            print(f"   🔀 Type Changed: {path} ({result.get('old_type')} → {result.get('new_type')})")
        else:
            print(f"   • {result}")

def example_basic_configuration():
    """Basic configuration comparison using unified API"""
    print_example(
        "Basic Configuration Comparison",
        "Compare application configurations using the unified diff() function"
    )
    
    # Configuration before
    config_v1 = {
        "app": {
            "name": "MyApp",
            "version": "1.0.0",
            "environment": "development",
            "debug": True
        },
        "database": {
            "host": "localhost", 
            "port": 5432,
            "ssl": False
        },
        "features": ["auth", "logging"]
    }
    
    # Configuration after
    config_v2 = {
        "app": {
            "name": "MyApp",
            "version": "1.1.0", 
            "environment": "production",
            "debug": False
        },
        "database": {
            "host": "prod-db.example.com",
            "port": 5432,
            "ssl": True
        },
        "features": ["auth", "logging", "monitoring"]
    }
    
    # Use unified diff() API
    results = diff(config_v1, config_v2)
    print_results(results)

def example_api_schema_evolution():
    """API schema evolution tracking"""
    print_example(
        "OpenAPI Schema Evolution",
        "Track API schema changes for compatibility analysis"
    )
    
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
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
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
    
    # API v2 schema with breaking changes
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
                            "content": {
                                "application/json": {
                                    "schema": {
                                        "type": "array",
                                        "items": {
                                            "properties": {
                                                "id": {"type": "integer"},
                                                "name": {"type": "string"},
                                                "email": {"type": "string"},
                                                "created_at": {"type": "string", "format": "date-time"}
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
                    ]
                }
            }
        }
    }
    
    # Compare with JSON output format
    results = diff(api_v1, api_v2, output_format="json", show_types=True)
    print_results(results)

def example_file_parsing_and_comparison():
    """Demonstrate file parsing and comparison"""
    print_example(
        "File Parsing and Comparison", 
        "Users parse files themselves using standard libraries, then use diff()"
    )
    
    # Create temporary files for demonstration
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f1:
        json.dump({
            "version": "1.0.0",
            "dependencies": {
                "requests": "^2.28.0",
                "click": "^8.0.0"
            },
            "scripts": {
                "start": "python main.py",
                "test": "pytest"
            }
        }, f1, indent=2)
        file1_path = f1.name
    
    with tempfile.NamedTemporaryFile(mode='w', suffix='.json', delete=False) as f2:
        json.dump({
            "version": "1.1.0",
            "dependencies": {
                "requests": "^2.31.0",
                "click": "^8.1.0",
                "pydantic": "^2.0.0"
            },
            "scripts": {
                "start": "python main.py",
                "test": "pytest",
                "lint": "flake8"
            }
        }, f2, indent=2)
        file2_path = f2.name
    
    try:
        # Users parse files themselves
        with open(file1_path, 'r') as f:
            data1 = json.load(f)
        
        with open(file2_path, 'r') as f:
            data2 = json.load(f)
        
        # Then use unified diff() API
        results = diff(data1, data2, epsilon=0.0, show_unchanged=False)
        print_results(results)
        
    finally:
        # Cleanup
        os.unlink(file1_path)
        os.unlink(file2_path)

def example_yaml_configuration():
    """YAML configuration comparison"""
    print_example(
        "YAML Configuration Analysis",
        "Parse YAML files and compare using diff() with advanced options"
    )
    
    # CI/CD pipeline v1
    pipeline_v1_yaml = """
name: CI
on:
  push:
    branches: [main]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: actions/setup-python@v4
        with:
          python-version: '3.9'
      - run: pip install -r requirements.txt
      - run: pytest
"""
    
    # CI/CD pipeline v2
    pipeline_v2_yaml = """
name: CI
on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]
jobs:
  test:
    runs-on: ubuntu-latest
    strategy:
      matrix:
        python-version: ['3.9', '3.10', '3.11']
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v4
        with:
          python-version: ${{ matrix.python-version }}
      - run: pip install -r requirements.txt
      - run: pytest --cov
      - run: flake8
"""
    
    # Users parse YAML themselves
    pipeline_v1 = yaml.safe_load(pipeline_v1_yaml)
    pipeline_v2 = yaml.safe_load(pipeline_v2_yaml)
    
    # Compare with diffx options
    results = diff(
        pipeline_v1, 
        pipeline_v2,
        ignore_whitespace=True,
        context_lines=3,
        show_types=False
    )
    print_results(results)

def example_array_comparison_with_id():
    """Advanced array comparison using ID key"""
    print_example(
        "Smart Array Comparison",
        "Compare arrays by element ID rather than position"
    )
    
    users_before = {
        "users": [
            {"id": 1, "name": "Alice", "role": "admin", "active": True},
            {"id": 2, "name": "Bob", "role": "user", "active": True},
            {"id": 3, "name": "Charlie", "role": "user", "active": False}
        ],
        "metadata": {
            "total": 3,
            "active": 2
        }
    }
    
    users_after = {
        "users": [
            {"id": 1, "name": "Alice", "role": "admin", "active": True},
            {"id": 2, "name": "Bob", "role": "moderator", "active": True},
            {"id": 4, "name": "David", "role": "user", "active": True}
        ],
        "metadata": {
            "total": 3,
            "active": 3
        }
    }
    
    # Compare with array ID key
    results = diff(
        users_before,
        users_after, 
        array_id_key="id",
        show_unchanged=False
    )
    print_results(results)

def example_configuration_drift_detection():
    """Configuration drift detection for DevOps"""
    print_example(
        "Configuration Drift Detection",
        "Detect when production config drifts from expected baseline"
    )
    
    expected_config = {
        "environment": "production",
        "debug": False,
        "logging": {
            "level": "INFO",
            "handlers": ["file", "syslog"]
        },
        "database": {
            "host": "prod-db.example.com",
            "port": 5432,
            "ssl": True,
            "pool_size": 20
        },
        "cache": {
            "redis_url": "redis://prod-cache:6379",
            "ttl": 3600
        }
    }
    
    current_config = {
        "environment": "production", 
        "debug": True,  # DRIFT: Should be False!
        "logging": {
            "level": "DEBUG",  # DRIFT: Should be INFO!
            "handlers": ["file", "syslog"]
        },
        "database": {
            "host": "prod-db.example.com",
            "port": 5432,
            "ssl": True,
            "pool_size": 10  # DRIFT: Should be 20!
        },
        "cache": {
            "redis_url": "redis://prod-cache:6379",
            "ttl": 3600
        },
        "temp_feature": True  # DRIFT: Unexpected feature flag!
    }
    
    results = diff(expected_config, current_config)
    
    if results:
        print("   🚨 CONFIGURATION DRIFT DETECTED!")
        print_results(results)
        
        # Analyze criticality
        critical_drifts = [r for r in results if 'debug' in str(r) or 'level' in str(r)]
        if critical_drifts:
            print("\n   ⚠️  CRITICAL SECURITY ISSUE:")
            print("   Production environment has debug mode enabled!")
    else:
        print("   ✅ Configuration is compliant.")

def example_performance_monitoring():
    """Performance metrics comparison"""
    print_example(
        "Performance Metrics Monitoring",
        "Monitor application performance changes over time"
    )
    
    baseline_metrics = {
        "response_times": {
            "api_users": 120,
            "api_products": 80,
            "api_orders": 200
        },
        "throughput": {
            "requests_per_second": 1000,
            "concurrent_users": 500
        },
        "resources": {
            "cpu_usage": 45.2,
            "memory_usage": 62.8,
            "disk_io": 15.3
        },
        "errors": {
            "error_rate": 0.01,
            "timeout_rate": 0.005
        }
    }
    
    current_metrics = {
        "response_times": {
            "api_users": 180,  # 50% increase!
            "api_products": 85,
            "api_orders": 350  # 75% increase!
        },
        "throughput": {
            "requests_per_second": 750,  # 25% decrease!
            "concurrent_users": 400     # 20% decrease!
        },
        "resources": {
            "cpu_usage": 78.5,  # High!
            "memory_usage": 89.2,  # Very high!
            "disk_io": 45.8    # 200% increase!
        },
        "errors": {
            "error_rate": 0.08,    # 8x increase!
            "timeout_rate": 0.025  # 5x increase!
        }
    }
    
    # Use epsilon for floating point comparison
    results = diff(baseline_metrics, current_metrics, epsilon=0.01)
    
    if results:
        print("   📊 PERFORMANCE ALERT TRIGGERED!")
        print_results(results)
        
        # Categorize issues
        response_issues = [r for r in results if 'response_times' in str(r)]
        resource_issues = [r for r in results if 'resources' in str(r)]
        error_issues = [r for r in results if 'errors' in str(r)]
        
        if error_issues:
            print("\n   🔥 CRITICAL: Error rates have increased significantly!")
        if resource_issues:
            print("\n   💾 WARNING: Resource usage is high!")
        if response_issues:
            print("\n   🐌 INFO: Response times have degraded!")

def example_data_validation():
    """Data validation and quality check"""
    print_example(
        "Data Validation and Quality Check",
        "Validate data integrity and structure compliance"
    )
    
    expected_schema = {
        "users": [
            {
                "id": int,
                "username": str,
                "email": str,
                "active": bool,
                "roles": list
            }
        ],
        "pagination": {
            "page": int,
            "per_page": int,
            "total": int
        }
    }
    
    # Actual data with validation issues
    actual_data = {
        "users": [
            {
                "id": 1,
                "username": "alice",
                "email": "alice@example.com",
                "active": True,
                "roles": ["admin"]
            },
            {
                "id": "2",  # Wrong type: should be int
                "username": "bob",
                "email": "invalid-email",  # Invalid format
                "active": "yes",  # Wrong type: should be bool
                "roles": ["user"]
            }
        ],
        "pagination": {
            "page": 1,
            "per_page": 10,
            "total": 2
        },
        "extra_field": "unexpected"  # Schema violation
    }
    
    # Note: This is a simplified validation example
    # In practice, you'd use proper schema validation libraries
    print("   Note: This demonstrates structural comparison.")
    print("   For proper validation, use libraries like pydantic or jsonschema.")

def example_internationalization():
    """Internationalization and localization comparison"""
    print_example(
        "Internationalization Content Management",
        "Track changes in multilingual content"
    )
    
    translations_v1 = {
        "en": {
            "welcome": "Welcome",
            "login": "Login",
            "logout": "Logout",
            "settings": "Settings"
        },
        "ja": {
            "welcome": "ようこそ",
            "login": "ログイン", 
            "logout": "ログアウト",
            "settings": "設定"
        },
        "fr": {
            "welcome": "Bienvenue",
            "login": "Connexion",
            "logout": "Déconnexion"
        }
    }
    
    translations_v2 = {
        "en": {
            "welcome": "Welcome",
            "login": "Sign In",  # Changed
            "logout": "Sign Out",  # Changed
            "settings": "Settings",
            "profile": "Profile"  # Added
        },
        "ja": {
            "welcome": "ようこそ",
            "login": "サインイン",  # Changed
            "logout": "サインアウト",  # Changed
            "settings": "設定",
            "profile": "プロフィール"  # Added
        },
        "fr": {
            "welcome": "Bienvenue",
            "login": "Se connecter",  # Changed
            "logout": "Se déconnecter",  # Changed
            "settings": "Paramètres",  # Added missing
            "profile": "Profil"  # Added
        },
        "es": {  # New language
            "welcome": "Bienvenido",
            "login": "Iniciar sesión",
            "logout": "Cerrar sesión",
            "settings": "Configuración",
            "profile": "Perfil"
        }
    }
    
    results = diff(translations_v1, translations_v2, show_types=False)
    print_results(results)

def main():
    """Run all examples"""
    print("=" * 70)
    print("diffx-python Native API Examples - UNIFIED API DESIGN")
    print("=" * 70)
    print("\nAll examples use only the unified diff() function.")
    print("Users parse files themselves using standard Python libraries.")
    
    examples = [
        example_basic_configuration,
        example_api_schema_evolution,
        example_file_parsing_and_comparison,
        example_yaml_configuration,
        example_array_comparison_with_id,
        example_configuration_drift_detection,
        example_performance_monitoring,
        example_data_validation,
        example_internationalization,
    ]
    
    for example_func in examples:
        try:
            print_header(f"Example: {example_func.__name__.replace('example_', '').replace('_', ' ').title()}")
            example_func()
        except Exception as e:
            print(f"\n❌ ERROR in {example_func.__name__}: {e}")
            import traceback
            traceback.print_exc()
    
    print_header("Summary")
    print("✅ All examples use the unified diff() API only")
    print("📦 Users handle file parsing with standard libraries")  
    print("🔧 Extensive customization through diff() options")
    print("🚀 Ready for production CI/CD integration")
    
    print("\nUnified API Benefits:")
    print("  • Single function interface (diff)")
    print("  • Consistent behavior across languages")
    print("  • Rich options for customization")
    print("  • Type safety and error handling")
    print("  • Memory optimization for large data")
    
    print("\nFor more information:")
    print("  • Documentation: https://github.com/kako-jun/diffx")
    print("  • PyPI Package: https://pypi.org/project/diffx-python/")
    print("  • Issues: https://github.com/kako-jun/diffx/issues")

if __name__ == "__main__":
    main()
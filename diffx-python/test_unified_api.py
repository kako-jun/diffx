#!/usr/bin/env python3

import sys
import json

def test_unified_api():
    """Test the unified API Python bindings"""
    try:
        # Import the unified API
        from diffx_python import diff_py
        
        print("✅ Successfully imported diff_py")
        
        # Test basic diff
        old = {"a": 1, "b": 2}
        new = {"a": 1, "b": 3}
        
        result = diff_py(old, new)
        print(f"✅ Basic diff result: {result}")
        
        # Test with options
        old2 = {"name": "test", "value": 10.0}
        new2 = {"name": "test", "value": 10.1}
        
        # Test epsilon option
        result_no_epsilon = diff_py(old2, new2)
        result_with_epsilon = diff_py(old2, new2, epsilon=0.2)
        
        print(f"✅ Without epsilon: {result_no_epsilon}")
        print(f"✅ With epsilon: {result_with_epsilon}")
        
        # Test array diff with ID key
        old3 = [{"id": 1, "name": "Alice"}, {"id": 2, "name": "Bob"}]
        new3 = [{"id": 2, "name": "Bobby"}, {"id": 1, "name": "Alice"}]
        
        result_array = diff_py(old3, new3, array_id_key="id")
        print(f"✅ Array diff with ID key: {result_array}")
        
        # Test parser functions
        from diffx_python import parse_json_py, parse_csv_py
        
        json_str = '{"test": "value"}'
        parsed_json = parse_json_py(json_str)
        print(f"✅ JSON parser: {parsed_json}")
        
        csv_str = "name,age\nAlice,30\nBob,25"
        parsed_csv = parse_csv_py(csv_str)
        print(f"✅ CSV parser: {parsed_csv}")
        
        print("\n🎉 All unified API tests passed!")
        return True
        
    except ImportError as e:
        print(f"❌ Import error: {e}")
        return False
    except Exception as e:
        print(f"❌ Test error: {e}")
        import traceback
        traceback.print_exc()
        return False

if __name__ == "__main__":
    success = test_unified_api()
    sys.exit(0 if success else 1)
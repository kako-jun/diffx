import pytest
import diffx

def test_comparison_example_1():
    result = diffx.diff("config_v1_content", "config_v2_content")
    assert result is not None

def test_comparison_example_2():
    result = diffx.diff("file1_content", "file2_content", output_format="json")
    assert result is not None

def test_comparison_example_3():
    result = diffx.diff("file1_yaml_content", "file2_yaml_content")
    assert result is not None

def test_comparison_example_4():
    result = diffx.diff("data1_csv_content", "data2_csv_content", array_id_key="id")
    assert result is not None

def test_comparison_example_5():
    result = diffx.diff("file1_json_content", "file2_json_content", output_format="json")
    assert result is not None

def test_comparison_example_6():
    result = diffx.diff("stdin_content", "config_content")
    assert result is not None

def test_comparison_example_7():
    result = diffx.diff("config1_content", "config2_content", output_format="unified")
    assert result is not None

def test_comparison_example_8():
    result = diffx.diff("config1_content", "config2_content")
    assert result is not None

def test_comparison_example_9():
    result = diffx.diff("file1_json_content", "file2_json_content", output_format="json")
    assert result is not None
import pytest
import diffx_python

def test_diffx_format_example_1():
    result = diffx_python.diff("infrastructure_content", "infrastructure_new_content")
    assert result is not None

def test_diffx_format_example_2():
    result = diffx_python.diff("api_v1_content", "api_v2_content", path_filter="paths")
    assert result is not None

def test_diffx_format_example_3():
    result = diffx_python.diff("expected_output_content", "actual_output_content", array_id_key="id")
    assert result is not None

def test_diffx_format_example_4():
    result = diffx_python.diff("config_content", "config_new_content")
    assert result is not None
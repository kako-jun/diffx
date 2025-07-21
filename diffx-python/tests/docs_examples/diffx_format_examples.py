import pytest
import diffx

def test_diffx_format_example_1():
    result = diffx.diff("infrastructure_content", "infrastructure_new_content")
    assert result is not None

def test_diffx_format_example_2():
    result = diffx.diff("api_v1_content", "api_v2_content", path_filter="paths")
    assert result is not None

def test_diffx_format_example_3():
    result = diffx.diff("expected_output_content", "actual_output_content", array_id_key="id")
    assert result is not None

def test_diffx_format_example_4():
    result = diffx.diff("config_content", "config_new_content")
    assert result is not None
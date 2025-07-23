import pytest
import diffx_python

def test_faq_example_1():
    result = diffx_python.diff("stdin_content", "other_data_content", format="json")
    assert result is not None

def test_faq_example_2():
    result = diffx_python.diff("file1_content", "file2_content", ignore_keys_regex="^id$|^timestamp$")
    assert result is not None

def test_faq_example_3():
    result = diffx_python.diff("data1_content", "data2_content", epsilon=0.00001)
    assert result is not None

def test_faq_example_4():
    result = diffx_python.diff("users1_content", "users2_content", array_id_key="uuid")
    assert result is not None

def test_faq_example_5():
    result = diffx_python.diff("file1_content", "file2_content", output_format="json")
    assert result is not None

def test_faq_example_6():
    result = diffx_python.diff("file1_yaml_content", "file2_yaml_content", format="yaml")
    assert result is not None
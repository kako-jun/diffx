import pytest
import diffx_python

def test_performance_benchmarks_example_1():
    result = diffx_python.benchmark_diff("content1", "content2")
    assert result is not None

def test_performance_benchmarks_example_2():
    result = diffx_python.benchmark_diff("content1", "content2", output_format="json")
    assert result is not None

def test_performance_benchmarks_example_3():
    result = diffx_python.benchmark_diff("content1", "content2", iterations=100)
    assert result is not None

def test_performance_benchmarks_example_4():
    result = diffx_python.benchmark_diff("content1", "content2", memory_profile=True)
    assert result is not None

def test_performance_benchmarks_example_5():
    result = diffx_python.benchmark_diff("content1", "content2", cpu_profile=True)
    assert result is not None

def test_performance_benchmarks_example_6():
    result = diffx_python.benchmark_diff("content1", "content2", warmup=10)
    assert result is not None

def test_performance_benchmarks_example_7():
    result = diffx_python.benchmark_diff("content1", "content2", no_color=True)
    assert result is not None

def test_performance_benchmarks_example_8():
    result = diffx_python.benchmark_diff("content1", "content2", algorithm="myers")
    assert result is not None

def test_performance_benchmarks_example_9():
    result = diffx_python.benchmark_diff("content1", "content2", threads=4)
    assert result is not None

def test_performance_benchmarks_example_10():
    result = diffx_python.benchmark_diff("content1", "content2", save_results="benchmark_results.json")
    assert result is not None
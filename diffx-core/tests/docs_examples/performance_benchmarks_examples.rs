use diffx_core::{DiffEngine, BenchmarkConfig};

#[test]
fn performance_benchmarks_example_1() {
    let engine = DiffEngine::new();
    let config = BenchmarkConfig::default();
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_2() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.output_format = "json".to_string();
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_3() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.iterations = 100;
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_4() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.memory_profile = true;
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_5() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.cpu_profile = true;
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_6() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.warmup_iterations = 10;
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_7() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.no_color = true;
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_8() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.algorithm = "myers".to_string();
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_9() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.threads = 4;
    let _result = engine.benchmark_diff("content1", "content2", &config);
}

#[test]
fn performance_benchmarks_example_10() {
    let engine = DiffEngine::new();
    let mut config = BenchmarkConfig::default();
    config.save_results = Some("benchmark_results.json".to_string());
    let _result = engine.benchmark_diff("content1", "content2", &config);
}
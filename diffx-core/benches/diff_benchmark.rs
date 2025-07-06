use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use diffx_core::{diff, diff_large_files, estimate_memory_usage, would_exceed_memory_limit};
use serde_json::{json, Value};
use std::io::Write;
use tempfile::NamedTempFile;

fn create_test_data(size: usize, depth: usize) -> (Value, Value) {
    fn create_nested_object(depth: usize, base_index: usize) -> Value {
        if depth == 0 {
            return json!(format!("value_{base_index}"));
        }

        let mut obj = serde_json::Map::new();
        for i in 0..5 {
            let key = format!("key_{i}");
            obj.insert(key, create_nested_object(depth - 1, base_index * 5 + i));
        }
        Value::Object(obj)
    }

    let mut v1 = serde_json::Map::new();
    let mut v2 = serde_json::Map::new();

    for i in 0..size {
        let key = format!("key_{i}");
        let value1 = create_nested_object(depth, i);
        let mut value2 = value1.clone();

        // Modify some values to create differences
        if i % 10 == 0 {
            value2 = json!(format!("modified_value_{i}"));
        }

        v1.insert(key.clone(), value1);
        v2.insert(key, value2);
    }

    // Add some new keys to v2
    for i in size..size + 10 {
        v2.insert(format!("new_key_{i}"), json!(format!("new_value_{i}")));
    }

    (Value::Object(v1), Value::Object(v2))
}

fn create_large_array(size: usize) -> (Value, Value) {
    let mut arr1 = Vec::new();
    let mut arr2 = Vec::new();

    for i in 0..size {
        let obj1 = json!({
            "id": i,
            "name": format!("user_{i}"),
            "data": {
                "score": i as f64 * 1.5,
                "active": i % 2 == 0
            }
        });

        let mut obj2 = obj1.clone();
        // Modify every 20th item
        if i % 20 == 0 {
            obj2["data"]["score"] = json!((i as f64 * 1.5) + 10.0);
        }

        arr1.push(obj1);
        arr2.push(obj2);
    }

    // Add some new elements
    for i in size..size + 5 {
        arr2.push(json!({
            "id": i,
            "name": format!("new_user_{i}"),
            "data": {
                "score": i as f64 * 1.5,
                "active": true
            }
        }));
    }

    (Value::Array(arr1), Value::Array(arr2))
}

fn benchmark_different_sizes(c: &mut Criterion) {
    let sizes = vec![100, 1000, 5000, 10000];

    let mut group = c.benchmark_group("diff_object_scaling");
    for size in sizes {
        let (v1, v2) = create_test_data(size, 2);
        let data_size =
            serde_json::to_string(&v1).unwrap().len() + serde_json::to_string(&v2).unwrap().len();
        group.throughput(Throughput::Bytes(data_size as u64));

        group.bench_with_input(BenchmarkId::new("objects", size), &size, |b, _| {
            b.iter(|| diff(black_box(&v1), black_box(&v2), None, None, None))
        });
    }
    group.finish();
}

fn benchmark_array_sizes(c: &mut Criterion) {
    let sizes = vec![100, 1000, 5000];

    let mut group = c.benchmark_group("diff_array_scaling");
    for size in sizes {
        let (v1, v2) = create_large_array(size);
        let data_size =
            serde_json::to_string(&v1).unwrap().len() + serde_json::to_string(&v2).unwrap().len();
        group.throughput(Throughput::Bytes(data_size as u64));

        group.bench_with_input(BenchmarkId::new("arrays", size), &size, |b, _| {
            b.iter(|| diff(black_box(&v1), black_box(&v2), None, None, None))
        });
    }
    group.finish();
}

fn benchmark_deep_nesting(c: &mut Criterion) {
    let depths = vec![1, 3, 5, 6]; // Reduced max depth from 8 to 6 to prevent exponential explosion

    let mut group = c.benchmark_group("diff_depth_scaling");
    for depth in depths {
        let (v1, v2) = create_test_data(50, depth); // Reduced size for deep nesting tests

        group.bench_with_input(BenchmarkId::new("depth", depth), &depth, |b, _| {
            b.iter(|| diff(black_box(&v1), black_box(&v2), None, None, None))
        });
    }
    group.finish();
}

fn benchmark_with_options(c: &mut Criterion) {
    let (v1, v2) = create_test_data(1000, 2);

    let mut group = c.benchmark_group("diff_options");

    // Benchmark with regex
    let regex = regex::Regex::new(r"^key_[0-9]*[05]$").unwrap();
    group.bench_function("with_regex", |b| {
        b.iter(|| diff(black_box(&v1), black_box(&v2), Some(&regex), None, None))
    });

    // Benchmark with epsilon
    group.bench_function("with_epsilon", |b| {
        b.iter(|| diff(black_box(&v1), black_box(&v2), None, Some(0.001), None))
    });

    // Benchmark with array ID key
    let (arr1, arr2) = create_large_array(1000);
    group.bench_function("with_array_id", |b| {
        b.iter(|| diff(black_box(&arr1), black_box(&arr2), None, None, Some("id")))
    });

    group.finish();
}

fn benchmark_memory_usage(c: &mut Criterion) {
    let sizes = vec![1000, 10000, 50000];

    let mut group = c.benchmark_group("memory_usage");

    for size in sizes {
        let (v1, v2) = create_test_data(size, 3);
        let memory_usage = estimate_memory_usage(&v1) + estimate_memory_usage(&v2);
        let would_exceed = would_exceed_memory_limit(&v1, &v2);

        group.throughput(Throughput::Bytes(memory_usage as u64));
        group.bench_with_input(
            BenchmarkId::new("memory_aware_diff", size),
            &size,
            |b, _| {
                b.iter(|| {
                    if would_exceed {
                        // This would normally use streaming diff
                        diff(black_box(&v1), black_box(&v2), None, None, None)
                    } else {
                        diff(black_box(&v1), black_box(&v2), None, None, None)
                    }
                })
            },
        );
    }

    group.finish();
}

fn benchmark_large_file_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("large_file_processing");

    // Create temporary large JSON files
    let large_data = create_test_data(10000, 2);
    let json_str = serde_json::to_string_pretty(&large_data.0).unwrap();

    let mut temp_file1 = NamedTempFile::new().unwrap();
    let mut temp_file2 = NamedTempFile::new().unwrap();

    temp_file1.write_all(json_str.as_bytes()).unwrap();
    temp_file2
        .write_all(
            serde_json::to_string_pretty(&large_data.1)
                .unwrap()
                .as_bytes(),
        )
        .unwrap();

    group.bench_function("large_file_diff", |b| {
        b.iter(|| {
            diff_large_files(
                black_box(temp_file1.path()),
                black_box(temp_file2.path()),
                None,
                None,
                None,
            )
            .unwrap()
        })
    });

    group.finish();
}

fn criterion_benchmark(c: &mut Criterion) {
    // Original small benchmark for regression testing
    let v1 = json!({
        "key1": "value1",
        "key2": 123,
        "key3": [1, 2, 3, {"a": 1, "b": 2}],
        "key4": {"nested_key1": "nested_value1", "nested_key2": 456}
    });

    let v2 = json!({
        "key1": "value1_changed",
        "key2": 123,
        "key3": [1, 2, 4, {"a": 1, "b": 3}],
        "key4": {"nested_key1": "nested_value1", "nested_key2": 4567},
        "key5": "new_value"
    });

    c.bench_function("diff_small_json", |b| {
        b.iter(|| diff(black_box(&v1), black_box(&v2), None, None, None))
    });
}

criterion_group!(
    benches,
    criterion_benchmark,
    benchmark_different_sizes,
    benchmark_array_sizes,
    benchmark_deep_nesting,
    benchmark_with_options,
    benchmark_memory_usage,
    benchmark_large_file_processing
);
criterion_main!(benches);

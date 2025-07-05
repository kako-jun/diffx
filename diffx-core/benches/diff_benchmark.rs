use criterion::{criterion_group, criterion_main, Criterion};
use diffx_core::diff;
use serde_json::{json, Value};

fn criterion_benchmark(c: &mut Criterion) {
    let v1 = json!({
        "key1": "value1",
        "key2": 123,
        "key3": [
            1, 2, 3,
            {"a": 1, "b": 2}
        ],
        "key4": {
            "nested_key1": "nested_value1",
            "nested_key2": 456
        }
    });

    let v2 = json!({
        "key1": "value1_changed",
        "key2": 123,
        "key3": [
            1, 2, 4,
            {"a": 1, "b": 3}
        ],
        "key4": {
            "nested_key1": "nested_value1",
            "nested_key2": 4567
        },
        "key5": "new_value"
    });

    c.bench_function("diff_small_json", |b| {
        b.iter(|| diff(&v1, &v2, None, None, None))
    });

    // Large JSON for more realistic benchmark
    let mut large_v1 = serde_json::Map::new();
    let mut large_v2 = serde_json::Map::new();

    for i in 0..1000 {
        large_v1.insert(format!("key{}", i), json!(format!("value{}", i)));
        large_v2.insert(format!("key{}", i), json!(format!("value{}", i)));
    }
    large_v2.insert("key500".to_string(), json!("value500_changed"));
    large_v2.insert("new_key".to_string(), json!("new_value"));

    let large_json_v1 = Value::Object(large_v1);
    let large_json_v2 = Value::Object(large_v2);

    c.bench_function("diff_large_json", |b| {
        b.iter(|| diff(&large_json_v1, &large_json_v2, None, None, None))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

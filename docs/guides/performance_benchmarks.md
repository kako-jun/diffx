# Performance Benchmarks

This document provides performance benchmarks for `diffx` core operations and guidance for performance optimization.

## Benchmark Results

### Environment
- **Platform**: Ubuntu 22.04 (GitHub Actions runner)
- **CPU**: AMD EPYC 7763 (or equivalent CI environment)
- **Rust**: 1.75+ with release optimizations
- **Test Date**: January 2025

### Core Diff Operations

| Test Case | Data Size | Time | Throughput |
|-----------|-----------|------|------------|
| Small JSON | ~200 bytes | **1.3µs** | ~150MB/s |
| Large JSON | ~25KB | **281µs** | ~89MB/s |

### Detailed Results

#### Small JSON Diff
```
Test data: Nested JSON with 5 keys, arrays, and objects
Time: 1.3076µs - 1.3341µs (median: 1.32µs)
```

#### Large JSON Diff  
```
Test data: 1000 key object with string values
Time: 272.26µs - 290.94µs (median: 281µs)
```

## Performance Characteristics

### Time Complexity
- **Object comparison**: O(n) where n = number of keys
- **Array comparison**: O(n×m) where n,m = array lengths
- **Array with ID key**: O(n+m) with semantic tracking
- **Nested structures**: O(depth × elements)

### Memory Usage
- **Small files (<1MB)**: ~2x input size
- **Large files (>10MB)**: ~1.5x input size
- **Streaming**: Not implemented (loads full files)

## Performance Regression Detection

### CI Thresholds
- **Small JSON**: < 2.0µs (50% safety margin)
- **Large JSON**: < 500µs (75% safety margin)

### Monitoring
- **GitHub Actions**: Automatic on every PR/push
- **Weekly runs**: Sunday 2 AM UTC for long-term tracking
- **Alerts**: Automatic failure if thresholds exceeded

## Optimization Guidelines

### For Maximum Performance

1. **Use appropriate formats**
   ```bash
   # Fastest: Binary JSON formats
   diffx data1.json data2.json
   
   # Slower: Complex nested YAML
   diffx complex1.yaml complex2.yaml
   ```

2. **Optimize array comparisons**
   ```bash
   # Efficient: Use ID-based comparison
   diffx users1.json users2.json --array-id-key id
   
   # Inefficient: Index-based comparison on large arrays
   diffx large_array1.json large_array2.json
   ```

3. **Filter early**
   ```bash
   # Focus comparison on relevant parts
   diffx config1.json config2.json --path "database"
   diffx data1.json data2.json --ignore-keys-regex "^(timestamp|_.*)"
   ```

### Performance Options

```bash
# Semantic array comparison
diffx file1.json file2.json --array-id-key id

# Skip internal fields
diffx file1.json file2.json --ignore-keys-regex "^_.*"

# Efficient float comparison  
diffx file1.json file2.json --epsilon 0.001
```

## Comparison with Other Tools

| Tool | Small JSON | Large JSON | Features |
|------|------------|------------|----------|
| **diffx** | **1.3µs** | **281µs** | Semantic, multi-format |
| traditional diff | ~2ms | ~50ms | Text-based, formatting sensitive |
| jq + diff | ~5ms | ~100ms | JSON-only, requires preprocessing |

### Advantages
- **10-100x faster** than text-based tools on structured data
- **Format-agnostic**: Same performance across JSON/YAML/TOML
- **Semantic focus**: Ignores irrelevant formatting differences

## Real-World Performance

### Typical Use Cases

1. **CI/CD Config Validation** (1-10KB files)
   - Time: < 10µs
   - Suitable for: Real-time validation

2. **API Schema Comparison** (10-100KB files)  
   - Time: < 1ms
   - Suitable for: Development workflows

3. **Large Data Files** (1-10MB files)
   - Time: 10-100ms 
   - Suitable for: Batch processing

4. **Directory Comparison** (100+ files)
   - Time: 1-10s depending on size
   - Suitable for: Deployment validation

### Performance Tips

1. **Batch operations**: Use directory comparison for multiple files
2. **Parallel processing**: CI/CD can run multiple diffx instances
3. **Early filtering**: Use `--path` to focus on changed sections
4. **Config optimization**: Set appropriate `array_id_key` for datasets

## Regression Testing

### Running Benchmarks Locally

```bash
# Run full benchmark suite
cargo bench --package diffx-core

# Quick performance check
cargo build --release
time target/release/diffx large_file1.json large_file2.json
```

### Continuous Monitoring

Our CI pipeline automatically:
- Runs benchmarks on every PR
- Compares against baseline performance
- Fails builds with >50% regression
- Archives results for historical tracking

### Historical Performance

Track performance over time at:
- GitHub Actions artifacts
- Criterion benchmark reports
- Release performance summaries

---

**Note**: Benchmark results may vary based on hardware, system load, and data characteristics. The provided figures represent typical performance in CI environments.
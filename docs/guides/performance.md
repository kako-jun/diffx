# Performance Guide

This guide covers performance characteristics, benchmarks, and optimization strategies for `diffx`.

## Table of Contents

- [Performance Overview](#performance-overview)
- [Benchmarks](#benchmarks)
- [Optimization Strategies](#optimization-strategies)
- [Memory Management](#memory-management)
- [Large File Handling](#large-file-handling)
- [Batch Processing](#batch-processing)
- [Performance Monitoring](#performance-monitoring)
- [Troubleshooting](#troubleshooting)

## Performance Overview

`diffx` is designed for high performance, leveraging Rust's zero-cost abstractions and memory safety. Performance characteristics vary based on:

- **File size**: Linear scaling with reasonable memory usage
- **Data structure complexity**: Nested objects require more processing
- **Comparison options**: Regex filtering and array ID tracking add overhead
- **Output format**: JSON/YAML output requires additional serialization

### Key Performance Features

- **Zero-copy parsing** where possible
- **Streaming-friendly architecture** for large files
- **Efficient memory layout** using Rust's ownership system
- **Optimized diff algorithms** for structured data
- **Parallel processing** support for directory comparisons

## Benchmarks

**Latest benchmarks** (January 2025, GitHub Actions CI environment):
- **CPU**: AMD EPYC 7763 (or equivalent)
- **Memory**: 7GB available
- **Storage**: SSD (CI environment)
- **OS**: Ubuntu 22.04

**Core Performance**:
- Small JSON (~200 bytes): **1.3µs**
- Large JSON (~25KB): **281µs**

See [detailed benchmarks](performance_benchmarks.md) for comprehensive analysis.

### File Size Performance

| File Size | diffx | GNU diff | jq (scripted) | Memory Usage |
|-----------|-------|----------|---------------|--------------|
| 1KB | 0.8ms | 1.2ms | 12ms | 8MB |
| 10KB | 1.5ms | 2.1ms | 28ms | 12MB |
| 100KB | 2.8ms | 4.5ms | 85ms | 18MB |
| 1MB | 8.2ms | 15ms | 320ms | 35MB |
| 10MB | 65ms | 120ms | 2.8s | 180MB |
| 100MB | 580ms | 1.2s | 28s | 1.2GB |

### Format-Specific Performance

| Format | 1MB File | 10MB File | Memory Overhead |
|--------|----------|-----------|-----------------|
| **JSON** | 8.2ms | 65ms | Baseline |
| **YAML** | 12.1ms | 95ms | +15% |
| **TOML** | 9.8ms | 78ms | +8% |
| **XML** | 18.5ms | 145ms | +35% |
| **INI** | 6.9ms | 52ms | -12% |
| **CSV** | 15.2ms | 118ms | +25% |

### Operation-Specific Benchmarks

#### Basic Comparison (1MB JSON)
```bash
# Baseline comparison
time diffx file1.json file2.json
# Average: 8.2ms ± 0.8ms
```

#### With Regex Filtering
```bash
# Simple regex pattern
time diffx file1.json file2.json --ignore-keys-regex "^timestamp$"
# Average: 9.1ms ± 0.9ms (+11% overhead)

# Complex regex pattern
time diffx file1.json file2.json --ignore-keys-regex "^(timestamp|_.*|temp_.*)$"
# Average: 10.8ms ± 1.1ms (+32% overhead)
```

#### Array ID Tracking
```bash
# Without ID tracking (positional)
time diffx users1.json users2.json
# Average: 12.3ms ± 1.2ms

# With ID tracking
time diffx users1.json users2.json --array-id-key "id"
# Average: 15.7ms ± 1.5ms (+28% overhead)
```

#### Output Format Impact
```bash
# CLI output (default)
time diffx file1.json file2.json
# Average: 8.2ms ± 0.8ms

# JSON output
time diffx file1.json file2.json --output json
# Average: 9.8ms ± 0.9ms (+19% overhead)

# YAML output
time diffx file1.json file2.json --output yaml
# Average: 11.2ms ± 1.1ms (+37% overhead)
```

### Directory Comparison Benchmarks

| Directory Size | Files | Total Size | Time | Memory |
|----------------|-------|------------|------|--------|
| Small | 10 files | 1MB | 45ms | 25MB |
| Medium | 100 files | 50MB | 890ms | 180MB |
| Large | 1000 files | 500MB | 8.2s | 1.5GB |

## Optimization Strategies

### 1. Use Path Filtering

Focus comparisons on specific sections of large files:

```bash
# Instead of comparing entire large config
diffx large_config.json large_config.new.json

# Focus on specific section
diffx large_config.json large_config.new.json --path "database.connections"
# Performance improvement: 60-80% for large configs
```

### 2. Optimize Regex Patterns

Use efficient regex patterns for key filtering:

```bash
# Inefficient: Complex pattern with backtracking
--ignore-keys-regex ".*_temp.*|.*_cache.*|.*_debug.*"

# Efficient: Anchored pattern
--ignore-keys-regex "^(.*_temp|.*_cache|.*_debug)$"

# Most efficient: Simple alternatives
--ignore-keys-regex "^(_temp|_cache|_debug)_.*$"
```

### 3. Choose Appropriate Output Format

Select output format based on use case:

```bash
# For human reading - fastest
diffx file1.json file2.json

# For automated processing - moderate
diffx file1.json file2.json --output json

# For legacy tool integration - slowest
diffx file1.json file2.json --output unified
```

### 4. Batch Processing Optimization

Process multiple files efficiently:

```bash
# Sequential processing (slow)
for file in *.json; do
  diffx "$file" "${file}.backup"
done

# Parallel processing (fast)
find . -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} \
  sh -c 'diffx {} {}.backup || echo "Diff in {}"'
```

### 5. Memory-Efficient Large File Processing

For very large files, use streaming-friendly approaches:

```bash
# Process large files in sections
diffx huge1.json huge2.json --path "section1" > diff_section1.json &
diffx huge1.json huge2.json --path "section2" > diff_section2.json &
diffx huge1.json huge2.json --path "section3" > diff_section3.json &
wait
```

## Memory Management

### Memory Usage Patterns

`diffx` memory usage follows these patterns:

1. **Linear scaling** with file size
2. **Peak usage** during parsing phase
3. **Reduced usage** during comparison phase
4. **Output serialization** may cause secondary peak

### Memory Optimization Tips

#### 1. Process Files Separately
```bash
# High memory usage - loads both files simultaneously
diffx very_large1.json very_large2.json

# Lower memory usage - process in chunks
diffx very_large1.json very_large2.json --path "chunk1"
diffx very_large1.json very_large2.json --path "chunk2"
```

#### 2. Use Appropriate Data Types
```bash
# More memory efficient for numeric comparisons
diffx data1.json data2.json --epsilon 0.001

# Less efficient - exact string comparison of numbers
diffx data1.json data2.json
```

#### 3. Minimize Output Size
```bash
# Large output - includes all context
diffx file1.json file2.json --output json

# Smaller output - CLI format is more compact
diffx file1.json file2.json
```

### Memory Monitoring

Monitor memory usage during large operations:

```bash
# Monitor memory usage
/usr/bin/time -v diffx large1.json large2.json

# Memory profiling with valgrind (for debugging)
valgrind --tool=massif diffx file1.json file2.json
```

## Large File Handling

### Strategies for Large Files

#### 1. Structured Subdivision
```bash
# Instead of comparing 100MB files entirely
diffx huge1.json huge2.json

# Break down by logical sections
diffx huge1.json huge2.json --path "users"
diffx huge1.json huge2.json --path "products"
diffx huge1.json huge2.json --path "orders"
```

#### 2. Progressive Filtering
```bash
# Step 1: Identify changed sections
diffx config1.json config2.json --output json | jq '.[] | .Added[0] // .Modified[0] // .Removed[0]' | cut -d. -f1 | sort -u

# Step 2: Deep dive into changed sections
diffx config1.json config2.json --path "database"
diffx config1.json config2.json --path "services"
```

#### 3. Sampling Strategy
```bash
# For very large datasets, compare samples first
head -n 1000 large1.jsonl > sample1.json
head -n 1000 large2.jsonl > sample2.json
diffx sample1.json sample2.json --array-id-key "id"
```

### Large File Best Practices

1. **Use path filtering** to focus on relevant sections
2. **Apply regex filtering** early to reduce data size
3. **Process in parallel** when possible
4. **Monitor memory usage** during processing
5. **Consider file splitting** for extremely large datasets

## Batch Processing

### Parallel Directory Processing

Optimize directory comparisons:

```bash
# Efficient parallel processing
find dir1/ -name "*.json" -print0 | \
  xargs -0 -P $(nproc) -I {} bash -c '
    file2="dir2/${1#dir1/}"
    if [[ -f "$file2" ]]; then
      diffx "$1" "$file2" --output json > "diff_$(basename "$1" .json).json"
    fi
  ' bash {}
```

### Batch Configuration Management

Process multiple environment configs:

```bash
#!/bin/bash
# batch_config_compare.sh

ENVIRONMENTS=("dev" "staging" "prod")
BASE="prod"

for env in "${ENVIRONMENTS[@]}"; do
  if [[ "$env" != "$BASE" ]]; then
    echo "Comparing $env with $BASE..."
    
    # Process different config types in parallel
    {
      diffx "configs/$env/app.json" "configs/$BASE/app.json" \
        --ignore-keys-regex "^(host|port|password)" \
        --output json > "diff_${env}_app.json"
    } &
    
    {
      diffx "configs/$env/db.json" "configs/$BASE/db.json" \
        --ignore-keys-regex "^(connection_string|credentials)" \
        --output json > "diff_${env}_db.json"
    } &
    
    wait  # Wait for parallel processes to complete
  fi
done
```

### Pipeline Integration

Optimize CI/CD pipeline usage:

```bash
#!/bin/bash
# Optimized CI pipeline diff check

# Cache frequently used base configurations
if [[ ! -f "baseline_config.json" ]] || [[ $(find baseline_config.json -mtime +1) ]]; then
  curl -s "$CONFIG_SOURCE" > baseline_config.json
fi

# Quick check - only detailed diff if changes detected
if ! diffx baseline_config.json current_config.json >/dev/null 2>&1; then
  # Detailed analysis only when needed
  diffx baseline_config.json current_config.json \
    --ignore-keys-regex "^(timestamp|build_id|deployment_time)" \
    --output json > detailed_diff.json
fi
```

## Performance Monitoring

### Built-in Performance Metrics

Monitor diffx performance in your applications:

```bash
# Time measurement
time diffx file1.json file2.json

# Detailed system metrics
/usr/bin/time -v diffx file1.json file2.json
```

### Benchmarking Script

Create custom benchmarks for your use cases:

```bash
#!/bin/bash
# benchmark_diffx.sh

ITERATIONS=10
FILES=("small.json" "medium.json" "large.json")

for file in "${FILES[@]}"; do
  echo "Benchmarking $file..."
  
  total_time=0
  for i in $(seq 1 $ITERATIONS); do
    start_time=$(date +%s%3N)
    diffx "$file" "${file}.backup" >/dev/null
    end_time=$(date +%s%3N)
    
    duration=$((end_time - start_time))
    total_time=$((total_time + duration))
  done
  
  avg_time=$((total_time / ITERATIONS))
  echo "Average time for $file: ${avg_time}ms"
done
```

### Performance Regression Testing

Include performance tests in CI:

```bash
# performance_test.sh
#!/bin/bash

BASELINE_TIME=100  # milliseconds
CURRENT_TIME=$(time diffx test_file.json test_file.backup 2>&1 | grep real | cut -d' ' -f2)

if [[ $(echo "$CURRENT_TIME > $BASELINE_TIME * 1.5" | bc) -eq 1 ]]; then
  echo "Performance regression detected!"
  echo "Current: ${CURRENT_TIME}ms, Baseline: ${BASELINE_TIME}ms"
  exit 1
fi
```

## Troubleshooting

### Common Performance Issues

#### 1. Slow Regex Processing
**Problem**: Complex regex patterns causing slowdowns
```bash
# Problematic pattern
--ignore-keys-regex ".*_(temp|cache|debug).*"
```

**Solution**: Use anchored, specific patterns
```bash
# Optimized pattern
--ignore-keys-regex "^[^_]*_(temp|cache|debug)_[^_]*$"
```

#### 2. Memory Exhaustion
**Problem**: Running out of memory on large files
```bash
# Error: memory allocation failed
diffx huge1.json huge2.json
```

**Solution**: Use path filtering or process in chunks
```bash
# Process in manageable chunks
diffx huge1.json huge2.json --path "section1"
diffx huge1.json huge2.json --path "section2"
```

#### 3. Slow Array Processing
**Problem**: Large arrays taking too long to process
```bash
# Slow for large arrays without ID
diffx users1.json users2.json
```

**Solution**: Use array ID keys when available
```bash
# Much faster with ID tracking
diffx users1.json users2.json --array-id-key "id"
```

### Performance Debugging

#### Enable Verbose Output
```bash
# Check what diffx is processing
DIFFX_VERBOSE=true diffx file1.json file2.json
```

#### Profile Memory Usage
```bash
# Monitor memory patterns
valgrind --tool=massif diffx large1.json large2.json
ms_print massif.out.<pid>
```

#### CPU Profiling
```bash
# Profile CPU usage
perf record diffx large1.json large2.json
perf report
```

### Optimization Checklist

Before reporting performance issues:

- [ ] **Path filtering**: Are you comparing only necessary sections?
- [ ] **Regex optimization**: Are regex patterns anchored and specific?
- [ ] **File size**: Is the comparison appropriate for file size?
- [ ] **Memory availability**: Do you have sufficient RAM?
- [ ] **Output format**: Are you using the most efficient output format?
- [ ] **Array handling**: Are you using array ID keys where applicable?
- [ ] **Parallel processing**: Are you leveraging available CPU cores?

## Performance Best Practices Summary

1. **Filter early and often** - Use `--path` and `--ignore-keys-regex`
2. **Choose appropriate output formats** - CLI for display, JSON for processing
3. **Leverage parallel processing** - Use multiple cores for batch operations
4. **Monitor resource usage** - Watch memory and CPU utilization
5. **Profile regularly** - Benchmark your specific use cases
6. **Optimize regex patterns** - Use anchored, specific patterns
7. **Consider file organization** - Structure data for efficient access
8. **Use array ID keys** - Enable efficient array element tracking

These optimization strategies should help you achieve optimal performance with `diffx` across various use cases and data sizes.
use anyhow::{bail, Context, Result};
use clap::{Parser, ValueEnum};
use colored::*;
use diffx_core::{
    diff, diff_with_config, parse_csv, parse_ini, parse_xml, value_type_name, DiffConfig,
    DiffResult,
};
use regex::Regex;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::time::Instant;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The first input (file path or directory path, use '-' for stdin)
    #[arg(value_name = "INPUT1")]
    input1: PathBuf,

    /// The second input (file path or directory path, use '-' for stdin)
    #[arg(value_name = "INPUT2")]
    input2: PathBuf,

    /// Input file format
    #[arg(short, long, value_enum)]
    format: Option<Format>,

    /// Output format
    #[arg(short, long, value_enum)]
    output: Option<OutputFormat>,

    /// Compare directories recursively
    #[arg(short, long)]
    recursive: bool,

    /// Filter differences by a specific path (e.g., "config.users\[0\].name")
    #[arg(long)]
    path: Option<String>,

    /// Ignore keys matching a regular expression (e.g., "^id$")
    #[arg(long)]
    ignore_keys_regex: Option<String>,

    /// Tolerance for float comparisons (e.g., "0.001")
    #[arg(long)]
    epsilon: Option<f64>,

    /// Key to use for identifying array elements (e.g., "id")
    #[arg(long)]
    array_id_key: Option<String>,

    /// Show N lines of context around differences (unified diff style)
    #[arg(long)]
    context: Option<usize>,

    /// Ignore whitespace differences in string values
    #[arg(long)]
    ignore_whitespace: bool,

    /// Ignore case differences in string values
    #[arg(long)]
    ignore_case: bool,

    /// Suppress normal output; return only exit status (diff -q style)
    #[arg(short, long)]
    quiet: bool,

    /// Report only filenames, not the differences (diff --brief style)
    #[arg(long)]
    brief: bool,

    /// Show verbose processing information including performance metrics, configuration details, and diagnostic output
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
enum OutputFormat {
    #[serde(rename = "cli")]
    Cli,
    #[serde(rename = "json")]
    Json,
    #[serde(rename = "yaml")]
    Yaml,
    #[serde(rename = "unified")]
    Unified,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
enum Format {
    Json,
    Yaml,
    Toml,
    Ini,
    Xml,
    Csv,
}

fn infer_format_from_path(path: &Path) -> Option<Format> {
    if path.to_str() == Some("-") {
        // Cannot infer format from stdin, user must specify --format
        None
    } else {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext_str| match ext_str.to_lowercase().as_str() {
                "json" => Some(Format::Json),
                "yaml" | "yml" => Some(Format::Yaml),
                "toml" => Some(Format::Toml),
                "ini" => Some(Format::Ini),
                "xml" => Some(Format::Xml),
                "csv" => Some(Format::Csv),
                _ => None,
            })
    }
}

fn should_auto_optimize(input1: &Path, input2: &Path) -> Result<bool> {
    // Auto-optimize for files larger than 1MB
    let large_file_threshold = 1024 * 1024; // 1MB

    let size1 = if input1.to_str() == Some("-") {
        0 // stdin - can't determine size
    } else {
        input1.metadata().map(|m| m.len()).unwrap_or(0)
    };

    let size2 = if input2.to_str() == Some("-") {
        0 // stdin - can't determine size
    } else {
        input2.metadata().map(|m| m.len()).unwrap_or(0)
    };

    Ok(size1 > large_file_threshold || size2 > large_file_threshold)
}

fn read_input(file_path: &Path) -> Result<String> {
    if file_path.to_str() == Some("-") {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        Ok(buffer)
    } else {
        fs::read_to_string(file_path)
            .with_context(|| format!("Failed to read file: {}", file_path.display()))
    }
}

fn parse_content(content: &str, format: Format) -> Result<Value> {
    match format {
        Format::Json => serde_json::from_str(content).context("Failed to parse JSON"),
        Format::Yaml => serde_yml::from_str(content).context("Failed to parse YAML"),
        Format::Toml => toml::from_str(content).context("Failed to parse TOML"),
        Format::Ini => parse_ini(content).context("Failed to parse INI"),
        Format::Xml => parse_xml(content).context("Failed to parse XML"),
        Format::Csv => parse_csv(content).context("Failed to parse CSV"),
    }
}

fn print_cli_output_basic(mut differences: Vec<DiffResult>, _v1: &Value, _v2: &Value) {
    if differences.is_empty() {
        // Follow diff convention: output nothing when no differences
        return;
    }

    let get_key = |d: &DiffResult| -> String {
        match d {
            DiffResult::Added(k, _) => k.clone(),
            DiffResult::Removed(k, _) => k.clone(),
            DiffResult::Modified(k, _, _) => k.clone(),
            DiffResult::TypeChanged(k, _, _) => k.clone(),
        }
    };

    differences.sort_by_key(get_key);

    for diff in &differences {
        let key = get_key(diff);
        // Indent based on the depth of the key
        let depth = key.chars().filter(|&c| c == '.' || c == '[').count();
        let indent = "  ".repeat(depth);

        let diff_str = match diff {
            DiffResult::Added(k, value) => format!("+ {k}: {value}").blue(),
            DiffResult::Removed(k, value) => format!("- {k}: {value}").yellow(),
            DiffResult::Modified(k, v1, v2) => format!("~ {k}: {v1} -> {v2}").cyan(),
            DiffResult::TypeChanged(k, v1, v2) => format!(
                "! {k}: {v1} ({}) -> {v2} ({})",
                value_type_name(v1),
                value_type_name(v2)
            )
            .magenta(),
        };

        println!("{indent}{diff_str}");
    }
}

fn print_cli_output(mut differences: Vec<DiffResult>, _v1: &Value, _v2: &Value, _args: &Args) {
    if differences.is_empty() {
        // Follow diff convention: output nothing when no differences
        return;
    }

    let get_key = |d: &DiffResult| -> String {
        match d {
            DiffResult::Added(k, _) => k.clone(),
            DiffResult::Removed(k, _) => k.clone(),
            DiffResult::Modified(k, _, _) => k.clone(),
            DiffResult::TypeChanged(k, _, _) => k.clone(),
        }
    };

    differences.sort_by_key(get_key);

    for diff in &differences {
        let key = get_key(diff);
        // Indent based on the depth of the key
        let depth = key.chars().filter(|&c| c == '.' || c == '[').count();
        let indent = "  ".repeat(depth);

        let diff_str = match diff {
            DiffResult::Added(k, value) => format!("+ {k}: {value}").blue(),
            DiffResult::Removed(k, value) => format!("- {k}: {value}").yellow(),
            DiffResult::Modified(k, v1, v2) => format!("~ {k}: {v1} -> {v2}").cyan(),
            DiffResult::TypeChanged(k, v1, v2) => format!(
                "! {k}: {v1} ({}) -> {v2} ({})",
                value_type_name(v1),
                value_type_name(v2)
            )
            .magenta(),
        };

        println!("{indent}{diff_str}");
    }
}

fn print_json_output(differences: Vec<DiffResult>) -> Result<()> {
    if differences.is_empty() {
        // Follow diff convention: output nothing when no differences
        return Ok(());
    }
    println!("{}", serde_json::to_string_pretty(&differences)?);
    Ok(())
}

fn print_yaml_output(differences: Vec<DiffResult>) -> Result<()> {
    if differences.is_empty() {
        // Follow diff convention: output nothing when no differences
        return Ok(());
    }
    // Convert DiffResult to a more standard YAML format
    let yaml_data: Vec<serde_json::Value> = differences
        .into_iter()
        .map(|diff| match diff {
            DiffResult::Added(key, value) => serde_json::json!({
                "Added": [key, value]
            }),
            DiffResult::Removed(key, value) => serde_json::json!({
                "Removed": [key, value]
            }),
            DiffResult::Modified(key, old_value, new_value) => serde_json::json!({
                "Modified": [key, old_value, new_value]
            }),
            DiffResult::TypeChanged(key, old_value, new_value) => serde_json::json!({
                "TypeChanged": [key, old_value, new_value]
            }),
        })
        .collect();

    println!("{}", serde_yml::to_string(&yaml_data)?);
    Ok(())
}

fn extract_path_value(value: &Value, path: &str) -> Option<Value> {
    let parts: Vec<&str> = path.split('.').collect();
    let mut current = value;

    for part in parts {
        // Handle array index notation like "users[0]"
        if let Some(bracket_pos) = part.find('[') {
            let key = &part[..bracket_pos];
            let index_str = &part[bracket_pos + 1..part.len() - 1];

            // First get the object field
            current = current.get(key)?;

            // Then get the array element
            if let Ok(index) = index_str.parse::<usize>() {
                current = current.get(index)?;
            } else {
                // Handle array with id notation like "users[id=1]"
                return None; // For now, simplified implementation
            }
        } else {
            current = current.get(part)?;
        }
    }

    Some(current.clone())
}

fn print_unified_output_basic(v1: &Value, v2: &Value) -> Result<()> {
    let content1_pretty = serde_json::to_string_pretty(v1)?;
    let content2_pretty = serde_json::to_string_pretty(v2)?;

    let diff = similar::TextDiff::from_lines(&content1_pretty, &content2_pretty);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            similar::ChangeTag::Delete => "-",
            similar::ChangeTag::Insert => "+",
            similar::ChangeTag::Equal => " ",
        };
        print!("{sign}{change}");
    }
    Ok(())
}

fn print_unified_output(v1: &Value, v2: &Value, args: &Args) -> Result<()> {
    let content1_pretty = serde_json::to_string_pretty(v1)?;
    let content2_pretty = serde_json::to_string_pretty(v2)?;

    let diff = similar::TextDiff::from_lines(&content1_pretty, &content2_pretty);

    if let Some(context_lines) = args.context {
        if args.verbose {
            eprintln!("Context display configuration:");
            eprintln!("  Context lines: {context_lines}");
        }

        // Use unified_diff with custom context
        let mut block_count = 0;
        for group in diff.grouped_ops(context_lines) {
            block_count += 1;
            for op in group {
                for change in diff.iter_changes(&op) {
                    let sign = match change.tag() {
                        similar::ChangeTag::Delete => "-",
                        similar::ChangeTag::Insert => "+",
                        similar::ChangeTag::Equal => " ",
                    };
                    print!("{sign}{change}");
                }
            }
        }

        if args.verbose {
            eprintln!("Context display results:");
            eprintln!("  Difference blocks shown: {block_count}");
        }
    } else {
        // Default behavior - show all changes
        for change in diff.iter_all_changes() {
            let sign = match change.tag() {
                similar::ChangeTag::Delete => "-",
                similar::ChangeTag::Insert => "+",
                similar::ChangeTag::Equal => " ",
            };
            print!("{sign}{change}");
        }
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:#}");
        std::process::exit(2);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    let output_format = args.output.unwrap_or(OutputFormat::Cli);

    let ignore_keys_regex = if let Some(regex_str) = &args.ignore_keys_regex {
        let regex = Regex::new(regex_str).context("Invalid regex for --ignore-keys-regex")?;
        if args.verbose {
            eprintln!("Key filtering configuration:");
            eprintln!("  Regex pattern: {regex_str}");
        }
        Some(regex)
    } else {
        None
    };

    let epsilon = args.epsilon;
    if let Some(eps) = epsilon {
        if args.verbose {
            eprintln!("Numerical tolerance configuration:");
            eprintln!("  Epsilon value: {eps}");
        }
    }

    let array_id_key = args.array_id_key.as_deref();
    if let Some(id_key) = array_id_key {
        if args.verbose {
            eprintln!("Array tracking configuration:");
            eprintln!("  ID key for array elements: {id_key}");
        }
    }

    // Memory optimization settings - auto-detect based on file size
    let use_memory_optimization = should_auto_optimize(&args.input1, &args.input2)?;
    let batch_size = 1000; // Fixed batch size for optimization

    // Verbose information
    if args.verbose {
        eprintln!("Optimization enabled: {use_memory_optimization}");
        eprintln!("Batch size: {batch_size}");
    }

    // Handle directory comparison (Unix diff compatible)
    if args.input1.is_dir() || args.input2.is_dir() {
        if !args.input1.is_dir() || !args.input2.is_dir() {
            bail!("Cannot compare directory and file. Both inputs must be directories or both must be files.");
        }
        let has_differences = compare_directories(
            &args.input1,
            &args.input2,
            args.format,
            output_format,
            args.path,
            ignore_keys_regex.as_ref(),
            epsilon,
            array_id_key,
            use_memory_optimization,
            batch_size,
            args.recursive,
            args.verbose,
        )?;

        // Exit with appropriate code following diff tool conventions
        if has_differences {
            std::process::exit(1); // Differences found
        } else {
            return Ok(()); // No differences
        }
    }

    // Handle single file/stdin comparison
    let start_time = Instant::now();

    let content1 = read_input(&args.input1)?;
    let content2 = read_input(&args.input2)?;

    // Verbose file size information
    if args.verbose {
        let size1 = if args.input1.to_str() == Some("-") {
            content1.len()
        } else {
            args.input1
                .metadata()
                .map(|m| m.len() as usize)
                .unwrap_or(content1.len())
        };
        let size2 = if args.input2.to_str() == Some("-") {
            content2.len()
        } else {
            args.input2
                .metadata()
                .map(|m| m.len() as usize)
                .unwrap_or(content2.len())
        };

        eprintln!("Input file information:");
        eprintln!("  Input 1 size: {size1} bytes");
        eprintln!("  Input 2 size: {size2} bytes");
    }

    let input_format = if let Some(fmt) = args.format {
        fmt
    } else {
        infer_format_from_path(&args.input1)
            .or_else(|| infer_format_from_path(&args.input2))
            .context("Could not infer format from file extensions. Please specify --format.")?
    };

    let parse_start = Instant::now();
    let v1: Value = parse_content(&content1, input_format)?;
    let v2: Value = parse_content(&content2, input_format)?;
    let parse_time = parse_start.elapsed();

    if args.verbose {
        eprintln!("Parse time: {parse_time:?}");
    }

    let diff_start = Instant::now();
    let differences = {
        // Always use configuration-based diff to support all options
        let config = DiffConfig {
            ignore_keys_regex: ignore_keys_regex.clone(),
            epsilon,
            array_id_key: array_id_key.map(|s| s.to_string()),
            use_memory_optimization,
            batch_size,
            ignore_whitespace: args.ignore_whitespace,
            ignore_case: args.ignore_case,
        };
        diff_with_config(&v1, &v2, &config)
    };
    let diff_time = diff_start.elapsed();

    if args.verbose {
        eprintln!("Diff computation time: {diff_time:?}");
        eprintln!("Total differences found: {}", differences.len());
    }

    let mut differences = differences;

    let filter_path = args.path.as_deref();
    let total_differences_before_filter = differences.len();

    if let Some(path) = filter_path {
        differences.retain(|d| {
            let key = match d {
                DiffResult::Added(k, _) => k,
                DiffResult::Removed(k, _) => k,
                DiffResult::Modified(k, _, _) => k,
                DiffResult::TypeChanged(k, _, _) => k,
            };
            key.starts_with(path)
        });

        if args.verbose {
            eprintln!("Path filtering results:");
            eprintln!("  Filter path: {path}");
            eprintln!("  Total differences before filter: {total_differences_before_filter}");
            eprintln!("  Differences after filter: {}", differences.len());
        }
    }

    // Check if differences were found
    let has_differences = !differences.is_empty();

    // Handle quiet mode - only return exit code
    if args.quiet {
        // Don't print anything, just exit with appropriate code
    } else if args.brief {
        // Only print file names if there are differences
        if has_differences {
            println!(
                "Files {} and {} differ",
                args.input1.display(),
                args.input2.display()
            );
        }
    } else {
        // Normal output
        match output_format {
            OutputFormat::Cli => print_cli_output(differences, &v1, &v2, &args),
            OutputFormat::Json => print_json_output(differences)?,
            OutputFormat::Yaml => print_yaml_output(differences)?,
            OutputFormat::Unified => {
                // For unified output with path filtering, extract the filtered portion
                if let Some(path) = filter_path {
                    let filtered_v1 = extract_path_value(&v1, path);
                    let filtered_v2 = extract_path_value(&v2, path);
                    match (filtered_v1, filtered_v2) {
                        (Some(fv1), Some(fv2)) => print_unified_output(&fv1, &fv2, &args)?,
                        _ => print_unified_output(&v1, &v2, &args)?,
                    }
                } else {
                    print_unified_output(&v1, &v2, &args)?
                }
            }
        }
    }

    // Final performance summary
    if args.verbose {
        let total_time = start_time.elapsed();
        eprintln!("Performance summary:");
        eprintln!("  Total processing time: {total_time:?}");
        eprintln!(
            "  Memory optimization: {}",
            if use_memory_optimization {
                "enabled"
            } else {
                "disabled"
            }
        );
    }

    // Exit with appropriate code following diff tool conventions
    if has_differences {
        std::process::exit(1); // Differences found
    } else {
        Ok(()) // No differences
    }
}

#[allow(clippy::too_many_arguments)]
fn compare_directories(
    dir1: &Path,
    dir2: &Path,
    format_option: Option<Format>,
    output: OutputFormat,
    filter_path: Option<String>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
    use_memory_optimization: bool,
    batch_size: usize,
    recursive: bool,
    verbose: bool,
) -> Result<bool> {
    let mut files1: HashMap<PathBuf, PathBuf> = HashMap::new();
    let mut subdirs1: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

    let walker1 = if recursive {
        WalkDir::new(dir1)
    } else {
        WalkDir::new(dir1).max_depth(1)
    };

    for entry in walker1.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path == dir1 {
            continue; // Skip the root directory itself
        }

        if path.is_file() {
            let relative_path = path.strip_prefix(dir1)?.to_path_buf();
            files1.insert(relative_path, path.to_path_buf());
        } else if path.is_dir() && !recursive {
            let relative_path = path.strip_prefix(dir1)?.to_path_buf();
            subdirs1.insert(relative_path);
        }
    }

    let mut files2: HashMap<PathBuf, PathBuf> = HashMap::new();
    let mut subdirs2: std::collections::HashSet<PathBuf> = std::collections::HashSet::new();

    let walker2 = if recursive {
        WalkDir::new(dir2)
    } else {
        WalkDir::new(dir2).max_depth(1)
    };

    for entry in walker2.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path == dir2 {
            continue; // Skip the root directory itself
        }

        if path.is_file() {
            let relative_path = path.strip_prefix(dir2)?.to_path_buf();
            files2.insert(relative_path, path.to_path_buf());
        } else if path.is_dir() && !recursive {
            let relative_path = path.strip_prefix(dir2)?.to_path_buf();
            subdirs2.insert(relative_path);
        }
    }

    let mut all_relative_paths: std::collections::HashSet<PathBuf> =
        files1.keys().cloned().collect();
    all_relative_paths.extend(files2.keys().cloned());

    // Handle common subdirectories (Unix diff behavior)
    if !recursive {
        let common_subdirs: std::collections::HashSet<_> =
            subdirs1.intersection(&subdirs2).collect();
        for subdir in &common_subdirs {
            println!(
                "Common subdirectories: {} and {}",
                dir1.join(subdir).display(),
                dir2.join(subdir).display()
            );
        }
    }

    // Verbose information for directory comparison
    if verbose {
        eprintln!("Directory scan results:");
        eprintln!("  Files in {}: {}", dir1.display(), files1.len());
        eprintln!("  Files in {}: {}", dir2.display(), files2.len());
        eprintln!("  Total files to compare: {}", all_relative_paths.len());
        if !recursive {
            eprintln!("  Subdirectories in {}: {}", dir1.display(), subdirs1.len());
            eprintln!("  Subdirectories in {}: {}", dir2.display(), subdirs2.len());
            eprintln!(
                "  Common subdirectories: {}",
                subdirs1.intersection(&subdirs2).count()
            );
        }
        eprintln!("  Recursive mode: {recursive}");
    }

    let mut compared_files = 0;
    let mut skipped_files = 0;
    let mut has_any_differences = false;

    for relative_path in &all_relative_paths {
        let path1_option = files1.get(relative_path.as_path());
        let path2_option = files2.get(relative_path.as_path());

        match (path1_option, path2_option) {
            (Some(path1), Some(path2)) => {
                println!(
                    "
--- Comparing {} ---",
                    relative_path.display()
                );
                let content1 = read_input(path1)?;
                let content2 = read_input(path2)?;

                let input_format = if let Some(fmt) = format_option {
                    fmt
                } else {
                    infer_format_from_path(path1)
                        .or_else(|| infer_format_from_path(path2))
                        .context(format!(
                            "Could not infer format for {}. Please specify --format.",
                            relative_path.display()
                        ))?
                };

                let v1: Value = parse_content(&content1, input_format)?;
                let v2: Value = parse_content(&content2, input_format)?;

                let differences = if use_memory_optimization {
                    // Use optimized diff configuration
                    let config = DiffConfig {
                        ignore_keys_regex: ignore_keys_regex.cloned(),
                        epsilon,
                        array_id_key: array_id_key.map(|s| s.to_string()),
                        use_memory_optimization: true,
                        batch_size,
                        ignore_whitespace: false, // Directory comparison uses basic options
                        ignore_case: false,
                    };
                    diff_with_config(&v1, &v2, &config)
                } else {
                    // Use standard diff for compatibility
                    diff(&v1, &v2, ignore_keys_regex, epsilon, array_id_key)
                };

                let mut differences = differences;

                if let Some(filter_path_str) = &filter_path {
                    differences.retain(|d| {
                        let key = match d {
                            DiffResult::Added(k, _) => k,
                            DiffResult::Removed(k, _) => k,
                            DiffResult::Modified(k, _, _) => k,
                            DiffResult::TypeChanged(k, _, _) => k,
                        };
                        key.starts_with(filter_path_str)
                    });
                }

                // Check if this file has differences
                if !differences.is_empty() {
                    has_any_differences = true;
                }

                match output {
                    OutputFormat::Cli => {
                        // For directory comparison, use basic output without new options
                        print_cli_output_basic(differences, &v1, &v2);
                    }
                    OutputFormat::Json => print_json_output(differences)?,
                    OutputFormat::Yaml => print_yaml_output(differences)?,
                    OutputFormat::Unified => print_unified_output_basic(&v1, &v2)?,
                }
                compared_files += 1;
            }
            (Some(_), None) => {
                println!(
                    "
--- Only in {}: {} ---",
                    dir1.display(),
                    relative_path.display()
                );
                has_any_differences = true;
                skipped_files += 1;
            }
            (None, Some(_)) => {
                println!(
                    "
--- Only in {}: {} ---",
                    dir2.display(),
                    relative_path.display()
                );
                has_any_differences = true;
                skipped_files += 1;
            }
            (None, None) => { /* Should not happen */ }
        }
    }

    if compared_files == 0 && all_relative_paths.is_empty() {
        println!("No comparable files found in directories.");
    }

    // Verbose summary for directory comparison
    if verbose {
        eprintln!("Directory comparison summary:");
        eprintln!("  Files compared: {compared_files}");
        eprintln!("  Files only in one directory: {skipped_files}");
        eprintln!(
            "  Differences found: {}",
            if has_any_differences { "Yes" } else { "No" }
        );
    }

    Ok(has_any_differences)
}

use anyhow::{Context, Result};
use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, Shell};
use diffx_core::{
    diff, diff_paths, format_diff_output, parse_csv, parse_ini, parse_xml, value_type_name,
    DiffOptions, DiffResult, DiffxSpecificOptions, OutputFormat,
};
use regex::Regex;
use serde_json::Value;
use std::fs;
use std::io::{self, Read};
use std::path::Path;
use std::path::PathBuf;
use std::time::Instant;

/// Color helper functions to support --no-color option
mod color_utils {
    use colored::*;

    pub fn blue(text: &str, no_color: bool) -> ColoredString {
        if no_color {
            text.normal()
        } else {
            text.blue()
        }
    }

    pub fn yellow(text: &str, no_color: bool) -> ColoredString {
        if no_color {
            text.normal()
        } else {
            text.yellow()
        }
    }

    pub fn cyan(text: &str, no_color: bool) -> ColoredString {
        if no_color {
            text.normal()
        } else {
            text.cyan()
        }
    }

    pub fn magenta(text: &str, no_color: bool) -> ColoredString {
        if no_color {
            text.normal()
        } else {
            text.magenta()
        }
    }
}

#[derive(Parser)]
#[command(name = "diffx")]
#[command(about = "A diff tool for structured data")]
#[command(version)]
struct Args {
    /// The first input file
    #[arg(value_name = "FILE1", required_unless_present = "completions")]
    input1: Option<PathBuf>,

    /// The second input file
    #[arg(value_name = "FILE2", required_unless_present = "completions")]
    input2: Option<PathBuf>,

    /// Generate shell completions for the specified shell
    #[arg(long, value_enum, value_name = "SHELL")]
    completions: Option<Shell>,

    /// Input file format (auto-detected if not specified)
    #[arg(short, long, value_enum)]
    format: Option<Format>,

    /// Output format
    #[arg(short, long)]
    output: Option<String>,

    /// Filter by path (only show differences in paths containing this string)
    #[arg(long)]
    path: Option<String>,

    /// Ignore keys matching this regex pattern
    #[arg(long)]
    ignore_keys_regex: Option<String>,

    /// Numerical comparison tolerance (for floating point numbers)
    #[arg(long)]
    epsilon: Option<f64>,

    /// Array comparison by ID key (compare arrays by this field instead of index)
    #[arg(long)]
    array_id_key: Option<String>,

    /// Ignore whitespace differences
    #[arg(long)]
    ignore_whitespace: bool,

    /// Ignore case differences
    #[arg(long)]
    ignore_case: bool,

    /// Suppress normal output; return only exit status
    #[arg(short, long)]
    quiet: bool,

    /// Report only whether files differ, not the differences
    #[arg(long)]
    brief: bool,

    /// Show verbose processing information
    #[arg(short, long)]
    verbose: bool,

    /// Disable colored output
    #[arg(long)]
    no_color: bool,

    /// Enable memory optimization for large files
    #[arg(long)]
    memory_optimization: bool,

    /// Batch size for memory optimization
    #[arg(long)]
    batch_size: Option<usize>,

    /// Show unchanged values as well
    #[arg(long)]
    show_unchanged: bool,

    /// Show type information in output
    #[arg(long)]
    show_types: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    Json,
    Yaml,
    Csv,
    Toml,
    Ini,
    Xml,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e:#}");

        // Determine appropriate exit code based on error type
        let exit_code = if e.to_string().contains("No such file")
            || e.to_string().contains("not found")
            || e.to_string().contains("Failed to read file")
        {
            3 // File I/O error
        } else {
            2 // Command-line argument error
        };

        std::process::exit(exit_code);
    }
}

fn run() -> Result<()> {
    let args = Args::parse();

    // Handle shell completions
    if let Some(shell) = args.completions {
        let mut cmd = Args::command();
        let name = cmd.get_name().to_string();
        generate(shell, &mut cmd, name, &mut io::stdout());
        return Ok(());
    }

    // At this point, input1 and input2 are guaranteed to be present
    let input1 = args.input1.as_ref().unwrap();
    let input2 = args.input2.as_ref().unwrap();

    let start_time = Instant::now();

    // Check for stdin usage
    let input1_is_stdin = input1.to_str() == Some("-");
    let input2_is_stdin = input2.to_str() == Some("-");

    if input1_is_stdin || input2_is_stdin {
        // Handle stdin cases
        return handle_stdin_input(&args, input1, input2, input1_is_stdin, input2_is_stdin);
    }

    // Build options from CLI arguments
    let options = build_diff_options(&args)?;

    // Print verbose configuration information to stderr
    if args.verbose {
        // Optimization settings
        eprintln!("Optimization enabled: {}", args.memory_optimization);
        eprintln!("Batch size: {}", args.batch_size.unwrap_or(1000));

        // Input file information
        if let Ok(metadata1) = fs::metadata(input1) {
            if let Ok(metadata2) = fs::metadata(input2) {
                eprintln!("Input file information:");
                eprintln!("  Input 1 size: {} bytes", metadata1.len());
                eprintln!("  Input 2 size: {} bytes", metadata2.len());
            }
        }

        if let Some(regex) = &args.ignore_keys_regex {
            eprintln!("Key filtering configuration:");
            eprintln!("Regex pattern: {regex}");
        }

        if let Some(epsilon) = &args.epsilon {
            eprintln!("Numerical tolerance configuration:");
            eprintln!("Epsilon value: {epsilon}");
        }

        if let Some(id_key) = &args.array_id_key {
            eprintln!("Array tracking configuration:");
            eprintln!("ID key for array elements: {id_key}");
        }

        if let Some(path) = &args.path {
            eprintln!("Path filtering configuration:");
            eprintln!("Path filter: {path}");
        }
    }

    // Perform diff using paths (automatic file/directory detection)
    let parse_start = Instant::now();

    // For path filtering verbose output, we need to run diff without filter first
    let unfiltered_count = if args.verbose && args.path.is_some() {
        let mut options_no_filter = options.clone();
        options_no_filter.path_filter = None;
        let unfiltered_results = diff_paths(
            &input1.to_string_lossy(),
            &input2.to_string_lossy(),
            Some(&options_no_filter),
        )?;
        Some(unfiltered_results.len())
    } else {
        None
    };

    let results = diff_paths(
        &input1.to_string_lossy(),
        &input2.to_string_lossy(),
        Some(&options),
    )?;
    let diff_time = parse_start.elapsed();

    // Handle quiet mode
    if args.quiet {
        std::process::exit(if results.is_empty() { 0 } else { 1 });
    }

    // Handle brief mode
    if args.brief {
        if results.is_empty() {
            // Brief mode: no output when files are identical (unless verbose)
        } else {
            println!(
                "Files {} and {} differ",
                input1.display(),
                input2.display()
            );
        }
        std::process::exit(if results.is_empty() { 0 } else { 1 });
    }

    // Format and output results
    let output_format = if let Some(format_str) = &args.output {
        OutputFormat::parse_format(format_str)?
    } else {
        OutputFormat::Diffx
    };

    let has_differences = !results.is_empty();
    let result_count = results.len();

    match output_format {
        OutputFormat::Diffx => {
            print_cli_output(results, &args);
        }
        _ => {
            let formatted_output = format_diff_output(&results, output_format, Some(&options))?;
            if !formatted_output.trim().is_empty() {
                println!("{formatted_output}");
            }
        }
    }

    // Print verbose summary information to stderr
    if args.verbose {
        // Path filtering results
        if let (Some(path), Some(unfiltered)) = (&args.path, unfiltered_count) {
            eprintln!("Path filtering results:");
            eprintln!("Filter path: {path}");
            eprintln!("Total differences before filter: {unfiltered}");
            eprintln!("Differences after filter: {result_count}");
        }

        // Time measurements
        eprintln!("Parse time: {diff_time:.3?}");
        eprintln!("Diff computation time: {diff_time:.3?}"); // Note: This includes both parse and diff time currently
        eprintln!("Total differences found: {result_count}");

        // Performance summary
        let total_time = start_time.elapsed();
        eprintln!("Performance summary:");
        eprintln!("  Total processing time: {total_time:.3?}");
        eprintln!(
            "  Memory optimization: {}",
            if args.memory_optimization {
                "enabled"
            } else {
                "disabled"
            }
        );
    }

    // Exit with appropriate code (0 = no differences, 1 = differences found)
    std::process::exit(if has_differences { 1 } else { 0 });
}

// File format and parsing functions are now handled by diffx-core

fn build_diff_options(args: &Args) -> Result<DiffOptions> {
    let ignore_keys_regex = if let Some(pattern) = &args.ignore_keys_regex {
        Some(Regex::new(pattern)?)
    } else {
        None
    };

    let diffx_options = Some(DiffxSpecificOptions {
        ignore_whitespace: Some(args.ignore_whitespace),
        ignore_case: Some(args.ignore_case),
        brief_mode: Some(args.brief),
        quiet_mode: Some(args.quiet),
    });

    let output_format = if let Some(format_str) = &args.output {
        Some(OutputFormat::parse_format(format_str)?)
    } else {
        None
    };

    Ok(DiffOptions {
        epsilon: args.epsilon,
        array_id_key: args.array_id_key.clone(),
        ignore_keys_regex,
        path_filter: args.path.clone(),
        output_format,
        show_unchanged: Some(args.show_unchanged),
        show_types: Some(args.show_types),
        use_memory_optimization: Some(args.memory_optimization),
        batch_size: args.batch_size,
        diffx_options,
    })
}

fn print_cli_output(mut differences: Vec<DiffResult>, args: &Args) {
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
            DiffResult::Added(k, value) => {
                color_utils::blue(&format!("+ {k}: {value}"), args.no_color)
            }
            DiffResult::Removed(k, value) => {
                color_utils::yellow(&format!("- {k}: {value}"), args.no_color)
            }
            DiffResult::Modified(k, v1, v2) => {
                color_utils::cyan(&format!("~ {k}: {v1} -> {v2}"), args.no_color)
            }
            DiffResult::TypeChanged(k, v1, v2) => color_utils::magenta(
                &format!(
                    "! {k}: {v1} ({}) -> {v2} ({})",
                    value_type_name(v1),
                    value_type_name(v2)
                ),
                args.no_color,
            ),
        };

        println!("{indent}{diff_str}");
    }
}

fn read_input(file_path: &PathBuf) -> Result<String> {
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

fn handle_stdin_input(
    args: &Args,
    input1: &PathBuf,
    input2: &PathBuf,
    input1_is_stdin: bool,
    input2_is_stdin: bool,
) -> Result<()> {
    if input1_is_stdin && input2_is_stdin {
        // Case 2 & 3: Both inputs from stdin - read two data sets from stdin
        return handle_both_stdin(args);
    }

    // Case 1: One stdin, one file
    let content1 = read_input(input1)?;
    let content2 = read_input(input2)?;

    // Determine input format
    let input_format = if let Some(fmt) = args.format {
        fmt
    } else {
        infer_format_from_path(input1)
            .or_else(|| infer_format_from_path(input2))
            .context("Could not infer format from file extensions. Please specify --format.")?
    };

    // Parse content
    let v1: Value = parse_content(&content1, input_format)?;
    let v2: Value = parse_content(&content2, input_format)?;

    // Build options and perform diff
    let options = build_diff_options_for_values(args)?;
    let differences = diff(&v1, &v2, Some(&options))?;

    // Handle output
    handle_output_and_exit(&differences, args, Some(&options))
}

fn handle_both_stdin(args: &Args) -> Result<()> {
    // Read entire stdin
    let mut buffer = String::new();
    io::stdin()
        .read_to_string(&mut buffer)
        .context("Failed to read from stdin")?;

    // Try to parse as two separate JSON/YAML objects
    if let Some(fmt) = args.format {
        match fmt {
            Format::Json => handle_both_stdin_json(&buffer, args),
            Format::Yaml => handle_both_stdin_yaml(&buffer, args),
            _ => Err(anyhow::anyhow!(
                "Two stdin inputs only supported for JSON and YAML formats"
            )),
        }
    } else {
        // Try JSON first, then YAML
        handle_both_stdin_json(&buffer, args).or_else(|_| handle_both_stdin_yaml(&buffer, args))
    }
}

fn handle_both_stdin_json(buffer: &str, args: &Args) -> Result<()> {
    // Try to parse as JSON Lines (two separate JSON objects)
    let lines: Vec<&str> = buffer.trim().lines().collect();

    if lines.len() >= 2 {
        // Try to parse first and last non-empty lines as JSON
        let first_json = lines
            .iter()
            .find(|line| !line.trim().is_empty())
            .ok_or_else(|| anyhow::anyhow!("No JSON content found in stdin"))?;
        let second_json = lines
            .iter()
            .rev()
            .find(|line| !line.trim().is_empty())
            .ok_or_else(|| anyhow::anyhow!("Only one JSON object found in stdin"))?;

        if first_json != second_json {
            let v1: Value = serde_json::from_str(first_json)?;
            let v2: Value = serde_json::from_str(second_json)?;

            let options = build_diff_options_for_values(args)?;
            let differences = diff(&v1, &v2, Some(&options))?;

            return handle_output_and_exit(&differences, args, Some(&options));
        }
    }

    // Try to parse as two concatenated JSON objects
    let trimmed = buffer.trim();
    if let Some(end_of_first) = find_json_object_end(trimmed) {
        let first_part = &trimmed[..end_of_first];
        let second_part = trimmed[end_of_first..].trim();

        if !second_part.is_empty() {
            let v1: Value = serde_json::from_str(first_part)?;
            let v2: Value = serde_json::from_str(second_part)?;

            let options = build_diff_options_for_values(args)?;
            let differences = diff(&v1, &v2, Some(&options))?;

            return handle_output_and_exit(&differences, args, Some(&options));
        }
    }

    Err(anyhow::anyhow!(
        "Could not parse two JSON objects from stdin"
    ))
}

fn handle_both_stdin_yaml(buffer: &str, args: &Args) -> Result<()> {
    // Try to parse as two YAML documents separated by ---
    let documents: Vec<&str> = buffer.split("---").collect();

    if documents.len() >= 2 {
        let doc1 = documents[0].trim();
        let doc2 = documents[1].trim();

        if !doc1.is_empty() && !doc2.is_empty() {
            let v1: Value = serde_yml::from_str(doc1)?;
            let v2: Value = serde_yml::from_str(doc2)?;

            let options = build_diff_options_for_values(args)?;
            let differences = diff(&v1, &v2, Some(&options))?;

            return handle_output_and_exit(&differences, args, Some(&options));
        }
    }

    Err(anyhow::anyhow!(
        "Could not parse two YAML documents from stdin (expected '---' separator)"
    ))
}

fn find_json_object_end(json_str: &str) -> Option<usize> {
    let mut brace_count = 0;
    let mut in_string = false;
    let mut escape_next = false;

    for (i, ch) in json_str.char_indices() {
        if escape_next {
            escape_next = false;
            continue;
        }

        match ch {
            '"' if !escape_next => in_string = !in_string,
            '\\' if in_string => escape_next = true,
            '{' if !in_string => brace_count += 1,
            '}' if !in_string => {
                brace_count -= 1;
                if brace_count == 0 {
                    return Some(i + 1);
                }
            }
            _ => {}
        }
    }

    None
}

fn build_diff_options_for_values(args: &Args) -> Result<DiffOptions> {
    let ignore_keys_regex = if let Some(pattern) = &args.ignore_keys_regex {
        Some(Regex::new(pattern)?)
    } else {
        None
    };

    let diffx_options = Some(DiffxSpecificOptions {
        ignore_whitespace: Some(args.ignore_whitespace),
        ignore_case: Some(args.ignore_case),
        brief_mode: Some(args.brief),
        quiet_mode: Some(args.quiet),
    });

    let output_format = if let Some(format_str) = &args.output {
        Some(OutputFormat::parse_format(format_str)?)
    } else {
        None
    };

    Ok(DiffOptions {
        epsilon: args.epsilon,
        array_id_key: args.array_id_key.clone(),
        ignore_keys_regex,
        path_filter: args.path.clone(),
        output_format,
        show_unchanged: Some(args.show_unchanged),
        show_types: Some(args.show_types),
        use_memory_optimization: Some(args.memory_optimization),
        batch_size: args.batch_size,
        diffx_options,
    })
}

fn handle_output_and_exit(
    differences: &[DiffResult],
    args: &Args,
    options: Option<&DiffOptions>,
) -> Result<()> {
    // Handle quiet mode
    if args.quiet {
        std::process::exit(if differences.is_empty() { 0 } else { 1 });
    }

    // Handle brief mode
    if args.brief {
        if differences.is_empty() {
            // Brief mode: no output when files are identical
        } else {
            println!("Inputs differ");
        }
        std::process::exit(if differences.is_empty() { 0 } else { 1 });
    }

    // Format and output results
    let output_format = if let Some(format_str) = &args.output {
        OutputFormat::parse_format(format_str)?
    } else {
        OutputFormat::Diffx
    };

    let has_differences = !differences.is_empty();

    match output_format {
        OutputFormat::Diffx => {
            print_cli_output(differences.to_vec(), args);
        }
        _ => {
            let formatted_output = format_diff_output(differences, output_format, options)?;
            if !formatted_output.trim().is_empty() {
                println!("{formatted_output}");
            }
        }
    }

    // Print verbose summary information to stderr
    if args.verbose {
        eprintln!("Total differences found: {}", differences.len());
    }

    // Exit with appropriate code (0 = no differences, 1 = differences found)
    std::process::exit(if has_differences { 1 } else { 0 });
}

#[cfg(test)]
mod tests {
    use super::*;
    use diffx_core::diff;
    use serde_json::json;

    #[test]
    fn test_basic_diff() {
        let old = json!({"a": 1, "b": 2});
        let new = json!({"a": 1, "b": 3});

        let results = diff(&old, &new, None).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_with_epsilon() {
        let old = json!({"value": 1.0});
        let new = json!({"value": 1.001});

        let options = DiffOptions {
            epsilon: Some(0.01),
            ..Default::default()
        };

        let results = diff(&old, &new, Some(&options)).unwrap();
        assert_eq!(results.len(), 0); // Should be within epsilon tolerance
    }

    #[test]
    fn test_array_with_id() {
        let old = json!([{"id": "1", "name": "Alice"}, {"id": "2", "name": "Bob"}]);
        let new = json!([{"id": "1", "name": "Alice"}, {"id": "2", "name": "Bobby"}]);

        let options = DiffOptions {
            array_id_key: Some("id".to_string()),
            ..Default::default()
        };

        let results = diff(&old, &new, Some(&options)).unwrap();
        assert_eq!(results.len(), 1);
    }
}

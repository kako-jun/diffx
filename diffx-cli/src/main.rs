use anyhow::Result;
use clap::{Parser, ValueEnum};
use diffx_core::{
    diff_paths, DiffOptions, DiffxSpecificOptions, OutputFormat, format_output
};
use regex::Regex;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "diffx")]
#[command(about = "A unified diff tool for structured data")]
#[command(version)]
struct Args {
    /// The first input file
    #[arg(value_name = "FILE1")]
    input1: PathBuf,

    /// The second input file  
    #[arg(value_name = "FILE2")]
    input2: PathBuf,

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

    /// Number of context lines for diff output
    #[arg(long)]
    context: Option<usize>,

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

fn main() -> Result<()> {
    let args = Args::parse();

    // Build options from CLI arguments
    let options = build_diff_options(&args)?;

    // Perform diff using paths (automatic file/directory detection)
    let results = diff_paths(
        &args.input1.to_string_lossy(),
        &args.input2.to_string_lossy(),
        Some(&options)
    )?;

    // Handle quiet mode
    if args.quiet {
        std::process::exit(if results.is_empty() { 0 } else { 1 });
    }

    // Handle brief mode
    if args.brief {
        if results.is_empty() {
            if args.verbose {
                println!("Files {} and {} are identical", 
                    args.input1.display(), args.input2.display());
            }
        } else {
            println!("Files {} and {} differ", 
                args.input1.display(), args.input2.display());
        }
        std::process::exit(if results.is_empty() { 0 } else { 1 });
    }

    // Format and output results
    let output_format = if let Some(format_str) = &args.output {
        OutputFormat::from_str(format_str)?
    } else {
        OutputFormat::Diffx
    };
    let formatted_output = format_output(&results, output_format)?;

    if !formatted_output.trim().is_empty() {
        println!("{}", formatted_output);
    } else if args.verbose {
        println!("No differences found");
    }

    // Exit with appropriate code (0 = no differences, 1 = differences found)
    std::process::exit(if results.is_empty() { 0 } else { 1 });
}

// File format and parsing functions are now handled by diffx-core

fn build_diff_options(args: &Args) -> Result<DiffOptions> {
    let ignore_keys_regex = if let Some(pattern) = &args.ignore_keys_regex {
        Some(Regex::new(pattern)?)
    } else {
        None
    };

    let diffx_options = Some(DiffxSpecificOptions {
        context_lines: args.context,
        ignore_whitespace: Some(args.ignore_whitespace),
        ignore_case: Some(args.ignore_case),
        brief_mode: Some(args.brief),
        quiet_mode: Some(args.quiet),
    });

    let output_format = if let Some(format_str) = &args.output {
        Some(OutputFormat::from_str(format_str)?)
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

#[cfg(test)]
mod tests {
    use super::*;
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
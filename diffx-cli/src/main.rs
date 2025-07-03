use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use colored::*;
use diffx_core::{diff, value_type_name, DiffResult};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Read};
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
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Cli)]
    output: OutputFormat,

    /// Compare directories recursively
    #[arg(short, long)]
    recursive: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum OutputFormat {
    Cli,
    Json,
    Yaml,
    Toml,
    Unified,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Json,
    Yaml,
    Toml,
}

fn infer_format_from_path(path: &Path) -> Option<Format> {
    if path.to_str() == Some("-") {
        // Cannot infer format from stdin, user must specify --format
        None
    } else {
        path.extension()
            .and_then(|ext| ext.to_str())
            .and_then(|ext_str| {
                match ext_str.to_lowercase().as_str() {
                    "json" => Some(Format::Json),
                    "yaml" | "yml" => Some(Format::Yaml),
                    "toml" => Some(Format::Toml),
                    _ => None,
                }
            })
    }
}

fn read_input(file_path: &Path) -> Result<String> {
    if file_path.to_str() == Some("-") {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).context("Failed to read from stdin")?;
        Ok(buffer)
    } else {
        fs::read_to_string(file_path).context(format!("Failed to read file: {}", file_path.display()))
    }
}

fn parse_content(content: &str, format: Format) -> Result<Value> {
    match format {
        Format::Json => serde_json::from_str(content).context("Failed to parse JSON"),
        Format::Yaml => serde_yaml::from_str(content).context("Failed to parse YAML"),
        Format::Toml => toml::from_str(content).context("Failed to parse TOML"),
    }
}

fn print_cli_output(differences: Vec<DiffResult>) {
    if differences.is_empty() {
        println!("No differences found.");
    } else {
        for diff in differences {
            match diff {
                DiffResult::Added(key, value) => {
                    println!("{}", format!("+ {}: {}", key, value).blue());
                }
                DiffResult::Removed(key, value) => {
                    println!("{}", format!("- {}: {}", key, value).yellow());
                }
                DiffResult::Modified(key, value1, value2) => {
                    println!("{}", format!("~ {}: {} -> {}", key, value1, value2).cyan());
                }
                DiffResult::TypeChanged(key, value1, value2) => {
                    println!("{}", format!("! {}: {} ({}) -> {} ({})", key, value1, value_type_name(&value1), value2, value_type_name(&value2)).magenta());
                }
            }
        }
    }
}

fn print_json_output(differences: Vec<DiffResult>) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(&differences)?);
    Ok(())
}

fn print_yaml_output(differences: Vec<DiffResult>) -> Result<()> {
    println!("{}", serde_yaml::to_string(&differences)?);
    Ok(())
}

fn print_unified_output(v1: &Value, v2: &Value) -> Result<()> {
    let content1_pretty = serde_json::to_string_pretty(v1)?;
    let content2_pretty = serde_json::to_string_pretty(v2)?;

    let diff = similar::TextDiff::from_lines(&content1_pretty, &content2_pretty);

    for change in diff.iter_all_changes() {
        let sign = match change.tag() {
            similar::ChangeTag::Delete => "-",
            similar::ChangeTag::Insert => "+",
            similar::ChangeTag::Equal => " ",
        };
        print!("{}{}", sign, change);
    }
    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Handle directory comparison
    if args.recursive {
        if !args.input1.is_dir() || !args.input2.is_dir() {
            bail!("Both inputs must be directories for recursive comparison.");
        }
        compare_directories(&args.input1, &args.input2, args.format, args.output)?;
        return Ok(());
    }

    // Handle single file/stdin comparison
    let content1 = read_input(&args.input1)?;
    let content2 = read_input(&args.input2)?;

    let input_format = if let Some(fmt) = args.format {
        fmt
    } else {
        infer_format_from_path(&args.input1)
            .or_else(|| infer_format_from_path(&args.input2))
            .context("Could not infer format from file extensions. Please specify --format.")?
    };

    let v1: Value = parse_content(&content1, input_format)?;
    let v2: Value = parse_content(&content2, input_format)?;

    let differences = diff(&v1, &v2);

    match args.output {
        OutputFormat::Cli => print_cli_output(differences),
        OutputFormat::Json => print_json_output(differences)?,
        OutputFormat::Yaml => print_yaml_output(differences)?,
        OutputFormat::Toml => {
            // TOML output is not directly supported for DiffResult due to TOML's strict type system.
            // It's kept here for completeness of OutputFormat enum, but will result in an error.
            eprintln!("Error: TOML output is not supported for structured diff results.");
            bail!("TOML output not supported");
        }
        OutputFormat::Unified => print_unified_output(&v1, &v2)?,
    }

    Ok(())
}

fn compare_directories(
    dir1: &Path,
    dir2: &Path,
    format: Option<Format>,
    output: OutputFormat,
) -> Result<()> {
    let mut files1: HashMap<PathBuf, PathBuf> = HashMap::new();
    for entry in WalkDir::new(dir1).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(dir1)?.to_path_buf();
            files1.insert(relative_path, path.to_path_buf());
        }
    }

    let mut files2: HashMap<PathBuf, PathBuf> = HashMap::new();
    for entry in WalkDir::new(dir2).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let relative_path = path.strip_prefix(dir2)?.to_path_buf();
            files2.insert(relative_path, path.to_path_buf());
        }
    }

    let mut all_relative_paths: std::collections::HashSet<PathBuf> = files1.keys().cloned().collect();
    all_relative_paths.extend(files2.keys().cloned());

    let mut compared_files = 0;

    for relative_path in &all_relative_paths {
        let path1_option = files1.get(relative_path.as_path());
        let path2_option = files2.get(relative_path.as_path());

        match (path1_option, path2_option) {
            (Some(path1), Some(path2)) => {
                println!("
--- Comparing {} ---", relative_path.display());
                let content1 = read_input(path1)?;
                let content2 = read_input(path2)?;

                let input_format = if let Some(fmt) = format {
                    fmt
                } else {
                    infer_format_from_path(path1)
                        .or_else(|| infer_format_from_path(path2))
                        .context(format!("Could not infer format for {}. Please specify --format.", relative_path.display()))?
                };

                let v1: Value = parse_content(&content1, input_format)?;
                let v2: Value = parse_content(&content2, input_format)?;

                let differences = diff(&v1, &v2);

                match output {
                    OutputFormat::Cli => print_cli_output(differences),
                    OutputFormat::Json => print_json_output(differences)?,
                    OutputFormat::Yaml => print_yaml_output(differences)?,
                    OutputFormat::Toml => {
                        eprintln!("Error: TOML output is not supported for structured diff results.");
                        bail!("TOML output not supported");
                    }
                    OutputFormat::Unified => print_unified_output(&v1, &v2)?,
                }
                compared_files += 1;
            },
            (Some(_), None) => {
                println!("
--- Only in {}: {} ---", dir1.display(), relative_path.display());
            },
            (None, Some(_)) => {
                println!("
--- Only in {}: {} ---", dir2.display(), relative_path.display());
            },
            (None, None) => { /* Should not happen */ }
        }
    }

    if compared_files == 0 && all_relative_paths.is_empty() {
        println!("No comparable files found in directories.");
    }

    Ok(())
}
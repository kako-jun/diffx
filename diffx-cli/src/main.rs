use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use colored::*;
use diffx_core::{diff, value_type_name, DiffResult};
use serde_json::Value;
use std::fs;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The first file to compare
    #[arg(value_name = "FILE1")]
    file1: String,

    /// The second file to compare
    #[arg(value_name = "FILE2")]
    file2: String,

    /// Input file format
    #[arg(short, long, value_enum)]
    format: Option<Format>,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Cli)]
    output: OutputFormat,
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

fn infer_format_from_path(path: &str) -> Option<Format> {
    Path::new(path)
        .extension()
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

    let content1 = fs::read_to_string(&args.file1)?;
    let content2 = fs::read_to_string(&args.file2)?;

    let input_format = if let Some(fmt) = args.format {
        fmt
    } else {
        infer_format_from_path(&args.file1)
            .or_else(|| infer_format_from_path(&args.file2))
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
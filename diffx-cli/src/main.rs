use anyhow::{Context, Result, bail};
use clap::{Parser, ValueEnum};
use colored::*;
use diffx_core::{diff, value_type_name, DiffResult, parse_ini, parse_xml, parse_csv};
use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::io::{self, Read};
use walkdir::WalkDir;
use regex::Regex;

#[derive(Debug, Deserialize, Default)]
struct Config {
    #[serde(default)]
    output: Option<OutputFormat>,
    #[serde(default)]
    format: Option<Format>,
}

fn load_config() -> Config {
    let config_path = dirs::config_dir()
        .map(|p| p.join("diffx").join("config.toml"))
        .or_else(|| {
            // Fallback for systems without a standard config directory
            Some(PathBuf::from(".diffx.toml"))
        });

    if let Some(path) = config_path {
        if path.exists() {
            match fs::read_to_string(&path) {
                Ok(content) => {
                    match toml::from_str(&content) {
                        Ok(config) => return config,
                        Err(e) => eprintln!("Warning: Could not parse config file {}: {}", path.display(), e),
                    }
                }
                Err(e) => eprintln!("Warning: Could not read config file {}: {}", path.display(), e),
            }
        }
    }
    Config::default()
}


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

    /// Filter differences by a specific path (e.g., "config.users[0].name")
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
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug, Serialize, Deserialize)]
enum OutputFormat {
    Cli,
    Json,
    Yaml,
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
            .and_then(|ext_str| {
                match ext_str.to_lowercase().as_str() {
                    "json" => Some(Format::Json),
                    "yaml" | "yml" => Some(Format::Yaml),
                    "toml" => Some(Format::Toml),
                    "ini" => Some(Format::Ini),
                    "xml" => Some(Format::Xml),
                    "csv" => Some(Format::Csv),
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
        Format::Ini => parse_ini(content).context("Failed to parse INI"),
        Format::Xml => parse_xml(content).context("Failed to parse XML"),
        Format::Csv => parse_csv(content).context("Failed to parse CSV"),
    }
}

fn print_cli_output(mut differences: Vec<DiffResult>, _v1: &Value, _v2: &Value) {
    if differences.is_empty() {
        println!("No differences found.");
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

    differences.sort_by(|a, b| get_key(a).cmp(&get_key(b)));

    for diff in &differences {
        let key = get_key(diff);
        // Indent based on the depth of the key
        let depth = key.chars().filter(|&c| c == '.' || c == '[').count();
        let indent = "  ".repeat(depth);

        let diff_str = match diff {
            DiffResult::Added(k, value) => format!("+ {}: {}", k, value).blue(),
            DiffResult::Removed(k, value) => format!("- {}: {}", k, value).yellow(),
            DiffResult::Modified(k, v1, v2) => format!("~ {}: {} -> {}", k, v1, v2).cyan(),
            DiffResult::TypeChanged(k, v1, v2) => {
                format!("! {}: {} ({}) -> {} ({})", k, v1, value_type_name(v1), v2, value_type_name(v2))
                    .magenta()
            }
        };

        println!("{}{}", indent, diff_str);
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
    let config = load_config();

    let output_format = args.output.or(config.output).unwrap_or(OutputFormat::Cli);
    let input_format_from_config = config.format;

    let ignore_keys_regex = if let Some(regex_str) = &args.ignore_keys_regex {
        Some(Regex::new(regex_str).context("Invalid regex for --ignore-keys-regex")?)
    } else {
        None
    };

    let epsilon = args.epsilon;
    let array_id_key = args.array_id_key.as_deref();

    // Handle directory comparison
    if args.recursive {
        if !args.input1.is_dir() || !args.input2.is_dir() {
            bail!("Both inputs must be directories for recursive comparison.");
        }
        compare_directories(&args.input1, &args.input2, args.format.or(input_format_from_config), output_format, args.path, ignore_keys_regex.as_ref(), epsilon, array_id_key)?;
        return Ok(());
    }

    // Handle single file/stdin comparison
    let content1 = read_input(&args.input1)?;
    let content2 = read_input(&args.input2)?;

    let input_format = if let Some(fmt) = args.format {
        fmt
    } else if let Some(fmt) = input_format_from_config {
        fmt
    } else {
        infer_format_from_path(&args.input1)
            .or_else(|| infer_format_from_path(&args.input2))
            .context("Could not infer format from file extensions. Please specify --format or configure in diffx.toml.")?
    };

    let v1: Value = parse_content(&content1, input_format)?;
    let v2: Value = parse_content(&content2, input_format)?;

    let mut differences = diff(&v1, &v2, ignore_keys_regex.as_ref(), epsilon, array_id_key);

    if let Some(filter_path) = args.path {
        differences.retain(|d| {
            let key = match d {
                DiffResult::Added(k, _) => k,
                DiffResult::Removed(k, _) => k,
                DiffResult::Modified(k, _, _) => k,
                DiffResult::TypeChanged(k, _, _) => k,
            };
            key.starts_with(&filter_path)
        });
    }

    match output_format {
        OutputFormat::Cli => print_cli_output(differences, &v1, &v2),
        OutputFormat::Json => print_json_output(differences)?,
        OutputFormat::Yaml => print_yaml_output(differences)?,
        OutputFormat::Unified => print_unified_output(&v1, &v2)?,
    }

    Ok(())
}

fn compare_directories(
    dir1: &Path,
    dir2: &Path,
    format_option: Option<Format>,
    output: OutputFormat,
    filter_path: Option<String>,
    ignore_keys_regex: Option<&Regex>,
    epsilon: Option<f64>,
    array_id_key: Option<&str>,
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

                let input_format = if let Some(fmt) = format_option {
                    fmt
                } else {
                    infer_format_from_path(path1)
                        .or_else(|| infer_format_from_path(path2))
                        .context(format!("Could not infer format for {}. Please specify --format or configure in diffx.toml.", relative_path.display()))?
                };

                let v1: Value = parse_content(&content1, input_format)?;
                let v2: Value = parse_content(&content2, input_format)?;

                let mut differences = diff(&v1, &v2, ignore_keys_regex, epsilon, array_id_key);

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

                match output {
                    OutputFormat::Cli => print_cli_output(differences, &v1, &v2),
                    OutputFormat::Json => print_json_output(differences)?,
                    OutputFormat::Yaml => print_yaml_output(differences)?,
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
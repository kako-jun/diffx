use anyhow::Result;
use serde_json::Value;
use std::path::Path;

use super::{parse_csv, parse_ini, parse_json, parse_toml, parse_xml, parse_yaml};

#[derive(Debug, Clone, Copy)]
pub enum FileFormat {
    Json,
    Yaml,
    Csv,
    Toml,
    Ini,
    Xml,
}

pub fn detect_format_from_path(path: &Path) -> FileFormat {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("json") => FileFormat::Json,
        Some("yaml") | Some("yml") => FileFormat::Yaml,
        Some("csv") => FileFormat::Csv,
        Some("toml") => FileFormat::Toml,
        Some("ini") | Some("cfg") => FileFormat::Ini,
        Some("xml") => FileFormat::Xml,
        _ => FileFormat::Json, // Default fallback
    }
}

pub fn parse_content_by_format(content: &str, format: FileFormat) -> Result<Value> {
    match format {
        FileFormat::Json => parse_json(content),
        FileFormat::Yaml => parse_yaml(content),
        FileFormat::Csv => parse_csv(content),
        FileFormat::Toml => parse_toml(content),
        FileFormat::Ini => parse_ini(content),
        FileFormat::Xml => parse_xml(content),
    }
}

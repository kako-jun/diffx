use anyhow::Result;
use csv::ReaderBuilder;
use serde_json::Value;

pub fn parse_csv(content: &str) -> Result<Value> {
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_reader(content.as_bytes());

    let headers = reader.headers()?.clone();
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        let mut map = serde_json::Map::new();

        for (i, field) in record.iter().enumerate() {
            if let Some(header) = headers.get(i) {
                map.insert(header.to_string(), Value::String(field.to_string()));
            }
        }

        records.push(Value::Object(map));
    }

    Ok(Value::Array(records))
}

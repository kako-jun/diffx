use quick_xml::de::from_str;
use serde_json::Value;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let content1 = fs::read_to_string("large_test1.xml")?;
    let content2 = fs::read_to_string("large_test2.xml")?;
    
    println!("Content 1:\n{}", content1);
    println!("\nContent 2:\n{}", content2);
    
    let value1: Value = from_str(&content1)?;
    let value2: Value = from_str(&content2)?;
    
    println!("\nParsed Value 1:\n{}", serde_json::to_string_pretty(&value1)?);
    println!("\nParsed Value 2:\n{}", serde_json::to_string_pretty(&value2)?);
    
    println!("\nAre they equal? {}", value1 == value2);
    
    Ok(())
}
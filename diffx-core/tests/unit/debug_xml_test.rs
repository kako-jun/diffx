use diffx_core::parse_xml;
use serde_json::Value;

#[test]
fn debug_xml_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let content1 = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <item id="1">value1</item>
    <item id="2">value2</item>
    <item id="5000">50000</item>
    <item id="9999">value9999</item>
</root>"#;
    
    let content2 = r#"<?xml version="1.0" encoding="UTF-8"?>
<root>
    <item id="1">value1</item>
    <item id="2">value2</item>
    <item id="5000">50001</item>
    <item id="9999">value9999</item>
</root>"#;
    
    println!("Content 1:\n{}", content1);
    println!("\nContent 2:\n{}", content2);
    
    let value1: Value = parse_xml(content1)?;
    let value2: Value = parse_xml(content2)?;
    
    println!("\nParsed Value 1:\n{}", serde_json::to_string_pretty(&value1)?);
    println!("\nParsed Value 2:\n{}", serde_json::to_string_pretty(&value2)?);
    
    println!("\nAre they equal? {}", value1 == value2);
    
    Ok(())
}
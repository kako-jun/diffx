use anyhow::{anyhow, Result};
use quick_xml::events::Event;
use quick_xml::Reader;
use serde_json::Value;

pub fn parse_xml(content: &str) -> Result<Value> {
    let mut reader = Reader::from_str(content);
    reader.trim_text(true);

    // Stack-based parsing for nested structures
    let mut stack: Vec<(String, serde_json::Map<String, Value>)> = Vec::new();
    let mut root: Option<(String, serde_json::Map<String, Value>)> = None;
    let mut current_text = String::new();

    loop {
        match reader.read_event() {
            Ok(Event::Start(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut element = serde_json::Map::new();

                // Parse attributes
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let value = String::from_utf8_lossy(&attr.value).to_string();
                    element.insert(key, Value::String(value));
                }

                // If we have text content to add to parent
                if !current_text.trim().is_empty() && !stack.is_empty() {
                    let (_, parent) = stack.last_mut().unwrap();
                    parent.insert(
                        "text".to_string(),
                        Value::String(current_text.trim().to_string()),
                    );
                }
                current_text.clear();

                // Push new element to stack
                stack.push((tag_name, element));
            }
            Ok(Event::Text(e)) => {
                let text = e.unescape().unwrap_or_default().to_string();
                if !text.trim().is_empty() {
                    current_text.push_str(&text);
                }
            }
            Ok(Event::CData(e)) => {
                // Handle CDATA sections
                let cdata_text = String::from_utf8_lossy(&e).to_string();
                current_text.push_str(&cdata_text);
            }
            Ok(Event::End(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();

                if let Some((name, mut element)) = stack.pop() {
                    if name == tag_name {
                        // Add any remaining text content
                        if !current_text.trim().is_empty() {
                            // If element only has text (no attributes or children), make it a simple string
                            if element.is_empty() {
                                let text_value = Value::String(current_text.trim().to_string());
                                current_text.clear();

                                if let Some((_, parent)) = stack.last_mut() {
                                    // Add to parent
                                    add_to_parent(parent, &name, text_value);
                                } else {
                                    // This is the root element
                                    root = Some((
                                        name.clone(),
                                        serde_json::Map::from_iter(vec![(name, text_value)]),
                                    ));
                                }
                                continue;
                            } else {
                                element.insert(
                                    "text".to_string(),
                                    Value::String(current_text.trim().to_string()),
                                );
                            }
                        }
                        current_text.clear();

                        // Convert element to Value
                        let element_value = if element.is_empty() {
                            Value::Object(serde_json::Map::new())
                        } else if element.len() == 1 && element.contains_key("text") {
                            element.get("text").unwrap().clone()
                        } else {
                            Value::Object(element)
                        };

                        if let Some((_, parent)) = stack.last_mut() {
                            // Add to parent
                            add_to_parent(parent, &name, element_value);
                        } else {
                            // This is the root element
                            let mut root_map = serde_json::Map::new();
                            root_map.insert(name.clone(), element_value);
                            root = Some((name.clone(), root_map));
                        }
                    }
                }
            }
            Ok(Event::Empty(ref e)) => {
                let tag_name = String::from_utf8_lossy(e.name().as_ref()).to_string();
                let mut element = serde_json::Map::new();

                // Parse attributes
                for attr in e.attributes().flatten() {
                    let key = String::from_utf8_lossy(attr.key.as_ref()).to_string();
                    let value = String::from_utf8_lossy(&attr.value).to_string();
                    element.insert(key, Value::String(value));
                }

                let element_value = Value::Object(element);

                if let Some((_, parent)) = stack.last_mut() {
                    // Add to parent
                    add_to_parent(parent, &tag_name, element_value);
                } else {
                    // This is a root-level empty element
                    let mut root_map = serde_json::Map::new();
                    root_map.insert(tag_name.clone(), element_value);
                    root = Some((tag_name.clone(), root_map));
                }
            }
            Ok(Event::Eof) => break,
            Err(e) => return Err(anyhow!("XML parsing error: {e}")),
            _ => {}
        }
    }

    // Return the root element
    if let Some((_, root_map)) = root {
        Ok(Value::Object(root_map))
    } else {
        Ok(Value::Object(serde_json::Map::new()))
    }
}

// Helper function to add a child element to a parent
fn add_to_parent(parent: &mut serde_json::Map<String, Value>, key: &str, value: Value) {
    if let Some(existing) = parent.get_mut(key) {
        match existing {
            Value::Array(arr) => {
                arr.push(value);
            }
            other => {
                let _ = std::mem::replace(other, Value::Array(vec![other.clone(), value]));
            }
        }
    } else {
        parent.insert(key.to_string(), value);
    }
}

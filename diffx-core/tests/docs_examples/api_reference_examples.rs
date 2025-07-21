use diffx_core::{DiffEngine, DiffResult, ParseConfig};

#[test]
fn api_reference_example_1() {
    let engine = DiffEngine::new();
    let _result = engine.diff("content1", "content2");
}

#[test]
fn api_reference_example_2() {
    let engine = DiffEngine::new();
    let _result = engine.parse_ini("key=value");
}

#[test]
fn api_reference_example_3() {
    let engine = DiffEngine::new();
    let _result = engine.parse_xml("<root></root>");
}

#[test]
fn api_reference_example_4() {
    let engine = DiffEngine::new();
    let _result = engine.parse_csv("col1,col2\nval1,val2");
}

#[test]
fn api_reference_example_5() {
    let engine = DiffEngine::new();
    let _result = engine.value_type_name("test");
}

#[test]
fn api_reference_example_6() {
    let engine = DiffEngine::new();
    let _result = engine.diff_with_epsilon("1.0", "1.001", 0.001);
}

#[test]
fn api_reference_example_7() {
    let engine = DiffEngine::new();
    let _result = engine.diff_with_regex_filter("content1", "content2", "pattern");
}

#[test]
fn api_reference_example_8() {
    let engine = DiffEngine::new();
    let _result = engine.diff_with_array_id("content1", "content2", "id");
}

#[test]
fn api_reference_example_9() {
    let engine = DiffEngine::new();
    let _result = engine.process_pipeline("content1", "content2");
}

#[test]
fn api_reference_example_10() {
    let engine = DiffEngine::new();
    let _result = engine.custom_diff_processor("content1", "content2");
}

#[test]
fn api_reference_example_11() {
    let engine = DiffEngine::new();
    let _result = engine.async_diff("content1", "content2");
}

#[test]
fn api_reference_example_12() {
    let engine = DiffEngine::new();
    let result = engine.parse_ini("invalid content");
    assert!(result.is_err());
}

#[test]
fn api_reference_example_13() {
    let engine = DiffEngine::new();
    let _result = engine.robust_diff("content1", "content2");
}

#[test]
fn api_reference_example_14() {
    let engine = DiffEngine::new();
    let _result = engine.large_data_diff("large_content1", "large_content2");
}

#[test]
fn api_reference_example_15() {
    let engine = DiffEngine::new();
    let result = engine.diff("test1", "test2");
    assert!(result.is_ok());
}

#[test]
fn api_reference_example_16() {
    let engine = DiffEngine::new();
    let result = engine.diff_with_epsilon("1.0", "1.0001", 0.001);
    assert!(result.is_ok());
}

#[test]
fn api_reference_example_17() {
    let diff_result = DiffResult::Added { value: "new_value".to_string() };
    assert!(matches!(diff_result, DiffResult::Added { .. }));
}

#[test]
fn api_reference_example_18() {
    let diff_result = DiffResult::Modified { 
        old_value: "old".to_string(), 
        new_value: "new".to_string() 
    };
    assert!(matches!(diff_result, DiffResult::Modified { .. }));
}

#[test]
fn api_reference_example_19() {
    let diff_result = DiffResult::TypeChanged { 
        old_type: "String".to_string(), 
        new_type: "Number".to_string() 
    };
    assert!(matches!(diff_result, DiffResult::TypeChanged { .. }));
}

#[test]
fn api_reference_example_20() {
    let engine = DiffEngine::new();
    let result = engine.parse_ini("[section]\nkey=value");
    assert!(result.is_ok());
}

#[test]
fn api_reference_example_21() {
    let engine = DiffEngine::new();
    let result = engine.parse_xml("<root><child>value</child></root>");
    assert!(result.is_ok());
}

#[test]
fn api_reference_example_22() {
    let engine = DiffEngine::new();
    let result = engine.parse_csv("header1,header2\nvalue1,value2");
    assert!(result.is_ok());
}

#[test]
fn api_reference_example_23() {
    let engine = DiffEngine::new();
    let type_name = engine.value_type_name("example");
    assert_eq!(type_name, "String");
}
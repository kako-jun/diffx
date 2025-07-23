use diffx_core::{DiffEngine, DiffConfig};

#[test]
fn comparison_example_1() {
    let engine = DiffEngine::new();
    let _result = engine.diff("config_v1_content", "config_v2_content");
}

#[test]
fn comparison_example_2() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("file1_content", "file2_content", &config);
}

#[test]
fn comparison_example_3() {
    let engine = DiffEngine::new();
    let _result = engine.diff("file1_yaml_content", "file2_yaml_content");
}

#[test]
fn comparison_example_4() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("id".to_string());
    let _result = engine.diff_with_config("data1_csv_content", "data2_csv_content", &config);
}

#[test]
fn comparison_example_5() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("file1_json_content", "file2_json_content", &config);
}

#[test]
fn comparison_example_6() {
    let engine = DiffEngine::new();
    let _result = engine.diff("stdin_content", "config_content");
}

#[test]
fn comparison_example_7() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "unified".to_string();
    let _result = engine.diff_with_config("config1_content", "config2_content", &config);
}

#[test]
fn comparison_example_8() {
    let engine = DiffEngine::new();
    let _result = engine.diff("config1_content", "config2_content");
}

#[test]
fn comparison_example_9() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("file1_json_content", "file2_json_content", &config);
}
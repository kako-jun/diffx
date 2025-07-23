use diffx_core::{DiffEngine};

#[test]
fn faq_example_1() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "json".to_string();
    let _result = engine.diff_with_config("stdin_content", "other_data_content", &config);
}

#[test]
fn faq_example_2() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.ignore_keys_regex = Some("^id$|^timestamp$".to_string());
    let _result = engine.diff_with_config("file1_content", "file2_content", &config);
}

#[test]
fn faq_example_3() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.epsilon = Some(0.00001);
    let _result = engine.diff_with_config("data1_content", "data2_content", &config);
}

#[test]
fn faq_example_4() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("uuid".to_string());
    let _result = engine.diff_with_config("users1_content", "users2_content", &config);
}

#[test]
fn faq_example_5() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.output_format = "json".to_string();
    let _result = engine.diff_with_config("file1_content", "file2_content", &config);
}

#[test]
fn faq_example_6() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.format = "yaml".to_string();
    let _result = engine.diff_with_config("file1_yaml_content", "file2_yaml_content", &config);
}

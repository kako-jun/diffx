use diffx_core::{DiffEngine};

#[test]
fn diffx_format_example_1() {
    let engine = DiffEngine::new();
    let _result = engine.diff("infrastructure_content", "infrastructure_new_content");
}

#[test]
fn diffx_format_example_2() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.path_filter = Some("paths".to_string());
    let _result = engine.diff_with_config("api_v1_content", "api_v2_content", &config);
}

#[test]
fn diffx_format_example_3() {
    let engine = DiffEngine::new();
    let mut config = DiffConfig::default();
    config.array_id_key = Some("id".to_string());
    let _result =
        engine.diff_with_config("expected_output_content", "actual_output_content", &config);
}

#[test]
fn diffx_format_example_4() {
    let engine = DiffEngine::new();
    let _result = engine.diff("config_content", "config_new_content");
}

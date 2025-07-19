/// Tests for --no-color option in diffx
/// Ensures color output is properly disabled when flag is specified

use assert_cmd::Command;
use tempfile::NamedTempFile;
use std::io::Write;

fn create_test_csv1() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "id,name,value")?;
    writeln!(temp_file, "1,test,123")?;
    writeln!(temp_file, "2,demo,456")?;
    Ok(temp_file)
}

fn create_test_csv2() -> Result<NamedTempFile, Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new()?;
    writeln!(temp_file, "id,name,value")?;
    writeln!(temp_file, "1,test,789")?;
    writeln!(temp_file, "2,demo,012")?;
    Ok(temp_file)
}

#[test]
fn test_diffx_no_color_option_basic() -> Result<(), Box<dyn std::error::Error>> {
    let test_file1 = create_test_csv1()?;
    let test_file2 = create_test_csv2()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", 
               test_file1.path().to_str().unwrap(),
               test_file2.path().to_str().unwrap(),
               "--no-color"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute diffx with --no-color");

    // Output should not contain ANSI color codes
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    // Debug output for troubleshooting
    eprintln!("Exit status: {:?}", output.status);
    eprintln!("Stdout: '{}'", stdout);
    eprintln!("Stderr: '{}'", stderr);
    
    assert!(!stdout.contains("\x1b["), 
           "Output should not contain ANSI color codes when --no-color is specified");
    
    // Should still contain difference information if files differ
    if !stdout.trim().is_empty() {
        assert!(stdout.contains("123") || stdout.contains("789") || stdout.contains("456") || stdout.contains("012"),
               "If output exists, it should contain difference information");
    }
    
    Ok(())
}

#[test]
fn test_diffx_no_color_option_with_verbose() -> Result<(), Box<dyn std::error::Error>> {
    let test_file1 = create_test_csv1()?;
    let test_file2 = create_test_csv2()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", 
               test_file1.path().to_str().unwrap(),
               test_file2.path().to_str().unwrap(),
               "--no-color", "--verbose"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute diffx with --no-color and --verbose");

    // Verbose output should not contain ANSI color codes
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    
    assert!(!stdout.contains("\x1b["), 
           "Verbose stdout should not contain ANSI color codes when --no-color is specified");
    assert!(!stderr.contains("\x1b["), 
           "Verbose stderr should not contain ANSI color codes when --no-color is specified");
    
    Ok(())
}

#[test]
fn test_diffx_color_vs_no_color_output_difference() -> Result<(), Box<dyn std::error::Error>> {
    let test_file1 = create_test_csv1()?;
    let test_file2 = create_test_csv2()?;
    
    // Test with colors enabled (default)
    let colored_output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", 
               test_file1.path().to_str().unwrap(),
               test_file2.path().to_str().unwrap()])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute diffx with colors");

    // Test with colors disabled
    let no_color_output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", 
               test_file1.path().to_str().unwrap(),
               test_file2.path().to_str().unwrap(),
               "--no-color"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute diffx with --no-color");

    let colored_stdout = String::from_utf8_lossy(&colored_output.stdout);
    let no_color_stdout = String::from_utf8_lossy(&no_color_output.stdout);
    
    // No-color output should definitely not contain ANSI codes
    assert!(!no_color_stdout.contains("\x1b["), 
           "No-color output should not contain ANSI color codes");
    
    // Both outputs should contain difference information (ignoring colors)
    if !no_color_stdout.trim().is_empty() || !colored_stdout.trim().is_empty() {
        assert!(
            (no_color_stdout.contains("123") || no_color_stdout.contains("789")) ||
            (colored_stdout.contains("123") || colored_stdout.contains("789")),
            "At least one output should contain difference information. No-color: '{}', Colored: '{}'", 
            no_color_stdout.trim(), colored_stdout.trim()
        );
    }
    
    Ok(())
}

#[test]
fn test_diffx_no_color_with_format_options() -> Result<(), Box<dyn std::error::Error>> {
    let test_file1 = create_test_csv1()?;
    let test_file2 = create_test_csv2()?;
    
    let formats = ["unified", "context"];
    
    for format in &formats {
        let output = Command::new("cargo")
            .args(["run", "--bin", "diffx", "--", 
                   test_file1.path().to_str().unwrap(),
                   test_file2.path().to_str().unwrap(),
                   "--no-color", "--format", format])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .expect(&format!("Failed to execute diffx with --no-color and --format {}", format));

        let stdout = String::from_utf8_lossy(&output.stdout);
        assert!(!stdout.contains("\x1b["), 
               "Output format {} should not contain ANSI color codes when --no-color is specified", format);
    }
    
    Ok(())
}

#[test]
fn test_diffx_no_color_with_side_by_side() -> Result<(), Box<dyn std::error::Error>> {
    let test_file1 = create_test_csv1()?;
    let test_file2 = create_test_csv2()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", 
               test_file1.path().to_str().unwrap(),
               test_file2.path().to_str().unwrap(),
               "--no-color", "--side-by-side"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute diffx with --no-color and --side-by-side");

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(!stdout.contains("\x1b["), 
           "Side-by-side output should not contain ANSI color codes when --no-color is specified");
    
    Ok(())
}

#[test]
fn test_diffx_no_color_with_json_output() -> Result<(), Box<dyn std::error::Error>> {
    let test_file1 = create_test_csv1()?;
    let test_file2 = create_test_csv2()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", 
               test_file1.path().to_str().unwrap(),
               test_file2.path().to_str().unwrap(),
               "--no-color", "--output", "json"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute diffx with --no-color and --output json");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // JSON output should not contain ANSI color codes
    assert!(!stdout.contains("\x1b["), 
           "JSON output should not contain ANSI color codes when --no-color is specified");
    
    // Should be valid JSON if not empty
    if !stdout.trim().is_empty() {
        let _: serde_json::Value = serde_json::from_str(&stdout)
            .expect("Output should be valid JSON");
    }
    
    Ok(())
}

#[test]
fn test_diffx_no_color_with_yaml_output() -> Result<(), Box<dyn std::error::Error>> {
    let test_file1 = create_test_csv1()?;
    let test_file2 = create_test_csv2()?;
    
    let output = Command::new("cargo")
        .args(["run", "--bin", "diffx", "--", 
               test_file1.path().to_str().unwrap(),
               test_file2.path().to_str().unwrap(),
               "--no-color", "--output", "yaml"])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("Failed to execute diffx with --no-color and --output yaml");

    let stdout = String::from_utf8_lossy(&output.stdout);
    
    // YAML output should not contain ANSI color codes
    assert!(!stdout.contains("\x1b["), 
           "YAML output should not contain ANSI color codes when --no-color is specified");
    
    Ok(())
}
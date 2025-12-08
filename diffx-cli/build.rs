use clap::{Arg, Command, ValueHint};
use clap_complete::Shell;
use clap_mangen::Man;
use std::env;
use std::fs;
use std::path::PathBuf;

fn build_cli() -> Command {
    Command::new("diffx")
        .version(env!("CARGO_PKG_VERSION"))
        .about("A diff tool for structured data")
        .arg(
            Arg::new("FILE1")
                .help("The first input file")
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("FILE2")
                .help("The second input file")
                .value_hint(ValueHint::FilePath),
        )
        .arg(
            Arg::new("completions")
                .long("completions")
                .value_name("SHELL")
                .value_parser(clap::value_parser!(Shell))
                .help("Generate shell completions for the specified shell"),
        )
        .arg(
            Arg::new("format")
                .short('f')
                .long("format")
                .value_name("FORMAT")
                .help("Input file format (auto-detected if not specified)")
                .value_parser(["json", "yaml", "csv", "toml", "ini", "xml"]),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .long("output")
                .value_name("OUTPUT")
                .help("Output format"),
        )
        .arg(
            Arg::new("path")
                .long("path")
                .value_name("PATH")
                .help("Filter by path (only show differences in paths containing this string)"),
        )
        .arg(
            Arg::new("ignore-keys-regex")
                .long("ignore-keys-regex")
                .value_name("PATTERN")
                .help("Ignore keys matching this regex pattern"),
        )
        .arg(
            Arg::new("epsilon")
                .long("epsilon")
                .value_name("VALUE")
                .help("Numerical comparison tolerance (for floating point numbers)"),
        )
        .arg(
            Arg::new("array-id-key")
                .long("array-id-key")
                .value_name("KEY")
                .help("Array comparison by ID key (compare arrays by this field instead of index)"),
        )
        .arg(
            Arg::new("ignore-whitespace")
                .long("ignore-whitespace")
                .action(clap::ArgAction::SetTrue)
                .help("Ignore whitespace differences"),
        )
        .arg(
            Arg::new("ignore-case")
                .long("ignore-case")
                .action(clap::ArgAction::SetTrue)
                .help("Ignore case differences"),
        )
        .arg(
            Arg::new("quiet")
                .short('q')
                .long("quiet")
                .action(clap::ArgAction::SetTrue)
                .help("Suppress normal output; return only exit status"),
        )
        .arg(
            Arg::new("brief")
                .long("brief")
                .action(clap::ArgAction::SetTrue)
                .help("Report only whether files differ, not the differences"),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue)
                .help("Show verbose processing information"),
        )
        .arg(
            Arg::new("no-color")
                .long("no-color")
                .action(clap::ArgAction::SetTrue)
                .help("Disable colored output"),
        )
        .arg(
            Arg::new("memory-optimization")
                .long("memory-optimization")
                .action(clap::ArgAction::SetTrue)
                .help("Enable memory optimization for large files"),
        )
        .arg(
            Arg::new("batch-size")
                .long("batch-size")
                .value_name("SIZE")
                .help("Batch size for memory optimization"),
        )
        .arg(
            Arg::new("show-unchanged")
                .long("show-unchanged")
                .action(clap::ArgAction::SetTrue)
                .help("Show unchanged values as well"),
        )
        .arg(
            Arg::new("show-types")
                .long("show-types")
                .action(clap::ArgAction::SetTrue)
                .help("Show type information in output"),
        )
}

fn main() {
    // Only generate man page and completions for release builds
    let out_dir = match env::var_os("OUT_DIR") {
        Some(dir) => PathBuf::from(dir),
        None => return,
    };

    let cmd = build_cli();

    // Generate man page
    let man_dir = out_dir.join("man");
    fs::create_dir_all(&man_dir).unwrap();

    let man = Man::new(cmd.clone());
    let mut buffer = Vec::new();
    man.render(&mut buffer).unwrap();
    fs::write(man_dir.join("diffx.1"), buffer).unwrap();

    // Generate shell completions
    let completions_dir = out_dir.join("completions");
    fs::create_dir_all(&completions_dir).unwrap();

    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::PowerShell] {
        let mut cmd = build_cli();
        clap_complete::generate_to(shell, &mut cmd, "diffx", &completions_dir).unwrap();
    }

    println!("cargo:rerun-if-changed=build.rs");
}

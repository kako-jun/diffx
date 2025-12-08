//! Example-based tests using trycmd
//!
//! These tests serve dual purposes:
//! 1. Automated regression testing
//! 2. Living documentation that shows real input/output examples
//!
//! The Markdown files in `tests/examples/` can be directly used
//! in user documentation (like mdBook).

#[test]
fn cli_examples() {
    trycmd::TestCases::new()
        .case("tests/examples/*.md")
        .run();
}

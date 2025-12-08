//! Example-based tests using trycmd
//!
//! These tests serve dual purposes:
//! 1. Automated regression testing
//! 2. Living documentation that shows real input/output examples
//!
//! The Markdown files in `docs/examples/` are the source of truth
//! for user documentation and are verified by these tests.

#[test]
fn cli_examples() {
    trycmd::TestCases::new()
        .case("../docs/examples/*.md")
        .run();
}

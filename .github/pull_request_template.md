# Pull Request

## 📋 Summary

<!-- Provide a brief description of the changes in this PR -->

**Type of Change:**
- [ ] 🐛 Bug fix (non-breaking change which fixes an issue)
- [ ] ✨ New feature (non-breaking change which adds functionality)
- [ ] 💥 Breaking change (fix or feature that would cause existing functionality to not work as expected)
- [ ] 📚 Documentation update
- [ ] 🔧 Refactoring (no functional changes)
- [ ] ⚡ Performance improvement
- [ ] 🧪 Test improvements

## 🔗 Related Issues

<!-- Link to related issues using "Fixes #123" or "Relates to #123" -->

Fixes #
Relates to #

## 📝 Changes Made

<!-- Describe the specific changes made in this PR -->

- 
- 
- 

## 🧪 Testing

<!-- Describe how you tested your changes -->

**Test Coverage:**
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] Manual testing performed
- [ ] Benchmarks run (if applicable)

**Test Results:**
```bash
# Paste relevant test output here
cargo test --workspace
```

## 📚 Documentation

<!-- Check all that apply -->

- [ ] Code is self-documenting with clear variable/function names
- [ ] Public APIs are documented with /// comments
- [ ] README.md updated (if needed)
- [ ] docs/ updated (if needed)
- [ ] CHANGELOG.md updated (if needed)

## ✅ Checklist

**Before submitting this PR, please ensure:**

### Code Quality
- [ ] Code follows the project's style guidelines
- [ ] `cargo fmt --all` has been run
- [ ] `cargo clippy --workspace --all-targets --all-features -- -D warnings` passes
- [ ] All tests pass: `cargo test --workspace`
- [ ] Code builds without warnings: `cargo build --workspace`

### Testing
- [ ] New code has appropriate test coverage
- [ ] All existing tests still pass
- [ ] Edge cases have been considered and tested

### Documentation
- [ ] Public APIs are properly documented
- [ ] Complex logic includes inline comments
- [ ] User-facing changes are documented

### Breaking Changes
- [ ] Breaking changes are clearly documented
- [ ] Migration guide provided (if applicable)
- [ ] Version bump planned appropriately

## 🎯 Performance Impact

<!-- If this PR affects performance, please describe the impact -->

- [ ] No performance impact
- [ ] Performance improvement (include benchmark results)
- [ ] Potential performance regression (justified by benefits)

**Benchmark Results (if applicable):**
```bash
# Include before/after benchmark results
cargo bench --package diffx-core
```

## 🔍 Reviewer Notes

<!-- Any specific areas you'd like reviewers to focus on -->

**Focus Areas:**
- 
- 
- 

**Questions for Reviewers:**
- 
- 
- 

## 📸 Screenshots (if applicable)

<!-- Include screenshots for UI changes or CLI output changes -->

## 🚀 Deployment Notes

<!-- Any special considerations for deployment -->

- [ ] No special deployment requirements
- [ ] Requires configuration changes
- [ ] Requires data migration
- [ ] Other: 

---

**By submitting this PR, I confirm that:**
- [ ] I have read and followed the [Contributing Guidelines](../CONTRIBUTING.md)
- [ ] My code follows the project's coding standards
- [ ] I have tested my changes thoroughly
- [ ] I understand this will be released under the MIT License
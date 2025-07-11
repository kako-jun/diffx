# GitHub Flow Setup Guide

This guide explains how to set up GitHub Flow for the diffx project.

## 🚀 Automated Setup

Run the setup script to configure GitHub Flow automatically:

```bash
./scripts/setup-github-workflow.sh
```

## 📋 Manual Setup Steps

### 1. Create GitHub Labels

```bash
# Create labels from the configuration
jq -c '.[]' .github/labels.json | while read label; do
    name=$(echo "$label" | jq -r '.name')
    color=$(echo "$label" | jq -r '.color')
    description=$(echo "$label" | jq -r '.description')
    
    gh label create "$name" --color "$color" --description "$description" || \
    gh label edit "$name" --color "$color" --description "$description"
done
```

### 2. Set Up Branch Protection

```bash
# Protect main branch (requires admin permissions)
gh api repos/:owner/:repo/branches/main/protection \
  --method PUT \
  --input .github/branch-protection.json
```

### 3. Configure Git Hooks

```bash
# Enable pre-push hooks for all contributors
git config core.hooksPath .githooks

# Or for global setup
echo "git config core.hooksPath .githooks" >> ~/.gitconfig
```

### 4. Create GitHub Project Board

```bash
# Create project board for tracking
gh project create --title "diffx Development" \
  --description "Development tracking for diffx structured diff tool"
```

## 🔧 Developer Workflow

### Creating a Feature Branch

```bash
# Start from latest main
git checkout main
git pull origin main

# Create feature branch
git checkout -b feature/add-binary-support

# Make changes...
# Test changes...
./scripts/ci-local.sh

# Commit and push
git add .
git commit -m "feat(core): add binary format support"
git push origin feature/add-binary-support
```

### Creating a Pull Request

```bash
# Using GitHub CLI
gh pr create \
  --title "feat: add binary format support" \
  --body "Adds support for binary file comparison with hexadecimal diff output"

# Or use the web interface
```

## 📊 Project Management

### Issue Labels

The project uses a structured labeling system:

- **Priority**: `priority: critical`, `priority: high`, `priority: medium`, `priority: low`
- **Type**: `type: bug`, `type: feature`, `type: docs`, `type: refactor`, `type: performance`
- **Status**: `status: in-progress`, `status: blocked`, `status: ready-for-review`
- **Difficulty**: `difficulty: good-first-issue`, `difficulty: easy`, `difficulty: medium`, `difficulty: hard`
- **Area**: `area: core`, `area: cli`, `area: npm`, `area: pypi`, `area: docs`

### Branch Protection Rules

The `main` branch is protected with:
- ✅ Require pull request reviews (1 approval)
- ✅ Dismiss stale reviews when new commits are pushed
- ✅ Require status checks to pass (CI)
- ✅ Require branches to be up to date before merging
- ✅ Include administrators in restrictions
- ✅ Delete head branches after merge

### Pre-push Validation

The `.githooks/pre-push` hook automatically runs:
- Code formatting checks
- Linting (Clippy)
- Full test suite
- Build verification
- CLI smoke tests

## 🎯 Best Practices

1. **Always branch from main**: Ensure you have the latest changes
2. **Use descriptive branch names**: `feature/add-xml-support`, `fix/yaml-parsing-bug`
3. **Keep PRs focused**: One feature or fix per PR
4. **Link issues**: Use "Fixes #123" or "Closes #123" in PR descriptions
5. **Update documentation**: Include relevant docs updates in your PR
6. **Test thoroughly**: Run `./scripts/ci-local.sh` before pushing

## 🔍 Troubleshooting

### Branch Protection Not Working
- Verify you have admin permissions on the repository
- Check that the branch protection API call succeeded
- Ensure the branch name matches exactly ("main")

### Pre-push Hook Not Running
- Verify the hook file is executable: `chmod +x .githooks/pre-push`
- Check git configuration: `git config core.hooksPath`
- Ensure you're in the correct directory

### CI Failures
- Run `./scripts/ci-local.sh` locally to reproduce
- Check the specific failing step in the GitHub Actions logs
- Verify all dependencies are available in the CI environment

## 📚 References

- [GitHub Flow Documentation](https://docs.github.com/en/get-started/quickstart/github-flow)
- [Conventional Commits](https://www.conventionalcommits.org/)
- [diffx Contributing Guide](../../CONTRIBUTING.md)
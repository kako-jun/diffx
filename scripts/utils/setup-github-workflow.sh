#!/bin/bash
set -e

# Find the project root directory (where Cargo.toml exists)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PROJECT_NAME=$(basename "$PROJECT_ROOT")

# Change to project root
cd "$PROJECT_ROOT"

echo "🚀 Setting up GitHub Workflow automation for $PROJECT_NAME project"
echo "========================================================="

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is not installed. Please install it first:"
    echo "   https://cli.github.com/"
    exit 1
fi

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Not in a git repository"
    exit 1
fi

echo ""
echo "🔧 Step 1: Configuring Repository Settings" 
echo "------------------------------------------"
echo "Setting up auto-merge, branch deletion, and merge options..."
REPO_FULL_NAME=$(gh repo view --json nameWithOwner -q .nameWithOwner)

# Enable auto-merge and automatic branch deletion
if gh api "repos/$REPO_FULL_NAME" --method PATCH \
    --field allow_auto_merge=true \
    --field delete_branch_on_merge=true \
    --field allow_squash_merge=true \
    --field allow_merge_commit=true \
    --field allow_rebase_merge=true > /dev/null 2>&1; then
    echo "✅ Repository settings configured:"
    echo "   - Auto-merge enabled (for solo development)"
    echo "   - Automatic branch deletion after merge"
    echo "   - All merge types enabled (merge, squash, rebase)"
else
    echo "❌ Failed to configure repository settings"
fi

echo ""
echo "🎯 Step 2: Workflow Summary"
echo "---------------------------"
echo "GitHub Workflow is now configured with the following features:"
echo ""
echo "✅ Solo Development Features:"
echo "   - Auto-merge enabled (no review required for owner)"
echo "   - Automatic branch deletion after merge"
echo "   - All merge types available"
echo ""
echo "📋 Recommended Development Workflow:"
echo "1. Create feature branch: git checkout -b feature/name"
echo "2. Make changes and commit: git commit -m \"...\""
echo "3. Push branch: git push -u origin feature/name"
echo "4. Create PR: gh pr create --title \"...\" --body \"...\""
echo "5. Auto-merge: gh pr merge --auto --squash"
echo "6. CI will automatically merge after tests pass"
echo ""
echo "🔧 Quick Commands:"
echo "   ./scripts/create-pr.sh \"Title\" \"Description\" - Create PR with auto-merge"
echo "   ./scripts/ci-local.sh - Run local CI validation"
echo ""
echo "🎉 GitHub Workflow setup completed successfully!"
echo ""
echo "Your repository is ready for efficient solo development with:"
echo "- Streamlined PR workflow"
echo "- Automatic quality checks"
echo "- Clean branch management"
echo "- Future contributor readiness"
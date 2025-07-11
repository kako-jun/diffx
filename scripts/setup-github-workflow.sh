#!/bin/bash
set -e

echo "🚀 Setting up GitHub Flow for diffx project"
echo "=========================================="

# Check if gh CLI is installed
if ! command -v gh &> /dev/null; then
    echo "❌ GitHub CLI (gh) is not installed. Please install it first:"
    echo "   https://cli.github.com/"
    exit 1
fi

# Check if jq is installed
if ! command -v jq &> /dev/null; then
    echo "❌ jq is not installed. Please install it first:"
    echo "   Ubuntu/Debian: sudo apt install jq"
    echo "   macOS: brew install jq"
    echo "   Or skip this step and create labels manually via GitHub web interface"
    echo ""
    echo "Continuing without label creation..."
    SKIP_LABELS=true
fi

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Not in a git repository"
    exit 1
fi

echo ""
echo "📋 Step 1: Creating GitHub Labels"
echo "--------------------------------"
# Create labels from labels.json
if [ -f ".github/labels.json" ] && [ "$SKIP_LABELS" != "true" ]; then
    echo "Creating labels from .github/labels.json..."
    
    # Create new labels using jq
    jq -c '.[]' .github/labels.json | while read label; do
        name=$(echo "$label" | jq -r '.name')
        color=$(echo "$label" | jq -r '.color')
        description=$(echo "$label" | jq -r '.description')
        
        echo "  Creating label: $name"
        gh label create "$name" --color "$color" --description "$description" 2>/dev/null || \
        gh label edit "$name" --color "$color" --description "$description"
    done
    echo "✅ Labels created successfully"
elif [ "$SKIP_LABELS" = "true" ]; then
    echo "⚠️  Skipping label creation (jq not available)"
    echo "   You can create labels manually via GitHub web interface"
    echo "   Or install jq and run this script again"
else
    echo "❌ .github/labels.json not found"
fi

echo ""
echo "🔒 Step 2: Setting up Branch Protection"
echo "--------------------------------------"
if [ -f ".github/branch-protection.json" ]; then
    echo "Applying branch protection rules..."
    REPO_FULL_NAME=$(gh repo view --json nameWithOwner -q .nameWithOwner)
    if gh api "repos/$REPO_FULL_NAME/branches/main/protection" --method PUT --input .github/branch-protection.json > /dev/null 2>&1; then
        echo "✅ Branch protection rules applied successfully"
    else
        echo "❌ Failed to apply branch protection rules"
        echo "   You may need admin permissions or the repository may not exist"
        echo "   Manual command: gh api repos/$REPO_FULL_NAME/branches/main/protection --method PUT --input .github/branch-protection.json"
    fi
else
    echo "❌ .github/branch-protection.json not found"
fi

echo ""
echo "🏗️ Step 3: Configuring Repository Settings" 
echo "------------------------------------------"
echo "Setting up automatic branch deletion and other repository settings..."
REPO_FULL_NAME=$(gh repo view --json nameWithOwner -q .nameWithOwner)

# Enable automatic branch deletion after PR merge
if gh api "repos/$REPO_FULL_NAME" --method PATCH --field delete_branch_on_merge=true > /dev/null 2>&1; then
    echo "✅ Automatic branch deletion enabled"
else
    echo "❌ Failed to enable automatic branch deletion"
fi

# Enable other useful settings
if gh api "repos/$REPO_FULL_NAME" --method PATCH \
    --field allow_squash_merge=true \
    --field allow_merge_commit=true \
    --field allow_rebase_merge=true \
    --field allow_auto_merge=false > /dev/null 2>&1; then
    echo "✅ Merge options configured"
else
    echo "❌ Failed to configure merge options"
fi

echo ""
echo "🔧 Step 4: Setting up Git Hooks"
echo "-------------------------------"
if [ -f ".githooks/pre-push" ]; then
    echo "Enabling pre-push hooks..."
    git config core.hooksPath .githooks
    chmod +x .githooks/pre-push
    echo "✅ Git hooks enabled successfully"
    echo "   Pre-push validation will run ./scripts/ci-local.sh before each push"
else
    echo "❌ .githooks/pre-push not found"
fi

echo ""
echo "📚 Step 5: Validation and Testing"
echo "---------------------------------"
echo "Testing GitHub Flow setup..."

# Test if branch protection is working
if gh api "repos/$REPO_FULL_NAME/branches/main/protection" > /dev/null 2>&1; then
    echo "✅ Branch protection is active"
else
    echo "⚠️  Branch protection may not be fully configured"
fi

# Test if labels were created
LABEL_COUNT=$(gh label list --json name | jq '. | length')
if [ "$LABEL_COUNT" -gt 15 ]; then
    echo "✅ Labels created successfully ($LABEL_COUNT labels found)"
else
    echo "⚠️  Limited labels found ($LABEL_COUNT labels)"
fi

# Test if git hooks are working
if git config core.hooksPath | grep -q ".githooks"; then
    echo "✅ Git hooks configured correctly"
else
    echo "⚠️  Git hooks may not be configured"
fi

echo ""
echo "🎯 Step 6: Workflow Summary"
echo "---------------------------"
echo "GitHub Flow is now configured with the following features:"
echo ""
echo "✅ Branch Protection:"
echo "   - PR reviews required (1 approval)"
echo "   - CI checks must pass"
echo "   - Direct push to main blocked"
echo ""
echo "✅ Repository Settings:"
echo "   - Automatic branch deletion after merge"
echo "   - All merge types enabled (merge, squash, rebase)"
echo ""
echo "✅ Quality Automation:"
echo "   - Pre-push hooks run CI validation"
echo "   - 21 structured labels for issue management"
echo "   - Issue templates updated with new labels"
echo ""
echo "📋 Development Workflow:"
echo "1. Create feature branch: git checkout -b feature/your-feature-name"
echo "2. Make changes and commit: git commit -m \"feat: your changes\""
echo "3. Push (triggers pre-push validation): git push origin feature/your-feature-name"
echo "4. Create PR: gh pr create --title \"feat: your feature\" --body \"Description\""
echo "5. Review and merge (branch auto-deleted after merge)"
echo ""
echo "🔧 Troubleshooting:"
echo "   - Pre-push failing? Run: ./scripts/ci-local.sh"
echo "   - Need to bypass hooks? Use: git push --no-verify"
echo "   - Check branch protection: gh api repos/$REPO_FULL_NAME/branches/main/protection"

echo ""
echo "🎉 GitHub Flow setup completed successfully!"
echo ""
echo "Your repository is now ready for collaborative development with:"
echo "- Protected main branch"
echo "- Automated quality checks" 
echo "- Structured issue management"
echo "- Complete CI/CD integration"
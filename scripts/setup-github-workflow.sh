#!/bin/bash
set -e

echo "🚀 Setting up Simplified GitHub Workflow for diffx project"
echo "========================================================="

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
echo "🔒 Step 2: Branch Protection (DISABLED)"
echo "--------------------------------------"
echo "⚠️  Branch protection is intentionally disabled for fast development"
echo "   Direct pushes to main are allowed for urgent situations"
echo "   This prioritizes development speed over process enforcement"
REPO_FULL_NAME=$(gh repo view --json nameWithOwner -q .nameWithOwner)
if gh api "repos/$REPO_FULL_NAME/branches/main/protection" --method DELETE > /dev/null 2>&1; then
    echo "✅ Branch protection removed successfully"
else
    echo "ℹ️  No branch protection to remove (already disabled)"
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
echo "🔧 Step 4: Git Hooks (DISABLED)"
echo "-------------------------------"
echo "⚠️  Pre-push hooks are intentionally disabled"
echo "   Pushes should be fast and unrestricted when urgent"
echo "   Validation can be run manually with: ./scripts/ci-local.sh"
if git config core.hooksPath | grep -q ".githooks"; then
    git config --unset core.hooksPath
    echo "✅ Git hooks disabled successfully"
else
    echo "ℹ️  Git hooks already disabled"
fi

echo ""
echo "📚 Step 5: Validation and Testing"
echo "---------------------------------"
echo "Testing simplified workflow setup..."

# Test if branch protection is disabled (as intended)
if gh api "repos/$REPO_FULL_NAME/branches/main/protection" > /dev/null 2>&1; then
    echo "⚠️  Branch protection is still active (should be disabled)"
else
    echo "✅ Branch protection is disabled (as intended)"
fi

# Test if labels were created
if [ "$SKIP_LABELS" != "true" ]; then
    LABEL_COUNT=$(gh label list --json name | jq '. | length' 2>/dev/null || echo "0")
    if [ "$LABEL_COUNT" -gt 15 ]; then
        echo "✅ Labels created successfully ($LABEL_COUNT labels found)"
    else
        echo "⚠️  Limited labels found ($LABEL_COUNT labels)"
    fi
fi

# Test if git hooks are disabled (as intended)
if git config core.hooksPath | grep -q ".githooks"; then
    echo "⚠️  Git hooks are still active (should be disabled)"
else
    echo "✅ Git hooks are disabled (as intended)"
fi

echo ""
echo "🎯 Step 6: Workflow Summary"
echo "---------------------------"
echo "Simplified GitHub Workflow is now configured with the following features:"
echo ""
echo "✅ Repository Settings:"
echo "   - Automatic branch deletion after merge"
echo "   - All merge types enabled (merge, squash, rebase)"
echo "   - Structured labels for issue management"
echo ""
echo "⚠️  Intentionally Disabled (for fast development):"
echo "   - Branch protection (direct push to main allowed)"
echo "   - Pre-push hooks (no validation blocking pushes)"
echo "   - PR review requirements (optional)"
echo ""
echo "📋 Simplified Development Workflow:"
echo "1. Option A - Direct to main: git push origin main (fast, for urgent fixes)"
echo "2. Option B - Feature branch: git checkout -b feature/name && git push && gh pr create"
echo "3. Manual validation: ./scripts/ci-local.sh (run when convenient)"
echo "4. CI/CD runs automatically on push (but doesn't block merges)"
echo ""
echo "🔧 Philosophy:"
echo "   - Prioritize development speed over process enforcement"
echo "   - Allow unrestricted pushes when urgent"
echo "   - Optional validation rather than mandatory blocks"
echo "   - Developers can choose their workflow based on urgency"

echo ""
echo "🎉 Simplified GitHub Workflow setup completed successfully!"
echo ""
echo "Your repository is now ready for fast, flexible development with:"
echo "- Unrestricted push access"
echo "- Optional quality checks"
echo "- Structured issue management"
echo "- Flexible CI/CD integration"
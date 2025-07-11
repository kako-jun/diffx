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

# Check if we're in a git repository
if ! git rev-parse --git-dir > /dev/null 2>&1; then
    echo "❌ Not in a git repository"
    exit 1
fi

echo ""
echo "📋 Step 1: Creating GitHub Labels"
echo "--------------------------------"
# Create labels from labels.json
if [ -f ".github/labels.json" ]; then
    echo "Creating labels from .github/labels.json..."
    
    # Delete existing labels (optional, commented out for safety)
    # gh label list --json name -q '.[].name' | xargs -I {} gh label delete {} --yes
    
    # Create new labels
    jq -c '.[]' .github/labels.json | while read label; do
        name=$(echo "$label" | jq -r '.name')
        color=$(echo "$label" | jq -r '.color')
        description=$(echo "$label" | jq -r '.description')
        
        echo "  Creating label: $name"
        gh label create "$name" --color "$color" --description "$description" 2>/dev/null || \
        gh label edit "$name" --color "$color" --description "$description"
    done
    echo "✅ Labels created successfully"
else
    echo "❌ .github/labels.json not found"
fi

echo ""
echo "🔒 Step 2: Setting up Branch Protection"
echo "--------------------------------------"
echo "Please run the following command to set up branch protection:"
echo ""
echo "gh api repos/:owner/:repo/branches/main/protection \\"
echo "  --method PUT \\"
echo "  --input .github/branch-protection.json"
echo ""
echo "Note: You need admin permissions to set branch protection rules."

echo ""
echo "📊 Step 3: Creating GitHub Project Board"
echo "---------------------------------------"
echo "To create a project board, run:"
echo ""
echo "gh project create --title \"diffx Development\" \\"
echo "  --description \"Development tracking for diffx structured diff tool\""

echo ""
echo "🎯 Step 4: Workflow Recommendations"
echo "----------------------------------"
echo "1. Always create feature branches:"
echo "   git checkout -b feature/your-feature-name"
echo ""
echo "2. Run local CI before pushing:"
echo "   ./scripts/ci-local.sh"
echo ""
echo "3. Create PR with:"
echo "   gh pr create --title \"feat: your feature\" --body \"Description\""
echo ""
echo "4. Link issues to PRs:"
echo "   - Use 'Fixes #123' in PR description"
echo "   - Use 'Closes #123' for automatic issue closing"

echo ""
echo "✅ Setup script completed!"
echo ""
echo "📚 Next steps:"
echo "1. Run the branch protection command above"
echo "2. Update CONTRIBUTING.md with new workflow"
echo "3. Create your first feature branch and PR!"
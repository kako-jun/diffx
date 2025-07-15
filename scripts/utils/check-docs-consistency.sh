#!/bin/bash

# 3言語ドキュメント整合性チェックスクリプト
# Three-language documentation consistency checker

set -e

# Find the project root directory (where Cargo.toml exists)
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"
PROJECT_NAME=$(basename "$PROJECT_ROOT")

# Change to project root
cd "$PROJECT_ROOT"

echo "🔍 3言語ドキュメント整合性チェック開始"
echo "🔍 Starting 3-language documentation consistency check"
echo "🔍 开始检查3语言文档一致性"
echo "============================================="

# 色付きログ用関数
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

log_success() {
    echo -e "${GREEN}✅ $1${NC}"
}

log_error() {
    echo -e "${RED}❌ $1${NC}"
}

log_warning() {
    echo -e "${YELLOW}⚠️  $1${NC}"
}

# エラーカウンター
ERROR_COUNT=0
WARNING_COUNT=0

# 対象ドキュメントファイルの定義
declare -a DOCS=(
    "README"
    "docs/reference/cli-reference"
    "docs/user-guide/getting-started"
    "docs/user-guide/examples"
)

# 言語サフィックス
declare -a LANGUAGES=("" "_ja" "_zh")
declare -a LANGUAGE_NAMES=("English" "Japanese" "Chinese")

# 1. ファイル存在チェック
echo ""
echo "📁 1. ファイル存在チェック / File existence check / 文件存在检查"
echo "-----------------------------------------------------------"

for doc in "${DOCS[@]}"; do
    echo ""
    echo "📄 Checking: $doc"
    
    for i in "${!LANGUAGES[@]}"; do
        suffix="${LANGUAGES[$i]}"
        lang_name="${LANGUAGE_NAMES[$i]}"
        
        file_path="${doc}${suffix}.md"
        
        if [ -f "$PROJECT_ROOT/$file_path" ]; then
            log_success "$lang_name: $file_path exists"
        else
            log_error "$lang_name: $file_path NOT FOUND"
            ((ERROR_COUNT++))
        fi
    done
done

# 2. 基本構造チェック（見出しの数）
echo ""
echo "📊 2. 基本構造チェック / Basic structure check / 基本结构检查"
echo "-----------------------------------------------------------"

for doc in "${DOCS[@]}"; do
    echo ""
    echo "📄 Checking structure: $doc"
    
    # 各言語の見出し数を取得
    declare -a heading_counts=()
    
    for i in "${!LANGUAGES[@]}"; do
        suffix="${LANGUAGES[$i]}"
        lang_name="${LANGUAGE_NAMES[$i]}"
        file_path="${doc}${suffix}.md"
        
        if [ -f "$PROJECT_ROOT/$file_path" ]; then
            # 見出し（#で始まる行）をカウント
            count=$(grep -c "^#" "$PROJECT_ROOT/$file_path" 2>/dev/null || echo "0")
            heading_counts[$i]=$count
            echo "  $lang_name: $count headings"
        else
            heading_counts[$i]=0
        fi
    done
    
    # 見出し数の一致チェック
    en_count=${heading_counts[0]}
    ja_count=${heading_counts[1]}
    zh_count=${heading_counts[2]}
    
    if [ "$en_count" -eq "$ja_count" ] && [ "$ja_count" -eq "$zh_count" ]; then
        log_success "Heading counts match ($en_count headings)"
    else
        log_warning "Heading counts differ: EN=$en_count, JA=$ja_count, ZH=$zh_count"
        ((WARNING_COUNT++))
    fi
done

# 3. コードブロック数チェック
echo ""
echo "💻 3. コードブロック数チェック / Code block count check / 代码块数量检查"
echo "-------------------------------------------------------------------"

for doc in "${DOCS[@]}"; do
    echo ""
    echo "📄 Checking code blocks: $doc"
    
    # 各言語のコードブロック数を取得
    declare -a code_counts=()
    
    for i in "${!LANGUAGES[@]}"; do
        suffix="${LANGUAGES[$i]}"
        lang_name="${LANGUAGE_NAMES[$i]}"
        file_path="${doc}${suffix}.md"
        
        if [ -f "$PROJECT_ROOT/$file_path" ]; then
            # コードブロック（```で始まる行）をカウント
            count=$(grep -c "^```" "$PROJECT_ROOT/$file_path" 2>/dev/null || echo "0")
            # コードブロックは開始と終了で2つずつなので2で割る
            count=$((count / 2))
            code_counts[$i]=$count
            echo "  $lang_name: $count code blocks"
        else
            code_counts[$i]=0
        fi
    done
    
    # コードブロック数の一致チェック
    en_count=${code_counts[0]}
    ja_count=${code_counts[1]}
    zh_count=${code_counts[2]}
    
    if [ "$en_count" -eq "$ja_count" ] && [ "$ja_count" -eq "$zh_count" ]; then
        log_success "Code block counts match ($en_count blocks)"
    else
        log_warning "Code block counts differ: EN=$en_count, JA=$ja_count, ZH=$zh_count"
        ((WARNING_COUNT++))
    fi
done

# 4. CLIオプション整合性チェック（cli-referenceのみ）
echo ""
echo "⚙️  4. CLIオプション整合性チェック / CLI option consistency check / CLI选项一致性检查"
echo "-------------------------------------------------------------------------------"

cli_ref_base="docs/reference/cli-reference"
if [ -f "$PROJECT_ROOT/${cli_ref_base}.md" ]; then
    echo ""
    echo "📄 Checking CLI options in cli-reference"
    
    # 各言語のCLIオプション数を取得（--で始まる行）
    declare -a option_counts=()
    
    for i in "${!LANGUAGES[@]}"; do
        suffix="${LANGUAGES[$i]}"
        lang_name="${LANGUAGE_NAMES[$i]}"
        file_path="${cli_ref_base}${suffix}.md"
        
        if [ -f "$PROJECT_ROOT/$file_path" ]; then
            # CLIオプション（--で始まる行）をカウント
            count=$(grep -c "^#### \`--" "$PROJECT_ROOT/$file_path" 2>/dev/null || echo "0")
            option_counts[$i]=$count
            echo "  $lang_name: $count CLI options"
        else
            option_counts[$i]=0
        fi
    done
    
    # CLIオプション数の一致チェック
    en_count=${option_counts[0]}
    ja_count=${option_counts[1]}
    zh_count=${option_counts[2]}
    
    if [ "$en_count" -eq "$ja_count" ] && [ "$ja_count" -eq "$zh_count" ]; then
        log_success "CLI option counts match ($en_count options)"
    else
        log_error "CLI option counts differ: EN=$en_count, JA=$ja_count, ZH=$zh_count"
        ((ERROR_COUNT++))
    fi
fi

# 5. 特定キーワードの整合性チェック（簡略版）
echo ""
echo "🔍 5. 特定キーワード整合性チェック / Specific keyword consistency check / 特定关键词一致性检查"
echo "-------------------------------------------------------------------------------------"

# 重要なキーワードリスト
declare -a KEYWORDS=("${PROJECT_NAME}" "JSON" "YAML")

for doc in "${DOCS[@]}"; do
    echo ""
    echo "📄 Checking keywords: $doc"
    
    for keyword in "${KEYWORDS[@]}"; do
        declare -a keyword_counts=()
        
        for i in "${!LANGUAGES[@]}"; do
            suffix="${LANGUAGES[$i]}"
            file_path="${doc}${suffix}.md"
            
            if [ -f "$PROJECT_ROOT/$file_path" ]; then
                # 大文字小文字を区別してキーワードをカウント
                count=$(grep -c "$keyword" "$PROJECT_ROOT/$file_path" 2>/dev/null || echo "0")
                keyword_counts[$i]=$count
            else
                keyword_counts[$i]=0
            fi
        done
        
        # キーワード数の一致チェック（許容範囲: ±30%）
        en_count=${keyword_counts[0]}
        ja_count=${keyword_counts[1]}
        zh_count=${keyword_counts[2]}
        
        if [ "$en_count" -gt 2 ]; then
            # 30%の許容範囲を計算
            min_count=$((en_count * 7 / 10))
            max_count=$((en_count * 13 / 10))
            
            if [ "$ja_count" -ge "$min_count" ] && [ "$ja_count" -le "$max_count" ] && 
               [ "$zh_count" -ge "$min_count" ] && [ "$zh_count" -le "$max_count" ]; then
                echo "  ✅ $keyword: EN=$en_count, JA=$ja_count, ZH=$zh_count (OK)"
            else
                log_warning "$keyword counts vary: EN=$en_count, JA=$ja_count, ZH=$zh_count"
                ((WARNING_COUNT++))
            fi
        fi
    done
done

# 結果サマリー
echo ""
echo "📊 チェック結果サマリー / Check Result Summary / 检查结果摘要"
echo "============================================="
echo "🔍 対象ドキュメント数: ${#DOCS[@]}"
echo "🌐 対象言語数: ${#LANGUAGES[@]}"
echo ""

if [ "$ERROR_COUNT" -eq 0 ] && [ "$WARNING_COUNT" -eq 0 ]; then
    log_success "すべてのチェックが正常に完了しました"
    log_success "All checks passed successfully"
    log_success "所有检查都成功通过"
    exit 0
elif [ "$ERROR_COUNT" -eq 0 ]; then
    log_warning "警告: $WARNING_COUNT 件の軽微な不整合が見つかりました"
    log_warning "Warnings: $WARNING_COUNT minor inconsistencies found"
    log_warning "警告: 发现 $WARNING_COUNT 个轻微不一致问题"
    exit 0
else
    log_error "エラー: $ERROR_COUNT 件の重要な不整合が見つかりました"
    log_error "Errors: $ERROR_COUNT critical inconsistencies found"
    log_error "错误: 发现 $ERROR_COUNT 个严重不一致问题"
    if [ "$WARNING_COUNT" -gt 0 ]; then
        log_warning "警告: $WARNING_COUNT 件の軽微な不整合も見つかりました"
        log_warning "Warnings: $WARNING_COUNT minor inconsistencies also found"
        log_warning "警告: 还发现 $WARNING_COUNT 个轻微不一致问题"
    fi
    exit 1
fi
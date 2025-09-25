#!/bin/bash

# GitHub Actions Workflow Validation Script
# This script checks if the workflow files are valid and up-to-date

echo "🔍 Checking GitHub Actions workflows..."
echo "======================================"

WORKFLOW_DIR=".github/workflows"

if [ ! -d "$WORKFLOW_DIR" ]; then
    echo "❌ Workflow directory not found: $WORKFLOW_DIR"
    exit 1
fi

echo "📁 Found workflow directory: $WORKFLOW_DIR"
echo ""

# Count workflow files
WORKFLOW_COUNT=$(find "$WORKFLOW_DIR" -name "*.yml" -o -name "*.yaml" | wc -l)
echo "📄 Found $WORKFLOW_COUNT workflow files:"

for file in "$WORKFLOW_DIR"/*.yml "$WORKFLOW_DIR"/*.yaml; do
    if [ -f "$file" ]; then
        echo "  - $(basename "$file")"
    fi
done

echo ""

# Check for deprecated actions
echo "🔍 Checking for deprecated actions..."
echo "-----------------------------------"

DEPRECATED_FOUND=false

# Check for deprecated upload-artifact versions
if grep -r "actions/upload-artifact@v[123]" "$WORKFLOW_DIR" > /dev/null 2>&1; then
    echo "⚠️  Found deprecated upload-artifact versions:"
    grep -rn "actions/upload-artifact@v[123]" "$WORKFLOW_DIR"
    DEPRECATED_FOUND=true
fi

# Check for deprecated cache versions
if grep -r "actions/cache@v[123]" "$WORKFLOW_DIR" > /dev/null 2>&1; then
    echo "⚠️  Found deprecated cache versions:"
    grep -rn "actions/cache@v[123]" "$WORKFLOW_DIR"
    DEPRECATED_FOUND=true
fi

# Check for deprecated setup-java versions
if grep -r "actions/setup-java@v[123]" "$WORKFLOW_DIR" > /dev/null 2>&1; then
    echo "⚠️  Found deprecated setup-java versions:"
    grep -rn "actions/setup-java@v[123]" "$WORKFLOW_DIR"
    DEPRECATED_FOUND=true
fi

if [ "$DEPRECATED_FOUND" = false ]; then
    echo "✅ No deprecated action versions found!"
fi

echo ""

# Check for current recommended versions
echo "🔍 Checking for current recommended versions..."
echo "---------------------------------------------"

CURRENT_VERSIONS_FOUND=true

# Check for v4 versions
if ! grep -r "actions/upload-artifact@v4" "$WORKFLOW_DIR" > /dev/null 2>&1; then
    echo "⚠️  upload-artifact@v4 not found - consider updating"
    CURRENT_VERSIONS_FOUND=false
fi

if ! grep -r "actions/cache@v4" "$WORKFLOW_DIR" > /dev/null 2>&1; then
    echo "⚠️  cache@v4 not found - consider updating"
    CURRENT_VERSIONS_FOUND=false
fi

if ! grep -r "actions/setup-java@v4" "$WORKFLOW_DIR" > /dev/null 2>&1; then
    echo "⚠️  setup-java@v4 not found - consider updating"
    CURRENT_VERSIONS_FOUND=false
fi

if [ "$CURRENT_VERSIONS_FOUND" = true ]; then
    echo "✅ Current recommended versions found!"
fi

echo ""

# Basic YAML syntax check (if yq is available)
echo "🔍 Checking YAML syntax..."
echo "-------------------------"

if command -v yq > /dev/null 2>&1; then
    YAML_VALID=true
    for file in "$WORKFLOW_DIR"/*.yml "$WORKFLOW_DIR"/*.yaml; do
        if [ -f "$file" ]; then
            if ! yq eval '.' "$file" > /dev/null 2>&1; then
                echo "❌ YAML syntax error in: $(basename "$file")"
                YAML_VALID=false
            fi
        fi
    done
    
    if [ "$YAML_VALID" = true ]; then
        echo "✅ All YAML files have valid syntax!"
    fi
else
    echo "⚠️  yq not found - skipping YAML syntax check"
    echo "   Install yq with: sudo snap install yq"
fi

echo ""

# Summary
echo "📋 Summary"
echo "=========="

if [ "$DEPRECATED_FOUND" = false ] && [ "$CURRENT_VERSIONS_FOUND" = true ]; then
    echo "✅ All workflows are up-to-date and ready to use!"
    exit 0
else
    echo "⚠️  Some issues found - check the output above"
    exit 1
fi
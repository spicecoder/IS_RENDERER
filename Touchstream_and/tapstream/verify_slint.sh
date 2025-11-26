#!/bin/bash
# Quick verification script for common Slint issues

echo "🔍 Verifying Slint file: ui/tapstream.slint"
echo

errors=0

# Check 1: No LineEdit (should be TextInput)
if grep -q "LineEdit" ui/tapstream.slint; then
    echo "❌ Found 'LineEdit' - should be 'TextInput'"
    grep -n "LineEdit" ui/tapstream.slint
    errors=$((errors + 1))
else
    echo "✅ No 'LineEdit' found"
fi

# Check 2: placeholder-text usage (correct in Slint)
count=$(grep -c "placeholder-text" ui/tapstream.slint)
if [ $count -gt 0 ]; then
    echo "✅ Found $count placeholder-text properties (correct)"
else
    echo "⚠️  No placeholder-text found (may be intentional)"
fi

# Check 3: No two-way binding on nested properties
if grep -q "<=.*\\..*\\." ui/tapstream.slint; then
    echo "❌ Found two-way binding on nested property (metadata.field)"
    grep -n "<=.*\\..*\\." ui/tapstream.slint
    errors=$((errors + 1))
else
    echo "✅ No two-way binding on nested properties"
fi

# Check 4: edited callback usage
if grep -q "edited(..*)" ui/tapstream.slint; then
    echo "❌ Found 'edited' with arguments - should be 'edited =>' with no args"
    grep -n "edited(..*)" ui/tapstream.slint
    errors=$((errors + 1))
else
    echo "✅ All 'edited' callbacks have correct signature"
fi

echo
if [ $errors -eq 0 ]; then
    echo "✅ All checks passed! Slint file looks good."
    echo "   Ready to build with: cargo build --release"
    exit 0
else
    echo "❌ Found $errors issue(s). Please fix before building."
    exit 1
fi

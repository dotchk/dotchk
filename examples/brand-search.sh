#!/bin/bash
# Brand Search Script - Find available domains for your brand across TLDs

BRAND_NAME="${1:-mybrand}"

echo "=== Brand Search for: $BRAND_NAME ==="
echo

# Check popular TLDs
echo "1. Checking popular TLDs (showing available only)..."
dotchk tld "$BRAND_NAME" --popular --available-only

echo
echo "2. Checking tech-focused TLDs..."
dotchk tld "$BRAND_NAME" --tlds dev,app,io,tech,cloud,digital,online --available-only

echo
echo "3. Checking business TLDs..."
dotchk tld "$BRAND_NAME" --tlds business,company,ventures,enterprises,solutions --available-only

echo
echo "4. Checking variations..."
# Direct checking shows all results
dotchk \
    "$BRAND_NAME.com" \
    "get$BRAND_NAME.com" \
    "try$BRAND_NAME.com" \
    "my$BRAND_NAME.com" \
    "$BRAND_NAME-app.com" \
    "${BRAND_NAME}hq.com"

echo
echo "5. Finding available patterns with your brand..."
dotchk pattern "${BRAND_NAME}-[a-z]{3}.com" --limit 20

echo
echo "6. Generating full report..."
dotchk tld "$BRAND_NAME" --popular --stats --output "${BRAND_NAME}-availability.csv"

echo
echo "Report saved to: ${BRAND_NAME}-availability.csv"
echo
echo "Tip: Pattern search (step 5) shows only available domains automatically!"
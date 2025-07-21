#!/bin/bash
# Competitive Analysis - Check domain variations of competitors

echo "=== Competitive Domain Analysis ==="
echo

# Define competitors (customize these)
COMPETITORS=("google" "facebook" "amazon" "netflix" "uber")
TLDS=("com" "net" "org" "io" "dev" "app" "ai" "cloud")

# Check each competitor's domain portfolio
for company in "${COMPETITORS[@]}"; do
    echo
    echo "=== Analyzing: $company ==="
    
    # Check standard TLDs
    echo "1. Standard domains:"
    dotchk tld "$company" --tlds "$(IFS=,; echo "${TLDS[*]}")"
    
    # Check common variations
    echo
    echo "2. Common variations:"
    variations=(
        "get$company"
        "try$company" 
        "$company-app"
        "${company}hq"
        "my$company"
        "${company}2"
        "the$company"
    )
    
    for variant in "${variations[@]}"; do
        dotchk check "$variant.com" "$variant.io" --timeout 500
    done
    
    # Export results
    dotchk tld "$company" --all --output "analysis-${company}.csv" &
done

# Wait for background jobs
wait

echo
echo "=== Summary Report ==="
echo

# Generate summary
for company in "${COMPETITORS[@]}"; do
    if [ -f "analysis-${company}.csv" ]; then
        total=$(tail -n +2 "analysis-${company}.csv" | wc -l)
        available=$(grep -c ",true," "analysis-${company}.csv" || true)
        echo "$company: $available available out of $total checked"
    fi
done

# Find gaps in the market
echo
echo "=== Market Gaps Analysis ==="
echo "Checking for available domains with competitor keywords..."

for company in "${COMPETITORS[@]}"; do
    # Check if competitor+industry combinations are available
    dotchk pattern "${company}[a-z]{3,5}\.(com|io|dev)" --limit 50 --available-only
done
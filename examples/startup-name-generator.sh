#!/bin/bash
# Startup Name Generator - Find available startup-style domains

echo "=== Startup Domain Name Generator ==="
echo

# Common startup prefixes
echo "1. Checking domains with common startup prefixes..."
for prefix in get try use my the; do
    echo "   Checking $prefix* patterns..."
    dotchk pattern "${prefix}[a-z]{4,6}.com" --limit 50 --available-only
done

# Tech-style domains
echo
echo "2. Checking tech-style domains..."
dotchk pattern "[a-z]{4,6}(tech|labs|hub|box|bot).com" --limit 100 --available-only

# AI/ML themed
echo
echo "3. Checking AI/ML themed domains..."
dotchk pattern "(ai|ml|smart|auto|robo)[a-z]{3,5}.com" --limit 100 --available-only

# API/Dev tools style
echo
echo "4. Checking API/developer tool style domains..."
dotchk pattern "[a-z]{3,6}(api|sdk|dev|kit|js).com" --limit 100 --available-only

# Modern TLD combinations
echo
echo "5. Checking modern TLD combinations..."
prefixes=("super" "mega" "ultra" "next" "meta" "web" "cloud" "data" "tech" "digital")
for prefix in "${prefixes[@]}"; do
    dotchk tld "$prefix" --tlds io,dev,app,ai --available-only
done

# Two-word combinations
echo
echo "6. Checking two-word combinations..."
dotchk pattern "(fast|quick|smart|easy|simple)(pay|chat|mail|shop|work).com" --available-only

# Export comprehensive results
echo
echo "7. Generating comprehensive report..."
{
    echo "=== Available Startup Domains ==="
    dotchk pattern "(get|try|use|my)[a-z]{4}.com" --limit 200 --available-only
    dotchk pattern "[a-z]{5}(ly|ify|fy).com" --limit 200 --available-only  
    dotchk pattern "[a-z]{4,6}.(io|dev|app)" --limit 300 --available-only
} > startup-domains-report.txt

echo
echo "Report saved to: startup-domains-report.txt"
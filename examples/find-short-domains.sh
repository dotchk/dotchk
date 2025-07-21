#!/bin/bash
# Find available short domains

echo "=== Finding Short Available Domains ==="
echo
echo "Note: Pattern searches automatically show only available domains"
echo

# 3-letter .com domains
echo "1. Searching 3-letter .com domains..."
dotchk pattern "[a-z]{3}.com" --limit 1000 --output short-com-domains.csv

echo
echo "2. Searching 3-letter .io domains..."
dotchk pattern "[a-z]{3}.io" --limit 1000 --output short-io-domains.csv

echo
echo "3. Searching pronounceable 4-letter domains (CVCV pattern)..."
# Pattern: consonant-vowel-consonant-vowel
dotchk pattern "[bcdfghjklmnpqrstvwxyz][aeiou][bcdfghjklmnpqrstvwxyz][aeiou].com" \
    --limit 500 --output pronounceable-domains.csv

echo
echo "4. Searching number-letter combinations..."
dotchk pattern "[0-9][a-z]{2}.io" --output number-letter-domains.csv

echo
echo "5. Searching tech-style domains..."
dotchk pattern "dev-[a-z]{3}.com" --limit 200 --output tech-domains.csv

# Show statistics
echo
echo "=== Results Summary ==="
for file in short-com-domains.csv short-io-domains.csv pronounceable-domains.csv number-letter-domains.csv tech-domains.csv; do
    if [ -f "$file" ]; then
        count=$(tail -n +2 "$file" | wc -l)
        echo "$file: $count available domains found"
    fi
done

echo
echo "Tip: All results are available domains only. No filtering needed!"
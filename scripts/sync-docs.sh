#!/bin/bash
# Sync documentation from README to website

# Extract sections from README and generate docs/index.md
# This is a simple example - could be enhanced with better parsing

cat > docs/index.md << 'EOF'
# dotchk

<div class="hero-section">
  <h2>High-Performance Domain<br>Availability Checker</h2>
  <p>Check thousands of domains per second across 1,440+ TLDs with unparalleled speed and accuracy.</p>
</div>

---

EOF

# Extract content from README starting from Installation
sed -n '/^## Installation/,/^---$/p' README.md | sed '$d' >> docs/index.md

echo "Documentation synced from README.md to docs/index.md"
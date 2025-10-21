# dotchk

Fast domain availability checker. Check thousands of domains per second across 1,080+ TLDs.

⚠️ Checks NS records (fast), not WHOIS (accurate). Verify with WHOIS before purchasing.

## Install

```bash
git clone https://github.com/dotchk/dotchk.git
cd dotchk
./install.sh
```

Or manually:
```bash
cargo install --path .
```

## Quick Start

```bash
dotchk example.com
dotchk example.com test.io startup.dev
dotchk pattern "[a-z]{3}.com" --limit 100
dotchk tld mybrand --popular --available-only
```

## Commands

### Direct Checking

Check one or more domains:

```bash
# Single domain
dotchk example.com

# Multiple domains
dotchk example.com mysite.io startup.dev brand.com

# With custom timeout
dotchk example.com --timeout 5000

# Export results
dotchk example.com test.io --output results.csv
```

### Pattern Discovery

Generate and check domains matching regex patterns. **Shows only available domains** for easier discovery.

```bash
# 3-letter .com domains
dotchk pattern "[a-z]{3}.com" --limit 100

# 4-letter .io domains
dotchk pattern "[a-z]{4}.io" --limit 50

# Short numeric domains
dotchk pattern "[a-z]{2,3}[0-9]{1,2}.com" --limit 100

# Tech startup names (get + word)
dotchk pattern "get[a-z]{4,6}.com" --limit 50

# App names with numbers
dotchk pattern "app[0-9]{2,3}.io" --limit 100

# AI-related domains
dotchk pattern "(ai|ml|gpt|llm)[a-z]{3,5}.ai" --limit 50

# Multiple prefixes
dotchk pattern "(get|try|use|my)[a-z]{4,8}.com" --limit 100

# Multiple prefixes and TLDs
dotchk pattern "(get|try|use)awesome.(com|io|app|dev)"

# With statistics
dotchk pattern "[a-z]{3}.io" --limit 50 --stats

# Export results
dotchk pattern "[a-z]{3,4}.ai" --limit 100 --output ai-domains.csv
```

**Pattern Syntax:**
- `[a-z]` - any letter (a-z)
- `[0-9]` - any digit (0-9)
- `{3}` - exactly 3 of preceding
- `{2,4}` - between 2 and 4 of preceding
- `(get|try)` - one of these options (alternation)

**Pattern Examples:**
- `[a-z]{3}` → abc, xyz, foo
- `[a-z]{2,4}` → ab, abc, abcd
- `[0-9]{3}` → 123, 456, 789
- `app[0-9]{2}` → app01, app99
- `(ai|ml)lab` → ailab, mllab
- `get[a-z]{4}.(com|io)` → getfood.com, getfood.io

### TLD Scanning

Check your brand name across different domain extensions:

```bash
# Popular TLDs (com, net, org, io, dev, app, co, me, ai, xyz, info, biz)
dotchk tld mybrand --popular

# Show only available
dotchk tld mybrand --popular --available-only

# Tech TLDs (io, dev, app, tech, cloud, ai, digital, etc.)
dotchk tld mybrand --tech

# Business TLDs (com, biz, business, company, corp, inc, etc.)
dotchk tld mybrand --business

# Creative TLDs (design, studio, art, media, etc.)
dotchk tld mybrand --creative

# Retail TLDs (shop, store, buy, market, etc.)
dotchk tld mybrand --retail

# Country codes (us, uk, de, fr, ca, au, jp, etc.)
dotchk tld mybrand --country

# Combine multiple groups
dotchk tld mybrand --tech --business

# Combine with available-only filter
dotchk tld mybrand --tech --business --available-only

# All 1,080+ public TLDs (takes longer)
dotchk tld mybrand --all

# With statistics
dotchk tld mybrand --tech --stats

# Specific TLDs only
dotchk tld startup --tlds com,io,dev,app,ai

# Export results
dotchk tld mybrand --popular --output brand-check.csv
```

**TLD Groups:**
- `--popular` - 12 most common (com, net, org, io, dev, app, co, me, ai, xyz, info, biz)
- `--tech` - 20 technology-related
- `--business` - 20 business-related
- `--creative` - 16 creative industry
- `--retail` - 15 e-commerce/retail
- `--country` - 42 popular country codes
- `--all` - 1,080+ public TLDs

### Bulk Checking

Check many domains from a file:

```bash
# Check all domains in file
dotchk bulk domains.txt

# Show only available
dotchk bulk domains.txt --available-only

# With statistics
dotchk bulk domains.txt --stats

# Export to CSV
dotchk bulk domains.txt --output results.csv

# Export with stats
dotchk bulk domains.txt --output results.csv --stats

# Increase parallelism for large lists
dotchk bulk domains.txt --parallel 500

# Increase timeout for slow TLDs
dotchk bulk domains.txt --timeout 5000
```

**File format** (one domain per line, # for comments):
```
example.com
mysite.io
startup.dev
# This is a comment
brand.com
techstartup.ai
```

## Options

- `--parallel N` - Concurrent checks (default: 100, max: 1000)
- `--timeout MS` - Query timeout (default: 3000ms)
- `--available-only` - Show only available
- `--output FILE` - Export to CSV
- `--stats` - Summary statistics
- `--limit N` - Max domains to generate (pattern)
- `--tlds LIST` - Comma-separated list (tld)

## Use Cases

**Finding startup domain:**
```bash
dotchk myawesomestartup.com
dotchk tld myawesomestartup --popular --available-only
dotchk pattern "(get|try|use)awesome.(com|io|app)"
```

**Brand protection:**
```bash
dotchk tld yourbrand --all --output brand-report.csv
```

**Domain investment:**
```bash
dotchk pattern "[a-z]{3}.io" --limit 100 --stats
```

## Limitations

Checks NS records for speed. False positives occur when domains are registered but have no nameservers configured.

Use for discovery and bulk scanning. Always verify with WHOIS before purchasing.

## Documentation

- [AI Assistant Guide](docs/ai-guide.md)

## Contributing

1. Fork the repository
2. Create feature branch
3. Make changes
4. Run `cargo test`
5. Submit pull request

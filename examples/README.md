# Domain Checker Examples

This directory contains example scripts and documentation for the dotchk tool.

## Documentation

- **[USAGE_GUIDE.md](USAGE_GUIDE.md)** - Comprehensive usage guide with detailed examples
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Quick reference card for common commands

## Scripts Overview

### 1. brand-search.sh
Search for available domains for your brand across multiple TLDs and variations.

```bash
./brand-search.sh mybrand
```

Features:
- Checks popular TLDs
- Checks tech and business focused TLDs
- Tests common variations (get*, try*, my*)
- Generates CSV report

### 2. find-short-domains.sh
Find available short domains (3-4 letters).

```bash
./find-short-domains.sh
```

Features:
- Searches for 3-letter .com and .io domains
- Finds pronounceable 4-letter combinations
- Searches number-letter combinations
- Exports results to CSV files

### 3. startup-name-generator.sh
Generate and check startup-style domain names.

```bash
./startup-name-generator.sh
```

Features:
- Common startup prefixes (get, try, use, my)
- Tech-style suffixes (tech, labs, hub, box)
- AI/ML themed domains
- Modern TLD combinations

### 4. monitor-domain.sh
Monitor a domain and get notified when it becomes available.

```bash
# Basic monitoring (check every hour)
./monitor-domain.sh desired-domain.com

# Check every 5 minutes
./monitor-domain.sh desired-domain.com 300

# With email notifications
./monitor-domain.sh desired-domain.com 3600 your@email.com
```

Features:
- Continuous monitoring
- Desktop notifications (macOS/Linux)
- Email alerts (if configured)
- Logging to file
- Checks related TLDs when domain becomes available

### 5. competitive-analysis.sh
Analyze domain portfolios of competitors.

```bash
./competitive-analysis.sh
```

Features:
- Checks multiple competitors across TLDs
- Tests common variations
- Exports detailed reports
- Identifies market gaps

### 6. bulk-check-template.txt
Template file for bulk domain checking.

```bash
dotchk bulk bulk-check-template.txt --stats --output results.csv
```

## Quick Start

1. Make scripts executable:
```bash
chmod +x *.sh
```

2. Run a script:
```bash
./brand-search.sh yourbrand
```

3. Customize the scripts for your needs by editing the variables at the top of each file.

## Advanced Usage

### Combining Scripts
You can combine multiple scripts for comprehensive analysis:

```bash
# First, find short available domains
./find-short-domains.sh

# Then monitor the best candidates
cat short-com-domains.csv | grep -E '^[aeiou]' | head -5 | while read domain; do
    ./monitor-domain.sh "$domain" 3600 &
done
```

### Custom Patterns
Create your own pattern scripts:

```bash
# Industry-specific domains
dotchk pattern "(cyber|security|protect|guard)[a-z]{2,4}\.(com|io)" --available-only

# Location-based
dotchk pattern "(sf|nyc|la|chi)[a-z]{3,6}\.com" --available-only

# Product names
dotchk pattern "(super|mega|pro|max)[a-z]{4,6}\.app" --available-only
```

### Automation
Schedule regular checks with cron:

```bash
# Add to crontab -e
# Check for available domains daily at 2 AM
0 2 * * * /path/to/find-short-domains.sh > /path/to/logs/daily-check.log 2>&1

# Monitor specific domain every hour
0 * * * * /path/to/monitor-domain.sh mydream.com 3600
```

## Performance Tips

1. **Use appropriate parallelism**:
   - For few domains: `--parallel 50`
   - For hundreds: `--parallel 200`
   - For thousands: `--parallel 500`

2. **Set realistic timeouts**:
   - Fast TLDs (.com, .net): `--timeout 1000`
   - Slower TLDs: `--timeout 3000`
   - International: `--timeout 5000`

3. **Filter results**:
   - Use `--available-only` to reduce output
   - Export to CSV for further analysis
   - Use `grep` for specific patterns

## Export Format

All CSV exports follow this format:
```csv
domain,available,response_time_ms,checked_at,error
example.com,false,67,2024-03-20T10:30:45Z,
mybrand.io,true,123,2024-03-20T10:30:45Z,
```

You can process these with standard tools:
```bash
# Sort by response time
sort -t, -k3 -n results.csv

# Find fastest available domains
awk -F, '$2=="true" {print $1","$3}' results.csv | sort -t, -k2 -n

# Count by TLD
cut -d. -f2 results.csv | sort | uniq -c | sort -nr
```
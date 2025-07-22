# dotchk

A high-performance domain availability checker that helps you find the perfect domain name. Check thousands of domains per second across 1,080+ public TLDs.

> **Important**: This tool checks NS (nameserver) records, not WHOIS. Domains registered without nameservers may show as "available". Always verify with WHOIS before purchasing.

## Table of Contents
- [Features](#features)
- [Installation](#installation)
- [Quick Start](#quick-start)
- [Commands](#commands)
- [Examples](#examples)
- [Performance](#performance)
- [Limitations](#limitations)
- [Contributing](#contributing)
- [License](#license)

## Features

- **Fast** - Check 10,000+ domains per second
- **Direct checking** - Just type `dotchk example.com`
- **Pattern discovery** - Find available domains matching patterns
- **1,080+ TLDs** - Comprehensive public TLD support
- **Bulk checking** - Process large lists efficiently
- **Beautiful output** - Clean, colorful, and informative
- **CSV export** - Export results for analysis

## Installation

### Quick Install (Recommended)

1. **Clone the repository**:
   ```bash
   git clone https://github.com/dotchk/dotchk.git
   cd dotchk
   ```

2. **Run the install script**:

   **macOS/Linux:**
   ```bash
   ./install.sh
   ```

   **Windows (PowerShell):**
   ```powershell
   powershell -ExecutionPolicy Bypass -File install.ps1
   ```

The install script will:
- Install Rust if needed
- Build and install dotchk
- Verify everything is working

### Manual Installation

If you prefer to install manually:

1. **Install Rust** (if not already installed):
   ```bash
   # On macOS/Linux:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   
   # On Windows:
   # Download and run: https://win.rustup.rs
   ```

2. **Install dotchk**:
   ```bash
   cargo install --path .
   ```

3. **Verify installation**:
   ```bash
   dotchk --version
   ```

### Alternative: Download Pre-built Binary
*Coming soon: Pre-built binaries for Windows, macOS, and Linux*

## Quick Start

### Check if a domain is available
```bash
dotchk example.com
```

### Check multiple domains
```bash
dotchk example.com mysite.io startup.dev
```

### Find available domains for your brand
```bash
dotchk tld mybrand --popular --available-only
```

### Discover short domain names
```bash
dotchk pattern "[a-z]{3}.com" --limit 100
```

## Commands

### 1. Direct Domain Checking (Simplest!)

Just type domain names after `dotchk`:

```bash
# Check one domain
dotchk example.com

# Check multiple domains
dotchk example.com test.io startup.dev
```

**Output:**
```
example.com → TAKEN (47ms)
test.io → TAKEN (92ms) 
startup.dev → AVAILABLE (38ms)
myapp.xyz → TAKEN (156ms)
coolproject.app → AVAILABLE (41ms)
```

### 2. Pattern-Based Discovery

Find available domains matching patterns:

```bash
# Find 3-letter .com domains
dotchk pattern "[a-z]{3}.com" --limit 10
```

**Output:**
```
Searching for pattern: [a-z]{3}.com
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 10/10

aaa.com → TAKEN (45ms)
aab.com → TAKEN (52ms)
aac.com → TAKEN (48ms)
aad.com → TAKEN (61ms)
aae.com → TAKEN (44ms)
aaf.com → AVAILABLE (39ms) ✓
aag.com → TAKEN (55ms)
aah.com → TAKEN (42ms)
aai.com → TAKEN (49ms)
aaj.com → AVAILABLE (38ms) ✓

✓ Found 2 available domains
```

```bash

# Tech startup names (get + 4 letters)
dotchk pattern "get[a-z]{4}.com" --limit 50

# App names with numbers
dotchk pattern "app[0-9]{3}.io" --limit 100
```

**Pattern Syntax:**
- `[a-z]` - Any lowercase letter
- `[A-Z]` - Any uppercase letter
- `[0-9]` - Any digit
- `{3}` - Exactly 3 characters
- `{2,4}` - Between 2 and 4 characters
- `(a|b|c)` - Alternation - one of the options

### 3. TLD Scanning

Check your brand across multiple domain extensions:

```bash
# Check popular TLDs (com, net, org, io, dev, app, co, me, ai, xyz, info, biz)
dotchk tld mybrand --popular

# Check specific TLDs
dotchk tld startup --tlds com,io,dev,app

# Show only available TLDs
dotchk tld mybrand --popular --available-only
```

**Output with --tech flag:**
```
Checking mybrand across tech TLDs...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 20/20

mybrand.io → TAKEN (89ms)
mybrand.dev → AVAILABLE (42ms) ✓
mybrand.app → AVAILABLE (38ms) ✓
mybrand.tech → TAKEN (156ms)
mybrand.cloud → AVAILABLE (44ms) ✓
mybrand.ai → TAKEN (67ms)
mybrand.digital → AVAILABLE (51ms) ✓
mybrand.software → TAKEN (48ms)
mybrand.systems → AVAILABLE (40ms) ✓
mybrand.network → AVAILABLE (39ms) ✓
... (10 more)

✓ Found 6 available domains
```

```bash
# Check with statistics
dotchk tld mybrand --tech --business --stats
```

**Output with --stats:**
```
Checking mybrand across tech and business TLDs...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 40/40

[mybrand]
mybrand.com → TAKEN (45ms)
mybrand.io → TAKEN (89ms)
mybrand.dev → AVAILABLE (42ms) ✓
mybrand.app → AVAILABLE (38ms) ✓
mybrand.tech → TAKEN (156ms)
mybrand.ai → TAKEN (67ms)
mybrand.biz → AVAILABLE (51ms) ✓
mybrand.business → AVAILABLE (48ms) ✓
mybrand.company → TAKEN (44ms)
mybrand.inc → AVAILABLE (39ms) ✓
... (30 more)

╭─────────────────────────────────────╮
│           Summary Stats             │
├─────────────────────────────────────┤
│ Total checked      : 40             │
│ Available          : 15 (37.5%)     │
│ Taken              : 24 (60.0%)     │
│ Errors             : 1 (2.5%)       │
│                                     │
│ Registered in 24 of 40 TLDs (60.0%) │
│                                     │
│ Avg response time  : 72ms           │
│ Min response time  : 38ms           │
│ Max response time  : 156ms          │
│ Total time         : 1.8s           │
╰─────────────────────────────────────╯
```

```bash
# Check ALL 1,080+ public TLDs (excludes private, adult, gambling TLDs)
dotchk tld uniquename --all --available-only
```

**TLD Group Options:**
```bash
# Check tech-related TLDs
dotchk tld mybrand --tech

# Check business TLDs  
dotchk tld mybrand --business

# Check creative industry TLDs
dotchk tld mybrand --creative

# Check retail/shopping TLDs
dotchk tld mybrand --retail

# Check popular country TLDs
dotchk tld mybrand --country

# Combine multiple groups
dotchk tld mybrand --tech --business --available-only
```

**TLD Groups:**
- `--popular` - Most common TLDs (com, net, org, io, dev, app, co, me, ai, xyz, info, biz)
- `--tech` - Technology TLDs (io, dev, app, tech, cloud, ai, digital, software, etc.)
- `--business` - Business TLDs (com, biz, business, company, enterprises, corp, inc, etc.)
- `--creative` - Creative industry TLDs (design, studio, art, media, photography, etc.)
- `--retail` - E-commerce TLDs (shop, store, buy, sale, market, boutique, etc.)
- `--country` - Popular country codes (us, uk, de, fr, ca, au, jp, br, etc.)

### 4. Bulk Checking

Check many domains from a file:

```bash
# Check all domains in file
dotchk bulk domains.txt

# Show only available domains
dotchk bulk domains.txt --available-only

# Export results to CSV
dotchk bulk domains.txt --output results.csv --stats
```

**File format** (domains.txt):
```
example.com
mysite.io  
startup.dev
# Comments are ignored
brand.com
techstartup.ai
```

**Output:**
```
Checking 5 domains...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 5/5

example.com → TAKEN (45ms)
mysite.io → AVAILABLE (92ms) ✓
startup.dev → TAKEN (38ms)
brand.com → TAKEN (67ms)
techstartup.ai → TIMEOUT (3000ms) ⚠

✓ Found 1 available domain
⚠ 1 domain timed out

Exported results to results.csv
```

## Examples

### Finding a Domain for Your Startup

```bash
# 1. Check if your preferred name is available
dotchk myawesomestartup.com

# 2. If taken, check variations across popular TLDs
dotchk tld myawesomestartup --popular --available-only

# 3. Try prefixes/suffixes
dotchk getmyawesomestartup.com trymyawesomestartup.com

# 4. Find similar short names
dotchk pattern "[a-z]{4}startup.com" --limit 100

# 5. Try multiple prefixes and extensions at once
dotchk pattern "(get|try|use|my)awesome.(com|io|app)"
```

**Output:**
```
Generating domains from pattern...
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 12/12

getawesome.com → TAKEN (67ms)
tryawesome.com → AVAILABLE (45ms) ✓
useawesome.com → TAKEN (52ms)
myawesome.com → TAKEN (89ms)
getawesome.io → TAKEN (94ms)
tryawesome.io → AVAILABLE (41ms) ✓  
useawesome.io → AVAILABLE (38ms) ✓
myawesome.io → TAKEN (156ms)
getawesome.app → AVAILABLE (44ms) ✓
tryawesome.app → AVAILABLE (39ms) ✓
useawesome.app → TAKEN (48ms)
myawesome.app → AVAILABLE (42ms) ✓

✓ Found 6 available domains
```

### Brand Protection

```bash
# Check your brand across all major TLDs
dotchk tld yourbrand --all --output brand-report.csv

# Monitor competitor domains
echo "competitor1.com
competitor2.net
competitor3.io" > competitors.txt
dotchk bulk competitors.txt
```

### Domain Investment

```bash
# Find valuable 3-letter domains
dotchk pattern "[a-z]{3}.io" --limit 50 --stats
```

**Output with --stats:**
```
Searching for pattern: [a-z]{3}.io
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ 100% 50/50

aaa.io → TAKEN (89ms)
aab.io → TAKEN (92ms)
aac.io → TIMEOUT (3000ms) ⚠
aad.io → AVAILABLE (45ms) ✓
aae.io → ERROR: Invalid domain ⚠
... (45 more results)

╭─────────────────────────────────────╮
│           Summary Stats             │
├─────────────────────────────────────┤
│ Total checked      : 50             │
│ Available          : 8 (16%)        │
│ Taken              : 39 (78%)       │
│ Errors             : 3 (6%)         │
│                                     │
│ Avg response time  : 127ms          │
│ Min response time  : 38ms           │
│ Max response time  : 3000ms         │
│ Total time         : 2.4s           │
╰─────────────────────────────────────╯
```

```bash
# Check trending keywords
for keyword in ai ml crypto web3 defi nft; do
  dotchk tld $keyword --popular --available-only
done
```

### Monitoring a Domain

Create a simple monitoring script:

```bash
#!/bin/bash
# Save as: monitor-domain.sh
while true; do
  if dotchk desired-domain.com | grep -q "AVAILABLE"; then
    echo "desired-domain.com is available!"
    # Add notification here (email, desktop notification, etc.)
  fi
  sleep 3600  # Check every hour
done
```

## Performance

### Speed Benchmarks
- **Single domain**: ~58ms
- **Parallel checks**: ~10ms per domain
- **Pattern generation**: ~42μs for 1000 domains
- **Theoretical max**: ~9,268 domains/second

### Optimization Tips

1. **Adjust parallelism based on your needs:**
   - Small checks (<100 domains): `--parallel 100` (default for most commands)
   - TLD checks: `--parallel 200` (default for `tld` command)
   - Medium checks (100-1000): `--parallel 200`
   - Large checks (1000+): `--parallel 500`
   - Massive checks: `--parallel 1000`

2. **Timeout settings:**
   - Default timeout: `3000ms` (3 seconds) for most commands
   - `check` command default: `500ms` (optimized for single domains)
   - Fast checks: `--timeout 1000` (for well-connected servers)
   - Slow/problematic TLDs: `--timeout 5000` or higher

3. **Memory usage:**
   - The tool caches results in memory
   - For very large checks (>1M domains), process in batches

## Command Options

### Global Options
- `-q, --quiet` - Minimal output (errors only)
- `-h, --help` - Show help information
- `-V, --version` - Show version

### Performance Options
- `--parallel N` - Number of concurrent checks (default: 100, max: 1000)
- `--timeout MS` - Query timeout in milliseconds (default: 3000)

### Output Options
- `--available-only` - Only show available domains
- `--output FILE` - Export results to CSV file
- `--stats` - Show summary statistics

### Pattern Options
- `--limit N` - Maximum domains to generate

### TLD Options
- `--popular` - Check popular TLDs (com, net, org, io, dev, app, co, me, ai, xyz, info, biz)
- `--tech` - Check technology-related TLDs (20 TLDs)
- `--business` - Check business TLDs (20 TLDs)
- `--creative` - Check creative industry TLDs (16 TLDs)  
- `--retail` - Check e-commerce/retail TLDs (15 TLDs)
- `--country` - Check popular country code TLDs (42 countries)
- `--tlds LIST` - Check specific TLDs (comma-separated)
- `--all` - Check all 1,080+ public TLDs (excludes private, adult, gambling, religious, and non-ASCII TLDs)

## CSV Export Format

Results are exported in this format:
```csv
domain,available,response_time_ms,checked_at,error
example.com,false,67,2024-03-20T10:30:45Z,
mybrand.io,true,123,2024-03-20T10:30:45Z,
invalid.xyz,false,0,2024-03-20T10:30:45Z,Unsupported TLD
```

## Limitations

### Important: NS Records vs WHOIS

This tool checks NS (nameserver) records for speed. This means:

**False Positives - These will show as "available" but aren't:**
- Newly registered domains without DNS setup
- Parked domains without nameservers
- Domains with removed DNS configuration

**When to Use This Tool:**
- Initial availability scanning
- Bulk domain discovery
- Pattern-based searching
- Quick availability checks

**When NOT to Use:**
- Final purchase decisions
- Legal verification
- 100% accuracy requirements

**Always verify with WHOIS before attempting to register any domain.**

## Troubleshooting

### Common Issues

**"Domain shows available but I can't register it"**
- The domain is registered but has no nameservers
- Always verify with WHOIS before purchasing

**Example of edge cases:**
```bash
$ dotchk edge-cases.com newly-registered.io parked-domain.net

edge-cases.com → AVAILABLE (45ms) ✓
newly-registered.io → AVAILABLE (89ms) ✓  # May be false positive!
parked-domain.net → AVAILABLE (67ms) ✓    # May be false positive!
```

**"Getting timeout errors"**
```bash
$ dotchk problematic.no difficult.cn slowserver.xyz

problematic.no → TIMEOUT (3000ms) ⚠
difficult.cn → TIMEOUT (3000ms) ⚠  
slowserver.xyz → TAKEN (2847ms)    # Almost timed out
```

Solutions:
- Increase timeout: `--timeout 5000`
- Reduce parallelism: `--parallel 50`
- Check your internet connection

**"Too many errors"**
- You may be rate-limited
- Reduce parallelism: `--parallel 25`
- Add delays between large batches

### Debug Mode

Enable detailed logging:
```bash
# Debug level
RUST_LOG=debug dotchk example.com

# Trace level (very verbose)
RUST_LOG=trace dotchk example.com
```

## Library Usage

For developers who want to use dotchk as a Rust library:

Add to your `Cargo.toml`:
```toml
[dependencies]
dotchk = "1.3"
tokio = { version = "1", features = ["full"] }
```

Example usage:
```rust
use dotchk::{Checker, Pattern, CheckResult};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a checker with custom settings
    let checker = Checker::builder()
        .max_parallel(200)
        .timeout_ms(500)  // Default timeout
        .build()
        .await?;
    
    // Check single domain
    let result = checker.check("example.com").await;
    println!("{}: {}", result.domain, 
        if result.available { "AVAILABLE" } else { "TAKEN" }
    );
    
    // Pattern matching
    let pattern = Pattern::compile("[a-z]{3}.io")?;
    let domains = pattern.generate(Some(100));
    let results = checker.check_batch(domains).await;
    
    // Filter available domains
    let available: Vec<CheckResult> = results
        .into_iter()
        .filter(|r| r.available && r.error.is_none())
        .collect();
    
    println!("Found {} available domains", available.len());
    
    // Export results to CSV
    use dotchk::CsvExporter;
    let exporter = CsvExporter::new("results.csv");
    exporter.export(&available)?;
    
    Ok(())
}
```

## Contributing

We welcome contributions! Please see [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

### Quick Contribution Guide
1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Run tests: `cargo test`
5. Submit a pull request

## License

This software is licensed under the PolyForm Noncommercial License 1.0.0.

- **Free for personal use, research, education, and non-profits**
- **Commercial use requires a license**

For commercial licensing, please contact: [your-email@example.com]

See [LICENSE](LICENSE) file for full terms.

---

**Remember**: This tool is for discovery and research. Always verify availability with official WHOIS before attempting to register any domain.
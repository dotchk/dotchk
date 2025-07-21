# dotchk

A high-performance domain availability checker that helps you find the perfect domain name. Check thousands of domains per second across 1,440+ TLDs.

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
- **1,440+ TLDs** - Comprehensive TLD support
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
```

### 2. Pattern-Based Discovery

Find available domains matching patterns. **Only shows available domains**:

```bash
# Find 3-letter .com domains
dotchk pattern "[a-z]{3}.com" --limit 100

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

# Check ALL 1,440+ TLDs (comprehensive but slower)
dotchk tld uniquename --all --available-only
```

**Popular TLD Groups:**
- **Tech**: io, dev, app, tech, cloud, ai, digital
- **Business**: com, biz, business, company, enterprises
- **Creative**: design, studio, art, media, digital
- **Countries**: uk, de, fr, es, it, jp, au, ca

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
dotchk pattern "(get|try|use|my)awesome.(com|io|app)" --limit 10
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
dotchk pattern "[a-z]{3}.io" --limit 1000

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
   - Small checks (<100 domains): `--parallel 100` (default)
   - Medium checks (100-1000): `--parallel 200`
   - Large checks (1000+): `--parallel 500`
   - Massive checks: `--parallel 1000`

2. **Timeout settings:**
   - Fast TLDs (.com, .net): `--timeout 1000` (default)
   - International TLDs: `--timeout 3000`
   - Slow/problematic TLDs: `--timeout 5000`

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
- `--timeout MS` - Query timeout in milliseconds (default: 1000)

### Output Options
- `--available-only` - Only show available domains
- `--output FILE` - Export results to CSV file
- `--stats` - Show summary statistics

### Pattern Options
- `--limit N` - Maximum domains to generate

### TLD Options
- `--popular` - Check popular TLDs only
- `--tlds LIST` - Check specific TLDs (comma-separated)
- `--all` - Check all 1,440+ TLDs

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

**"Getting timeout errors"**
- Increase timeout: `--timeout 3000`
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

For developers who want to use this as a Rust library:

```rust
use domain_checker::{Checker, Pattern};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let checker = Checker::builder()
        .max_parallel(200)
        .timeout_ms(2000)
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
    
    // Find available only
    let available: Vec<_> = results
        .into_iter()
        .filter(|r| r.available && r.error.is_none())
        .collect();
    
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
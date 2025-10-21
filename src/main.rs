use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

mod cli;

/// Check if a string looks like a pattern (contains [], {}, etc.)
fn is_pattern(s: &str) -> bool {
    s.contains('[') || s.contains('{') || s.contains('(') || s.contains('|')
}

#[derive(Parser)]
#[command(name = "dotchk")]
#[command(about = "High-performance domain availability checker", long_about = None)]
#[command(
    after_help = r#"NOTE: This tool uses NS record queries. Domains registered without nameservers may show as 'available'. Always verify with WHOIS before purchasing.

QUICK EXAMPLES:
    # Check if domains are available
    dotchk example.com mysite.org

    # Pattern detection - automatically finds available domains
    dotchk "test[a-z]{3}.com" "site[0-9]{2}.io"

    # Find available domains across popular TLDs
    dotchk tld mybrand --popular

    # Generate and check 3-letter .com domains (shows only available)
    dotchk pattern "[a-z]{3}\.com" --limit 100

    # Bulk check from file
    dotchk bulk domains.txt --stats

For more examples: https://github.com/dotchk/dotchk/tree/main/examples"#
)]
struct Cli {
    /// Run in quiet mode (only show essential output)
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Domains to check directly (if no subcommand is provided)
    #[arg(required = false)]
    domains: Vec<String>,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Check if domains are available (explicit command)
    #[command(
        about = "Check if domains are available",
        long_about = "Check domain availability by querying nameserver records at authoritative TLD servers.",
        after_help = r#"EXAMPLES:
    # Check single domain
    domain-checker check example.com
    
    # Check multiple domains
    domain-checker check site1.com site2.net site3.org
    
    # Export results to CSV
    domain-checker check domain.com --output results.csv
    
    # Fast check with custom timeout
    domain-checker check mybrand.com --timeout 500 --parallel 200"#
    )]
    Check {
        /// Domains to check (e.g., example.com mysite.org)
        domains: Vec<String>,

        /// Maximum parallel queries (higher = faster, but may hit rate limits)
        #[arg(long, default_value = "100")]
        parallel: usize,

        /// Query timeout in milliseconds (increase for slow TLDs)
        #[arg(long, default_value = "500")]
        timeout: u64,

        /// Export results to CSV file
        #[arg(long, short)]
        output: Option<PathBuf>,
    },

    /// Generate and check domains from a pattern (shows only available)
    #[command(
        about = "Generate and check domains from a pattern",
        long_about = "Generate domain names using patterns and check their availability. Only shows available domains.",
        after_help = r#"PATTERN SYNTAX:
    [a-z]     - Lowercase letters
    [A-Z]     - Uppercase letters  
    [0-9]     - Digits
    {{n}}     - Exactly n characters
    {{n,m}}   - Between n and m characters
    .tld      - Dots before common TLDs (com, net, org, io, etc.) are auto-escaped
    \.        - Manual escape for literal dots in other positions

EXAMPLES:
    # Find available 3-letter .com domains
    domain-checker pattern "[a-z]{3}.com" --limit 100
    
    # Find available tech startup style domains
    domain-checker pattern "tech-[a-z]{4}.io"
    
    # Available numbered domains
    domain-checker pattern "site[0-9]{3}.com" --limit 50
    
    # Export available domains with statistics
    domain-checker pattern "(get|try|use)[a-z]{4}.com" --limit 1000 --output results.csv --stats"#
    )]
    Pattern {
        /// Pattern to generate domains (e.g., "tech-[a-z]{4}.com")
        pattern: String,

        /// Limit number of domains to generate (default: all combinations)
        #[arg(long)]
        limit: Option<usize>,

        /// Maximum parallel queries
        #[arg(long, default_value = "100")]
        parallel: usize,

        /// Query timeout in milliseconds
        #[arg(long, default_value = "3000")]
        timeout: u64,

        /// Export results to CSV file
        #[arg(long, short)]
        output: Option<PathBuf>,

        /// Show statistics at the end
        #[arg(long)]
        stats: bool,
    },

    /// Bulk check domains from a file
    #[command(
        about = "Bulk check domains from a file",
        long_about = "Check availability of domains listed in a text file (one domain per line).",
        after_help = r#"FILE FORMAT:
    # Comments start with #
    example.com
    mysite.net
    testdomain.org
    
    # Empty lines are ignored
    another-domain.io

EXAMPLES:
    # Basic bulk check
    domain-checker bulk domains.txt
    
    # Show statistics
    domain-checker bulk domains.txt --stats
    
    # High-performance check with export
    domain-checker bulk large-list.txt --parallel 500 --output results.csv
    
    # Check with custom timeout for international domains
    domain-checker bulk intl-domains.txt --timeout 3000"#
    )]
    Bulk {
        /// File containing domains (one per line)
        file: PathBuf,

        /// Maximum parallel queries
        #[arg(long, default_value = "100")]
        parallel: usize,

        /// Query timeout in milliseconds
        #[arg(long, default_value = "3000")]
        timeout: u64,

        /// Only show available domains
        #[arg(long)]
        available_only: bool,

        /// Export results to CSV file
        #[arg(long, short)]
        output: Option<PathBuf>,

        /// Show statistics at the end
        #[arg(long)]
        stats: bool,
    },

    /// Check domain availability across multiple TLDs
    #[command(
        about = "Check domain availability across multiple TLDs",
        long_about = "Check if a domain name is available across different top-level domains (TLDs).",
        after_help = r#"TLD OPTIONS:
    --popular  : com, net, org, io, dev, app, co, me, ai, xyz, info, biz (12 TLDs)
    --tech     : io, dev, app, tech, cloud, ai, digital, software, etc. (20 TLDs)
    --business : com, biz, business, company, enterprises, corp, inc, etc. (20 TLDs)
    --creative : design, studio, art, media, photography, video, etc. (16 TLDs)
    --retail   : shop, store, buy, sale, market, boutique, etc. (15 TLDs)
    --country  : us, uk, de, fr, ca, au, jp, br, etc. (42 popular countries)
    --tlds     : Specify custom TLDs to check
    --all      : Check all 1,080+ public TLDs (excludes private/adult/gambling)
    
    Default: If no option specified, checks popular TLDs

EXAMPLES:
    # Check popular TLDs for your brand
    domain-checker tld mybrand --popular
    
    # Check tech-related TLDs
    domain-checker tld startup --tech
    
    # Check business TLDs
    domain-checker tld company --business
    
    # Check multiple groups (combine flags)
    domain-checker tld mybrand --tech --business
    
    # Check specific TLDs
    domain-checker tld startup --tlds com,io,dev,app
    
    # Find only available domains
    domain-checker tld brandname --popular --available-only
    
    # Export results with statistics
    domain-checker tld brandname --all --output tlds.csv --stats --parallel 500"#
    )]
    Tld {
        /// Domain names to check (without TLD, e.g., "mybrand" not "mybrand.com")
        domains: Vec<String>,

        /// Check popular TLDs (com, net, org, io, dev, app, co, me, ai, xyz, info, biz)
        #[arg(long, conflicts_with = "tlds", conflicts_with = "all")]
        popular: bool,

        /// Check tech-related TLDs (io, dev, app, tech, cloud, ai, digital, software, etc.)
        #[arg(long, conflicts_with = "tlds", conflicts_with = "all")]
        tech: bool,

        /// Check business TLDs (com, biz, business, company, enterprises, corp, inc, etc.)
        #[arg(long, conflicts_with = "tlds", conflicts_with = "all")]
        business: bool,

        /// Check creative TLDs (design, studio, art, media, photography, video, etc.)
        #[arg(long, conflicts_with = "tlds", conflicts_with = "all")]
        creative: bool,

        /// Check retail/shopping TLDs (shop, store, buy, sale, market, boutique, etc.)
        #[arg(long, conflicts_with = "tlds", conflicts_with = "all")]
        retail: bool,

        /// Check popular country TLDs (us, uk, de, fr, ca, au, jp, br, etc.)
        #[arg(long, conflicts_with = "tlds", conflicts_with = "all")]
        country: bool,

        /// Specific TLDs to check (comma-separated, e.g., com,io,dev)
        #[arg(
            long,
            value_delimiter = ',',
            conflicts_with = "popular",
            conflicts_with = "all"
        )]
        tlds: Option<Vec<String>>,

        /// Check all 1,440+ available TLDs (may take considerable time)
        #[arg(long, conflicts_with = "popular", conflicts_with = "tlds")]
        all: bool,

        /// Maximum parallel queries
        #[arg(long, default_value = "200")]
        parallel: usize,

        /// Query timeout in milliseconds
        #[arg(long, default_value = "3000")]
        timeout: u64,

        /// Only show available domains
        #[arg(long)]
        available_only: bool,

        /// Export results to CSV file
        #[arg(long, short)]
        output: Option<PathBuf>,

        /// Show statistics at the end
        #[arg(long)]
        stats: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Set quiet mode if requested
    if cli.quiet {
        cli::output::set_quiet_mode(true);
    }

    // Initialize tracing only if not in quiet mode
    if !cli.quiet {
        tracing_subscriber::registry()
            .with(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "dotchk=info".into()),
            )
            .with(tracing_subscriber::fmt::layer())
            .init();
    } else {
        // Initialize with no output in quiet mode
        tracing_subscriber::registry()
            .with(tracing_subscriber::EnvFilter::new("off"))
            .init();
    }

    // Handle direct domain checking without subcommand
    match cli.command {
        None => {
            // If no subcommand but we have domains, check if any are patterns
            if !cli.domains.is_empty() {
                // Separate patterns from regular domains
                let mut patterns = Vec::new();
                let mut regular_domains = Vec::new();

                for domain in cli.domains {
                    if is_pattern(&domain) {
                        patterns.push(domain);
                    } else {
                        regular_domains.push(domain);
                    }
                }

                // Handle patterns first (they show only available domains)
                for pattern in patterns {
                    cli::check_pattern(pattern, Some(10000), 100, 500, true, None, false).await?;
                }

                // Then handle regular domains
                if !regular_domains.is_empty() {
                    cli::check_domains(regular_domains, 100, 5000, false, None).await?;
                }
            } else {
                // No domains and no subcommand, show help
                use clap::CommandFactory;
                Cli::command().print_help()?;
            }
        }
        Some(Commands::Check {
            domains,
            parallel,
            timeout,
            output,
        }) => {
            // Show all results for check command
            cli::check_domains(domains, parallel, timeout, false, output).await?;
        }
        Some(Commands::Pattern {
            pattern,
            limit,
            parallel,
            timeout,
            output,
            stats,
        }) => {
            // ALWAYS show only available for patterns
            cli::check_pattern(pattern, limit, parallel, timeout, true, output, stats).await?;
        }
        Some(Commands::Bulk {
            file,
            parallel,
            timeout,
            available_only,
            output,
            stats,
        }) => {
            cli::bulk_check(file, parallel, timeout, available_only, output, stats).await?;
        }
        Some(Commands::Tld {
            domains,
            popular,
            tech,
            business,
            creative,
            retail,
            country,
            tlds,
            all,
            parallel,
            timeout,
            available_only,
            output,
            stats,
        }) => {
            cli::check_tlds(
                domains,
                popular,
                tech,
                business,
                creative,
                retail,
                country,
                tlds,
                all,
                parallel,
                timeout,
                available_only,
                output,
                stats,
            )
            .await?;
        }
    }

    Ok(())
}

//! High-performance domain availability checker
//!
//! `dotchk` checks domain availability by querying DNS NS (nameserver) records directly,
//! making it much faster than traditional WHOIS lookups. Domains without NS records are
//! considered potentially available.
//!
//! # Features
//!
//! - **Fast**: Parallel DNS queries with configurable concurrency
//! - **Accurate**: Queries authoritative TLD nameservers directly
//! - **Pattern-based**: Generate and check domains matching patterns like `[a-z]{3}.com`
//! - **Bulk checking**: Process lists of domains from files
//! - **Statistics**: Track response times, availability rates
//! - **Export**: Save results to CSV
//!
//! # Examples
//!
//! ## Basic domain check
//!
//! ```rust
//! use dotchk::Checker;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let checker = Checker::builder()
//!     .max_parallel(100)?
//!     .timeout_ms(5000)?
//!     .build()
//!     .await?;
//!
//! let result = checker.check("example.com").await?;
//! println!("{} is {}", result.domain, if result.available { "available" } else { "taken" });
//! # Ok(())
//! # }
//! ```
//!
//! ## Pattern-based domain generation
//!
//! ```rust
//! use dotchk::Pattern;
//!
//! # fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Generate domains matching pattern
//! let pattern = Pattern::compile("[a-z]{3}.io")?;
//! let domains: Vec<String> = pattern.generate(Some(100));
//! println!("Generated {} domains", domains.len());
//! # Ok(())
//! # }
//! ```
//!
//! ## Batch checking with statistics
//!
//! ```rust
//! use dotchk::{Checker, StatsExporter};
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! let checker = Checker::builder()
//!     .max_parallel(200)?
//!     .build()
//!     .await?;
//!
//! let domains = vec!["test1.com".to_string(), "test2.com".to_string()];
//! let results = checker.check_batch(domains).await;
//!
//! let stats = StatsExporter::calculate_stats(&results);
//! println!("Checked: {}, Available: {}",
//!     stats.total, stats.available);
//! # Ok(())
//! # }
//! ```
//!
//! # Note
//!
//! A domain showing "available" means no NS records were found. Always verify with an
//! official registrar before purchasing, as DNS may not reflect real-time registration status.

pub mod checker;
pub mod domain;
pub mod export;
pub mod pattern;
pub mod tld;

pub(crate) mod dns_batch;
pub(crate) mod dns_pipelined;
pub(crate) mod tld_registry;

pub use checker::{CheckResult, Checker};
pub use domain::Domain;
pub use export::{CsvExporter, StatsExporter};
pub use pattern::Pattern;
pub use tld::Tld;
pub use tld_registry::{get_public_tlds, get_tld_info, TLD_SERVERS};

use thiserror::Error;

/// Errors that can occur during domain availability checking.
#[derive(Error, Debug)]
pub enum DomainCheckerError {
    #[error("DNS query failed: {0}")]
    Dns(#[from] dns_pipelined::DnsError),

    #[error("Pattern syntax error: {0}")]
    Pattern(#[from] pattern::PatternError),

    #[error("Export failed: {0}")]
    Export(#[from] export::ExportError),

    #[error("Invalid domain '{0}': must be 1-253 chars, valid labels (max 63 chars each), format: label.label.tld")]
    InvalidDomain(String),

    #[error("TLD '.{0}' not supported or has no authoritative servers configured. Check https://www.iana.org/domains/root/db for valid TLDs")]
    UnsupportedTld(String),

    #[error("Query timeout: DNS server did not respond within timeout period. Try increasing --timeout")]
    Timeout,

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DomainCheckerError>;

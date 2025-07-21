pub mod checker;
pub mod dns_batch;
pub mod dns_pipelined;
pub mod export;
pub mod pattern;
pub mod tld;

pub use checker::{CheckResult, Checker};
pub use export::{CsvExporter, StatsExporter};
pub use pattern::Pattern;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum DomainCheckerError {
    #[error("DNS error: {0}")]
    Dns(#[from] dns_pipelined::DnsError),

    #[error("Pattern error: {0}")]
    Pattern(#[from] pattern::PatternError),

    #[error("Export error: {0}")]
    Export(#[from] export::ExportError),

    #[error("Invalid domain: {0}")]
    InvalidDomain(String),

    #[error("Unsupported TLD: {0}")]
    UnsupportedTld(String),

    #[error("Timeout")]
    Timeout,

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, DomainCheckerError>;

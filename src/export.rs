use crate::checker::CheckResult;
use crate::DomainCheckerError;
use csv::Writer;
use std::path::Path;
use thiserror::Error;

/// Export module Result type alias
pub type Result<T> = std::result::Result<T, ExportError>;

/// Errors that can occur during export operations
#[derive(Error, Debug)]
pub enum ExportError {
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// CSV exporter for domain check results
pub struct CsvExporter {
    path: String,
}

impl CsvExporter {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_string_lossy().to_string(),
        }
    }

    pub fn export(&self, results: &[std::result::Result<CheckResult, DomainCheckerError>]) -> Result<()> {
        let mut wtr = Writer::from_path(&self.path)?;

        // Write header
        wtr.write_record(["domain", "available", "error"])?;

        // Write records
        for result in results {
            match result {
                Ok(check) => {
                    wtr.write_record([&check.domain, &check.available.to_string(), ""])?;
                }
                Err(e) => {
                    // For errors, we don't know the domain unless we parse the error
                    // For now, skip or we could extract domain from error message
                    wtr.write_record(["unknown", "false", &e.to_string()])?;
                }
            }
        }

        wtr.flush()?;
        Ok(())
    }

    pub async fn export_stream<S>(&self, mut results: S) -> Result<usize>
    where
        S: futures::Stream<Item = std::result::Result<CheckResult, DomainCheckerError>> + Unpin,
    {
        use futures::StreamExt;

        let mut wtr = Writer::from_path(&self.path)?;
        let mut count = 0;

        // Write header
        wtr.write_record(["domain", "available", "error"])?;

        // Write records as they come in
        while let Some(result) = results.next().await {
            match result {
                Ok(check) => {
                    wtr.write_record([&check.domain, &check.available.to_string(), ""])?;
                }
                Err(e) => {
                    wtr.write_record(["unknown", "false", &e.to_string()])?;
                }
            }
            count += 1;

            // Flush periodically for large streams
            if count % 1000 == 0 {
                wtr.flush()?;
            }
        }

        wtr.flush()?;
        Ok(count)
    }

    pub fn export_available_only(&self, results: &[std::result::Result<CheckResult, DomainCheckerError>]) -> Result<()> {
        let mut wtr = Writer::from_path(&self.path)?;

        // Write header
        wtr.write_record(["domain"])?;

        // Write records - only successful checks that are available
        for check in results.iter().flatten() {
            if check.available {
                wtr.write_record([&check.domain])?;
            }
        }

        wtr.flush()?;
        Ok(())
    }
}

/// Statistics calculator for domain check results
pub struct StatsExporter;

impl StatsExporter {
    pub fn calculate_stats(results: &[std::result::Result<CheckResult, DomainCheckerError>]) -> Stats {
        let total = results.len();
        let available = results
            .iter()
            .filter(|r| matches!(r, Ok(check) if check.available))
            .count();
        let errors = results.iter().filter(|r| r.is_err()).count();

        Stats {
            total,
            available,
            unavailable: total - available - errors,
            errors,
        }
    }

    pub fn export_stats_csv<P: AsRef<Path>>(path: P, stats: &Stats) -> Result<()> {
        let mut wtr = Writer::from_path(path)?;

        wtr.write_record(["metric", "value"])?;

        wtr.write_record(["total_checked", &stats.total.to_string()])?;
        wtr.write_record(["available", &stats.available.to_string()])?;
        wtr.write_record(["unavailable", &stats.unavailable.to_string()])?;
        wtr.write_record(["errors", &stats.errors.to_string()])?;

        wtr.flush()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
/// Statistics about domain check results
pub struct Stats {
    pub total: usize,
    pub available: usize,
    pub unavailable: usize,
    pub errors: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stats_calculation_computes_correctly() {
        let results = vec![
            Ok(CheckResult {
                domain: "test1.com".to_string(),
                available: true,
            }),
            Ok(CheckResult {
                domain: "test2.com".to_string(),
                available: false,
            }),
            Err(crate::DomainCheckerError::Timeout),
        ];

        let stats = StatsExporter::calculate_stats(&results);
        assert_eq!(stats.total, 3);
        assert_eq!(stats.available, 1);
        assert_eq!(stats.unavailable, 1);
        assert_eq!(stats.errors, 1);
    }
}

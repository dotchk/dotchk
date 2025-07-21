use crate::checker::CheckResult;
use csv::Writer;
use std::path::Path;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ExportError {
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub struct CsvExporter {
    path: String,
}

impl CsvExporter {
    pub fn new<P: AsRef<Path>>(path: P) -> Self {
        Self {
            path: path.as_ref().to_string_lossy().to_string(),
        }
    }

    pub fn export(&self, results: &[CheckResult]) -> Result<(), ExportError> {
        let mut wtr = Writer::from_path(&self.path)?;

        // Write header
        wtr.write_record([
            "domain",
            "available",
            "response_time_ms",
            "checked_at",
            "error",
        ])?;

        // Write records
        for result in results {
            wtr.write_record([
                &result.domain,
                &result.available.to_string(),
                &result.response_time_ms.to_string(),
                &result.checked_at.to_rfc3339(),
                result.error.as_deref().unwrap_or(""),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    pub async fn export_stream<S>(&self, mut results: S) -> Result<usize, ExportError>
    where
        S: futures::Stream<Item = CheckResult> + Unpin,
    {
        use futures::StreamExt;

        let mut wtr = Writer::from_path(&self.path)?;
        let mut count = 0;

        // Write header
        wtr.write_record([
            "domain",
            "available",
            "response_time_ms",
            "checked_at",
            "error",
        ])?;

        // Write records as they come in
        while let Some(result) = results.next().await {
            wtr.write_record([
                &result.domain,
                &result.available.to_string(),
                &result.response_time_ms.to_string(),
                &result.checked_at.to_rfc3339(),
                result.error.as_deref().unwrap_or(""),
            ])?;
            count += 1;

            // Flush periodically for large streams
            if count % 1000 == 0 {
                wtr.flush()?;
            }
        }

        wtr.flush()?;
        Ok(count)
    }

    pub fn export_available_only(&self, results: &[CheckResult]) -> Result<(), ExportError> {
        let available_results: Vec<&CheckResult> = results
            .iter()
            .filter(|r| r.available && r.error.is_none())
            .collect();

        let mut wtr = Writer::from_path(&self.path)?;

        // Write header
        wtr.write_record(["domain", "response_time_ms", "checked_at"])?;

        // Write records
        for result in available_results {
            wtr.write_record([
                &result.domain,
                &result.response_time_ms.to_string(),
                &result.checked_at.to_rfc3339(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
}

pub struct StatsExporter;

impl StatsExporter {
    pub fn calculate_stats(results: &[CheckResult]) -> Stats {
        let total = results.len();
        let available = results
            .iter()
            .filter(|r| r.available && r.error.is_none())
            .count();
        let errors = results.iter().filter(|r| r.error.is_some()).count();

        let response_times: Vec<u32> = results
            .iter()
            .filter(|r| r.error.is_none())
            .map(|r| r.response_time_ms)
            .collect();

        let avg_response_time = if !response_times.is_empty() {
            response_times.iter().sum::<u32>() / response_times.len() as u32
        } else {
            0
        };

        let mut sorted_times = response_times.clone();
        sorted_times.sort_unstable();

        let p50 = percentile(&sorted_times, 50);
        let p95 = percentile(&sorted_times, 95);
        let p99 = percentile(&sorted_times, 99);

        Stats {
            total,
            available,
            unavailable: total - available - errors,
            errors,
            avg_response_time_ms: avg_response_time,
            p50_response_time_ms: p50,
            p95_response_time_ms: p95,
            p99_response_time_ms: p99,
        }
    }

    pub fn export_stats_csv<P: AsRef<Path>>(path: P, stats: &Stats) -> Result<(), ExportError> {
        let mut wtr = Writer::from_path(path)?;

        wtr.write_record(["metric", "value"])?;

        wtr.write_record(["total_checked", &stats.total.to_string()])?;
        wtr.write_record(["available", &stats.available.to_string()])?;
        wtr.write_record(["unavailable", &stats.unavailable.to_string()])?;
        wtr.write_record(["errors", &stats.errors.to_string()])?;
        wtr.write_record([
            "avg_response_time_ms",
            &stats.avg_response_time_ms.to_string(),
        ])?;
        wtr.write_record([
            "p50_response_time_ms",
            &stats.p50_response_time_ms.to_string(),
        ])?;
        wtr.write_record([
            "p95_response_time_ms",
            &stats.p95_response_time_ms.to_string(),
        ])?;
        wtr.write_record([
            "p99_response_time_ms",
            &stats.p99_response_time_ms.to_string(),
        ])?;

        wtr.flush()?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Stats {
    pub total: usize,
    pub available: usize,
    pub unavailable: usize,
    pub errors: usize,
    pub avg_response_time_ms: u32,
    pub p50_response_time_ms: u32,
    pub p95_response_time_ms: u32,
    pub p99_response_time_ms: u32,
}

fn percentile(sorted_data: &[u32], p: usize) -> u32 {
    if sorted_data.is_empty() {
        return 0;
    }

    let idx = ((p as f64 / 100.0) * (sorted_data.len() - 1) as f64) as usize;
    let idx = idx.min(sorted_data.len() - 1); // Ensure we don't go out of bounds
    sorted_data[idx]
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_stats_calculation() {
        let results = vec![
            CheckResult {
                domain: "test1.com".to_string(),
                available: true,
                response_time_ms: 50,
                checked_at: Utc::now(),
                error: None,
            },
            CheckResult {
                domain: "test2.com".to_string(),
                available: false,
                response_time_ms: 100,
                checked_at: Utc::now(),
                error: None,
            },
            CheckResult {
                domain: "test3.com".to_string(),
                available: false,
                response_time_ms: 0,
                checked_at: Utc::now(),
                error: Some("Timeout".to_string()),
            },
        ];

        let stats = StatsExporter::calculate_stats(&results);
        assert_eq!(stats.total, 3);
        assert_eq!(stats.available, 1);
        assert_eq!(stats.unavailable, 1);
        assert_eq!(stats.errors, 1);
    }
}

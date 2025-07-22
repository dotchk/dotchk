use crate::cli::output::{
    format_domain_result, print_export_success, print_footer_note, print_statistics,
    print_tld_statistics,
};
use anyhow::Result;
use dotchk::{export::StatsExporter, CheckResult, CsvExporter};
use std::path::PathBuf;

pub fn print_results(results: &[CheckResult], available_only: bool) {
    let mut has_available = false;

    for result in results {
        if available_only && (!result.available || result.error.is_some()) {
            continue;
        }

        println!("{}", format_domain_result(result));

        if result.available && result.error.is_none() {
            has_available = true;
        }
    }

    if has_available && !available_only {
        print_footer_note();
    }
}

pub fn export_results(results: &[CheckResult], path: &PathBuf, available_only: bool) -> Result<()> {
    let exporter = CsvExporter::new(path);

    if available_only {
        exporter.export_available_only(results)?;
        let available_count = results
            .iter()
            .filter(|r| r.available && r.error.is_none())
            .count();
        print_export_success(&path.display().to_string(), available_count);
    } else {
        exporter.export(results)?;
        print_export_success(&path.display().to_string(), results.len());
    }

    Ok(())
}

pub fn print_stats(results: &[CheckResult]) {
    let stats = StatsExporter::calculate_stats(results);
    print_statistics(&stats);
}

pub fn print_tld_stats(results: &[CheckResult]) {
    let stats = StatsExporter::calculate_stats(results);
    print_tld_statistics(&stats);
}

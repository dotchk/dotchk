use crate::cli::output::{
    format_domain_error, format_domain_result, print_export_success, print_footer_note, print_statistics,
    print_tld_statistics,
};
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use dotchk::{CheckResult, CsvExporter, DomainCheckerError, export::StatsExporter};
use std::path::PathBuf;

pub fn print_results(results: &[std::result::Result<CheckResult, DomainCheckerError>], available_only: bool) {
    let mut has_available = false;

    for result in results {
        match result {
            Ok(check) => {
                if available_only && !check.available {
                    continue;
                }

                println!("{}", format_domain_result(check));

                if check.available {
                    has_available = true;
                }
            }
            Err(_e) => {
                if !available_only {
                    println!("{}", format_domain_error("unknown"));
                }
            }
        }
    }

    if has_available && !available_only {
        print_footer_note();
    }
}

pub fn export_results(
    results: &[std::result::Result<CheckResult, DomainCheckerError>],
    path: &PathBuf,
    available_only: bool,
) -> Result<()> {
    let exporter = CsvExporter::new(path);

    if available_only {
        exporter.export_available_only(results)?;
        let available_count = results.iter().filter(|r| matches!(r, Ok(check) if check.available)).count();
        print_export_success(&path.display().to_string(), available_count);
    } else {
        exporter.export(results)?;
        print_export_success(&path.display().to_string(), results.len());
    }

    Ok(())
}

pub fn print_stats(results: &[std::result::Result<CheckResult, DomainCheckerError>]) {
    let stats = StatsExporter::calculate_stats(results);
    print_statistics(&stats);
}

pub fn print_tld_stats(results: &[std::result::Result<CheckResult, DomainCheckerError>]) {
    let stats = StatsExporter::calculate_stats(results);
    print_tld_statistics(&stats);
}

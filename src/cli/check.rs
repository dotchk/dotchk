type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use dotchk::Checker;
use std::path::PathBuf;

use super::output::create_spinner;
use super::utils::{export_results, print_results};

pub async fn check_domains(
    domains: Vec<String>,
    parallel: usize,
    timeout: u64,
    available_only: bool,
    output: Option<PathBuf>,
) -> Result<()> {
    let checker = Checker::builder().max_parallel(parallel)?.timeout_ms(timeout)?.build().await?;

    let spinner = create_spinner(&format!("Checking {} domains", domains.len()));
    let results = checker.check_batch(domains).await;
    spinner.finish_and_clear();

    print_results(&results, available_only);

    if let Some(path) = output {
        export_results(&results, &path, available_only)?;
    }

    Ok(())
}

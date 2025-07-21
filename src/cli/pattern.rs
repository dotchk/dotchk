use anyhow::Result;
use dotchk::{Checker, Pattern};
use futures::StreamExt;
use std::path::PathBuf;

use super::output::{create_progress_bar, format_domain_result, print_footer_note, print_info};
use super::utils::{export_results, print_stats};

pub async fn check_pattern(
    pattern_str: String,
    limit: Option<usize>,
    parallel: usize,
    timeout: u64,
    available_only: bool,
    output: Option<PathBuf>,
    show_stats: bool,
) -> Result<()> {
    let pattern = Pattern::compile(&pattern_str)?;
    let domains = pattern.generate(limit);

    print_info(&format!("Generated {} domains from pattern", domains.len()));

    let checker = Checker::builder()
        .max_parallel(parallel)
        .timeout_ms(timeout)
        .build()
        .await?;

    let mut results = Vec::new();
    let pb = create_progress_bar(domains.len() as u64, "Checking domains");
    let mut stream = Box::pin(checker.check_stream(domains));
    let mut has_available = false;

    while let Some(result) = stream.next().await {
        pb.inc(1);

        if !available_only || (result.available && result.error.is_none()) {
            pb.suspend(|| {
                println!("{}", format_domain_result(&result));
            });
        }

        if result.available && result.error.is_none() {
            has_available = true;
        }

        results.push(result);
    }

    pb.finish_and_clear();

    if has_available && !available_only {
        print_footer_note();
    }

    if show_stats {
        print_stats(&results);
    }

    if let Some(path) = output {
        export_results(&results, &path, available_only)?;
    }

    Ok(())
}

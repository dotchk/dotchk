type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use dotchk::Checker;
use futures::StreamExt;
use std::path::PathBuf;
use tokio::fs;

use super::output::{
    create_progress_bar, format_domain_error, format_domain_result, print_footer_note, print_info, print_warning,
};
use super::utils::{export_results, print_stats};

pub async fn bulk_check(
    file: PathBuf,
    parallel: usize,
    timeout: u64,
    available_only: bool,
    output: Option<PathBuf>,
    show_stats: bool,
) -> Result<()> {
    let content = fs::read_to_string(&file).await?;
    let domains: Vec<String> = content
        .lines()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !s.starts_with('#'))
        .collect();

    if domains.is_empty() {
        print_warning("No valid domains found in file");
        return Ok(());
    }

    print_info(&format!("Loaded {} domains from {}", domains.len(), file.display()));

    let checker = Checker::builder().max_parallel(parallel)?.timeout_ms(timeout)?.build().await?;

    let mut results = Vec::new();
    let pb = create_progress_bar(domains.len() as u64, "Checking domains");
    let mut stream = Box::pin(checker.check_stream(domains));
    let mut has_available = false;

    while let Some(result) = stream.next().await {
        pb.inc(1);

        match &result {
            Ok(check) => {
                if !available_only || check.available {
                    pb.suspend(|| {
                        println!("{}", format_domain_result(check));
                    });
                }

                if check.available {
                    has_available = true;
                }
            }
            Err(_e) => {
                if !available_only {
                    pb.suspend(|| {
                        println!("{}", format_domain_error("unknown"));
                    });
                }
            }
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

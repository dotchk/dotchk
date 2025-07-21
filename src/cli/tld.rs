use anyhow::Result;
use dotchk::{tld::TLD_SERVERS, CheckResult, Checker};
use futures::StreamExt;
use std::collections::HashMap;
use std::path::PathBuf;

use super::output::{
    create_progress_bar, format_tld_result, print_footer_note, print_header, print_info,
};
use super::utils::{export_results, print_stats};

const POPULAR_TLDS: &[&str] = &[
    "com", "net", "org", "io", "dev", "app", "co", "me", "ai", "xyz", "info", "biz",
];

#[allow(clippy::too_many_arguments)]
pub async fn check_tlds(
    domains: Vec<String>,
    popular: bool,
    tlds: Option<Vec<String>>,
    all: bool,
    parallel: usize,
    timeout: u64,
    available_only: bool,
    output: Option<PathBuf>,
    show_stats: bool,
) -> Result<()> {
    // Determine which TLDs to check
    let tlds_to_check = determine_tlds(popular, tlds, all);

    // Generate domain combinations
    let domains_to_check = generate_domain_combinations(&domains, &tlds_to_check);

    print_info(&format!(
        "Checking {} domain combinations across {} TLDs",
        domains_to_check.len(),
        tlds_to_check.len()
    ));

    let checker = Checker::builder()
        .max_parallel(parallel)
        .timeout_ms(timeout)
        .build()
        .await?;

    let mut results = Vec::new();
    let pb = create_progress_bar(domains_to_check.len() as u64, "Checking TLDs");
    let mut stream = Box::pin(checker.check_stream(domains_to_check));

    // Group results by base domain for better display
    let mut grouped_results: HashMap<String, Vec<CheckResult>> = HashMap::new();

    while let Some(result) = stream.next().await {
        pb.inc(1);
        let base_domain = extract_base_domain(&result.domain);
        results.push(result.clone());
        grouped_results.entry(base_domain).or_default().push(result);
    }

    pb.finish_and_clear();

    // Print results grouped by domain
    print_grouped_results(&domains, &grouped_results, available_only);

    if show_stats {
        print_stats(&results);
    }

    if let Some(path) = output {
        export_results(&results, &path, available_only)?;
    }

    Ok(())
}

fn determine_tlds(popular: bool, tlds: Option<Vec<String>>, all: bool) -> Vec<String> {
    if all {
        print_info(&format!("Using all {} available TLDs", TLD_SERVERS.len()));
        TLD_SERVERS.keys().map(|&s| s.to_string()).collect()
    } else if popular {
        print_info("Using popular TLDs");
        POPULAR_TLDS.iter().map(|&s| s.to_string()).collect()
    } else if let Some(tlds) = tlds {
        print_info(&format!("Using {} specified TLDs", tlds.len()));
        tlds
    } else {
        // Default to popular TLDs if nothing specified
        print_info("No TLDs specified, using popular TLDs");
        POPULAR_TLDS[..8].iter().map(|&s| s.to_string()).collect()
    }
}

fn generate_domain_combinations(domains: &[String], tlds_to_check: &[String]) -> Vec<String> {
    let mut domains_to_check = Vec::new();

    for domain in domains {
        // Remove any existing TLD from the domain
        let base_domain = domain.split('.').next().unwrap_or(domain);

        for tld in tlds_to_check {
            // Verify TLD is supported
            if TLD_SERVERS.get(tld.as_str()).is_some() {
                domains_to_check.push(format!("{base_domain}.{tld}"));
            } else {
                eprintln!("Warning: TLD '{tld}' is not supported, skipping");
            }
        }
    }

    domains_to_check
}

fn extract_base_domain(domain: &str) -> String {
    domain.split('.').next().unwrap_or(domain).to_string()
}

fn print_grouped_results(
    domains: &[String],
    grouped_results: &HashMap<String, Vec<CheckResult>>,
    available_only: bool,
) {
    let mut has_available = false;

    for domain in domains {
        let base_domain = domain.split('.').next().unwrap_or(domain);

        if let Some(domain_results) = grouped_results.get(base_domain) {
            let mut sorted_results = domain_results.clone();
            sorted_results.sort_by_key(|r| r.domain.clone());

            // Calculate max domain width for alignment
            let max_width = sorted_results
                .iter()
                .filter(|r| !available_only || (r.available && r.error.is_none()))
                .map(|r| r.domain.len())
                .max()
                .unwrap_or(0);

            // Only print header if we have results to show
            let has_results = sorted_results
                .iter()
                .any(|r| !available_only || (r.available && r.error.is_none()));

            if has_results {
                print_header(base_domain);
            }

            for result in sorted_results {
                if available_only && (!result.available || result.error.is_some()) {
                    continue;
                }

                println!("{}", format_tld_result(&result, true, max_width));

                if result.available && result.error.is_none() {
                    has_available = true;
                }
            }
        }
    }

    if has_available {
        print_footer_note();
    }
}

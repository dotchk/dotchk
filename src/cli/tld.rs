type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
use dotchk::{CheckResult, Checker, DomainCheckerError, TLD_SERVERS, get_public_tlds};
use futures::StreamExt;
use std::collections::HashMap;
use std::path::PathBuf;

use super::output::{
    create_progress_bar, format_tld_error, format_tld_result, print_footer_note, print_header, print_info,
};
use super::utils::{export_results, print_tld_stats};

const POPULAR_TLDS: &[&str] = &["com", "net", "org", "io", "dev", "app", "co", "me", "ai", "xyz", "info", "biz"];

const TECH_TLDS: &[&str] = &[
    "io", "dev", "app", "tech", "cloud", "ai", "digital", "online", "software", "systems", "codes", "tools", "network",
    "web", "site", "website", "host", "server", "data", "api",
];

const BUSINESS_TLDS: &[&str] = &[
    "com",
    "biz",
    "business",
    "company",
    "enterprises",
    "ventures",
    "holdings",
    "partners",
    "inc",
    "llc",
    "ltd",
    "corp",
    "group",
    "solutions",
    "services",
    "consulting",
    "agency",
    "marketing",
    "finance",
    "capital",
];

const CREATIVE_TLDS: &[&str] = &[
    "design",
    "studio",
    "art",
    "media",
    "digital",
    "creative",
    "works",
    "productions",
    "graphics",
    "photo",
    "photography",
    "video",
    "film",
    "music",
    "audio",
    "gallery",
];

const RETAIL_TLDS: &[&str] = &[
    "shop", "store", "buy", "sale", "deals", "market", "shopping", "boutique", "cheap", "bargains", "discount",
    "promo", "coupon", "auction", "bid",
];

const COUNTRY_POPULAR: &[&str] = &[
    "us", "uk", "de", "fr", "es", "it", "nl", "be", "ch", "at", "se", "no", "dk", "fi", "pl", "cz", "gr", "pt", "ie",
    "ca", "au", "nz", "jp", "kr", "cn", "in", "sg", "hk", "my", "th", "id", "ph", "vn", "br", "mx", "ar", "cl", "co",
    "za", "eg", "ae", "il",
];

#[allow(clippy::too_many_arguments)]
pub async fn check_tlds(
    domains: Vec<String>,
    popular: bool,
    tech: bool,
    business: bool,
    creative: bool,
    retail: bool,
    country: bool,
    tlds: Option<Vec<String>>,
    all: bool,
    parallel: usize,
    timeout: u64,
    available_only: bool,
    output: Option<PathBuf>,
    show_stats: bool,
) -> Result<()> {
    // Determine which TLDs to check
    let tlds_to_check = determine_tlds(popular, tech, business, creative, retail, country, tlds, all);

    // Generate domain combinations
    let domains_to_check = generate_domain_combinations(&domains, &tlds_to_check);

    print_info(&format!(
        "Checking {} domain combinations across {} TLDs",
        domains_to_check.len(),
        tlds_to_check.len()
    ));

    let checker = Checker::builder().max_parallel(parallel)?.timeout_ms(timeout)?.build().await?;

    let mut results = Vec::new();
    let pb = create_progress_bar(domains_to_check.len() as u64, "Checking TLDs");
    let mut stream = Box::pin(checker.check_stream(domains_to_check));

    while let Some(result) = stream.next().await {
        pb.inc(1);
        results.push(result);
    }

    pb.finish_and_clear();

    // Print results grouped by domain
    print_grouped_results(&domains, &results, available_only);

    if show_stats {
        print_tld_stats(&results);
    }

    if let Some(path) = output {
        export_results(&results, &path, available_only)?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn determine_tlds(
    popular: bool,
    tech: bool,
    business: bool,
    creative: bool,
    retail: bool,
    country: bool,
    tlds: Option<Vec<String>>,
    all: bool,
) -> Vec<String> {
    use std::collections::HashSet;

    if all {
        let public_tlds = get_public_tlds();
        print_info(&format!(
            "Using {} public TLDs (excluding private, adult, gambling, religious, and non-ASCII TLDs)",
            public_tlds.len()
        ));
        public_tlds.into_iter().map(|s| s.to_string()).collect()
    } else if let Some(tlds) = tlds {
        print_info(&format!("Using {} specified TLDs", tlds.len()));
        tlds
    } else {
        // Collect TLDs based on selected groups
        let mut selected_tlds = HashSet::new();
        let mut groups = Vec::new();

        if popular {
            groups.push("popular");
            for &tld in POPULAR_TLDS {
                selected_tlds.insert(tld);
            }
        }
        if tech {
            groups.push("tech");
            for &tld in TECH_TLDS {
                selected_tlds.insert(tld);
            }
        }
        if business {
            groups.push("business");
            for &tld in BUSINESS_TLDS {
                selected_tlds.insert(tld);
            }
        }
        if creative {
            groups.push("creative");
            for &tld in CREATIVE_TLDS {
                selected_tlds.insert(tld);
            }
        }
        if retail {
            groups.push("retail");
            for &tld in RETAIL_TLDS {
                selected_tlds.insert(tld);
            }
        }
        if country {
            groups.push("country");
            for &tld in COUNTRY_POPULAR {
                selected_tlds.insert(tld);
            }
        }

        if groups.is_empty() {
            // Default to popular TLDs if nothing specified
            print_info("No TLDs specified, using popular TLDs");
            POPULAR_TLDS.iter().map(|&s| s.to_string()).collect()
        } else {
            print_info(&format!(
                "Using {} TLDs from groups: {}",
                selected_tlds.len(),
                groups.join(", ")
            ));
            let mut result: Vec<String> = selected_tlds.into_iter().map(|s| s.to_string()).collect();
            result.sort(); // Sort for consistent output
            result
        }
    }
}

fn generate_domain_combinations(domains: &[String], tlds_to_check: &[String]) -> Vec<String> {
    let mut domains_to_check = Vec::new();

    for domain in domains {
        // Remove any existing TLD from the domain
        let base_domain = domain.split('.').next().unwrap_or(domain);

        for tld in tlds_to_check {
            // Verify TLD is supported (normalize to lowercase for lookup)
            let tld_lower = tld.to_lowercase();
            if TLD_SERVERS.get(tld_lower.as_str()).is_some() {
                domains_to_check.push(format!("{base_domain}.{tld_lower}"));
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
    results: &[std::result::Result<CheckResult, DomainCheckerError>],
    available_only: bool,
) {
    let mut has_available = false;

    // Group results by base domain
    let mut grouped_results: HashMap<String, Vec<&std::result::Result<CheckResult, DomainCheckerError>>> =
        HashMap::new();

    for result in results {
        let base_domain = match result {
            Ok(check) => extract_base_domain(&check.domain),
            Err(_) => "unknown".to_string(),
        };
        grouped_results.entry(base_domain).or_default().push(result);
    }

    for domain in domains {
        let base_domain = domain.split('.').next().unwrap_or(domain);
        // Convert to lowercase for lookup since domain checking normalizes to lowercase
        let base_domain_lower = base_domain.to_lowercase();

        if let Some(domain_results) = grouped_results.get(&base_domain_lower) {
            let mut sorted_results = domain_results.to_vec();
            sorted_results.sort_by_key(|r| match r {
                Ok(check) => check.domain.clone(),
                Err(_) => "unknown".to_string(),
            });

            // Calculate max domain width for alignment
            let max_width = sorted_results
                .iter()
                .filter_map(|r| match r {
                    Ok(check) if !available_only || check.available => Some(check.domain.len()),
                    _ => None,
                })
                .max()
                .unwrap_or(0);

            // Only print header if we have results to show
            let has_results = sorted_results.iter().any(|r| match r {
                Ok(check) => !available_only || check.available,
                Err(_) => !available_only,
            });

            if has_results {
                print_header(base_domain);
            }

            for result in sorted_results {
                match result {
                    Ok(check) => {
                        if available_only && !check.available {
                            continue;
                        }

                        println!("{}", format_tld_result(check, true, max_width));

                        if check.available {
                            has_available = true;
                        }
                    }
                    Err(_e) => {
                        if !available_only {
                            println!("{}", format_tld_error("unknown", true, max_width));
                        }
                    }
                }
            }
        }
    }

    if has_available {
        print_footer_note();
    }
}

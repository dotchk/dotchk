use crate::dns_pipelined::{DnsError, PipelinedDnsClient};
use crate::tld::get_tld_info;
use crate::DomainCheckerError;
use chrono::{DateTime, Utc};
use futures::stream::{self, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tracing::{debug, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub domain: String,
    /// Whether the domain appears available (no NS records found).
    /// Note: This may be a false positive if the domain is registered without nameservers.
    pub available: bool,
    pub response_time_ms: u32,
    pub checked_at: DateTime<Utc>,
    pub error: Option<String>,
}

pub struct Checker {
    dns_client: Arc<PipelinedDnsClient>,
    max_parallel: usize,
    semaphore: Arc<Semaphore>,
    timeout_ms: u64,
}

pub struct CheckerBuilder {
    max_parallel: usize,
    timeout_ms: u64,
    cache_ttl: Duration,
}

impl Default for CheckerBuilder {
    fn default() -> Self {
        Self {
            max_parallel: 100,
            timeout_ms: 3000,
            cache_ttl: Duration::from_secs(300),
        }
    }
}

impl CheckerBuilder {
    pub fn max_parallel(mut self, max_parallel: usize) -> Self {
        self.max_parallel = max_parallel;
        self
    }

    pub fn timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    pub fn cache_ttl(mut self, cache_ttl: Duration) -> Self {
        self.cache_ttl = cache_ttl;
        self
    }

    pub async fn build(self) -> Result<Checker, DomainCheckerError> {
        let client = PipelinedDnsClient::new("0.0.0.0:0", self.cache_ttl).await?;

        Ok(Checker {
            dns_client: Arc::new(client),
            max_parallel: self.max_parallel,
            semaphore: Arc::new(Semaphore::new(self.max_parallel)),
            timeout_ms: self.timeout_ms,
        })
    }
}

impl Checker {
    pub fn builder() -> CheckerBuilder {
        CheckerBuilder::default()
    }

    /// Check if a single domain is available.
    ///
    /// # Important Note
    ///
    /// This method uses NS (nameserver) record queries for performance.
    /// Domains that are registered but have no nameservers configured
    /// will be incorrectly reported as "available".
    ///
    /// Always verify with WHOIS before attempting to register a domain.
    pub async fn check(&self, domain: &str) -> CheckResult {
        let start = Instant::now();
        let domain = domain.to_lowercase();

        let result = match self.check_domain_internal(&domain).await {
            Ok(available) => CheckResult {
                domain: domain.clone(),
                available,
                response_time_ms: start.elapsed().as_millis() as u32,
                checked_at: Utc::now(),
                error: None,
            },
            Err(e) => CheckResult {
                domain: domain.clone(),
                available: false,
                response_time_ms: start.elapsed().as_millis() as u32,
                checked_at: Utc::now(),
                error: Some(e.to_string()),
            },
        };

        debug!(
            "Checked {}: available={}, time={}ms",
            result.domain, result.available, result.response_time_ms
        );

        result
    }

    async fn check_domain_internal(&self, domain: &str) -> Result<bool, DomainCheckerError> {
        if !is_valid_domain(domain) {
            debug!("Domain {} failed validation", domain);
            return Err(DomainCheckerError::InvalidDomain(domain.to_string()));
        }

        let tld_info = get_tld_info(domain)
            .ok_or_else(|| DomainCheckerError::UnsupportedTld(extract_tld(domain)))?;

        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| DomainCheckerError::Timeout)?;

        let timeout = self.timeout_ms.min(tld_info.timeout_ms);

        for (i, server) in tld_info.servers.iter().enumerate() {
            match self.dns_client.query_ns(domain, server, timeout).await {
                Ok(has_ns) => return Ok(!has_ns),
                Err(DnsError::NameError) => return Ok(true),
                Err(e) => {
                    if i == tld_info.servers.len() - 1 {
                        warn!("All servers failed for {}: {}", domain, e);
                        return Err(e.into());
                    }
                    debug!(
                        "Server {} failed for {}: {}, trying next",
                        server, domain, e
                    );
                }
            }
        }

        Err(DomainCheckerError::Timeout)
    }

    pub fn check_stream(&self, domains: Vec<String>) -> impl Stream<Item = CheckResult> + '_ {
        let futures: Vec<_> = domains
            .into_iter()
            .map(|domain| {
                let checker = self;
                async move { checker.check(&domain).await }
            })
            .collect();

        stream::iter(futures).buffer_unordered(self.max_parallel)
    }

    pub async fn check_batch(&self, domains: Vec<String>) -> Vec<CheckResult> {
        let mut results = Vec::with_capacity(domains.len());

        // Group domains by TLD for more efficient batching
        let mut domains_by_tld: std::collections::HashMap<String, Vec<(usize, String)>> =
            std::collections::HashMap::new();

        for (idx, domain) in domains.iter().enumerate() {
            let domain_lower = domain.to_lowercase();

            // First check if domain is valid
            if !is_valid_domain(&domain_lower) {
                results.push((
                    idx,
                    CheckResult {
                        domain: domain.clone(),
                        available: false,
                        response_time_ms: 0,
                        checked_at: Utc::now(),
                        error: Some("Invalid domain format".to_string()),
                    },
                ));
                continue;
            }

            if let Some(_tld_info) = get_tld_info(&domain_lower) {
                let tld = extract_tld(&domain_lower);
                domains_by_tld
                    .entry(tld)
                    .or_default()
                    .push((idx, domain_lower));
            } else {
                results.push((
                    idx,
                    CheckResult {
                        domain: domain.clone(),
                        available: false,
                        response_time_ms: 0,
                        checked_at: Utc::now(),
                        error: Some(format!("Unsupported TLD: {}", extract_tld(&domain_lower))),
                    },
                ));
            }
        }

        // Process each TLD group
        for (tld, domain_list) in domains_by_tld {
            if let Some(tld_info) = get_tld_info(&format!("example.{tld}")) {
                let timeout = self.timeout_ms.min(tld_info.timeout_ms);

                // Prepare batch queries for primary server
                let batch_queries: Vec<_> = domain_list
                    .iter()
                    .map(|(_, domain)| (domain.clone(), tld_info.servers[0].to_string(), timeout))
                    .collect();

                let start = Instant::now();
                let batch_results = self.dns_client.query_ns_batch(batch_queries).await;
                let elapsed = start.elapsed().as_millis() as u32;

                for ((idx, domain), result) in domain_list.iter().zip(batch_results) {
                    let check_result = match result {
                        Ok(has_ns) => CheckResult {
                            domain: domain.clone(),
                            available: !has_ns,
                            response_time_ms: elapsed,
                            checked_at: Utc::now(),
                            error: None,
                        },
                        Err(DnsError::NameError) => CheckResult {
                            domain: domain.clone(),
                            available: true,
                            response_time_ms: elapsed,
                            checked_at: Utc::now(),
                            error: None,
                        },
                        Err(e) => CheckResult {
                            domain: domain.clone(),
                            available: false,
                            response_time_ms: elapsed,
                            checked_at: Utc::now(),
                            error: Some(e.to_string()),
                        },
                    };
                    results.push((*idx, check_result));
                }
            }
        }

        // Sort results back to original order
        results.sort_by_key(|(idx, _)| *idx);
        results.into_iter().map(|(_, result)| result).collect()
    }
}

fn is_valid_domain(domain: &str) -> bool {
    if domain.is_empty() || domain.len() > 253 {
        return false;
    }

    let parts: Vec<&str> = domain.split('.').collect();

    if parts.len() < 2 {
        return false;
    }

    for part in &parts {
        if part.is_empty() || part.len() > 63 {
            return false;
        }

        if !part.chars().all(|c| {
            c.is_ascii_lowercase() || c.is_ascii_uppercase() || c.is_ascii_digit() || c == '-'
        }) {
            return false;
        }

        if part.starts_with('-') || part.ends_with('-') {
            return false;
        }
    }

    true
}

fn extract_tld(domain: &str) -> String {
    domain.split('.').next_back().unwrap_or("").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_domain() {
        assert!(is_valid_domain("example.com"));
        assert!(is_valid_domain("sub.example.com"));
        assert!(is_valid_domain("my-site.co.uk"));
        assert!(is_valid_domain("123.456.xyz"));

        assert!(!is_valid_domain(""));
        assert!(!is_valid_domain("example"));
        assert!(!is_valid_domain(".com"));
        assert!(!is_valid_domain("example..com"));
        assert!(!is_valid_domain("-example.com"));
        assert!(!is_valid_domain("example-.com"));
        assert!(!is_valid_domain("exam ple.com"));
        // Single letter TLDs do exist (e.g., .x, .i)
        assert!(is_valid_domain("example.c"));

        let long_label = "a".repeat(64);
        assert!(!is_valid_domain(&format!("{long_label}.com")));
    }

    #[test]
    fn test_extract_tld() {
        assert_eq!(extract_tld("example.com"), "com");
        assert_eq!(extract_tld("test.co.uk"), "uk");
        assert_eq!(extract_tld("domain.xyz"), "xyz");
    }

    #[test]
    fn test_is_valid_domain_edge_cases() {
        // Test maximum domain length (253 characters)
        let long_domain = format!(
            "{}.{}.{}.com",
            "a".repeat(63),
            "b".repeat(63),
            "c".repeat(63)
        );
        assert!(is_valid_domain(&long_domain));

        // Test domain that's too long
        let too_long = format!("{}.com", "a".repeat(254));
        assert!(!is_valid_domain(&too_long));

        // Test numeric labels (should be valid)
        assert!(is_valid_domain("123.456.com"));
        assert!(is_valid_domain("0.com"));

        // Test hyphens in middle (valid)
        assert!(is_valid_domain("test-domain.com"));
        assert!(is_valid_domain("test--domain.com"));

        // Test all numeric TLD (should be valid for our purposes)
        assert!(is_valid_domain("test.123"));
    }
}

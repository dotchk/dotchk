//! Domain availability checker using NS record queries.
//!
//! This module provides the main `Checker` API for determining if domains are available
//! by querying authoritative DNS servers for NS (nameserver) records.
//!
//! # How It Works
//!
//! 1. Extract TLD from domain (e.g., "com" from "example.com")
//! 2. Look up authoritative nameservers for that TLD (from pre-resolved IP list)
//! 3. Query those nameservers for NS records of the domain
//! 4. If no NS records exist → domain appears available
//! 5. If NS records exist → domain is registered
//!
//! # Important Limitations
//!
//! **This tool uses NS record queries, not WHOIS.** Domains registered without nameservers
//! configured will incorrectly appear as "available". Always verify with WHOIS before
//! attempting to register a domain.
//!
//! # Fallback Strategy
//!
//! When authoritative servers fail:
//! 1. Try all configured authoritative servers for the TLD (up to 3)
//! 2. If all fail, fall back to system DNS resolver
//! 3. If system resolver fails, return error
//!
//! This ensures maximum reliability while preferring authoritative answers.

use crate::dns_pipelined::{DnsError, PipelinedDnsClient};
use crate::tld_registry::get_tld_info;
use crate::DomainCheckerError;
use futures::stream::{self, Stream, StreamExt};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Semaphore;
use tracing::{debug, info, warn};

/// Result of a domain availability check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub domain: String,
    /// Whether the domain appears available (no NS records found).
    /// Note: This may be a false positive if the domain is registered without nameservers.
    pub available: bool,
}

/// Domain availability checker using DNS NS record queries
///
/// Notes:
/// - Clone shares the same semaphore to maintain global parallelism limit across clones
#[derive(Clone)]
pub struct Checker {
    dns_client: Arc<PipelinedDnsClient>,
    max_parallel: usize,
    semaphore: Arc<Semaphore>,
    timeout_ms: u64,
}

/// Builder for constructing a Checker with custom settings
pub struct CheckerBuilder {
    max_parallel: usize,
    timeout_ms: u64,
    cache_ttl: Duration,
}

impl Default for CheckerBuilder {
    fn default() -> Self {
        Self {
            max_parallel: 100,
            timeout_ms: 500,
            cache_ttl: Duration::from_secs(300),
        }
    }
}

impl CheckerBuilder {
    /// Set the maximum number of parallel DNS queries.
    ///
    /// # Errors
    ///
    /// Returns an error if max_parallel is 0 or exceeds 10,000.
    pub fn max_parallel(mut self, max_parallel: usize) -> Result<Self, DomainCheckerError> {
        if max_parallel == 0 {
            return Err(DomainCheckerError::Internal(
                "max_parallel must be at least 1".to_string(),
            ));
        }
        if max_parallel > 10_000 {
            return Err(DomainCheckerError::Internal(
                "max_parallel cannot exceed 10,000 (resource limits)".to_string(),
            ));
        }
        self.max_parallel = max_parallel;
        Ok(self)
    }

    /// Set the DNS query timeout in milliseconds.
    ///
    /// # Errors
    ///
    /// Returns an error if timeout_ms is 0 or exceeds 60,000 (60 seconds).
    pub fn timeout_ms(mut self, timeout_ms: u64) -> Result<Self, DomainCheckerError> {
        if timeout_ms == 0 {
            return Err(DomainCheckerError::Internal(
                "timeout_ms must be at least 1".to_string(),
            ));
        }
        if timeout_ms > 60_000 {
            return Err(DomainCheckerError::Internal(
                "timeout_ms cannot exceed 60,000 (60 seconds)".to_string(),
            ));
        }
        self.timeout_ms = timeout_ms;
        Ok(self)
    }

    /// Set the DNS cache TTL (time-to-live).
    ///
    /// # Errors
    ///
    /// Returns an error if cache_ttl is 0 or exceeds 1 hour.
    pub fn cache_ttl(mut self, cache_ttl: Duration) -> Result<Self, DomainCheckerError> {
        if cache_ttl.is_zero() {
            return Err(DomainCheckerError::Internal(
                "cache_ttl must be greater than 0".to_string(),
            ));
        }
        if cache_ttl > Duration::from_secs(3600) {
            return Err(DomainCheckerError::Internal(
                "cache_ttl cannot exceed 1 hour".to_string(),
            ));
        }
        self.cache_ttl = cache_ttl;
        Ok(self)
    }

    pub async fn build(self) -> Result<Checker, DomainCheckerError> {
        let client = PipelinedDnsClient::new("0.0.0.0:0", self.cache_ttl).await?;

        // Don't pre-resolve all nameservers at startup - it's too slow
        // They'll be resolved on-demand and cached

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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotchk::Checker;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let checker = Checker::builder()
    ///     .max_parallel(100)?
    ///     .timeout_ms(5000)?
    ///     .build()
    ///     .await?;
    ///
    /// let result = checker.check("example.com").await?;
    /// println!("{} - available: {}", result.domain, result.available);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check(&self, domain: &str) -> Result<CheckResult, DomainCheckerError> {
        let start = Instant::now();
        let domain = domain.to_lowercase();

        let available = self.check_domain_internal(&domain).await?;

        let result = CheckResult {
            domain: domain.clone(),
            available,
        };

        debug!(
            "Checked {}: available={}, time={}ms",
            result.domain,
            result.available,
            start.elapsed().as_millis()
        );

        Ok(result)
    }

    async fn check_domain_internal(&self, domain: &str) -> Result<bool, DomainCheckerError> {
        debug!("Starting check for domain: {}", domain);
        if !is_valid_domain(domain) {
            debug!("Domain {} failed validation", domain);
            return Err(DomainCheckerError::InvalidDomain(domain.to_string()));
        }

        let tld_info = get_tld_info(domain)
            .ok_or_else(|| DomainCheckerError::UnsupportedTld(extract_tld(domain)))?;
        debug!(
            "Got TLD info for {}: {} servers",
            domain,
            tld_info.servers.len()
        );

        debug!("Acquiring semaphore permit for {}", domain);
        let _permit = self.semaphore.acquire().await.map_err(|_| {
            warn!("Semaphore acquire failed for {}", domain);
            DomainCheckerError::Internal("concurrency semaphore closed unexpectedly".to_string())
        })?;
        debug!("Acquired semaphore permit for {}", domain);

        // Use the minimum of user timeout and TLD timeout to respect user's maximum
        let per_server_timeout = self.timeout_ms.min(tld_info.timeout_ms);
        debug!(
            "Checking {} with {} servers, per-server timeout {}ms (checker: {}ms, tld: {}ms)",
            domain,
            tld_info.servers.len(),
            per_server_timeout,
            self.timeout_ms,
            tld_info.timeout_ms
        );

        // The servers are already pre-resolved IPs, so we can use them directly
        // If there are no servers configured, fall back to local resolver
        if tld_info.servers.is_empty() {
            info!(
                "No authoritative servers configured for {}, falling back to local resolver",
                domain
            );
            match self.dns_client.query_local_resolver(domain).await {
                Ok(has_records) => {
                    info!(
                        "Local resolver query for {} succeeded: available={}",
                        domain, !has_records
                    );
                    return Ok(!has_records);
                }
                Err(local_err) => {
                    warn!("Local resolver also failed for {}: {:?}", domain, local_err);
                    return Err(DomainCheckerError::Timeout);
                }
            }
        }

        // Try the pre-resolved IP addresses (already limited to 3 by update_tlds.py)
        for (i, server_ip) in tld_info.servers.iter().enumerate() {
            debug!(
                "Trying server {} ({}/{}) for {}",
                server_ip,
                i + 1,
                tld_info.servers.len(),
                domain
            );
            match self
                .dns_client
                .query_ns(domain, server_ip, per_server_timeout)
                .await
            {
                Ok(has_ns) => {
                    debug!(
                        "Server {} succeeded for {}: has_ns={}",
                        server_ip, domain, has_ns
                    );
                    return Ok(!has_ns);
                }
                Err(DnsError::NameError) => {
                    debug!("Server {} returned NXDOMAIN for {}", server_ip, domain);
                    return Ok(true);
                }
                Err(e) => {
                    debug!("Server {} failed for {}: {:?}", server_ip, domain, e);
                    if i == tld_info.servers.len() - 1 {
                        warn!(
                            "All {} authoritative servers failed for {}: last error was {:?}",
                            tld_info.servers.len(),
                            domain,
                            e
                        );
                        // All authoritative servers failed, try local resolver as fallback
                        info!("Falling back to local resolver for {}", domain);
                        match self.dns_client.query_local_resolver(domain).await {
                            Ok(has_records) => {
                                info!(
                                    "Local resolver query for {} succeeded: available={}",
                                    domain, !has_records
                                );
                                return Ok(!has_records);
                            }
                            Err(local_err) => {
                                warn!("Local resolver also failed for {}: {:?}", domain, local_err);
                                return Err(e.into());
                            }
                        }
                    }
                }
            }
        }

        // This can happen if the TLD has no servers configured
        Err(DomainCheckerError::UnsupportedTld(extract_tld(domain)))
    }

    pub fn check_stream(&self, domains: Vec<String>) -> impl Stream<Item = Result<CheckResult, DomainCheckerError>> + '_ {
        let futures: Vec<_> = domains
            .into_iter()
            .map(|domain| {
                let checker = self;
                async move { checker.check(&domain).await }
            })
            .collect();

        stream::iter(futures).buffer_unordered(self.max_parallel)
    }

    /// Check multiple domains concurrently.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dotchk::Checker;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let checker = Checker::builder()
    ///     .max_parallel(200)?
    ///     .build()
    ///     .await?;
    ///
    /// let domains = vec!["test1.com".to_string(), "test2.com".to_string()];
    /// let results = checker.check_batch(domains).await;
    ///
    /// for result in results {
    ///     match result {
    ///         Ok(check) => println!("{}: {}", check.domain, if check.available { "available" } else { "taken" }),
    ///         Err(e) => eprintln!("Error: {}", e),
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn check_batch(&self, domains: Vec<String>) -> Vec<Result<CheckResult, DomainCheckerError>> {
        // For now, use individual checks to ensure proper fallback logic
        // This is less efficient but ensures consistency with server fallback and local resolver
        let futures: Vec<_> = domains
            .into_iter()
            .map(|domain| {
                let checker = self.clone();
                async move { checker.check(&domain).await }
            })
            .collect();

        // Process all checks concurrently with controlled parallelism
        stream::iter(futures)
            .buffer_unordered(self.max_parallel)
            .collect()
            .await
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

/// Extracts the TLD from a domain string.
///
/// Returns the last component after splitting by '.', or "unknown" if extraction fails.
/// This function is infallible and safe to use after domain validation.
fn extract_tld(domain: &str) -> String {
    domain
        .split('.')
        .next_back()
        .filter(|tld| !tld.is_empty())
        .unwrap_or("unknown")
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_valid_domain_accepts_valid_formats() {
        assert!(is_valid_domain("example.com"));
        assert!(is_valid_domain("sub.example.com"));
        assert!(is_valid_domain("my-site.co.uk"));
        assert!(is_valid_domain("123.456.xyz"));
        // Single letter TLDs do exist (e.g., .x, .i)
        assert!(is_valid_domain("example.c"));
    }

    #[test]
    fn is_valid_domain_rejects_invalid_formats() {
        assert!(!is_valid_domain(""));
        assert!(!is_valid_domain("example"));
        assert!(!is_valid_domain(".com"));
        assert!(!is_valid_domain("example..com"));
        assert!(!is_valid_domain("-example.com"));
        assert!(!is_valid_domain("example-.com"));
        assert!(!is_valid_domain("exam ple.com"));

        let long_label = "a".repeat(64);
        assert!(!is_valid_domain(&format!("{long_label}.com")));
    }

    #[test]
    fn extract_tld_returns_valid_tld() {
        assert_eq!(extract_tld("example.com"), "com");
        assert_eq!(extract_tld("test.co.uk"), "uk");
        assert_eq!(extract_tld("domain.xyz"), "xyz");
    }

    #[test]
    fn extract_tld_returns_unknown_on_invalid_domain() {
        assert_eq!(extract_tld(""), "unknown");
        assert_eq!(extract_tld("."), "unknown");
    }

    #[test]
    fn extract_tld_handles_single_label() {
        // Single label domains (no dots) return the whole label as TLD
        assert_eq!(extract_tld("nodot"), "nodot");
    }

    #[test]
    fn is_valid_domain_handles_edge_cases() {
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

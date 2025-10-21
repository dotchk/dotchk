//! Domain name type and validation.
//!
//! This module provides a type-safe wrapper for domain names,
//! ensuring validation happens at construction time.

use crate::tld::Tld;
use crate::DomainCheckerError;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

/// A validated domain name.
///
/// Domain names are normalized to lowercase and validated at construction.
/// Invalid domains cannot be constructed, ensuring type safety throughout the API.
///
/// # Examples
///
/// ```
/// use dotchk::Domain;
///
/// // Valid domain
/// let domain = Domain::new("example.com").unwrap();
/// assert_eq!(domain.as_str(), "example.com");
///
/// // Invalid domain - construction fails
/// assert!(Domain::new("").is_err());
/// assert!(Domain::new("invalid").is_err());
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Domain(String);

impl Domain {
    /// Create a new validated domain.
    ///
    /// The domain is normalized to lowercase and validated according to:
    /// - Total length: 1-253 characters
    /// - Label length: 1-63 characters per label
    /// - Characters: alphanumeric and hyphens only
    /// - Hyphens: not at start or end of labels
    /// - Format: at least two labels (e.g., "example.com")
    pub fn new(domain: impl Into<String>) -> Result<Self, DomainCheckerError> {
        let domain = domain.into().to_lowercase();

        if !is_valid_domain(&domain) {
            return Err(DomainCheckerError::InvalidDomain(domain));
        }

        Ok(Self(domain))
    }

    /// Returns the domain as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Extracts the TLD from this domain.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotchk::Domain;
    ///
    /// let domain = Domain::new("example.com").unwrap();
    /// assert_eq!(domain.tld().as_str(), "com");
    /// ```
    pub fn tld(&self) -> Tld {
        Tld::extract_from(&self.0)
    }

    /// Consumes self and returns the inner String.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Deref for Domain {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Domain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Domain {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

/// Validates a domain name according to DNS rules.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn domain_new_accepts_valid_formats() {
        assert!(Domain::new("example.com").is_ok());
        assert!(Domain::new("sub.example.com").is_ok());
        assert!(Domain::new("my-site.co.uk").is_ok());
        assert!(Domain::new("123.456.xyz").is_ok());
    }

    #[test]
    fn domain_new_rejects_invalid_formats() {
        assert!(Domain::new("").is_err());
        assert!(Domain::new("example").is_err());
        assert!(Domain::new(".com").is_err());
        assert!(Domain::new("example..com").is_err());
        assert!(Domain::new("-example.com").is_err());
        assert!(Domain::new("example-.com").is_err());
        assert!(Domain::new("exam ple.com").is_err());
    }

    #[test]
    fn domain_normalizes_to_lowercase() {
        let domain = Domain::new("Example.COM").unwrap();
        assert_eq!(domain.as_str(), "example.com");
    }

    #[test]
    fn domain_extracts_tld_correctly() {
        let domain = Domain::new("example.com").unwrap();
        assert_eq!(domain.tld().as_str(), "com");

        let domain = Domain::new("test.co.uk").unwrap();
        assert_eq!(domain.tld().as_str(), "uk");
    }

    #[test]
    fn domain_deref_works() {
        let domain = Domain::new("example.com").unwrap();
        assert!(domain.starts_with("example"));
        assert_eq!(domain.len(), 11);
    }

    #[test]
    fn domain_display_works() {
        let domain = Domain::new("example.com").unwrap();
        assert_eq!(format!("{}", domain), "example.com");
    }

}

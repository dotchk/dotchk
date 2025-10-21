//! Top-level domain (TLD) type and utilities.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

/// A top-level domain (TLD) string.
///
/// TLDs are normalized to lowercase.
///
/// # Examples
///
/// ```
/// use dotchk::Tld;
///
/// let tld = Tld::new("com");
/// assert_eq!(tld.as_str(), "com");
///
/// // Case normalization
/// let tld = Tld::new("COM");
/// assert_eq!(tld.as_str(), "com");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tld(String);

impl Tld {
    /// Create a new TLD from a string.
    ///
    /// The TLD is normalized to lowercase.
    pub fn new(tld: impl Into<String>) -> Self {
        Self(tld.into().to_lowercase())
    }

    /// Extract the TLD from a domain string.
    ///
    /// Returns the last component after splitting by '.'.
    /// If the domain has no dots, returns the entire string.
    ///
    /// # Examples
    ///
    /// ```
    /// use dotchk::Tld;
    ///
    /// let tld = Tld::extract_from("example.com");
    /// assert_eq!(tld.as_str(), "com");
    ///
    /// let tld = Tld::extract_from("test.co.uk");
    /// assert_eq!(tld.as_str(), "uk");
    /// ```
    pub fn extract_from(domain: &str) -> Self {
        let tld = domain
            .split('.')
            .next_back()
            .unwrap_or(domain);
        Self(tld.to_string())
    }

    /// Returns the TLD as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Consumes self and returns the inner String.
    pub fn into_string(self) -> String {
        self.0
    }
}

impl Deref for Tld {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl fmt::Display for Tld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for Tld {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tld_extract_from_works() {
        assert_eq!(Tld::extract_from("example.com").as_str(), "com");
        assert_eq!(Tld::extract_from("test.co.uk").as_str(), "uk");
        assert_eq!(Tld::extract_from("domain.xyz").as_str(), "xyz");
    }

    #[test]
    fn tld_normalizes_to_lowercase() {
        let tld = Tld::new("COM");
        assert_eq!(tld.as_str(), "com");
    }

    #[test]
    fn tld_display_works() {
        let tld = Tld::new("com");
        assert_eq!(format!("{}", tld), "com");
    }

    #[test]
    fn tld_deref_works() {
        let tld = Tld::new("com");
        assert!(tld.starts_with("c"));
        assert_eq!(tld.len(), 3);
    }
}

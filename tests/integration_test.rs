use dotchk::{Checker, Pattern};
use std::time::Duration;

#[tokio::test]
async fn test_real_domain_checks() {
    let checker = Checker::builder()
        .max_parallel(5).expect("Failed to set max_parallel")
        .timeout_ms(5000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    // Test known registered domains
    let registered_domains = vec![
        "google.com".to_string(),
        "github.com".to_string(),
        "stackoverflow.com".to_string(),
    ];

    let results = checker.check_batch(registered_domains).await;

    for result in results {
        match result {
            Ok(check) => {
                assert!(
                    !check.available,
                    "Domain {} should not be available",
                    check.domain
                );
            }
            Err(_) => {
                // Errors are acceptable (e.g., DNS timeout), just not "available"
            }
        }
    }
}

#[tokio::test]
async fn test_likely_unregistered_domains() {
    let checker = Checker::builder()
        .max_parallel(5).expect("Failed to set max_parallel")
        .timeout_ms(5000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    // Test domains that are very likely to be unregistered
    let unregistered_domains = vec![
        "this-is-a-very-long-domain-name-that-should-not-exist-2024.com".to_string(),
        "xyzabc123456789test.net".to_string(),
    ];

    let results = checker.check_batch(unregistered_domains).await;

    for result in results {
        match result {
            Ok(check) => {
                println!(
                    "Domain {} availability: {}",
                    check.domain, check.available
                );
            }
            Err(e) => {
                println!("Error checking domain: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_different_tlds() {
    let checker = Checker::builder()
        .max_parallel(10).expect("Failed to set max_parallel")
        .timeout_ms(5000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    let domains = vec![
        "example.com".to_string(),
        "example.net".to_string(),
        "example.org".to_string(),
        "example.io".to_string(),
        "example.dev".to_string(),
    ];

    let results = checker.check_batch(domains).await;

    // Verify we get results for all domains
    assert_eq!(results.len(), 5);

    for result in results {
        match result {
            Ok(check) => assert!(!check.domain.is_empty()),
            Err(_) => {} // Errors are acceptable
        }
    }
}

#[tokio::test]
async fn test_invalid_domains() {
    let checker = Checker::builder()
        .max_parallel(5).expect("Failed to set max_parallel")
        .timeout_ms(2000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    let invalid_domains = vec![
        "".to_string(),
        "invalid_domain.com".to_string(),   // underscore not allowed
        "-invalid.com".to_string(),         // starts with hyphen
        "invalid-.com".to_string(),         // ends with hyphen
        "invalid..com".to_string(),         // double dot
        ".com".to_string(),                 // no domain name
        "a".to_string(),                    // no TLD
        "test.invalid-tld-xyz".to_string(), // unsupported TLD
    ];

    let results = checker.check_batch(invalid_domains).await;

    for result in results {
        match result {
            Ok(check) => {
                // If it succeeded, it should be unavailable
                assert!(
                    !check.available,
                    "Invalid domain {} should be unavailable",
                    check.domain
                );
            }
            Err(_) => {
                // Errors are expected for invalid domains
            }
        }
    }
}

#[tokio::test]
async fn test_pattern_generation_limits() {
    let pattern = Pattern::compile("[a-z]{2}\\.com").unwrap();
    let domains = pattern.generate(Some(10));

    assert_eq!(domains.len(), 10);
    assert_eq!(domains[0], "aa.com");
    assert_eq!(domains[1], "ab.com");
    assert_eq!(domains[9], "aj.com");

    // Check all generated domains are unique
    let unique: std::collections::HashSet<_> = domains.iter().collect();
    assert_eq!(unique.len(), domains.len());
}

#[tokio::test]
async fn test_concurrent_checks() {
    let checker = Checker::builder()
        .max_parallel(100).expect("Failed to set max_parallel")
        .timeout_ms(3000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    // Generate 100 domains to check concurrently
    let pattern = Pattern::compile("test-[0-9]{6}\\.com").unwrap();
    let domains = pattern.generate(Some(100));

    let start = std::time::Instant::now();
    let results = checker.check_batch(domains).await;
    let duration = start.elapsed();

    assert_eq!(results.len(), 100);

    // With 100 parallel checks, this should complete much faster than sequential
    // Assuming ~100ms per check, sequential would take 10 seconds
    // Parallel should complete in under 2 seconds
    assert!(
        duration < Duration::from_secs(2),
        "Concurrent checks took too long: {duration:?}"
    );
}

#[tokio::test]
async fn test_edge_case_domains() {
    let checker = Checker::builder()
        .max_parallel(5).expect("Failed to set max_parallel")
        .timeout_ms(3000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    let edge_cases = vec![
        "a.com".to_string(),                          // single letter
        "0.com".to_string(),                          // single digit
        "test-123.com".to_string(),                   // alphanumeric with hyphen
        "very-long-domain-name-test.com".to_string(), // long but valid
        format!("{}.com", "a".repeat(63)),            // max label length
    ];

    let results = checker.check_batch(edge_cases).await;

    for result in results {
        match &result {
            Ok(check) => {
                println!(
                    "Edge case domain {}: available={}",
                    check.domain, check.available
                );
                // These are all valid domains, should succeed
            }
            Err(e) => {
                println!("Edge case error: {}", e);
                // Should not have "Invalid domain" errors for valid domains
                assert!(
                    !e.to_string().contains("Invalid domain"),
                    "Valid domain marked as invalid: {}",
                    e
                );
            }
        }
    }
}

#[tokio::test]
#[ignore] // This test actually exports a file
async fn test_csv_export() {
    use dotchk::CsvExporter;
    use std::path::Path;

    let checker = Checker::builder()
        .max_parallel(5).expect("Failed to set max_parallel")
        .timeout_ms(3000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    let domains = vec![
        "test-export-1.com".to_string(),
        "test-export-2.com".to_string(),
        "test-export-3.com".to_string(),
    ];

    let results = checker.check_batch(domains).await;

    let export_path = "/tmp/test_export.csv";
    let exporter = CsvExporter::new(export_path);
    exporter.export(&results).expect("Failed to export CSV");

    assert!(Path::new(export_path).exists());

    // Clean up
    std::fs::remove_file(export_path).ok();
}

#[tokio::test]
async fn test_timeout_handling() {
    let checker = Checker::builder()
        .max_parallel(1).expect("Failed to set max_parallel")
        .timeout_ms(1).expect("Failed to set timeout_ms") // Extremely short timeout
        .build()
        .await
        .expect("Failed to build checker");

    let result = checker.check("timeout-test.com").await;

    // With 1ms timeout, should either error or complete very quickly
    // In practice, DNS queries take longer than 1ms, so this should error
    match result {
        Ok(check) => {
            println!("Timeout test result: available={}", check.available);
        }
        Err(e) => {
            println!("Timeout test error: {}", e);
        }
    }

    // With such a short timeout, we expect the check to complete
    // The actual timeout might be overridden by TLD-specific timeouts
    // Just verify we got a result (available or error)
    // This test is really just checking that timeout doesn't cause a panic
}

#[test]
fn test_pattern_edge_cases() {
    // Empty pattern
    let pattern = Pattern::compile("").unwrap();
    let domains = pattern.generate(Some(1));
    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0], "");

    // Pattern with no variables
    let pattern = Pattern::compile("static.com").unwrap();
    let domains = pattern.generate(Some(10));
    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0], "static.com");

    // Complex pattern
    let pattern = Pattern::compile("test-[a-z]{2}[0-9]{2}\\.com").unwrap();
    let domains = pattern.generate(Some(5));
    // Pattern generates: aa00, ab00, ac00... (rightmost increments first)
    assert_eq!(domains[0], "test-aa00.com");
    assert_eq!(domains[1], "test-ab00.com");
    assert_eq!(domains[2], "test-ac00.com");
}

#[tokio::test]
async fn test_tld_checking_multiple_domains() {
    let checker = Checker::builder()
        .max_parallel(10).expect("Failed to set max_parallel")
        .timeout_ms(3000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    // Test checking the same base domain across multiple TLDs
    let tlds = ["com", "net", "org", "io", "dev"];
    let base_domain = "test-tld-check-2024";

    let domains: Vec<String> = tlds
        .iter()
        .map(|tld| format!("{base_domain}.{tld}"))
        .collect();

    let results = checker.check_batch(domains.clone()).await;

    assert_eq!(results.len(), tlds.len());

    // Create a set of expected domains for order-independent comparison
    let expected_domains: std::collections::HashSet<String> = domains.iter().cloned().collect();
    let result_domains: std::collections::HashSet<String> = results
        .iter()
        .filter_map(|r| r.as_ref().ok().map(|check| check.domain.clone()))
        .collect();

    assert_eq!(
        expected_domains, result_domains,
        "All domains should be checked"
    );

    // Verify each result has valid data
    for result in &results {
        match result {
            Ok(check) => {
                assert!(
                    domains.contains(&check.domain),
                    "Result domain should be in the input list"
                );
                println!(
                    "TLD check {}: available={}",
                    check.domain, check.available
                );
            }
            Err(e) => {
                println!("TLD check error: {}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_tld_popular_domains() {
    let checker = Checker::builder()
        .max_parallel(20).expect("Failed to set max_parallel")
        .timeout_ms(3000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    // Test popular TLDs
    let popular_tlds = ["com", "net", "org", "io", "dev", "app", "co", "me"];
    let test_domain = format!("unique-test-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs());

    let domains: Vec<String> = popular_tlds
        .iter()
        .map(|tld| format!("{test_domain}.{tld}"))
        .collect();

    let results = checker.check_batch(domains).await;

    // Count available domains
    let available_count = results
        .iter()
        .filter(|r| matches!(r, Ok(check) if check.available))
        .count();

    println!(
        "Available domains: {}/{}",
        available_count,
        popular_tlds.len()
    );

    // At least some domains should be available with a unique timestamp-based name
    assert!(
        available_count > 0,
        "Expected at least some domains to be available"
    );
}

#[test]
fn test_alternation_pattern_generation() {
    // Test single alternation group
    let pattern = Pattern::compile("(get|try|use)test.com").unwrap();
    let domains = pattern.generate(None);
    assert_eq!(domains.len(), 3);
    assert_eq!(domains, vec!["gettest.com", "trytest.com", "usetest.com"]);

    // Test multiple alternation groups
    let pattern = Pattern::compile("(web|app)-(dev|prod).(com|io)").unwrap();
    let domains = pattern.generate(None);
    assert_eq!(domains.len(), 8); // 2 × 2 × 2
    let expected = vec![
        "web-dev.com",
        "app-dev.com",
        "web-prod.com",
        "app-prod.com",
        "web-dev.io",
        "app-dev.io",
        "web-prod.io",
        "app-prod.io",
    ];
    assert_eq!(domains, expected);
}

#[test]
fn test_alternation_with_patterns() {
    // Test alternation combined with character classes
    let pattern = Pattern::compile("(get|try)[a-b]{2}.(com|net)").unwrap();
    let domains = pattern.generate(Some(10));
    assert_eq!(domains.len(), 10);

    // First few should be:
    assert_eq!(domains[0], "getaa.com");
    assert_eq!(domains[1], "tryaa.com");
    assert_eq!(domains[2], "getab.com");
    assert_eq!(domains[3], "tryab.com");
    assert_eq!(domains[4], "getba.com");
    assert_eq!(domains[5], "tryba.com");
    assert_eq!(domains[6], "getbb.com");
    assert_eq!(domains[7], "trybb.com");
    assert_eq!(domains[8], "getaa.net");
    assert_eq!(domains[9], "tryaa.net");
}

#[tokio::test]
async fn test_alternation_domain_checking() {
    let checker = Checker::builder()
        .max_parallel(10).expect("Failed to set max_parallel")
        .timeout_ms(3000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    // Generate domains with alternation pattern
    let pattern = Pattern::compile("(test|demo|sample)2024.(com|io|dev)").unwrap();
    let domains = pattern.generate(None);

    assert_eq!(domains.len(), 9); // 3 prefixes × 3 TLDs

    let results = checker.check_batch(domains.clone()).await;

    assert_eq!(results.len(), 9);

    // Create sets for order-independent comparison
    let expected_domains: std::collections::HashSet<String> = domains.iter().cloned().collect();
    let result_domains: std::collections::HashSet<String> = results
        .iter()
        .filter_map(|r| r.as_ref().ok().map(|check| check.domain.clone()))
        .collect();

    assert_eq!(
        expected_domains, result_domains,
        "All generated domains should be checked"
    );

    // Verify all domains were checked with valid data
    for result in &results {
        match result {
            Ok(check) => {
                assert!(
                    domains.contains(&check.domain),
                    "Result domain should be in the generated list"
                );
                println!(
                    "Alternation check {}: available={}",
                    check.domain, check.available
                );
            }
            Err(e) => {
                println!("Alternation check error: {}", e);
            }
        }
    }
}

#[test]
fn test_complex_alternation_patterns() {
    // Test the user's original pattern (with smaller numbers)
    let pattern = Pattern::compile("(get|try|use|my)[a-z]{2}.(com|io)").unwrap();
    let domains = pattern.generate(Some(20));

    assert_eq!(domains.len(), 20);

    // Verify pattern structure
    assert!(
        domains[0].starts_with("get")
            || domains[0].starts_with("try")
            || domains[0].starts_with("use")
            || domains[0].starts_with("my")
    );
    assert!(domains[0].ends_with(".com") || domains[0].ends_with(".io"));

    // Check middle part is 2 lowercase letters
    let prefix_len = if domains[0].starts_with("my") { 2 } else { 3 };
    let middle = &domains[0][prefix_len..prefix_len + 2];
    assert!(middle.chars().all(|c| c.is_ascii_lowercase()));
    assert_eq!(middle.len(), 2);
}

#[test]
fn test_alternation_edge_cases() {
    // Empty alternation options
    let pattern = Pattern::compile("(|test|)domain.com").unwrap();
    let domains = pattern.generate(None);
    // Empty options should be filtered out
    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0], "testdomain.com");

    // Single option in alternation (should still work)
    let pattern = Pattern::compile("(only)test.com").unwrap();
    let domains = pattern.generate(None);
    assert_eq!(domains.len(), 1);
    assert_eq!(domains[0], "onlytest.com");

    // Nested pattern elements
    let pattern = Pattern::compile("(pre|post)-[a-b](1|2).com").unwrap();
    let domains = pattern.generate(None);
    assert_eq!(domains.len(), 8); // 2 prefixes × 2 letters × 2 numbers
}

#[tokio::test]
async fn test_range_quantifier_domain_checking() {
    let checker = Checker::builder()
        .max_parallel(10).expect("Failed to set max_parallel")
        .timeout_ms(3000).expect("Failed to set timeout_ms")
        .build()
        .await
        .expect("Failed to build checker");

    // Generate domains with range quantifier
    let pattern = Pattern::compile("rngtest[0-9]{1,2}.com").unwrap();
    let domains = pattern.generate(Some(5)); // Just check first 5

    // Should have generated domains with 1 and 2 digit numbers
    let results = checker.check_batch(domains.clone()).await;

    assert_eq!(results.len(), 5);

    // Verify the pattern worked correctly
    for result in &results {
        match result {
            Ok(check) => {
                assert!(check.domain.starts_with("rngtest"));
                assert!(check.domain.ends_with(".com"));

                // Check that we have both 1 and 2 digit versions
                let number_part = check
                    .domain
                    .strip_prefix("rngtest")
                    .and_then(|s| s.strip_suffix(".com"))
                    .unwrap();
                assert!(number_part.len() == 1 || number_part.len() == 2);
                assert!(number_part.chars().all(|c| c.is_numeric()));

                println!(
                    "Range quantifier check {}: available={}",
                    check.domain, check.available
                );
            }
            Err(e) => {
                println!("Range quantifier check error: {}", e);
            }
        }
    }
}

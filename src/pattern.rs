use lazy_static::lazy_static;
use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PatternError {
    #[error("Invalid pattern: {0}")]
    InvalidPattern(String),

    #[error("Pattern too complex (would generate too many domains)")]
    TooComplex,
}

lazy_static! {
    // Common TLDs that should be recognized for auto-escaping
    static ref COMMON_TLDS: Vec<&'static str> = vec![
        "com", "net", "org", "io", "co", "uk", "de", "fr", "it", "es", "nl", "ru", "jp", "cn",
        "au", "ca", "br", "in", "kr", "mx", "us", "edu", "gov", "mil", "info", "biz", "name",
        "tv", "cc", "me", "app", "dev", "xyz", "ai", "tech", "online", "store", "site", "website",
        "blog", "shop", "cloud", "digital", "news", "club", "one", "world", "today", "life",
        "live", "studio", "email", "solutions", "services", "marketing", "finance", "network",
        "software", "systems", "technology", "agency", "academy", "center", "company", "computer",
        "design", "energy", "engineering", "expert", "global", "group", "host", "international",
        "media", "money", "photo", "photography", "plus", "pro", "social", "space", "support",
        "team", "tips", "tools", "video", "web", "work", "works", "zone"
    ];
}

/// Preprocesses a pattern to automatically escape dots before common TLDs
/// This allows users to write `domain[a-z].com` instead of `domain[a-z]\.com`
fn preprocess_pattern(pattern: &str) -> String {
    // Check if pattern already has escaped dots (user knows what they're doing)
    if pattern.contains("\\.") {
        return pattern.to_string();
    }

    let mut result = String::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '.' {
            // Check if this dot is followed by a known TLD
            let remaining: String = chars[i + 1..].iter().collect();

            // Check if the remaining string starts with a TLD
            let mut found_tld = false;
            for tld in COMMON_TLDS.iter() {
                // Check if it matches the TLD exactly or TLD followed by non-alphanumeric
                if remaining == *tld
                    || (remaining.starts_with(tld)
                        && remaining
                            .chars()
                            .nth(tld.len())
                            .is_none_or(|c| !c.is_alphanumeric()))
                {
                    // Escape this dot
                    result.push('\\');
                    result.push('.');
                    found_tld = true;
                    break;
                }
            }

            if !found_tld {
                // Not a TLD, just add the dot
                result.push('.');
            }
        } else {
            result.push(chars[i]);
        }
        i += 1;
    }

    result
}

pub struct Pattern {
    pattern: String,
}

impl Pattern {
    pub fn compile(pattern: &str) -> Result<Self, PatternError> {
        // Preprocess pattern to auto-escape dots before TLDs
        let processed_pattern = preprocess_pattern(pattern);

        // Check pattern complexity
        let parts = parse_pattern(&processed_pattern)?;
        let complexity = estimate_complexity(&parts);
        if complexity > 1_000_000 {
            return Err(PatternError::TooComplex);
        }

        let regex_pattern = pattern_to_regex(&processed_pattern)?;
        // Validate the regex pattern
        Regex::new(&regex_pattern).map_err(|e| PatternError::InvalidPattern(e.to_string()))?;

        Ok(Pattern {
            pattern: processed_pattern,
        })
    }

    pub fn generate_iter(&self) -> PatternIterator {
        PatternIterator::new(&self.pattern)
    }

    pub fn generate(&self, limit: Option<usize>) -> Vec<String> {
        let mut results = Vec::new();
        let mut iter = self.generate_iter();

        for _ in 0..limit.unwrap_or(usize::MAX) {
            match iter.next() {
                Some(domain) => results.push(domain),
                None => break,
            }
        }

        results
    }
}

pub struct PatternIterator {
    parts: Vec<PatternPart>,
    state: Vec<usize>,
    done: bool,
}

#[derive(Debug, Clone)]
enum PatternPart {
    Literal(String),
    CharClass(Vec<char>),
    Quantified(Vec<char>, usize, usize), // chars, min, max
    Alternation(Vec<String>),            // List of alternatives like ["get", "try", "use", "my"]
}

impl PatternIterator {
    fn new(pattern: &str) -> Self {
        let parts = parse_pattern(pattern)
            .unwrap_or_else(|_| vec![PatternPart::Literal(pattern.to_string())]);
        let state = initialize_state(&parts);

        PatternIterator {
            parts,
            state,
            done: false,
        }
    }
}

impl Iterator for PatternIterator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }

        // Generate current string
        let mut result = String::new();
        let mut state_idx = 0;

        for part in &self.parts {
            match part {
                PatternPart::Literal(s) => result.push_str(s),
                PatternPart::CharClass(chars) => {
                    if !chars.is_empty() && state_idx < self.state.len() {
                        result.push(chars[self.state[state_idx]]);
                        state_idx += 1;
                    }
                }
                PatternPart::Quantified(chars, min, max) => {
                    if !chars.is_empty() && state_idx < self.state.len() {
                        // For range quantifiers {min,max}, we need to determine the current length
                        // The first element of state for this quantifier stores the length
                        let length_idx = state_idx;
                        let current_length = if min == max {
                            *min
                        } else {
                            // For range quantifiers, state[length_idx] represents the offset from min
                            // But we need to handle when the actual length should be 0-max
                            if *min == 0 {
                                self.state[length_idx] // 0, 1, 2, ..., max
                            } else {
                                min + self.state[length_idx] // min, min+1, ..., max
                            }
                        };

                        // Skip the length indicator if it's a range
                        if min != max {
                            state_idx += 1;
                        }

                        // Generate characters for this length
                        for _i in 0..current_length {
                            if state_idx < self.state.len() {
                                result.push(chars[self.state[state_idx] % chars.len()]);
                                state_idx += 1;
                            }
                        }

                        // Skip any unused slots
                        state_idx = length_idx + if min == max { *max } else { max + 1 };
                    }
                }
                PatternPart::Alternation(options) => {
                    if !options.is_empty() && state_idx < self.state.len() {
                        result.push_str(&options[self.state[state_idx]]);
                        state_idx += 1;
                    }
                }
            }
        }

        // Increment state
        if !increment_state(&mut self.state, &self.parts) {
            self.done = true;
        }

        Some(result)
    }
}

fn initialize_state(parts: &[PatternPart]) -> Vec<usize> {
    let mut state = Vec::new();

    for part in parts {
        match part {
            PatternPart::Literal(_) => {}
            PatternPart::CharClass(_) => {
                state.push(0);
            }
            PatternPart::Quantified(_, min, max) => {
                // For range quantifiers, we need an extra slot for the length indicator
                if min == max {
                    state.extend(std::iter::repeat_n(0, *max));
                } else {
                    // One slot for length indicator + max slots for characters
                    state.extend(std::iter::repeat_n(0, max + 1));
                }
            }
            PatternPart::Alternation(_) => {
                state.push(0);
            }
        }
    }

    state
}

fn increment_state(state: &mut [usize], parts: &[PatternPart]) -> bool {
    let mut state_idx = 0;
    let mut carry = true;

    for part in parts {
        if !carry {
            break;
        }

        match part {
            PatternPart::Literal(_) => {}
            PatternPart::CharClass(chars) => {
                if state_idx < state.len() && !chars.is_empty() {
                    state[state_idx] += 1;
                    if state[state_idx] >= chars.len() {
                        state[state_idx] = 0;
                        carry = true;
                    } else {
                        carry = false;
                    }
                    state_idx += 1;
                }
            }
            PatternPart::Quantified(chars, min, max) => {
                if min == max {
                    // Fixed length quantifier
                    let length = *min;
                    let mut local_carry = true;

                    // Try to increment from rightmost position
                    for i in (0..length).rev() {
                        if state_idx + i < state.len() && !chars.is_empty() {
                            state[state_idx + i] += 1;
                            if state[state_idx + i] >= chars.len() {
                                state[state_idx + i] = 0;
                                local_carry = true;
                            } else {
                                local_carry = false;
                                carry = false;
                                break;
                            }
                        }
                    }

                    if local_carry {
                        carry = true;
                    }

                    state_idx += max;
                } else {
                    // Range quantifier {min,max}
                    let length_idx = state_idx;
                    let current_length = if *min == 0 {
                        state[length_idx]
                    } else {
                        min + state[length_idx]
                    };

                    // Only try to increment if we're within the current length
                    if current_length > 0 {
                        let mut local_carry = true;

                        // Try to increment character positions for current length
                        for i in (1..=current_length).rev() {
                            if state_idx + i < state.len() && !chars.is_empty() {
                                state[state_idx + i] += 1;
                                if state[state_idx + i] >= chars.len() {
                                    state[state_idx + i] = 0;
                                    local_carry = true;
                                } else {
                                    local_carry = false;
                                    carry = false;
                                    break;
                                }
                            }
                        }

                        // If we carried through all positions, try next length
                        if local_carry {
                            if current_length < *max {
                                // Move to next length
                                state[length_idx] += 1;
                                carry = false;
                                // Reset all character positions to 0 for the new length
                                for i in 1..=*max {
                                    if state_idx + i < state.len() {
                                        state[state_idx + i] = 0;
                                    }
                                }
                            } else {
                                // We've exhausted all lengths, reset to min and carry
                                state[length_idx] = 0;
                                carry = true;
                                // Reset all positions
                                for i in 1..=*max {
                                    if state_idx + i < state.len() {
                                        state[state_idx + i] = 0;
                                    }
                                }
                            }
                        }
                    } else {
                        // Length is 0 (for {0,n} patterns), increment length
                        if *min == 0 && state[length_idx] == 0 {
                            state[length_idx] = 1;
                            carry = false;
                        } else {
                            // This shouldn't happen in normal flow
                            carry = true;
                        }
                    }

                    state_idx += max + 1;
                }
            }
            PatternPart::Alternation(options) => {
                if state_idx < state.len() && !options.is_empty() {
                    state[state_idx] += 1;
                    if state[state_idx] >= options.len() {
                        state[state_idx] = 0;
                        carry = true;
                    } else {
                        carry = false;
                    }
                    state_idx += 1;
                }
            }
        }
    }

    !carry
}

fn estimate_complexity(parts: &[PatternPart]) -> usize {
    let mut complexity: usize = 1;

    for part in parts {
        match part {
            PatternPart::Literal(_) => {}
            PatternPart::CharClass(chars) => {
                complexity = complexity.saturating_mul(chars.len());
            }
            PatternPart::Quantified(chars, min, max) => {
                // For range quantifiers, calculate the sum of possibilities for each length
                if min == max {
                    // Fixed length: chars.len()^min
                    for _ in 0..*min {
                        complexity = complexity.saturating_mul(chars.len());
                    }
                } else {
                    // Range: sum of chars.len()^i for i from min to max
                    let mut range_complexity = 0usize;
                    for length in *min..=*max {
                        let mut length_complexity = 1usize;
                        for _ in 0..length {
                            length_complexity = length_complexity.saturating_mul(chars.len());
                        }
                        range_complexity = range_complexity.saturating_add(length_complexity);
                    }
                    complexity = complexity.saturating_mul(range_complexity);
                }
            }
            PatternPart::Alternation(options) => {
                complexity = complexity.saturating_mul(options.len());
            }
        }
    }

    complexity
}

fn pattern_to_regex(pattern: &str) -> Result<String, PatternError> {
    let mut regex = String::from("^");
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            '[' => {
                regex.push('[');
                i += 1;
                while i < chars.len() && chars[i] != ']' {
                    regex.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() {
                    regex.push(']');
                    i += 1;
                }
            }
            '{' => {
                let mut num = String::new();
                i += 1;
                while i < chars.len() && chars[i] != '}' && chars[i] != ',' {
                    num.push(chars[i]);
                    i += 1;
                }
                if i < chars.len() && chars[i] == ',' {
                    regex.push('{');
                    regex.push_str(&num);
                    regex.push(',');
                    i += 1;
                    let mut num2 = String::new();
                    while i < chars.len() && chars[i] != '}' {
                        num2.push(chars[i]);
                        i += 1;
                    }
                    regex.push_str(&num2);
                    regex.push('}');
                } else {
                    regex.push('{');
                    regex.push_str(&num);
                    regex.push('}');
                }
                if i < chars.len() {
                    i += 1;
                }
            }
            '\\' => {
                regex.push('\\');
                if i + 1 < chars.len() {
                    i += 1;
                    regex.push(chars[i]);
                }
                i += 1;
            }
            '.' | '^' | '$' | '*' | '+' | '?' | '(' | ')' | '|' => {
                regex.push('\\');
                regex.push(chars[i]);
                i += 1;
            }
            _ => {
                regex.push(chars[i]);
                i += 1;
            }
        }
    }

    regex.push('$');
    Ok(regex)
}

fn parse_pattern(pattern: &str) -> Result<Vec<PatternPart>, PatternError> {
    let mut parts = Vec::new();
    let chars: Vec<char> = pattern.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        if chars[i] == '[' {
            // Character class
            i += 1;
            let mut class_chars = Vec::new();

            while i < chars.len() && chars[i] != ']' {
                if chars[i] == '-'
                    && !class_chars.is_empty()
                    && i + 1 < chars.len()
                    && chars[i + 1] != ']'
                {
                    // Range
                    let start = *class_chars.last().unwrap();
                    i += 1;
                    let end = chars[i];
                    class_chars.pop();
                    for c in start as u8..=end as u8 {
                        class_chars.push(c as char);
                    }
                } else {
                    class_chars.push(chars[i]);
                }
                i += 1;
            }

            if i < chars.len() {
                i += 1; // Skip ']'
            }

            // Check for quantifier
            if i < chars.len() && chars[i] == '{' {
                i += 1;
                let mut num_str = String::new();
                while i < chars.len() && chars[i].is_numeric() {
                    num_str.push(chars[i]);
                    i += 1;
                }

                let min = num_str.parse::<usize>().unwrap_or(1);
                let max = if i < chars.len() && chars[i] == ',' {
                    i += 1;
                    let mut max_str = String::new();
                    while i < chars.len() && chars[i].is_numeric() {
                        max_str.push(chars[i]);
                        i += 1;
                    }
                    max_str.parse::<usize>().unwrap_or(min)
                } else {
                    min
                };

                if i < chars.len() && chars[i] == '}' {
                    i += 1;
                }

                parts.push(PatternPart::Quantified(class_chars, min, max));
            } else {
                parts.push(PatternPart::CharClass(class_chars));
            }
        } else if chars[i] == '(' {
            // Alternation group
            i += 1; // Skip '('
            let mut options = Vec::new();
            let mut current_option = String::new();

            while i < chars.len() && chars[i] != ')' {
                if chars[i] == '|' {
                    // End of current option
                    if !current_option.is_empty() {
                        options.push(current_option.clone());
                        current_option.clear();
                    }
                } else {
                    current_option.push(chars[i]);
                }
                i += 1;
            }

            // Add the last option
            if !current_option.is_empty() {
                options.push(current_option);
            }

            if i < chars.len() {
                i += 1; // Skip ')'
            }

            if !options.is_empty() {
                parts.push(PatternPart::Alternation(options));
            }
        } else if chars[i] == '\\' && i + 1 < chars.len() {
            // Escaped character
            i += 1;
            let mut literal = String::new();
            literal.push(chars[i]);
            i += 1;

            // Collect more literals if consecutive
            while i < chars.len() && chars[i] != '[' && chars[i] != '\\' && chars[i] != '(' {
                literal.push(chars[i]);
                i += 1;
            }

            parts.push(PatternPart::Literal(literal));
        } else {
            // Literal
            let mut literal = String::new();
            while i < chars.len() && chars[i] != '[' && chars[i] != '\\' && chars[i] != '(' {
                literal.push(chars[i]);
                i += 1;
            }

            if !literal.is_empty() {
                parts.push(PatternPart::Literal(literal));
            }
        }
    }

    Ok(parts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocess_pattern_auto_escape() {
        // Test auto-escaping of common TLDs
        assert_eq!(preprocess_pattern("domain[a-z].com"), "domain[a-z]\\.com");
        assert_eq!(preprocess_pattern("test[0-9]{3}.io"), "test[0-9]{3}\\.io");
        assert_eq!(preprocess_pattern("site.net"), "site\\.net");

        // Test that already escaped patterns are not modified
        assert_eq!(preprocess_pattern("domain[a-z]\\.com"), "domain[a-z]\\.com");

        // Test patterns with TLDs in the middle
        assert_eq!(
            preprocess_pattern("get[a-z].com-site"),
            "get[a-z]\\.com-site"
        );

        // Test patterns with multiple dots (only TLD dots are escaped)
        assert_eq!(preprocess_pattern("sub.domain.com"), "sub.domain\\.com");

        // Test that non-TLD dots are not escaped
        assert_eq!(preprocess_pattern("version1.2"), "version1.2");

        // Test edge cases
        assert_eq!(preprocess_pattern(".com"), "\\.com");
        assert_eq!(preprocess_pattern("com"), "com");
    }

    #[test]
    fn test_pattern_with_unescaped_dots() {
        // Test that patterns with unescaped dots work correctly
        let pattern = Pattern::compile("test[abc].com").unwrap();
        let domains = pattern.generate(Some(3));
        assert_eq!(domains.len(), 3);
        assert!(domains.contains(&"testa.com".to_string()));
        assert!(domains.contains(&"testb.com".to_string()));
        assert!(domains.contains(&"testc.com".to_string()));

        // Test with quantifiers
        let pattern = Pattern::compile("domain[0-9]{2}.io").unwrap();
        let domains = pattern.generate(Some(5));
        assert!(domains[0].ends_with(".io"));
        assert!(domains[0].starts_with("domain"));
    }

    #[test]
    fn test_alternation_single_group() {
        // Test simple alternation
        let pattern = Pattern::compile("(get|try|use)test.com").unwrap();
        let domains = pattern.generate(None);
        assert_eq!(domains.len(), 3);
        assert!(domains.contains(&"gettest.com".to_string()));
        assert!(domains.contains(&"trytest.com".to_string()));
        assert!(domains.contains(&"usetest.com".to_string()));
    }

    #[test]
    fn test_alternation_with_character_class() {
        // Test alternation combined with character classes
        let pattern = Pattern::compile("(get|try)[a-b].com").unwrap();
        let domains = pattern.generate(None);
        assert_eq!(domains.len(), 4); // 2 prefixes × 2 letters
        assert!(domains.contains(&"geta.com".to_string()));
        assert!(domains.contains(&"getb.com".to_string()));
        assert!(domains.contains(&"trya.com".to_string()));
        assert!(domains.contains(&"tryb.com".to_string()));
    }

    #[test]
    fn test_alternation_with_quantifier() {
        // Test alternation with quantifiers
        let pattern = Pattern::compile("(my|our)[a-b]{2}.com").unwrap();
        let domains = pattern.generate(None);
        assert_eq!(domains.len(), 8); // 2 prefixes × 2² letter combinations
        assert!(domains.contains(&"myaa.com".to_string()));
        assert!(domains.contains(&"myab.com".to_string()));
        assert!(domains.contains(&"myba.com".to_string()));
        assert!(domains.contains(&"mybb.com".to_string()));
        assert!(domains.contains(&"ouraa.com".to_string()));
        assert!(domains.contains(&"ourbb.com".to_string()));
    }

    #[test]
    fn test_multiple_alternation_groups() {
        // Test multiple alternation groups in one pattern
        let pattern = Pattern::compile("(get|try)app.(com|io)").unwrap();
        let domains = pattern.generate(None);
        assert_eq!(domains.len(), 4); // 2 prefixes × 2 TLDs
        assert!(domains.contains(&"getapp.com".to_string()));
        assert!(domains.contains(&"getapp.io".to_string()));
        assert!(domains.contains(&"tryapp.com".to_string()));
        assert!(domains.contains(&"tryapp.io".to_string()));
    }

    #[test]
    fn test_alternation_complex() {
        // Test complex pattern with alternation, character classes, and quantifiers
        let pattern = Pattern::compile("(web|app|api)[0-9]{2}.(com|net|org)").unwrap();
        let domains = pattern.generate(Some(10));
        assert_eq!(domains.len(), 10);

        // Check that first domain follows the pattern
        let first = &domains[0];
        assert!(first.starts_with("web") || first.starts_with("app") || first.starts_with("api"));
        assert!(first.ends_with(".com") || first.ends_with(".net") || first.ends_with(".org"));

        // Check middle part is 2 digits
        let middle = &first[3..5];
        assert!(middle.chars().all(|c| c.is_numeric()));
        assert_eq!(middle.len(), 2);
    }

    #[test]
    fn test_alternation_empty_option() {
        // Test alternation with empty options (should be ignored)
        let pattern = Pattern::compile("(get||try)test.com").unwrap();
        let domains = pattern.generate(None);
        assert_eq!(domains.len(), 2); // Empty option is ignored
        assert!(domains.contains(&"gettest.com".to_string()));
        assert!(domains.contains(&"trytest.com".to_string()));
    }

    #[test]
    fn test_alternation_order() {
        // Test that alternation generates in the correct order
        let pattern = Pattern::compile("(a|b|c)x.com").unwrap();
        let domains = pattern.generate(None);
        assert_eq!(domains, vec!["ax.com", "bx.com", "cx.com"]);
    }

    #[test]
    fn test_pattern_simple() {
        // Test both escaped and unescaped versions
        let pattern1 = Pattern::compile("test-[abc].com").unwrap();
        let pattern2 = Pattern::compile("test-[abc]\\.com").unwrap();

        let domains1 = pattern1.generate(Some(10));
        let domains2 = pattern2.generate(Some(10));

        assert_eq!(domains1, domains2);
        assert_eq!(domains1.len(), 3);
        assert!(domains1.contains(&"test-a.com".to_string()));
        assert!(domains1.contains(&"test-b.com".to_string()));
        assert!(domains1.contains(&"test-c.com".to_string()));
    }

    #[test]
    fn test_pattern_quantified() {
        let pattern = Pattern::compile("x[0-9]{2}.com").unwrap();
        let domains = pattern.generate(Some(10));
        println!("Generated domains: {:?}", &domains[..5]);
        assert!(domains.len() >= 5);
        assert!(domains[0].starts_with("x") && domains[0].ends_with(".com"));
        // The pattern should generate 2-digit numbers
        for domain in domains.iter().take(5) {
            assert_eq!(domain.len(), "x00.com".len());
        }
    }

    #[test]
    fn test_pattern_mixed() {
        let pattern = Pattern::compile("app-[a-z]{3}.io").unwrap();
        let domains = pattern.generate(Some(30));
        assert!(domains.len() >= 26);
        for domain in &domains {
            assert!(domain.starts_with("app-") && domain.ends_with(".io"));
        }
        // Check uniqueness
        let unique: std::collections::HashSet<_> = domains.iter().collect();
        assert_eq!(unique.len(), domains.len());
    }

    #[test]
    fn test_pattern_complexity() {
        // Should reject overly complex patterns
        let result = Pattern::compile("[a-z]{100}");
        assert!(result.is_err());

        // Should accept reasonable patterns
        let result = Pattern::compile("[a-z]{3}");
        assert!(result.is_ok());
    }

    #[test]
    fn test_pattern_special_chars() {
        let pattern = Pattern::compile("test\\-domain\\.[a-z]{3}").unwrap();
        let domains = pattern.generate(Some(3));
        assert_eq!(domains[0], "test-domain.aaa");
        assert_eq!(domains[1], "test-domain.aab");
        assert_eq!(domains[2], "test-domain.aac");
    }

    #[test]
    fn test_empty_pattern() {
        let pattern = Pattern::compile("").unwrap();
        let domains = pattern.generate(Some(10));
        assert_eq!(domains.len(), 1);
        assert_eq!(domains[0], "");
    }

    #[test]
    fn test_range_quantifier_simple() {
        // Test {1,2} range quantifier
        let pattern = Pattern::compile("test[a-b]{1,2}.com").unwrap();
        let domains = pattern.generate(None);

        println!("Simple range quantifier domains: {:?}", domains);
        println!("Total domains generated: {}", domains.len());

        // Should generate: a, b, aa, ab, ba, bb = 6 domains
        assert_eq!(domains.len(), 6);

        // Check single character versions
        assert!(domains.contains(&"testa.com".to_string()));
        assert!(domains.contains(&"testb.com".to_string()));

        // Check two character versions
        assert!(domains.contains(&"testaa.com".to_string()));
        assert!(domains.contains(&"testab.com".to_string()));
        assert!(domains.contains(&"testba.com".to_string()));
        assert!(domains.contains(&"testbb.com".to_string()));
    }

    #[test]
    fn test_range_quantifier_larger() {
        // Test {2,4} range quantifier
        let pattern = Pattern::compile("x[0-1]{2,4}.com").unwrap();
        let domains = pattern.generate(None);

        // Should generate: 2^2 + 2^3 + 2^4 = 4 + 8 + 16 = 28 domains
        assert_eq!(domains.len(), 28);

        // Check some samples
        assert!(domains.contains(&"x00.com".to_string())); // length 2
        assert!(domains.contains(&"x11.com".to_string())); // length 2
        assert!(domains.contains(&"x000.com".to_string())); // length 3
        assert!(domains.contains(&"x111.com".to_string())); // length 3
        assert!(domains.contains(&"x0000.com".to_string())); // length 4
        assert!(domains.contains(&"x1111.com".to_string())); // length 4
    }

    #[test]
    fn test_range_quantifier_with_alternation() {
        // Test combining range quantifiers with alternation
        let pattern = Pattern::compile("(get|try)[a-z]{1,2}.com").unwrap();
        let domains = pattern.generate(Some(100));

        // Should have both prefixes with 1 and 2 letter suffixes
        assert!(domains.contains(&"geta.com".to_string()));
        assert!(domains.contains(&"trya.com".to_string()));
        assert!(domains.contains(&"getaa.com".to_string()));
        assert!(domains.contains(&"tryaa.com".to_string()));

        // Total should be 2 * (26 + 26*26) = 2 * 702 = 1404
        assert_eq!(domains.len(), 100); // Limited to 100
    }

    #[test]
    fn test_range_quantifier_edge_cases() {
        // Test {0,2} - including empty
        let pattern = Pattern::compile("test[a-z]{0,2}.com").unwrap();
        let domains = pattern.generate(None);

        println!("Generated domains: {:?}", domains);

        // Should include "test.com" (0 characters)
        assert!(domains.contains(&"test.com".to_string()));
        // And single/double character versions
        assert!(domains.contains(&"testa.com".to_string()));
        assert!(domains.contains(&"testaa.com".to_string()));
    }
}

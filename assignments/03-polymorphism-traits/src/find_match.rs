use anyhow::Result;
use regex::bytes::Regex;

trait SearchNeedle {
    fn find_match<'a>(
        self,
        lines: Box<dyn Iterator<Item = String> + 'a>,
        ignore_case: bool,
        invert_match: bool,
    ) -> Result<Box<dyn Iterator<Item = String> + 'a>>;
}

struct StringNeedle {
    string: String,
}

struct RegexNeedle {
    string: Regex,
}

impl SearchNeedle for StringNeedle {
    fn find_match<'a>(
        self,
        lines: Box<dyn Iterator<Item = String> + 'a>,
        ignore_case: bool,
        invert_match: bool,
    ) -> Result<Box<dyn Iterator<Item = String> + 'a>> {
        // Check if the needle is empty
        if self.string.is_empty() {
            // If the needle is empty, match every line
            let filtered_lines = lines.filter(move |_| !invert_match);
            return Ok(Box::new(filtered_lines));
        }

        let filtered_lines = lines.filter(move |line| {
            let processed_line = if ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };
            let contains_needle = processed_line.contains(&self.string);
            if invert_match {
                !contains_needle
            } else {
                contains_needle
            }
        });

        Ok(Box::new(filtered_lines))
    }
}

impl SearchNeedle for RegexNeedle {
    fn find_match<'a>(
        self,
        lines: Box<dyn Iterator<Item = String> + 'a>,
        ignore_case: bool,
        invert_match: bool,
    ) -> Result<Box<dyn Iterator<Item = String> + 'a>> {
        let filtered_lines = lines.filter(move |line| {
            let processed_line = if ignore_case {
                line.to_lowercase()
            } else {
                line.to_string()
            };

            let contains_needle = self.string.is_match(processed_line.as_bytes());
            if invert_match {
                !contains_needle
            } else {
                contains_needle
            }
        });

        Ok(Box::new(filtered_lines))
    }
}

pub fn filter_lines<'a>(
    needle: String,
    lines: Box<dyn Iterator<Item = String> + 'a>,
    ignore_case: bool,
    invert_match: bool,
) -> Result<Box<dyn Iterator<Item = String> + 'a>> {
    let processed_needle = if ignore_case {
        needle.to_lowercase()
    } else {
        needle
    };

    if let Ok(re) = Regex::new(&processed_needle) {
        // If it's a valid regex, use regex matching
        RegexNeedle { string: re }.find_match(lines, ignore_case, invert_match)
    } else {
        // Otherwise, use string matching
        StringNeedle {
            string: processed_needle,
        }
        .find_match(lines, ignore_case, invert_match)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper function to create a vector of strings for testing
    fn get_test_lines() -> Box<dyn Iterator<Item = String>> {
        Box::new(
            vec![
                "The quick brown fox".to_string(),
                "jumps over the lazy dog".to_string(),
                "HELLO WORLD".to_string(),
                "rust is awesome".to_string(),
                "Regex is powerful".to_string(),
            ]
            .into_iter(),
        )
    }

    // Test: Case-sensitive string match (default)
    #[test]
    fn test_string_needle_case_sensitive() {
        let needle = "quick".to_string();
        let result = filter_lines(needle, get_test_lines(), false, false).unwrap();
        let matched: Vec<String> = result.collect();

        assert_eq!(matched, vec!["The quick brown fox"]);
    }

    // Test: Case-insensitive string match
    #[test]
    fn test_string_needle_case_insensitive() {
        let needle = "hello".to_string();
        let result = filter_lines(needle, get_test_lines(), true, false).unwrap();
        let matched: Vec<String> = result.collect();

        assert_eq!(matched, vec!["HELLO WORLD"]);
    }

    // Test: Inverted string match (matches lines that don't contain the needle)
    #[test]
    fn test_string_needle_inverted_match() {
        let needle = "dog".to_string();
        let result = filter_lines(needle, get_test_lines(), false, true).unwrap();
        let matched: Vec<String> = result.collect();

        assert_eq!(
            matched,
            vec![
                "The quick brown fox",
                "HELLO WORLD",
                "rust is awesome",
                "Regex is powerful",
            ]
        );
    }

    // Test: Regex-based match
    #[test]
    fn test_regex_needle_match() {
        let needle = r"\bquick\b".to_string(); // Word boundary regex for "quick"
        let result = filter_lines(needle, get_test_lines(), false, false).unwrap();
        let matched: Vec<String> = result.collect();

        assert_eq!(matched, vec!["The quick brown fox"]);
    }

    // Test: Regex-based inverted match
    #[test]
    fn test_regex_needle_inverted_match() {
        let needle = r"\bWORLD\b".to_string(); // Word boundary regex for "WORLD"
        let result = filter_lines(needle, get_test_lines(), false, true).unwrap();
        let matched: Vec<String> = result.collect();

        assert_eq!(
            matched,
            vec![
                "The quick brown fox",
                "jumps over the lazy dog",
                "rust is awesome",
                "Regex is powerful",
            ]
        );
    }

    // Test: Regex match on different lines
    #[test]
    fn test_regex_needle_different_lines() {
        let needle = r"rust".to_string();
        let result = filter_lines(needle, get_test_lines(), false, false).unwrap();
        let matched: Vec<String> = result.collect();

        assert_eq!(matched, vec!["rust is awesome"]);
    }

    // Test: Empty needle (should return all lines)
    #[test]
    fn test_empty_needle() {
        let needle = "".to_string(); // Empty string as needle
        let result = filter_lines(needle, get_test_lines(), false, false).unwrap();
        let matched: Vec<String> = result.collect();

        // All the lines should match an empty string
        assert_eq!(
            matched,
            vec![
                "The quick brown fox",
                "jumps over the lazy dog",
                "HELLO WORLD",
                "rust is awesome",
                "Regex is powerful",
            ]
        );
    }
}

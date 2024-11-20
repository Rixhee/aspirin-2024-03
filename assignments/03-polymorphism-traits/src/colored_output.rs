use anyhow::Result;
use colored::{Color, Colorize};
use std::io::Write;

struct ColoredString {
    string: String,
    needle: String,
}

struct PlainString {
    string: String,
}

pub trait WriteOutput {
    fn write_output(&self, writer: &mut dyn Write, color: Option<Color>) -> Result<()>;
}

fn split_at_substring<'a>(haystack: &'a str, needle: &str) -> Option<(&'a str, &'a str)> {
    if !needle.is_empty() {
        if let Some(pos) = haystack.find(needle) {
            let (prefix, suffix) = haystack.split_at(pos);
            let suffix = &suffix[needle.len()..];
            Some((prefix, suffix))
        } else {
            None
        }
    } else {
        None
    }
}

impl WriteOutput for ColoredString {
    fn write_output(&self, writer: &mut dyn Write, color: Option<Color>) -> Result<()> {
        if let Some(c) = color {
            if let Some((prefix, suffix)) = split_at_substring(&self.string, &self.needle) {
                writeln!(writer, "{}{}{}", prefix, self.needle.color(c), suffix)?;
            } else {
                // No match found, print entire line as plain text
                writeln!(writer, "{}", self.string)?;
            }
        } else {
            writeln!(writer, "{}", self.string)?;
        }
        Ok(())
    }
}

impl WriteOutput for PlainString {
    fn write_output(&self, writer: &mut dyn Write, _color: Option<Color>) -> Result<()> {
        writeln!(writer, "{}", self.string)?;
        Ok(())
    }
}

pub fn colored_output(
    lines: Box<dyn Iterator<Item = String>>,
    writer: &mut dyn Write,
    needle: String,
    color: Option<Color>,
) -> Result<()> {
    for line in lines {
        if color.is_some() {
            ColoredString {
                string: line.clone(),
                needle: needle.to_string(),
            }
            .write_output(writer, color)?;
        } else {
            PlainString {
                string: line.clone(),
            }
            .write_output(writer, None)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use colored::Colorize;
    use std::io::Cursor;

    // Tests for the split_at_substring function
    #[test]
    fn test_split_at_substring_found() {
        let haystack = "Hello, world!";
        let needle = "world";
        let result = split_at_substring(haystack, needle);
        assert_eq!(result, Some(("Hello, ", "!")));
    }

    #[test]
    fn test_split_at_substring_not_found() {
        let haystack = "Hello, world!";
        let needle = "planet";
        let result = split_at_substring(haystack, needle);
        assert_eq!(result, None);
    }

    #[test]
    fn test_split_at_substring_empty_needle() {
        let haystack = "Hello, world!";
        let needle = "";
        let result = split_at_substring(haystack, needle);
        assert_eq!(result, None); // Expecting None since empty needle should not match
    }

    // Test for ColoredString with a matching substring and color
    #[test]
    fn test_colored_string_with_color() {
        let mut output = Cursor::new(Vec::new());
        let colored_string = ColoredString {
            string: "Test string with match".to_string(),
            needle: "match".to_string(),
        };
        colored_string
            .write_output(&mut output, Some(Color::Red))
            .unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, format!("Test string with {}\n", "match".red()));
    }

    // Test for ColoredString with no color (plain text output)
    #[test]
    fn test_colored_string_without_color() {
        let mut output = Cursor::new(Vec::new());
        let colored_string = ColoredString {
            string: "Test string without color".to_string(),
            needle: "color".to_string(),
        };
        colored_string.write_output(&mut output, None).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "Test string without color\n");
    }

    // Test for ColoredString when the needle is not found (no color applied)
    #[test]
    fn test_colored_string_no_match() {
        let mut output = Cursor::new(Vec::new());
        let colored_string = ColoredString {
            string: "No match in this string".to_string(),
            needle: "absent".to_string(),
        };
        colored_string
            .write_output(&mut output, Some(Color::Blue))
            .unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "No match in this string\n");
    }

    // Test for PlainString output (always plain text)
    #[test]
    fn test_plain_string_output() {
        let mut output = Cursor::new(Vec::new());
        let plain_string = PlainString {
            string: "Plain string".to_string(),
        };
        plain_string.write_output(&mut output, None).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "Plain string\n");
    }

    // Test for colored_output function with color and matching needle
    #[test]
    fn test_colored_output_with_color() {
        let lines =
            Box::new(vec!["First match here".to_string(), "Another match".to_string()].into_iter());
        let mut output = Cursor::new(Vec::new());

        colored_output(lines, &mut output, "match".to_string(), Some(Color::Yellow)).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert!(written.contains("First "));
        assert!(written.contains(&"match".yellow().to_string()));
        assert!(written.contains("Another "));
        assert!(written.contains(&"match".yellow().to_string()));
    }

    // Test for colored_output function without color (plain output)
    #[test]
    fn test_colored_output_without_color() {
        let lines = Box::new(vec!["Line one".to_string(), "Line two".to_string()].into_iter());
        let mut output = Cursor::new(Vec::new());

        colored_output(lines, &mut output, "needle".to_string(), None).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "Line one\nLine two\n");
    }

    // Test for colored_output function with an empty iterator (no lines)
    #[test]
    fn test_colored_output_empty() {
        let lines = Box::new(Vec::<String>::new().into_iter()); // Empty iterator
        let mut output = Cursor::new(Vec::new());

        colored_output(
            lines,
            &mut output,
            "match".to_string(),
            Some(Color::Magenta),
        )
        .unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, ""); // Should be empty output
    }

    // Test for colored_output when the needle is not found in any line
    #[test]
    fn test_colored_output_no_match() {
        let lines = Box::new(
            vec![
                "This is a line".to_string(),
                "This is another line".to_string(),
            ]
            .into_iter(),
        );
        let mut output = Cursor::new(Vec::new());

        colored_output(
            lines,
            &mut output,
            "nonexistent".to_string(),
            Some(Color::Red),
        )
        .unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "This is a line\nThis is another line\n");
    }
}

use anyhow::Result;
use colored::{Color, Colorize};
use std::io::Write;

struct ColoredString {
    string: String,
}

struct PlainString {
    string: String,
}

pub trait WriteOutput {
    fn write_output(&self, writer: &mut dyn Write, color: Option<Color>) -> Result<()>;
}

impl WriteOutput for ColoredString {
    fn write_output(&self, writer: &mut dyn Write, color: Option<Color>) -> Result<()> {
        // If color is provided, then print colored string
        if let Some(c) = color {
            writeln!(writer, "{}", self.string.color(c))?;
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
    color: Option<Color>,
) -> Result<()> {
    // Run line by line and print colored string or plain statement according to the argument
    for line in lines {
        if color.is_some() {
            ColoredString { string: line }.write_output(writer, color)?;
        } else {
            PlainString { string: line }.write_output(writer, None)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // Test for writing colored output when color is provided
    #[test]
    fn test_colored_string_with_color() {
        let mut output = Cursor::new(Vec::new());
        let colored_string = ColoredString {
            string: "Colored text".to_string(),
        };
        colored_string
            .write_output(&mut output, Some(Color::Red))
            .unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert!(written.contains("Colored text"));
    }

    // Test for writing colored output when no color is provided (should fallback to uncolored)
    #[test]
    fn test_colored_string_without_color() {
        let mut output = Cursor::new(Vec::new());
        let colored_string = ColoredString {
            string: "Uncolored fallback".to_string(),
        };
        colored_string.write_output(&mut output, None).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "Uncolored fallback\n");
    }

    // Test for writing plain output with a string, no color should affect this
    #[test]
    fn test_plain_string_output() {
        let mut output = Cursor::new(Vec::new());
        let plain_string = PlainString {
            string: "Plain uncolored string".to_string(),
        };
        plain_string.write_output(&mut output, None).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "Plain uncolored string\n");
    }

    // Test for PlainString ignoring the color argument and always writing uncolored
    #[test]
    fn test_plain_string_ignores_color() {
        let mut output = Cursor::new(Vec::new());
        let plain_string = PlainString {
            string: "Always uncolored".to_string(),
        };
        plain_string
            .write_output(&mut output, Some(Color::Blue))
            .unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "Always uncolored\n");
    }

    // Test for colored_output with lines and a valid color
    #[test]
    fn test_colored_output_with_color() {
        let lines =
            Box::new(vec!["Colored Line 1".to_string(), "Colored Line 2".to_string()].into_iter());
        let mut output = Cursor::new(Vec::new());

        colored_output(lines, &mut output, Some(Color::Green)).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert!(written.contains("Colored Line 1"));
        assert!(written.contains("Colored Line 2"));
    }

    // Test for colored_output without color (should print plain)
    #[test]
    fn test_colored_output_without_color() {
        let lines = Box::new(
            vec![
                "Uncolored Line 1".to_string(),
                "Uncolored Line 2".to_string(),
            ]
            .into_iter(),
        );
        let mut output = Cursor::new(Vec::new());

        colored_output(lines, &mut output, None).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, "Uncolored Line 1\nUncolored Line 2\n");
    }

    // Test for mixed scenario: some lines colored, some uncolored
    #[test]
    fn test_mixed_output_with_some_colored_lines() {
        let lines =
            Box::new(vec!["Colored Line".to_string(), "Uncolored Line".to_string()].into_iter());
        let mut output = Cursor::new(Vec::new());

        // First line with color, second line without color
        colored_output(lines, &mut output, Some(Color::Yellow)).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert!(written.contains("Colored Line"));
        assert!(written.contains("Uncolored Line"));
    }

    // Test for empty iterator input (no lines)
    #[test]
    fn test_empty_lines_input() {
        let lines = Box::new(Vec::<String>::new().into_iter()); // Empty iterator
        let mut output = Cursor::new(Vec::new());

        let result = colored_output(lines, &mut output, Some(Color::Magenta));
        assert!(result.is_ok());

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert_eq!(written, ""); // Output should be empty
    }

    // Test for handling special characters or escape sequences in lines
    #[test]
    fn test_special_characters_in_lines() {
        let lines = Box::new(
            vec![
                "Line with \n new line".to_string(),
                "Tab \t in line".to_string(),
            ]
            .into_iter(),
        );
        let mut output = Cursor::new(Vec::new());

        colored_output(lines, &mut output, None).unwrap();

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert!(written.contains("Line with \n new line"));
        assert!(written.contains("Tab \t in line"));
    }

    // Test for multiple colors across different lines
    #[test]
    fn test_different_colors_per_line() {
        let mut output = Cursor::new(Vec::new());

        // We'll simulate changing colors between the lines manually
        let mut is_first = true;
        for line in vec!["Line 1", "Line 2"] {
            if is_first {
                ColoredString {
                    string: line.to_string(),
                }
                .write_output(&mut output, Some(Color::Blue))
                .unwrap();
                is_first = false;
            } else {
                ColoredString {
                    string: line.to_string(),
                }
                .write_output(&mut output, Some(Color::Red))
                .unwrap();
            }
        }

        let written = String::from_utf8(output.into_inner()).unwrap();
        assert!(written.contains("Line 1"));
        assert!(written.contains("Line 2"));
    }
}

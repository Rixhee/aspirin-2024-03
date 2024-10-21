use crate::filters::FilterResult;
use serde_json::{Number, Value};
use std::io::{self, Write};

const ANSI_ESC: &str = "\x1b[";
const RESET: &str = "\x1b[0m";

trait Printable {
    fn print(
        &self,
        colors: &str,
        compact: &bool,
        sort: &bool,
        indent: &usize,
        current_indent: usize,
        output: &mut dyn Write,
    ) -> io::Result<()>;
}

pub fn print_result(
    result: FilterResult,
    colors: &str,
    sort: &bool,
    indent: &usize,
    compact: &bool,
    output: &mut dyn Write,
) -> io::Result<()> {
    match result {
        FilterResult::SingleValue(value) => {
            value.print(colors, compact, sort, indent, 0, output)?;
        }
        FilterResult::Iterator(iterator) => {
            for item in iterator {
                item.print(colors, compact, sort, indent, 0, output)?;
            }
        }
    }

    writeln!(output)?;
    Ok(())
}

// Implementing Printable trait for serde_json::Value
impl Printable for Value {
    fn print(
        &self,
        colors: &str,
        compact: &bool,
        sort: &bool,
        indent: &usize,
        current_indent: usize,
        output: &mut dyn Write,
    ) -> io::Result<()> {
        match self {
            Value::Object(map) => {
                print_object(map, colors, compact, sort, indent, current_indent, output)?;
            }
            Value::Array(array) => {
                print_array(array, colors, compact, sort, indent, current_indent, output)?;
            }
            Value::Bool(bool) => {
                print_bool(*bool, colors, output)?;
            }
            Value::Number(num) => {
                print_number(num, colors, output)?;
            }
            Value::String(string) => {
                print_string(string, colors, output)?;
            }
            Value::Null => {
                print_null(colors, output)?;
            }
        }
        Ok(())
    }
}

// Function to calculate indentation
fn calculate_indentation(current_indent: usize) -> String {
    " ".repeat(current_indent)
}

// Function to print JSON objects
fn print_object(
    map: &serde_json::Map<String, Value>,
    colors: &str,
    compact: &bool,
    sort: &bool,
    indent: &usize,
    current_indent: usize,
    output: &mut dyn Write,
) -> io::Result<()> {
    let formats: Vec<&str> = colors.split(':').collect();
    writeln!(output, "{}m{{", format_ansi(6, &formats))?;

    let mut keys: Vec<String> = map.keys().cloned().collect();

    if *sort {
        keys.sort();
    }

    for (i, key) in keys.iter().enumerate() {
        if let Some(value) = map.get(key) {
            write!(
                output,
                "{}{}m\"{}\"{}: ",
                calculate_indentation(current_indent + indent),
                format_ansi(7, &formats),
                key,
                RESET
            )?;
            value.print(
                colors,
                compact,
                sort,
                indent,
                current_indent + indent,
                output,
            )?;
            if i < keys.len() - 1 {
                write!(output, "{}m,{}", format_ansi(6, &formats), RESET)?;
            }
            writeln!(output)?;
        }
    }
    write!(
        output,
        "{}{}m}}{}",
        calculate_indentation(current_indent),
        format_ansi(6, &formats),
        RESET
    )?;
    Ok(())
}

// Function to print JSON arrays
fn print_array(
    array: &[Value],
    colors: &str,
    compact: &bool,
    sort: &bool,
    indent: &usize,
    current_indent: usize,
    output: &mut dyn Write,
) -> io::Result<()> {
    let formats: Vec<&str> = colors.split(':').collect();
    writeln!(output, "{}m[{}", format_ansi(5, &formats), RESET)?;

    for (i, elem) in array.iter().enumerate() {
        write!(output, "{}", calculate_indentation(current_indent + indent))?;
        elem.print(
            colors,
            compact,
            sort,
            indent,
            current_indent + indent,
            output,
        )?;
        if i < array.len() - 1 {
            write!(output, "{}m,{}{}", format_ansi(5, &formats), RESET, RESET)?;
        }
        writeln!(output)?;
    }

    write!(
        output,
        "{}{}m]{}",
        calculate_indentation(current_indent),
        format_ansi(5, &formats),
        RESET
    )?;
    Ok(())
}

// Function to print booleans
fn print_bool(bool: bool, colors: &str, output: &mut dyn Write) -> io::Result<()> {
    let index = if bool { 2 } else { 1 };
    write!(
        output,
        "{}m{}{}",
        format_ansi(index, &colors.split(':').collect::<Vec<&str>>()),
        bool,
        RESET
    )?;
    Ok(())
}

// Function to print numbers
fn print_number(num: &Number, colors: &str, output: &mut dyn Write) -> io::Result<()> {
    write!(
        output,
        "{}m{}{}",
        format_ansi(3, &colors.split(':').collect::<Vec<&str>>()),
        num,
        RESET
    )?;
    Ok(())
}

// Function to print strings
fn print_string(string: &String, colors: &str, output: &mut dyn Write) -> io::Result<()> {
    write!(
        output,
        "{}m\"{}\"{}",
        format_ansi(4, &colors.split(':').collect::<Vec<&str>>()),
        string,
        RESET
    )?;
    Ok(())
}

// Function to print null values
fn print_null(colors: &str, output: &mut dyn Write) -> io::Result<()> {
    write!(
        output,
        "{}mnull{}",
        format_ansi(0, &colors.split(':').collect::<Vec<&str>>()),
        RESET
    )?;
    Ok(())
}

// Function for formatting ANSI escape sequences
fn format_ansi(index: usize, formats: &[&str]) -> String {
    let formatting: Vec<&str> = formats[index].split(';').collect();
    let color = formatting[1];
    let format_str = if formatting[0] == "1" {
        format!(";{}", formatting[0])
    } else {
        String::new()
    };
    format!("{}{}{}", ANSI_ESC, color, format_str)
}

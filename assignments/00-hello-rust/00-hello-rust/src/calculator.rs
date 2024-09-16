/// A robust calculator that can perform AND, OR, and XOR operations
/// on numbers in binary, decimal, and hexadecimal formats
use std::io;

#[derive(Debug, PartialEq)]
enum Operators {
    AND,
    OR,
    XOR,
}

/// Gets user input and returns it as a tuple of strings
fn get_input() -> (String, String, String) {
    println!("Please enter the first number: ");
    let mut first_number = String::new();
    io::stdin()
        .read_line(&mut first_number)
        .expect("Failed to read number");

    println!("Please enter the second number: ");
    let mut second_number = String::new();
    io::stdin()
        .read_line(&mut second_number)
        .expect("Failed to read number");

    println!("Please enter the desired operation: ");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read operator");

    return (first_number, second_number, operator);
}

/// Parses a number from a string in binary, decimal, and hexadecimal formats
/// and returns it as a u32
///
/// # Arguments
///
/// * `number`: &str - The number to be parsed
///
/// # Returns
///
/// u32 - The parsed number
fn parse_number(number: &str) -> u32 {
    if number.starts_with("0b") {
        return u32::from_str_radix(&number[2..], 2).unwrap();
    } else if number.starts_with("0x") {
        return u32::from_str_radix(&number[2..], 16).unwrap();
    } else {
        return number.parse::<u32>().unwrap();
    }
}

/// Parses an operator from a string and returns it as an enum
/// or an error message if the operator is not recognized
///
/// # Arguments
///
/// * `operator`: &str - The operator to be parsed
///
/// # Returns
///
/// Result<Operators, String> - The parsed operator
fn parse_operation(operator: &str) -> Result<Operators, String> {
    match operator {
        "&" | "and" | "AND" => Ok(Operators::AND),
        "|" | "or" | "OR" => Ok(Operators::OR),
        "^" | "xor" | "XOR" => Ok(Operators::XOR),
        _ => Err("Could not recognize the operator".to_owned()),
    }
}

/// Performs an operation on two numbers and returns the result
/// as a u32
///
/// # Arguments
///
/// * `first_number`: u32 - The first number
/// * `second_number`: u32 - The second number
/// * `operator`: Operators - The operator to be used
///
/// # Returns
///
/// u32 - The result of the operation
fn calculator(first_number: u32, second_number: u32, operator: Operators) -> u32 {
    match operator {
        Operators::AND => first_number & second_number,
        Operators::OR => first_number | second_number,
        Operators::XOR => first_number ^ second_number,
    }
}

/// Prints the result of an operation
///
/// # Arguments
///
/// * `first_number`: u32 - The first number
/// * `second_number`: u32 - The second number
/// * `operator`: Operators - The operator to be used
/// * `result`: u32 - The result of the operation
///
/// # Returns
///
/// None
fn print_output(first_number: u32, second_number: u32, operator: Operators, result: u32) {
    match operator {
        Operators::AND => println!(
            "The result of {} {} {} is {}",
            first_number, "&", second_number, result
        ),
        Operators::OR => println!(
            "The result of {} {} {} is {}",
            first_number, "|", second_number, result
        ),
        Operators::XOR => println!(
            "The result of {} {} {} is {}",
            first_number, "^", second_number, result
        ),
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use crate::calculator::{calculator, parse_number, parse_operation, Operators};

    #[test_case("0b0101", 5; "binary")]
    #[test_case("1010", 1010 ; "decimal")]
    #[test_case("0x10", 16 ; "hexadecimal")]
    fn test_parse_number(number: &str, expected: u32) {
        assert_eq!(parse_number(number), expected);
    }

    #[test_case("AND", Operators::AND ; "Uppercase AND")]
    #[test_case("and", Operators::AND ; "Lowercase and")]
    #[test_case("&", Operators::AND ; "Ampersand")]
    #[test_case("OR", Operators::OR ; "Uppercase OR")]
    #[test_case("or", Operators::OR ; "OR")]
    #[test_case("|", Operators::OR ; "Pipe")]
    #[test_case("XOR", Operators::XOR ; "Uppercase XOR")]
    #[test_case("xor", Operators::XOR ; "XOR")]
    #[test_case("^", Operators::XOR ; "Caret")]
    fn test_parse_operation(operator: &str, expected: Operators) {
        assert_eq!(parse_operation(operator).unwrap(), expected);
    }

    #[test_case(2, 27, Operators::AND, 2; "AND")]
    #[test_case(248, 58, Operators::OR, 250; "OR")]
    #[test_case(12, 32, Operators::XOR, 44; "XOR")]
    fn test_calculator(first_number: u32, second_number: u32, operator: Operators, result: u32) {
        assert_eq!(calculator(first_number, second_number, operator), result);
    }
}

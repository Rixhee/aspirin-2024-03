use serde_json::Value;
use std::fmt;
use std::num::ParseIntError;
use thiserror::Error;

use crate::functions::{add_function, delete_function, length_function};

#[derive(Error, Debug)]
pub enum MyErrors {
    #[error("Failed to read the provided JSON file: {0}")]
    JSONError(#[from] serde_json::Error),

    #[error("The specified key '{0}' was not found in the JSON data")]
    KeyNotFound(String),

    #[error("Invalid Needle: {0}")]
    InvalidNeedle(String),

    #[error("Dictionary not found")]
    DictionaryNotFound,

    #[error("List not found")]
    ListNotFound,

    #[error("You need an integer")]
    ParseError(#[from] ParseIntError),

    #[error("Index out of bounds")]
    IndexOutOfBounds,

    #[error("Missing brackets")]
    MissingBrackets,

    #[error("Invalid input for this function")]
    InvalidInput,
}

fn object_identifier_filter(input: &Value, needle: &str) -> Result<Value, MyErrors> {
    let key = &needle[1..];
    if let Some(value) = input.get(key) {
        Ok(value.clone())
    } else {
        Err(MyErrors::KeyNotFound(key.to_string()))
    }
}

fn array_index(input: &Value, index_str: &str) -> Result<Value, MyErrors> {
    let index: usize = index_str.parse().map_err(MyErrors::ParseError)?;

    // Check bounds and return the value
    if index < input.as_array().unwrap().len() {
        Ok(input[index].clone())
    } else {
        Err(MyErrors::IndexOutOfBounds)
    }
}

fn array_slice(input: &Value, index_str: &str) -> Result<Value, MyErrors> {
    let indices: Vec<&str> = index_str.split(':').collect();
    let start: usize = indices[0].parse().map_err(MyErrors::ParseError)?;
    let end: usize = indices[1].parse().map_err(MyErrors::ParseError)?;

    if start < end && end <= input.as_array().unwrap().len() {
        let slice = &input.as_array().unwrap()[start..end];
        Ok(Value::Array(slice.to_vec()))
    } else {
        Err(MyErrors::IndexOutOfBounds)
    }
}

fn array_iterator(input: &Value) -> Result<Box<dyn Iterator<Item = Value>>, MyErrors> {
    if let Some(array) = input.as_array() {
        let iter = array.clone().into_iter();
        return Ok(Box::new(iter));
    }

    Err(MyErrors::ListNotFound)
}

pub fn pipe(input: &Value, needle: &str) -> Result<FilterResult, MyErrors> {
    let mut current_input = input.clone(); // Clone to use it mutably
    if needle.contains(" | ") {
        let mut sub_needles = needle.split(" | ");

        while let Some(sub_needle) = sub_needles.next() {
            let filter_result = filter_input(&current_input.clone(), sub_needle)?;
            current_input = match filter_result {
                FilterResult::SingleValue(value) => value,
                FilterResult::Iterator(iter) => {
                    if let Some(next_needle) = sub_needles.next() {
                        let mut filtered_results = Vec::new();

                        for item in iter {
                            match filter_input(&item.clone(), next_needle) {
                                Ok(FilterResult::SingleValue(filtered_value)) => {
                                    filtered_results.push(filtered_value);
                                }
                                Ok(FilterResult::Iterator(nested_iter)) => {
                                    filtered_results.extend(nested_iter);
                                }
                                Err(e) => {
                                    return Err(e);
                                }
                            }
                        }

                        Value::Array(filtered_results)
                    } else {
                        return Ok(FilterResult::Iterator(iter));
                    }
                }
            };
        }

        Ok(FilterResult::SingleValue(current_input))
    } else {
        let filter_result = filter_input(&current_input.clone(), needle)?;
        Ok(filter_result)
    }
}

pub enum FilterResult {
    SingleValue(Value),
    Iterator(Box<dyn Iterator<Item = Value>>),
}

impl PartialEq for FilterResult {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FilterResult::SingleValue(a), FilterResult::SingleValue(b)) => a == b,
            (FilterResult::Iterator(_), FilterResult::Iterator(_)) => true, // Just compare the variants, not the content.
            _ => false,
        }
    }
}

impl fmt::Debug for FilterResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterResult::SingleValue(val) => write!(f, "SingleValue({:?})", val),
            FilterResult::Iterator(_) => write!(f, "Iterator(...)"), // Omit actual iterator content for simplicity
        }
    }
}

fn filter_input(input: &Value, needle: &str) -> Result<FilterResult, MyErrors> {
    let mut mutable_input = input.clone();

    if needle == "." {
        return Ok(FilterResult::SingleValue(mutable_input));
    } else if needle.starts_with(".") && needle.contains("[") && needle.contains("]") {
        if mutable_input.is_array() {
            let start_index = needle.find('[').ok_or(MyErrors::MissingBrackets)?;
            let end_index = needle.find(']').ok_or(MyErrors::MissingBrackets)?;
            let index_str = &needle[start_index + 1..end_index];

            if index_str.is_empty() {
                let array_iter = array_iterator(&mutable_input)?;
                return Ok(FilterResult::Iterator(array_iter));
            } else if index_str.contains(":") {
                let sliced_value = array_slice(&mutable_input, index_str)?;
                return Ok(FilterResult::SingleValue(sliced_value));
            } else {
                let indexed_value = array_index(&mutable_input, index_str)?;
                return Ok(FilterResult::SingleValue(indexed_value));
            }
        } else {
            return Err(MyErrors::ListNotFound);
        }
    } else if needle.starts_with(".") {
        let value = object_identifier_filter(&mutable_input, needle)?;
        return Ok(FilterResult::SingleValue(value));
    } else if !needle.starts_with(".") {
        match needle {
            _ if needle.starts_with("add") => {
                return Ok(FilterResult::SingleValue(add_function(&mutable_input)?));
            }
            _ if needle.starts_with("length") => {
                return Ok(FilterResult::SingleValue(length_function(&mutable_input)?));
            }
            _ if needle.starts_with("del") => {
                return Ok(FilterResult::SingleValue(delete_function(
                    &mut mutable_input,
                    needle,
                )?));
            }
            _ => return Err(MyErrors::InvalidNeedle(needle.to_string())),
        }
    }

    Err(MyErrors::InvalidNeedle(needle.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_object_identifier_filter() {
        let input = json!({"key": "value"});
        let result = object_identifier_filter(&input, ".key").unwrap();
        assert_eq!(result, json!("value"));

        let result = object_identifier_filter(&input, ".missing_key");
        assert!(result.is_err());
    }

    #[test]
    fn test_array_index() {
        let input = json!([10, 20, 30]);
        let result = array_index(&input, "1").unwrap();
        assert_eq!(result, json!(20));

        let result = array_index(&input, "3");
        assert!(result.is_err());
    }

    #[test]
    fn test_array_slice() {
        let input = json!([0, 1, 2, 3, 4, 5]);
        let result = array_slice(&input, "1:4").unwrap();
        assert_eq!(result, json!([1, 2, 3]));
    }

    #[test]
    fn test_array_iterator() {
        let input = json!([1, 2, 3]);
        let result = array_iterator(&input).unwrap();
        let values: Vec<Value> = result.collect();
        assert_eq!(values, vec![json!(1), json!(2), json!(3)]);

        let input = json!({});
        let result = array_iterator(&input);
        assert!(result.is_err());
    }

    #[test]
    fn test_pipe() {
        // Testing with a simple array
        let input = json!([1, 2, 3, 4]);

        // Test passing through input unchanged
        let result = pipe(&input, ".").unwrap();
        assert_eq!(result, FilterResult::SingleValue(json!([1, 2, 3, 4])));

        // Test getting the length of the array
        let result = pipe(&input, "length(.)").unwrap();
        assert_eq!(result, FilterResult::SingleValue(json!(4)));

        // Test deleting an element and then getting the length
        let result = pipe(&input, "del(.[1]) | length(.)").unwrap();
        assert_eq!(result, FilterResult::SingleValue(json!(3)));

        // Test multiple commands in a pipe
        let result = pipe(&input, "del(.[1]) | del(.[0]) | length(.)").unwrap();
        assert_eq!(result, FilterResult::SingleValue(json!(2)));

        // Test piping through an invalid command
        let result = pipe(&input, "invalid_command");
        assert!(result.is_err());

        // Test working with an object
        let obj_input = json!({"a": 1, "b": 2, "c": 3});
        let result = pipe(&obj_input, ".").unwrap();
        assert_eq!(result, FilterResult::SingleValue(obj_input.clone()));
    }

    #[test]
    fn test_filter_input() {
        let input = json!({"a": 1, "b": 2});
        let result = filter_input(&input, ".a").unwrap();
        assert_eq!(result, FilterResult::SingleValue(json!(1)));

        let result = filter_input(&input, ".missing_key");
        assert!(result.is_err());
    }
}

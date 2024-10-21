use serde_json::Number;
use serde_json::Value;

use crate::filters::MyErrors;

// Updated delete_function to accept a reference to Value
pub fn delete_function(input: &mut Value, needle: &str) -> Result<Value, MyErrors> {
    if needle.contains("(") && needle.contains(")") {
        let start_index = needle.find("(").ok_or(MyErrors::MissingBrackets)?;
        let end_index = needle.find(")").ok_or(MyErrors::MissingBrackets)?;
        let sub_needle = &needle[start_index + 1..end_index];

        if sub_needle.starts_with(".") && !sub_needle.contains("[") && !sub_needle.contains("]") {
            let key = &sub_needle[1..];
            if let Some(dict) = input.as_object_mut() {
                if dict.contains_key(key) {
                    dict.remove(key);
                    return Ok(Value::Object(dict.clone()));
                } else {
                    return Err(MyErrors::KeyNotFound(key.to_string()));
                }
            } else {
                return Err(MyErrors::DictionaryNotFound);
            }
        } else if sub_needle.starts_with(".")
            && sub_needle.contains("[")
            && sub_needle.contains("]")
        {
            let indices_start_index = needle.find("[").ok_or(MyErrors::MissingBrackets)?;
            let indices_end_index = needle.find("]").ok_or(MyErrors::MissingBrackets)?;
            let indices: Vec<usize> = needle[indices_start_index + 1..indices_end_index]
                .split(", ")
                .filter_map(|elem| elem.parse::<usize>().ok())
                .collect();

            if let Some(array) = input.as_array_mut() {
                let mut indices_to_remove = indices.clone();
                indices_to_remove.sort_unstable();
                indices_to_remove.reverse();

                for index in indices_to_remove {
                    if index < array.len() {
                        array.remove(index);
                    }
                }

                return Ok(Value::Array(array.clone()));
            } else {
                return Err(MyErrors::ListNotFound);
            }
        }
    } else {
        return Err(MyErrors::MissingBrackets);
    }

    Err(MyErrors::InvalidInput)
}

// Updated length_function to accept a reference to Value
pub fn length_function(input: &Value) -> Result<Value, MyErrors> {
    match input {
        Value::Array(arr) => Ok(Value::Number(Number::from(arr.len()))),
        Value::Number(num) => Ok(Value::Number(Number::from(
            num.as_i64()
                .expect("Cannot find the length of this number")
                .abs(),
        ))),
        Value::String(str) => Ok(Value::Number(Number::from(str.chars().count()))),
        Value::Null => Ok(Value::Number(Number::from(0))),
        Value::Object(dict) => Ok(Value::Number(Number::from(dict.keys().count()))),
        Value::Bool(_) => Err(MyErrors::InvalidInput),
    }
}

// The Add trait and associated structs remain unchanged
trait Add {
    fn add(&self) -> Result<Value, MyErrors>;
}

struct StringArray {
    arr: Vec<String>,
}

struct IntArray {
    arr: Vec<i64>,
}

impl Add for IntArray {
    fn add(&self) -> Result<Value, MyErrors> {
        let sum: i64 = self.arr.iter().sum();
        Ok(Value::Number(Number::from(sum)))
    }
}

impl Add for StringArray {
    fn add(&self) -> Result<Value, MyErrors> {
        Ok(Value::String(self.arr.concat()))
    }
}

// Updated add_function to accept a reference to Value
pub fn add_function(input: &Value) -> Result<Value, MyErrors> {
    if let Some(array) = input.as_array() {
        if array.iter().all(|elem| elem.is_number()) {
            let int_array = IntArray {
                arr: array.iter().filter_map(|elem| elem.as_i64()).collect(),
            };
            return int_array.add();
        } else if array.iter().all(|elem| elem.is_string()) {
            let string_array = StringArray {
                arr: array
                    .iter()
                    .filter_map(|elem| elem.as_str().map(String::from))
                    .collect(),
            };
            return string_array.add();
        }
    }

    Err(MyErrors::InvalidInput)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    // Tests for delete_function
    #[test]
    fn test_delete_key_from_object() {
        let mut json_value = json!({
            "key1": "value1",
            "key2": "value2",
        });

        let result = delete_function(&mut json_value, "del(.key1)").unwrap();
        assert_eq!(
            result,
            json!({
                "key2": "value2",
            })
        );
    }

    #[test]
    fn test_delete_index_from_array() {
        let mut json_value = json!(["item1", "item2", "item3"]);

        let result = delete_function(&mut json_value, "del(.[1])").unwrap();
        assert_eq!(result, json!(["item1", "item3"]));
    }

    #[test]
    fn test_delete_multiple_indices_from_array() {
        let mut json_value = json!(["item1", "item2", "item3", "item4"]);

        let result = delete_function(&mut json_value, "del(.[1, 3])").unwrap();
        assert_eq!(result, json!(["item1", "item3"]));
    }

    #[test]
    fn test_delete_nonexistent_key() {
        let mut json_value = json!({
            "key1": "value1",
        });

        let result = delete_function(&mut json_value, "del(.key2)");
        assert!(result.is_err()); // Should return an error
    }

    #[test]
    fn test_delete_nonexistent_index() {
        let mut json_value = json!({
            "items": ["item1", "item2"],
        });

        let result = delete_function(&mut json_value, "del(.[5])");
        assert!(result.is_err()); // Should return an error
    }

    // Tests for length_function
    #[test]
    fn test_length_of_array() {
        let json_value = json!([1, 2, 3, 4]);
        let result = length_function(&json_value).unwrap();
        assert_eq!(result, json!(4));
    }

    #[test]
    fn test_length_of_string() {
        let json_value = json!("Hello");
        let result = length_function(&json_value).unwrap();
        assert_eq!(result, json!(5));
    }

    #[test]
    fn test_length_of_number() {
        let json_value = json!(-42);
        let result = length_function(&json_value).unwrap();
        assert_eq!(result, json!(42)); // Length of the absolute value of the number
    }

    #[test]
    fn test_length_of_null() {
        let json_value = json!(null);
        let result = length_function(&json_value).unwrap();
        assert_eq!(result, json!(0)); // Length of null
    }

    #[test]
    fn test_length_of_object() {
        let json_value = json!({
            "key1": "value1",
            "key2": "value2",
        });
        let result = length_function(&json_value).unwrap();
        assert_eq!(result, json!(2)); // Number of keys
    }

    #[test]
    fn test_length_of_boolean() {
        let json_value = json!(true);
        let result = length_function(&json_value);
        assert!(result.is_err()); // Length is not defined for boolean
    }

    // Tests for add_function
    #[test]
    fn test_add_integers() {
        let json_value = json!([1, 2, 3, 4]);
        let result = add_function(&json_value).unwrap();
        assert_eq!(result, json!(10)); // 1 + 2 + 3 + 4 = 10
    }

    #[test]
    fn test_add_strings() {
        let json_value = json!(["Hello, ", "world!"]);
        let result = add_function(&json_value).unwrap();
        assert_eq!(result, json!("Hello, world!"));
    }

    #[test]
    fn test_add_mixed_array() {
        let json_value = json!([1, "Hello"]);
        let result = add_function(&json_value);
        assert!(result.is_err()); // Mixed types should return an error
    }

    #[test]
    fn test_add_empty_array() {
        let json_value = json!([]);
        let result = add_function(&json_value).unwrap();
        assert_eq!(result, json!(0)); // Empty array should return 0
    }
}

use std::collections::HashMap;
use std::fs;

#[allow(dead_code)]
fn get_most_common_words(file_path: &str) -> String {
    let content = fs::read_to_string(file_path).expect("Something went wrong reading the file");
    let mut word_map = HashMap::new();
    for word in content.split_whitespace() {
        let _ = *word_map
            .entry(
                word.to_lowercase()
                    .chars()
                    .filter(|c| c.is_alphanumeric())
                    .collect::<String>(),
            )
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut max_count = 0;
    let mut max_count_word = String::from("");
    for (key, value) in word_map {
        if value > max_count {
            max_count = value;
            max_count_word = key.to_string();
        }
    }

    max_count_word
}

#[allow(dead_code)]
fn get_unique_characters(input: String) -> Vec<char> {
    let mut counts = HashMap::new();

    for char in input.chars() {
        let _ = *counts
            .entry(char)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let mut unique_chars = Vec::new();
    for (key, value) in counts {
        if value == 1 {
            unique_chars.push(key);
        }
    }

    unique_chars
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use crate::hashmap::{get_most_common_words, get_unique_characters};

    #[test]
    fn test_get_most_common_words() {
        assert_eq!(get_most_common_words("poems/poem.txt"), String::from("to"));
        assert_eq!(
            get_most_common_words("poems/a_line_storm_song.txt"),
            String::from("the")
        );
        assert_eq!(
            get_most_common_words("poems/dream_variations.txt"),
            String::from("the")
        );
        assert_eq!(
            get_most_common_words("poems/haiku.txt"),
            String::from("candle")
        );
    }

    #[test]
    fn test_get_unique_characters() {
        assert!(check_unordered_vec_are_eq(
            &mut get_unique_characters(String::from("strawberry")),
            &mut vec!['s', 't', 'a', 'w', 'b', 'e', 'y']
        ));
        assert!(check_unordered_vec_are_eq(
            &mut get_unique_characters(String::from("black")),
            &mut vec!['b', 'l', 'a', 'c', 'k']
        ));
        assert!(check_unordered_vec_are_eq(
            &mut get_unique_characters(String::from("white")),
            &mut vec!['w', 'h', 'i', 't', 'e']
        ));
        assert!(check_unordered_vec_are_eq(
            &mut get_unique_characters(String::from("none")),
            &mut vec!['o', 'e']
        ));
    }

    fn check_unordered_vec_are_eq(vec1: &mut Vec<char>, vec2: &mut Vec<char>) -> bool {
        vec1.sort();
        vec2.sort();

        vec1 == vec2
    }
}

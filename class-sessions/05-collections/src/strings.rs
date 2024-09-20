#[allow(dead_code)]
fn merge_strings(arr: Vec<String>) -> String {
    arr.into_iter().fold(String::new(), |acc, s| acc + &s)
}

#[allow(dead_code)]
fn count_letter(word: String, letter: char) -> u8 {
    word.chars().filter(|c| c == &letter).count() as u8
}

#[allow(dead_code)]
fn selectively_capitalize(input: String, idx_to_capitalize: Vec<usize>) -> String {
    let mut result = String::new();
    let mut idx = 0;
    for c in input.chars() {
        if idx_to_capitalize.contains(&idx) {
            result.push_str(&c.to_uppercase().to_string());
        } else {
            result.push_str(&c.to_string());
        }
        idx += 1;
    }
    result
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use crate::strings::{count_letter, merge_strings, selectively_capitalize};

    #[test]
    fn test_merge_strings() {
        assert_eq!(
            merge_strings(vec![
                String::from("hello"),
                String::from(" "),
                String::from("world"),
                String::from("!")
            ]),
            "hello world!"
        );

        assert_eq!(
            merge_strings(vec![
                String::from("writing"),
                String::from(" "),
                String::from("tests"),
                String::from(" is annoy"),
                String::from("ing!!!!"),
            ]),
            "writing tests is annoying!!!!"
        );

        assert_eq!(
            merge_strings(vec![String::from("very annoying...")]),
            "very annoying..."
        );
    }

    #[test]
    fn test_count_letter() {
        assert_eq!(count_letter(String::from("strawberry"), 'r'), 3);
        assert_eq!(count_letter(String::from("blueberry"), 'b'), 2);
        assert_eq!(count_letter(String::from("raspberry"), 'y'), 1);
        assert_eq!(count_letter(String::from("blackberry"), 'z'), 0);
    }

    #[test]
    fn test_selectively_capitalize() {
        assert_eq!(
            selectively_capitalize(String::from("hello"), vec![1, 3]),
            String::from("hElLo")
        );

        assert_eq!(
            selectively_capitalize(String::from("aggressive hello"), (0..16).collect()),
            String::from("AGGRESSIVE HELLO")
        );

        assert_eq!(
            selectively_capitalize(String::from("goodbye"), vec![0, 2, 4, 6]),
            String::from("GoOdByE")
        );

        assert_eq!(
            selectively_capitalize(String::from("silent goodbye"), Vec::new()),
            String::from("silent goodbye")
        );
    }
}

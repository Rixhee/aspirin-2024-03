#[allow(dead_code)]
fn split_string<'a>(string: &'a str, delimeter: &str) -> Vec<&'a str> {
    let mut output = Vec::<&str>::new();
    let mut start_index = 0;
    let mut end_index;

    while start_index < string.len() {
        if let Some(index) = string[start_index..].find(delimeter) {
            end_index = start_index + index;
            output.push(&string[start_index..end_index]);
            start_index = end_index + delimeter.len();
        } else {
            end_index = string.len();
            output.push(&string[start_index..end_index]);
            break;
        }
    }

    output
}

#[derive(PartialEq, Debug)]
struct Differences<'a> {
    only_in_first: Vec<&'a str>,
    only_in_second: Vec<&'a str>,
}

#[allow(dead_code)]
fn find_differences<'a>(first_string: &'a str, second_string: &'a str) -> Differences<'a> {
    let mut output = Differences {
        only_in_first: Vec::<&str>::new(),
        only_in_second: Vec::<&str>::new(),
    };

    for word in first_string.split_whitespace() {
        if !second_string.contains(word) {
            output.only_in_first.push(word);
        }
    }

    for word in second_string.split_ascii_whitespace() {
        if !first_string.contains(word) {
            output.only_in_second.push(word);
        }
    }

    output
}

#[allow(dead_code)]
fn merge_names(first_name: &str, second_name: &str) -> String {
    if first_name.is_empty() && second_name.is_empty() {
        return String::new();
    } else if first_name.is_empty() {
        return second_name.to_string();
    } else if second_name.is_empty() {
        return first_name.to_string();
    }

    let mut output = String::new();
    let mut first_index = 0;
    let mut second_index = 0;
    let first_len = first_name.len();
    let second_len = second_name.len();
    let mut is_first_turn = true;

    while first_index < first_len || second_index < second_len {
        if is_first_turn {
            let starting_index = first_index;
            while first_index < first_len {
                let char = first_name.chars().nth(first_index).unwrap();
                if "aeiou".contains(char) && first_index != starting_index {
                    break;
                }
                output.push(char);
                first_index += 1;
            }
        } else {
            let starting_index = second_index;
            while second_index < second_len {
                let char = second_name.chars().nth(second_index).unwrap();
                if "aeiou".contains(char) && second_index != starting_index {
                    break;
                }
                output.push(char);
                second_index += 1;
            }
        }
        is_first_turn = !is_first_turn;
    }

    output.push_str(&first_name[first_index..]);
    output.push_str(&second_name[second_index..]);

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_string() {
        // First, make sure the lifetimes were correctly marked
        let matches;
        let string_to_split = String::from("Hello, World!");

        {
            let delimeter = String::from(", ");
            matches = split_string(&string_to_split, &delimeter);
        }
        println!("Matches can be printed! See: {:?}", matches);

        // Now check the split logic
        assert_eq!(split_string(&"", &""), Vec::<&str>::new());
        assert_eq!(
            split_string(&"Hello, World!", &", "),
            vec!["Hello", "World!"]
        );
        assert_eq!(
            split_string(
                &"I this think this that this sentence this is this very this confusing this ",
                &" this "
            ),
            vec!["I", "think", "that", "sentence", "is", "very", "confusing"]
        );
        assert_eq!(
            split_string(&"apple🍎banana🍎orange", &"🍎"),
            vec!["apple", "banana", "orange"]
        );
        assert_eq!(
            split_string(
                &"Ayush;put|a,lot~of`random;delimeters|in|this,sentence",
                &";"
            ),
            vec![
                "Ayush",
                "put|a,lot~of`random",
                "delimeters|in|this,sentence"
            ]
        );
    }

    #[test]
    fn test_find_differences() {
        assert_eq!(
            find_differences(&"", &""),
            Differences {
                only_in_first: Vec::new(),
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(&"pineapple pen", &"apple"),
            Differences {
                only_in_first: vec!["pineapple", "pen"],
                only_in_second: Vec::new()
            }
        );
        assert_eq!(
            find_differences(
                &"Sally sold seashells at the seashore",
                &"Seashells seashells at the seashore"
            ),
            Differences {
                only_in_first: vec!["Sally", "sold"],
                only_in_second: vec!["Seashells"]
            }
        );
        assert_eq!(
            find_differences(
                "How much wood could a wood chuck chuck",
                "If a wood chuck could chuck wood"
            ),
            Differences {
                only_in_first: vec!["How", "much"],
                only_in_second: vec!["If"]
            }
        );
        assert_eq!(
            find_differences(
                &"How much ground would a groundhog hog",
                &"If a groundhog could hog ground"
            ),
            Differences {
                only_in_first: vec!["How", "much", "would"],
                only_in_second: vec!["If", "could"]
            }
        );
    }

    #[test]
    fn test_merge_names() {
        assert_eq!(merge_names(&"alex", &"jake"), "aljexake");
        assert_eq!(merge_names(&"steven", &"stephen"), "ststevephenen");
        assert_eq!(merge_names(&"gym", &"rhythm"), "gymrhythm");
        assert_eq!(merge_names(&"walter", &"gibraltor"), "wgaltibreraltor");
        assert_eq!(merge_names(&"baker", &"quaker"), "bqakueraker");
        assert_eq!(merge_names(&"", &""), "");
        assert_eq!(merge_names(&"samesies", &"samesies"), "ssamamesesiieses");
        assert_eq!(merge_names(&"heather", &"meagan"), "hmeeathageran");
        assert_eq!(merge_names(&"panda", &"turtle"), "ptandurtlae");
        assert_eq!(merge_names(&"hot", &"sauce"), "hsotauce");
        assert_eq!(merge_names(&"", &"second"), "second");
        assert_eq!(merge_names(&"first", &""), "first");
    }
}

use std::cmp::Ordering;

#[allow(dead_code)]
fn get_fibonacci(fibonacci_size: usize) -> Vec<u32> {
    let fibonacci: Vec<u32> = (0..fibonacci_size)
        .scan((0, 1), |state: &mut (u32, u32), _| {
            let next = (*state).1;
            *state = ((*state).1, (*state).0 + (*state).1);
            Some(next)
        })
        .collect();
    fibonacci
}

#[allow(dead_code)]
fn binary_search(arr: Vec<u8>, search_val: u8) -> usize {
    let mut arr = arr;
    loop {
        let mid = arr.len() / 2;
        match arr[mid].cmp(&search_val) {
            Ordering::Less => arr = arr[mid + 1..].to_vec(),
            Ordering::Greater => arr = arr[..mid].to_vec(),
            Ordering::Equal => return mid,
        }
    }
}

#[allow(dead_code)]
fn filter_even_numbers(arr: Vec<u8>) -> Vec<u8> {
    arr.into_iter().filter(|x| x % 2 != 0).collect()
}

#[allow(dead_code)]
fn get_longest_increasing_subsequence_len(arr: Vec<u8>) -> u8 {
    let mut max: u8 = 0;
    let mut count: u8 = 1;

    for index in 0..(arr.len() - 1) {
        if arr[index] < arr[index + 1] {
            count += 1;
            println!("{}", count);
        } else {
            if count > max {
                max = count;
                println!("{}", max)
            }
            count = 1
        }
    }

    if count > max {
        count
    } else {
        max
    }
}

// Do not modify below here
#[cfg(test)]
mod tests {
    use crate::vectors::{
        binary_search, filter_even_numbers, get_fibonacci, get_longest_increasing_subsequence_len,
    };

    #[test]
    fn test_get_fibonacci() {
        assert_eq!(get_fibonacci(2), vec![1, 1]);
        assert_eq!(get_fibonacci(3), vec![1, 1, 2]);
        assert_eq!(get_fibonacci(4), vec![1, 1, 2, 3]);
        assert_eq!(get_fibonacci(5), vec![1, 1, 2, 3, 5]);
        assert_eq!(get_fibonacci(6), vec![1, 1, 2, 3, 5, 8]);
        assert_eq!(get_fibonacci(7), vec![1, 1, 2, 3, 5, 8, 13]);
        assert_eq!(get_fibonacci(8), vec![1, 1, 2, 3, 5, 8, 13, 21]);
        assert_eq!(get_fibonacci(9), vec![1, 1, 2, 3, 5, 8, 13, 21, 34]);
        assert_eq!(get_fibonacci(10), vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
    }

    #[test]
    fn test_binary_search() {
        let arr = vec![0, 10, 20, 50, 80, 100, 121, 144, 169, 250, 255];

        assert_eq!(binary_search(arr.clone(), 0), 0);
        assert_eq!(binary_search(arr.clone(), 10), 1);
        assert_eq!(binary_search(arr.clone(), 20), 2);
        assert_eq!(binary_search(arr.clone(), 50), 3);
        assert_eq!(binary_search(arr.clone(), 80), 4);
        assert_eq!(binary_search(arr.clone(), 100), 5);
        assert_eq!(binary_search(arr.clone(), 121), 6);
        assert_eq!(binary_search(arr.clone(), 144), 7);
        assert_eq!(binary_search(arr.clone(), 169), 8);
        assert_eq!(binary_search(arr.clone(), 250), 9);
        assert_eq!(binary_search(arr.clone(), 255), 10);
    }

    #[test]
    fn test_filter_even_numbers() {
        let arr = vec![0, 2, 4, 6, 8, 10];
        assert_eq!(filter_even_numbers(arr), Vec::new());

        let arr = vec![1, 3, 5, 7, 9, 11];
        assert_eq!(filter_even_numbers(arr.clone()), arr);

        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(filter_even_numbers(arr.clone()), vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_get_longest_increasing_subsequence_len() {
        let arr = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(get_longest_increasing_subsequence_len(arr), 10);

        let arr = vec![1, 2, 3, 2, 1, 2, 3, 4, 5];
        assert_eq!(get_longest_increasing_subsequence_len(arr), 5);

        let arr = vec![1, 0, 1, 0, 1, 0, 1, 0];
        assert_eq!(get_longest_increasing_subsequence_len(arr), 2);

        let arr = vec![0; 10];
        assert_eq!(get_longest_increasing_subsequence_len(arr), 1);
    }
}

use anyhow::Result;
use rand::Rng;

mod error;
mod thread_pool;
use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn random_vec(capacity: usize) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    rand::thread_rng().fill(&mut vec[..]);
    vec
}

fn main() -> Result<()> {
    let num_elem = 10_000_000;
    let data = random_vec(num_elem);

    let num_threads = 2;

    let start = std::time::Instant::now();
    let _output = concurrent_merge_sort(&data, num_threads);
    let end = std::time::Instant::now();

    println!("Time taken: {} ms", end.duration_since(start).as_millis());
    // println!("Output: {:?}", _output);
    assert!(num_elem == _output.len());
    Ok(())
}

fn concurrent_merge_sort(data: &[i64], num_threads: usize) -> Vec<i64> {
    let chunk_size = data.len() / num_threads;

    let mut pool = thread_pool::ThreadPool::<Vec<i64>>::new(num_threads).unwrap();
    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            data.len()
        } else {
            start + chunk_size
        };

        let mut chunk = data[start..end].to_vec();
        let _ = pool.execute(move || sort(&mut chunk));
    }

    pool.close();
    let result = pool.get_results();

    merge(&result)
}

fn sort(data: &mut [i64]) -> Vec<i64> {
    if data.len() <= 1 {
        return data.to_vec();
    }

    let mid = data.len() / 2;
    let (left, right) = data.split_at_mut(mid);

    let left_sorted = sort(&mut left.to_vec());
    let right_sorted = sort(&mut right.to_vec());

    let mut merged = Vec::with_capacity(data.len());
    let mut i = 0;
    let mut j = 0;

    while i < left_sorted.len() && j < right_sorted.len() {
        if left_sorted[i] <= right_sorted[j] {
            merged.push(left_sorted[i]);
            i += 1;
        } else {
            merged.push(right_sorted[j]);
            j += 1;
        }
    }

    while i < left_sorted.len() {
        merged.push(left_sorted[i]);
        i += 1;
    }

    while j < right_sorted.len() {
        merged.push(right_sorted[j]);
        j += 1;
    }

    merged
}

fn merge(data: &[Vec<i64>]) -> Vec<i64> {
    let mut result = Vec::new();
    let mut heap = BinaryHeap::new();

    for (list_index, list) in data.iter().enumerate() {
        if !list.is_empty() {
            heap.push(Reverse((list[0], list_index, 0)));
        }
    }

    while let Some(Reverse((value, list_index, elem_index))) = heap.pop() {
        result.push(value);

        if let Some(&next_value) = data[list_index].get(elem_index + 1) {
            heap.push(Reverse((next_value, list_index, elem_index + 1)));
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_vec() {
        let capacity = 10;
        let vec = random_vec(capacity);
        assert_eq!(vec.len(), capacity);
    }

    #[test]
    fn test_sort_single_chunk() {
        let mut data = vec![3, 1, 2, 5, 4];
        let sorted_data = sort(&mut data);
        assert_eq!(sorted_data, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_sort_empty() {
        let mut data: Vec<i64> = vec![];
        let sorted_data = sort(&mut data);
        assert_eq!(sorted_data, vec![]);
    }

    #[test]
    fn test_merge_multiple_sorted_chunks() {
        let chunks = vec![vec![1, 3, 5], vec![2, 4, 6], vec![0, 7, 8]];
        let merged = merge(&chunks);
        assert_eq!(merged, vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn test_merge_empty_chunks() {
        let chunks: Vec<Vec<i64>> = vec![];
        let merged = merge(&chunks);
        assert_eq!(merged, vec![]);
    }

    #[test]
    fn test_concurrent_merge_sort_small_data() {
        let data = vec![4, 2, 7, 1, 5, 3, 6];
        let sorted_data = concurrent_merge_sort(&data, 3);
        assert_eq!(sorted_data, vec![1, 2, 3, 4, 5, 6, 7]);
    }

    #[test]
    fn test_concurrent_merge_sort_large_data() {
        let data = random_vec(1000);
        let mut sorted_data = data.clone();
        sorted_data.sort();

        let concurrent_sorted_data = concurrent_merge_sort(&data, 10);
        assert_eq!(concurrent_sorted_data, sorted_data);
    }

    #[test]
    fn test_concurrent_merge_sort_edge_case_single_thread() {
        let data = vec![9, 3, 7, 1, 8];
        let mut sorted_data = data.clone();
        sorted_data.sort();

        let concurrent_sorted_data = concurrent_merge_sort(&data, 1);
        assert_eq!(concurrent_sorted_data, sorted_data);
    }

    #[test]
    fn test_concurrent_merge_sort_edge_case_empty_data() {
        let data: Vec<i64> = vec![];
        let concurrent_sorted_data = concurrent_merge_sort(&data, 4);
        assert_eq!(concurrent_sorted_data, vec![]);
    }

    #[test]
    fn test_concurrent_merge_sort_single_element() {
        let data = vec![42];
        let concurrent_sorted_data = concurrent_merge_sort(&data, 4);
        assert_eq!(concurrent_sorted_data, vec![42]);
    }

    #[test]
    fn test_concurrent_merge_sort_all_equal_elements() {
        let data = vec![5; 20];
        let concurrent_sorted_data = concurrent_merge_sort(&data, 4);
        assert_eq!(concurrent_sorted_data, vec![5; 20]);
    }
}

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::error::ThreadPoolError;

#[derive(Debug)]
pub struct ThreadPool<T>
where
    T: Send + 'static,
{
    workers: Vec<thread::JoinHandle<()>>,
    job_sender: Option<mpsc::Sender<Job<T>>>,
    result_receiver: Arc<Mutex<mpsc::Receiver<T>>>,
}

type Job<T> = Box<dyn FnOnce() -> T + Send + 'static>;

impl<T> ThreadPool<T>
where
    T: Send + 'static,
{
    /// Create a new ThreadPool with specified number of threads
    ///
    /// # Arguments
    /// * `num_threads` - Number of threads to create
    ///
    /// # Returns
    /// * `Result<ThreadPool<T>, ThreadPoolError>` - New thread pool or error if num_threads is 0
    pub fn new(num_threads: usize) -> Result<ThreadPool<T>, ThreadPoolError> {
        if num_threads == 0 {
            return Err(ThreadPoolError::ZeroThreads);
        }

        let (job_sender, job_receiver) = mpsc::channel::<Job<T>>();
        let (result_sender, result_receiver) = mpsc::channel::<T>();
        let job_receiver = Arc::new(Mutex::new(job_receiver));
        let result_receiver = Arc::new(Mutex::new(result_receiver));
        let mut workers = Vec::with_capacity(num_threads);

        for _ in 0..num_threads {
            let job_receiver = Arc::clone(&job_receiver);
            let result_sender = result_sender.clone();

            let worker = thread::spawn(move || loop {
                let message = job_receiver.lock().unwrap().recv();
                match message {
                    Ok(job) => {
                        let result = job();
                        if result_sender.send(result).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            });

            workers.push(worker);
        }

        Ok(ThreadPool {
            workers,
            job_sender: Some(job_sender),
            result_receiver,
        })
    }

    /// Execute a task in the thread pool
    ///
    /// # Arguments
    /// * `f` - Function to execute
    ///
    /// # Returns
    /// * `Result<(), ThreadPoolError>` - Success or error if sending fails
    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() -> T + Send + 'static,
    {
        let job = Box::new(f);
        if let Some(sender) = self.job_sender.as_ref() {
            match sender.send(job) {
                Ok(()) => Ok(()),
                Err(_) => Err(ThreadPoolError::Send),
            }
        } else {
            Err(ThreadPoolError::Send)
        }
    }

    pub fn close(&mut self) {
        self.job_sender.take();

        // Wait for each worker to finish.
        for worker in self.workers.drain(..) {
            worker.join().expect("Worker thread panicked");
        }
    }
    /// Get any available results from completed tasks
    ///
    /// # Returns
    /// * `Vec<T>` - Vector of results from completed tasks
    pub fn get_results(&self) -> Vec<T> {
        let mut results = Vec::new();

        let receiver = self.result_receiver.lock().unwrap();

        results.push(receiver.recv().unwrap());

        while let Ok(result) = receiver.try_recv() {
            results.push(result);
        }
        results
    }
}

impl<T> Drop for ThreadPool<T>
where
    T: Send + 'static,
{
    fn drop(&mut self) {
        drop(self.job_sender.clone());
        for worker in self.workers.drain(..) {
            worker.join().unwrap();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_thread_pool_execute_simple_task() {
        let mut pool = ThreadPool::<i32>::new(2).unwrap();

        pool.execute(|| 5 + 3).unwrap();
        pool.close();

        let results = pool.get_results();
        assert_eq!(results, vec![8]);
    }

    #[test]
    fn test_thread_pool_multiple_tasks() {
        let mut pool = ThreadPool::<i32>::new(4).unwrap();

        for i in 0..5 {
            pool.execute(move || i * 2).unwrap();
        }

        pool.close();
        let mut results = pool.get_results();
        results.sort(); // Sorting to ensure consistency
        assert_eq!(results, vec![0, 2, 4, 6, 8]);
    }

    #[test]
    fn test_thread_pool_task_order() {
        let mut pool = ThreadPool::<i32>::new(3).unwrap();

        pool.execute(|| 1).unwrap();
        pool.execute(|| 2).unwrap();
        pool.execute(|| 3).unwrap();

        pool.close();
        let results = pool.get_results();

        // We do not guarantee order since threads execute concurrently, so just check values
        assert_eq!(results.len(), 3);
        assert!(results.contains(&1));
        assert!(results.contains(&2));
        assert!(results.contains(&3));
    }

    #[test]
    fn test_thread_pool_parallel_execution() {
        let mut pool = ThreadPool::<usize>::new(4).unwrap();
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut count = counter.lock().unwrap();
                *count += 1;
                *count
            })
            .unwrap();
        }

        pool.close();
        let results = pool.get_results();
        assert_eq!(results.len(), 10);

        // Check if the counter has been incremented correctly in parallel
        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 10);
    }

    #[test]
    fn test_thread_pool_task_failure_handling() {
        let mut pool = ThreadPool::<Result<(), &str>>::new(2).unwrap();

        // Simulate a task that fails and returns an error
        pool.execute(|| Err("Task failed")).unwrap();
        pool.execute(|| Ok(())).unwrap();

        pool.close();
        let results: Vec<_> = pool.get_results();

        // Ensure that both the success and error are captured in results
        assert_eq!(results.len(), 2);
        assert!(results.contains(&Err("Task failed")));
        assert!(results.contains(&Ok(())));
    }

    #[test]
    fn test_thread_pool_delayed_tasks() {
        let mut pool = ThreadPool::<usize>::new(2).unwrap();

        pool.execute(|| {
            thread::sleep(Duration::from_millis(50));
            1
        })
        .unwrap();

        pool.execute(|| {
            thread::sleep(Duration::from_millis(100));
            2
        })
        .unwrap();

        pool.close();
        let results = pool.get_results();

        assert!(results.contains(&1));
        assert!(results.contains(&2));
    }

    #[test]
    fn test_thread_pool_get_results_before_close() {
        let mut pool = ThreadPool::<i32>::new(4).unwrap();

        pool.execute(|| 10).unwrap();
        pool.execute(|| 20).unwrap();

        // Retrieve results before closing the pool, should return empty or partial results
        let results = pool.get_results();
        assert!(
            results.is_empty() || results.len() <= 2,
            "Expected no or partial results before close"
        );

        pool.close();
    }

    #[test]
    fn test_thread_pool_double_close() {
        let mut pool = ThreadPool::<i32>::new(4).unwrap();
        pool.close();

        // Attempting to close again should have no effect or cause error
        pool.close();
    }
}

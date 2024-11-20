Number of threads with runtimes - [100: 6400ms, 64: 6089ms. 32: 5723ms,  16: 5361ms, 8: 4997ms, 4: 5677ms, 2: 7534ms]. From this data, 8 threads yield the fastest runtime of 4997 ms.

Increasing Threads (More than 8):

Pros: Adding threads generally improves parallelism, which can decrease runtime by distributing workloads across multiple CPU cores. It also provides redundancy, as each thread can handle failures or delays without impacting the entire operation.

Cons: Beyond a certain point, adding threads can lead to overhead from thread management, especially if the number of threads exceeds the number of available CPU cores. Excessive threads can cause contention for resources (such as cache or memory bandwidth) and degrade performance due to context-switching overhead.

Decreasing Threads (Less than 8):

Pros: Using fewer threads reduces overhead, which can be beneficial if there are limited CPU cores or if the dataset is small, minimizing thread management time. Lower thread counts are generally more efficient for memory access, as fewer threads mean less contention for shared memory.

Cons: Using too few threads fails to leverage the CPU’s full processing potential, leading to underutilization of cores, especially with large datasets. This can result in longer runtimes as the workload isn’t distributed effectively.
## Performance comparison between threads and async (Tokio) tasks

This crate attempts to validate the performance arguments for using tokio vs threads for executing a large number of i/o tasks.

### Setup
There are 50 threads (or tasks) trying to increment the numbers in 100 files. The files are created in the beginning with number 0. The execution stops when all 100 files have reached the number 10000. 

Each thread (or task) will take a lock on the file before attempting to modify it.

We will vary the number of threads and file to see the different behaviours

### Results




# Parallel Reduce Operation

## Solution

```rust
/// Basic ThreadPool stub for parallel_reduce implementation.
pub struct ThreadPool {
    worker_count: usize,
}

impl ThreadPool {
    pub fn new(size: usize) -> Self {
        assert!(size > 0, "thread pool size must be greater than zero");
        Self { worker_count: size }
    }
}

/// Apply an associative reduction function in parallel.
pub fn parallel_reduce<T, F>(
    pool: &ThreadPool,
    items: Vec<T>,
    identity: T,
    reduce_fn: F,
) -> T
where
    T: Send + Clone + 'static,
    F: Fn(T, T) -> T + Send + Copy + 'static,
{
    if items.is_empty() {
        return identity;
    }

    if items.len() == 1 {
        return items.into_iter().next().expect("items contains one value");
    }

    let worker_count = pool.worker_count.min(items.len());
    let chunk_size = items.len().div_ceil(worker_count);

    let handles: Vec<_> = items
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.to_vec();
            std::thread::spawn(move || {
                chunk
                    .into_iter()
                    .reduce(reduce_fn)
                    .expect("chunks are non-empty")
            })
        })
        .collect();

    handles
        .into_iter()
        .map(|handle| {
            handle
                .join()
                .expect("parallel reduction worker panicked")
        })
        .reduce(reduce_fn)
        .unwrap_or(identity)
}
```

## Explanation

The input is divided into at most `worker_count` chunks. Each spawned worker
reduces one non-empty chunk and returns its partial result through its
`JoinHandle`. The calling thread joins the handles in input order and combines
the partial results.

Returning values from the handles avoids an additional channel and preserves
the order of the chunks. A worker panic is propagated to the caller instead of
being replaced with the identity value, which prevents silent data loss.

The identity is returned for an empty collection. For a non-empty collection,
it is not inserted into every partial reduction. The reduction function must be
associative because chunking changes how operations are grouped. It does not
need to be commutative because the original element order is preserved.

The `ThreadPool` remains a sizing stub: it limits the number of chunks, while
this exercise still creates threads directly with `std::thread::spawn`.

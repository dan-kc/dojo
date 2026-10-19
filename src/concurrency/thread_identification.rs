// cargo test --lib concurrency::thread_identification::

//! Name workers worker-0, worker-1, ... and obtain each actual name and debug-formatted thread ID inside that worker. Return in creation order and join every worker. Zero count returns empty output; spawn failure may panic.

/// Create a function that demonstrates proper thread naming and identification.
/// Spawn `count` threads, each with a unique name, and collect their thread IDs.
/// Return a vector of thread names and IDs as tuples.
fn thread_identification(count: usize) -> Vec<(String, String)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_thread_identification() {
        let results = thread_identification(3);
        assert_eq!(results.len(), 3);

        // Verify all thread names are unique
        let names: HashSet<_> = results.iter().map(|(name, _)| name).collect();
        assert_eq!(names.len(), 3);

        // Verify all thread IDs are present
        for (name, id) in results {
            assert!(!name.is_empty());
            assert!(!id.is_empty());
        }
    }
}

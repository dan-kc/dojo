// cargo test memoizer

/// Implement HashMap-based memoization for expensive function calls.
/// Use interior mutability for caching within an immutable context.
#[allow(dead_code)]
pub struct Memoizer<K, V> {
    cache: std::cell::RefCell<std::collections::HashMap<K, V>>,
}

impl<K, V> Memoizer<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    pub fn new() -> Self {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn compute<F>(&self, key: K, compute_fn: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        todo!()
    }

    pub fn clear_cache(&self) {
        todo!()
    }

    pub fn cache_size(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memoizer() {
        let memoizer = Memoizer::new();
        let call_count = std::cell::RefCell::new(0);

        let expensive_fn = |x: &i32| {
            *call_count.borrow_mut() += 1;
            x * x
        };

        // First call should compute
        let result1 = memoizer.compute(5, expensive_fn);
        assert_eq!(result1, 25);
        assert_eq!(*call_count.borrow(), 1);

        // Second call should use cache
        let result2 = memoizer.compute(5, expensive_fn);
        assert_eq!(result2, 25);
        assert_eq!(*call_count.borrow(), 1); // No additional call

        // Different key should compute
        let result3 = memoizer.compute(3, expensive_fn);
        assert_eq!(result3, 9);
        assert_eq!(*call_count.borrow(), 2);

        assert_eq!(memoizer.cache_size(), 2);

        memoizer.clear_cache();
        assert_eq!(memoizer.cache_size(), 0);
    }

    #[test]
    fn test_memoizer_string_keys() {
        let memoizer = Memoizer::new();

        let compute_length = |s: &String| s.len();

        let result1 = memoizer.compute("hello".to_string(), compute_length);
        assert_eq!(result1, 5);

        let result2 = memoizer.compute("world".to_string(), compute_length);
        assert_eq!(result2, 5);

        let result3 = memoizer.compute("hello".to_string(), compute_length);
        assert_eq!(result3, 5); // Should be cached

        assert_eq!(memoizer.cache_size(), 2);
    }

    #[test]
    fn test_memoizer_clear_cache() {
        let memoizer = Memoizer::new();

        memoizer.compute(1, |x| x * 2);
        memoizer.compute(2, |x| x * 2);
        memoizer.compute(3, |x| x * 2);

        assert_eq!(memoizer.cache_size(), 3);

        memoizer.clear_cache();
        assert_eq!(memoizer.cache_size(), 0);
    }

    #[test]
    fn test_memoizer_different_types() {
        let memoizer: Memoizer<(i32, i32), i32> = Memoizer::new();

        let add_fn = |(a, b): &(i32, i32)| a + b;

        let result1 = memoizer.compute((3, 4), add_fn);
        assert_eq!(result1, 7);

        let result2 = memoizer.compute((1, 2), add_fn);
        assert_eq!(result2, 3);

        let result3 = memoizer.compute((3, 4), add_fn); // Should be cached
        assert_eq!(result3, 7);

        assert_eq!(memoizer.cache_size(), 2);
    }
}

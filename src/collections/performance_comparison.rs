// Performance Comparison Practice
//
// Learning objectives:
// - Measuring and comparing collection performance
// - Understanding HashMap vs Vec lookup characteristics
// - Using std::time for benchmarking
//
// Run with: cargo test performance_comparison

/// Performance comparison function that measures HashMap vs Vec lookup times.
/// Use for educational purposes to understand when to choose each collection.
pub fn performance_comparison(data_size: usize) -> (std::time::Duration, std::time::Duration) {
    todo!("Implement HashMap vs Vec performance comparison")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_comparison() {
        let (hashmap_time, vec_time) = performance_comparison(1000);
        
        // Both should complete in reasonable time
        assert!(hashmap_time < std::time::Duration::from_secs(1));
        assert!(vec_time < std::time::Duration::from_secs(1));
        
        // For large datasets and random access, HashMap should generally be faster
        // But this test is more for educational purposes
        println!("HashMap lookup time: {:?}", hashmap_time);
        println!("Vec lookup time: {:?}", vec_time);
    }

    #[test]
    fn test_performance_small_dataset() {
        let (hashmap_time, vec_time) = performance_comparison(10);
        
        // Even small datasets should complete quickly
        assert!(hashmap_time < std::time::Duration::from_millis(100));
        assert!(vec_time < std::time::Duration::from_millis(100));
    }

    #[test]
    fn test_performance_large_dataset() {
        let (hashmap_time, vec_time) = performance_comparison(10000);
        
        // Large datasets should still complete in reasonable time
        assert!(hashmap_time < std::time::Duration::from_secs(5));
        assert!(vec_time < std::time::Duration::from_secs(5));
        
        // Print results for educational insight
        println!("Large dataset - HashMap: {:?}, Vec: {:?}", hashmap_time, vec_time);
    }

    #[test]
    fn test_performance_zero_size() {
        let (hashmap_time, vec_time) = performance_comparison(0);
        
        // Zero-size should be very fast
        assert!(hashmap_time < std::time::Duration::from_millis(1));
        assert!(vec_time < std::time::Duration::from_millis(1));
    }

    #[test]
    fn test_performance_consistency() {
        // Run the same test multiple times to check consistency
        let mut hashmap_times = Vec::new();
        let mut vec_times = Vec::new();
        
        for _ in 0..3 {
            let (ht, vt) = performance_comparison(1000);
            hashmap_times.push(ht);
            vec_times.push(vt);
        }
        
        // All measurements should be reasonable
        for &time in &hashmap_times {
            assert!(time < std::time::Duration::from_secs(1));
        }
        for &time in &vec_times {
            assert!(time < std::time::Duration::from_secs(1));
        }
    }
}
// BTree vs HashMap Performance Comparison Practice
//
// Learning Objectives:
// - Compare performance characteristics of BTreeMap vs HashMap
// - Measure insertion, lookup, and iteration performance
// - Understand when to choose ordered vs unordered collections
// - Practice timing measurements with std::time
//
// Run with: cargo test --bin btree_performance_comparison

/// Compare performance between BTreeMap and HashMap for different operations.
/// Return timing results for insertion, lookup, and iteration.
fn performance_comparison(
    size: usize,
) -> (
    std::time::Duration, // BTreeMap insert
    std::time::Duration, // HashMap insert  
    std::time::Duration, // BTreeMap lookup
    std::time::Duration, // HashMap lookup
    std::time::Duration, // BTreeMap ordered iteration
    std::time::Duration, // HashMap unordered iteration
) {
    todo!("Implement performance comparison between BTreeMap and HashMap")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_comparison() {
        let (btree_insert, hash_insert, btree_lookup, hash_lookup, btree_iter, hash_iter) 
            = performance_comparison(1000);
        
        // All operations should complete reasonably quickly
        assert!(btree_insert < std::time::Duration::from_secs(1));
        assert!(hash_insert < std::time::Duration::from_secs(1));
        assert!(btree_lookup < std::time::Duration::from_secs(1));
        assert!(hash_lookup < std::time::Duration::from_secs(1));
        assert!(btree_iter < std::time::Duration::from_secs(1));
        assert!(hash_iter < std::time::Duration::from_secs(1));
        
        println!("BTreeMap insert: {:?}", btree_insert);
        println!("HashMap insert: {:?}", hash_insert);
        println!("BTreeMap lookup: {:?}", btree_lookup);
        println!("HashMap lookup: {:?}", hash_lookup);
        println!("BTreeMap iteration: {:?}", btree_iter);
        println!("HashMap iteration: {:?}", hash_iter);
    }
}
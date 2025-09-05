// BTree Simple Database Index Practice
//
// Learning Objectives:
// - Use BTreeMap to implement database-like indexing
// - Support exact matches, range queries, and prefix searches
// - Practice nested BTree collections (BTreeMap<String, BTreeSet<u32>>)
// - Handle insertion, deletion, and query operations efficiently
//
// Run with: cargo test --bin btree_simple_index

/// Use BTreeMap to implement a simple database index.
/// Support range queries, prefix searches, and ordered iteration.
struct SimpleIndex {
    index: std::collections::BTreeMap<String, std::collections::BTreeSet<u32>>, // key -> row_ids
}

impl SimpleIndex {
    fn new() -> Self {
        todo!("Implement new index")
    }

    fn insert(&mut self, key: String, row_id: u32) {
        todo!("Insert key-row_id pair")
    }

    fn find_exact(&self, key: &str) -> Vec<u32> {
        todo!("Find exact key matches")
    }

    fn find_range(&self, start: &str, end: &str) -> Vec<u32> {
        todo!("Find all row_ids for keys in range")
    }

    fn find_prefix(&self, prefix: &str) -> Vec<u32> {
        todo!("Find all row_ids for keys with given prefix")
    }

    fn remove(&mut self, key: &str, row_id: u32) {
        todo!("Remove specific key-row_id pair")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_index() {
        let mut index = SimpleIndex::new();
        
        index.insert("apple".to_string(), 1);
        index.insert("application".to_string(), 2);
        index.insert("apply".to_string(), 3);
        index.insert("banana".to_string(), 4);
        index.insert("band".to_string(), 5);
        
        // Test exact match
        let apple_rows = index.find_exact("apple");
        assert_eq!(apple_rows, vec![1]);
        
        // Test range query
        let app_range = index.find_range("app", "apq");
        assert!(app_range.contains(&1)); // apple
        assert!(app_range.contains(&2)); // application
        assert!(app_range.contains(&3)); // apply
        assert!(!app_range.contains(&4)); // banana not in range
        
        // Test prefix search
        let app_prefix = index.find_prefix("app");
        assert!(app_prefix.contains(&1)); // apple
        assert!(app_prefix.contains(&2)); // application
        assert!(app_prefix.contains(&3)); // apply
        
        let ban_prefix = index.find_prefix("ban");
        assert!(ban_prefix.contains(&4)); // banana
        assert!(ban_prefix.contains(&5)); // band
        
        // Test removal
        index.remove("apple", 1);
        let apple_after_remove = index.find_exact("apple");
        assert!(apple_after_remove.is_empty());
    }
}
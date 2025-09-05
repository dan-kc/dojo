// Compound Key Operations Practice
//
// Learning objectives:
// - Working with complex key types in HashMap
// - Implementing Hash and Eq for custom types
// - Using struct fields in hash keys
//
// Run with: cargo test compound_key_operations

/// Custom key type that demonstrates HashMap usage with complex keys.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompoundKey {
    category: String,
    id: u32,
    flags: Vec<bool>,
}

impl CompoundKey {
    pub fn new(category: String, id: u32, flags: Vec<bool>) -> Self {
        Self { category, id, flags }
    }
}

/// Create and manipulate a HashMap with compound keys.
/// Demonstrate grouping and querying with complex key structures.
pub fn compound_key_operations() -> std::collections::HashMap<CompoundKey, String> {
    todo!("Implement operations with compound keys")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_compound_key_operations() {
        let map = compound_key_operations();
        
        // Test should create map with various compound keys
        assert!(!map.is_empty());
        
        // Test that we can query with compound keys
        let key1 = CompoundKey::new("category1".to_string(), 1, vec![true, false]);
        let key2 = CompoundKey::new("category1".to_string(), 2, vec![false, true]);
        
        // At least some keys should exist in the test data
        assert!(map.contains_key(&key1) || map.contains_key(&key2) || map.len() > 0);
    }

    #[test]
    fn test_compound_key_equality() {
        let key1 = CompoundKey::new("test".to_string(), 1, vec![true, false]);
        let key2 = CompoundKey::new("test".to_string(), 1, vec![true, false]);
        let key3 = CompoundKey::new("test".to_string(), 1, vec![false, true]);
        
        assert_eq!(key1, key2);
        assert_ne!(key1, key3);
        
        let mut map = HashMap::new();
        map.insert(key1, "value1".to_string());
        
        assert_eq!(map.get(&key2), Some(&"value1".to_string()));
        assert_eq!(map.get(&key3), None);
    }

    #[test]
    fn test_compound_key_different_categories() {
        let key1 = CompoundKey::new("category_a".to_string(), 1, vec![true]);
        let key2 = CompoundKey::new("category_b".to_string(), 1, vec![true]);
        
        assert_ne!(key1, key2);
        
        let mut map = HashMap::new();
        map.insert(key1, "value_a".to_string());
        map.insert(key2, "value_b".to_string());
        
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_compound_key_different_ids() {
        let key1 = CompoundKey::new("same_category".to_string(), 1, vec![true]);
        let key2 = CompoundKey::new("same_category".to_string(), 2, vec![true]);
        
        assert_ne!(key1, key2);
        
        let mut map = HashMap::new();
        map.insert(key1, "value1".to_string());
        map.insert(key2, "value2".to_string());
        
        assert_eq!(map.len(), 2);
    }

    #[test]
    fn test_compound_key_different_flags() {
        let key1 = CompoundKey::new("category".to_string(), 1, vec![true, false, true]);
        let key2 = CompoundKey::new("category".to_string(), 1, vec![true, false, false]);
        
        assert_ne!(key1, key2);
        
        let mut map = HashMap::new();
        map.insert(key1, "flags1".to_string());
        map.insert(key2, "flags2".to_string());
        
        assert_eq!(map.len(), 2);
    }
}
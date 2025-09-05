// BTree Custom Ordering Practice
//
// Learning Objectives:
// - Implement custom ordering for BTreeSet elements
// - Use wrapper types with custom Ord implementations
// - Practice length-first then lexicographic ordering
// - Understand how custom ordering affects BTreeSet behavior
//
// Run with: cargo test --bin btree_custom_ordering

/// Implement a custom ordering for BTreeSet using a wrapper type.
/// Sort strings by length first, then lexicographically.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LengthFirstString(String);

impl Ord for LengthFirstString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!("Implement length-first ordering")
    }
}

impl PartialOrd for LengthFirstString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Create and demonstrate BTreeSet with custom ordering.
fn custom_ordered_set() -> std::collections::BTreeSet<LengthFirstString> {
    todo!("Create BTreeSet with length-first string ordering")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_custom_ordered_set() {
        let set = custom_ordered_set();
        assert!(!set.is_empty());
        
        // Convert to vector to test ordering
        let ordered: Vec<_> = set.iter().cloned().collect();
        
        // Verify that strings are ordered by length first, then lexicographically
        for i in 1..ordered.len() {
            let prev = &ordered[i-1];
            let curr = &ordered[i];
            
            if prev.0.len() == curr.0.len() {
                // Same length, should be lexicographically ordered
                assert!(prev.0 <= curr.0);
            } else {
                // Different lengths, shorter should come first
                assert!(prev.0.len() < curr.0.len());
            }
        }
    }

    #[test]
    fn test_length_first_string_ordering() {
        let mut set = BTreeSet::new();
        
        set.insert(LengthFirstString("zoo".to_string()));      // length 3
        set.insert(LengthFirstString("a".to_string()));        // length 1
        set.insert(LengthFirstString("apple".to_string()));    // length 5
        set.insert(LengthFirstString("an".to_string()));       // length 2
        set.insert(LengthFirstString("at".to_string()));       // length 2
        
        let ordered: Vec<_> = set.iter().map(|s| &s.0).collect();
        
        // Should be: "a" (len 1), "an", "at" (len 2, alphabetical), "zoo" (len 3), "apple" (len 5)
        assert_eq!(ordered, vec!["a", "an", "at", "zoo", "apple"]);
    }
}
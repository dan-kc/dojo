// BTree Custom Ordering Practice
//
// Learning Objectives:
// - Implement custom ordering for BTreeSet elements
// - Use wrapper types with custom Ord implementations
// - Practice length-first then lexicographic ordering
// - Understand how custom ordering affects BTreeSet behavior
//
// Run with: cargo test btree_custom_ordering

/// Implement a custom ordering for BTreeSet using a wrapper type.
/// Sort strings by length first, then lexicographically.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LengthFirstString(String);

impl Ord for LengthFirstString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!()
    }
}

impl PartialOrd for LengthFirstString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_length_first_string_ordering() {
        let mut set = BTreeSet::new();

        set.insert(LengthFirstString("zoo".to_string())); // length 3
        set.insert(LengthFirstString("a".to_string())); // length 1
        set.insert(LengthFirstString("apple".to_string())); // length 5
        set.insert(LengthFirstString("an".to_string())); // length 2
        set.insert(LengthFirstString("at".to_string())); // length 2

        let ordered: Vec<_> = set.iter().map(|s| &s.0).collect();

        // Should be: "a" (len 1), "an", "at" (len 2, alphabetical), "zoo" (len 3), "apple" (len 5)
        assert_eq!(ordered, vec!["a", "an", "at", "zoo", "apple"]);
    }
}

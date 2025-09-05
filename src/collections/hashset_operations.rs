// HashSet Operations Practice
//
// Learning Objectives:
// - Master HashSet creation, insertion, and membership testing
// - Use set operations (union, intersection, difference, symmetric_difference)
// - Practice with custom hash types and trait implementations
// - Implement set-based algorithms and data structures
// - Understand HashSet performance characteristics
// - Work with HashSet iteration and filtering
//
// Run with: cargo test --bin hashset_operations

/// Find all unique elements that appear in any of the input sets.
/// Use HashSet union operations efficiently.
fn union_all_sets<T>(sets: Vec<std::collections::HashSet<T>>) -> std::collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement union of multiple sets")
}

/// Find elements that appear in all input sets (intersection of all sets).
/// Handle empty input gracefully.
fn intersect_all_sets<T>(sets: Vec<std::collections::HashSet<T>>) -> std::collections::HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement intersection of multiple sets")
}

/// Find elements that are unique to each set (appear in exactly one set).
/// Return a HashMap mapping each unique element to the set index it came from.
fn find_unique_elements<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> std::collections::HashMap<T, usize>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement finding elements unique to each set")
}

/// Implement set partitioning based on a predicate function.
/// Return two sets: one with elements matching the predicate, one without.
fn partition_set<T, F>(
    set: std::collections::HashSet<T>,
    predicate: F,
) -> (std::collections::HashSet<T>, std::collections::HashSet<T>)
where
    T: Clone + std::hash::Hash + Eq,
    F: Fn(&T) -> bool,
{
    todo!("Implement set partitioning")
}

/// Custom hashable type for testing HashSet with complex objects.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        todo!("Implement hash based on email only (treating email as unique identifier)")
    }
}

impl Person {
    fn new(name: String, age: u32, email: String) -> Self {
        Self { name, age, email }
    }
}

/// Create and manipulate HashSet with custom Person objects.
/// Demonstrate that equality is based on email only.
fn person_set_operations() -> std::collections::HashSet<Person> {
    todo!("Implement operations with Person objects, showing email-based uniqueness")
}

/// Implement a simple spell checker using HashSet for dictionary lookup.
/// Return words that are not in the dictionary.
fn spell_check(
    text: &str,
    dictionary: &std::collections::HashSet<String>,
) -> std::collections::HashSet<String> {
    todo!("Implement spell checking using HashSet")
}

/// Find connected components in a graph represented as edges.
/// Use HashSet to track visited nodes and implement depth-first search.
fn find_connected_components(
    edges: Vec<(i32, i32)>,
) -> Vec<std::collections::HashSet<i32>> {
    todo!("Implement connected components using HashSet")
}

/// Implement set-based duplicate detection with detailed reporting.
/// Return both the duplicates and their frequencies across inputs.
fn analyze_duplicates<T>(
    collections: Vec<Vec<T>>,
) -> (std::collections::HashSet<T>, std::collections::HashMap<T, usize>)
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement duplicate analysis using HashSet")
}

/// Power set generation - return all possible subsets of a given set.
/// Use HashSet to represent each subset and return Vec of HashSets.
fn power_set<T>(
    set: std::collections::HashSet<T>,
) -> Vec<std::collections::HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement power set generation")
}

/// Implement Jaccard similarity coefficient between two sets.
/// Jaccard similarity = |A ∩ B| / |A ∪ B|
fn jaccard_similarity<T>(
    set_a: &std::collections::HashSet<T>,
    set_b: &std::collections::HashSet<T>,
) -> f64
where
    T: std::hash::Hash + Eq,
{
    todo!("Implement Jaccard similarity calculation")
}

/// Track set membership changes over time.
/// Maintain history of additions and removals to a set.
struct SetTracker<T> {
    current_set: std::collections::HashSet<T>,
    additions: Vec<T>,
    removals: Vec<T>,
}

impl<T> SetTracker<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    fn new() -> Self {
        todo!("Implement new SetTracker")
    }

    fn insert(&mut self, item: T) -> bool {
        todo!("Implement tracked insert")
    }

    fn remove(&mut self, item: &T) -> bool {
        todo!("Implement tracked remove")
    }

    fn contains(&self, item: &T) -> bool {
        self.current_set.contains(item)
    }

    fn addition_history(&self) -> &[T] {
        &self.additions
    }

    fn removal_history(&self) -> &[T] {
        &self.removals
    }

    fn current_set(&self) -> &std::collections::HashSet<T> {
        &self.current_set
    }
}

/// Implement set-based algorithm for finding common friends in a social network.
/// Given friendship connections, find mutual friends between two people.
fn find_mutual_friends(
    friendships: &std::collections::HashMap<String, std::collections::HashSet<String>>,
    person1: &str,
    person2: &str,
) -> std::collections::HashSet<String> {
    todo!("Implement mutual friends finder")
}

/// Use HashSet to implement efficient duplicate removal while preserving order.
/// Return Vec with duplicates removed, maintaining first occurrence order.
fn dedup_preserve_order<T>(items: Vec<T>) -> Vec<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement order-preserving deduplication using HashSet")
}

/// Implement set difference chain - consecutive differences between sets.
/// Given sets [A, B, C, D], return [A-B, B-C, C-D].
fn difference_chain<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> Vec<std::collections::HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement consecutive set differences")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_union_all_sets() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4, 5].iter().cloned().collect();
        let set3: HashSet<i32> = [5, 6, 7].iter().cloned().collect();
        
        let result = union_all_sets(vec![set1, set2, set3]);
        let expected: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7].iter().cloned().collect();
        
        assert_eq!(result, expected);
        
        // Test with empty sets
        let empty_result = union_all_sets(vec![HashSet::<i32>::new(), HashSet::<i32>::new()]);
        assert!(empty_result.is_empty());
    }

    #[test]
    fn test_intersect_all_sets() {
        let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 3, 4, 5].iter().cloned().collect();
        let set3: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
        
        let result = intersect_all_sets(vec![set1, set2, set3]);
        let expected: HashSet<i32> = [3, 4].iter().cloned().collect();
        
        assert_eq!(result, expected);
        
        // Test with no common elements
        let set1: HashSet<i32> = [1, 2].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4].iter().cloned().collect();
        let result = intersect_all_sets(vec![set1, set2]);
        assert!(result.is_empty());
        
        // Test with empty input
        let result: HashSet<i32> = intersect_all_sets(vec![]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_unique_elements() {
        let set1: HashSet<char> = ['a', 'b', 'c'].iter().cloned().collect();
        let set2: HashSet<char> = ['b', 'c', 'd'].iter().cloned().collect();
        let set3: HashSet<char> = ['c', 'd', 'e'].iter().cloned().collect();
        
        let result = find_unique_elements(vec![set1, set2, set3]);
        
        assert_eq!(result.get(&'a'), Some(&0)); // 'a' is unique to set 0
        assert_eq!(result.get(&'e'), Some(&2)); // 'e' is unique to set 2
        assert_eq!(result.get(&'b'), None); // 'b' appears in sets 0 and 1
        assert_eq!(result.get(&'c'), None); // 'c' appears in all sets
        assert_eq!(result.get(&'d'), None); // 'd' appears in sets 1 and 2
    }

    #[test]
    fn test_partition_set() {
        let set: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().cloned().collect();
        let (evens, odds) = partition_set(set, |&x| x % 2 == 0);
        
        let expected_evens: HashSet<i32> = [2, 4, 6, 8, 10].iter().cloned().collect();
        let expected_odds: HashSet<i32> = [1, 3, 5, 7, 9].iter().cloned().collect();
        
        assert_eq!(evens, expected_evens);
        assert_eq!(odds, expected_odds);
        
        // Test with all elements matching predicate
        let all_evens: HashSet<i32> = [2, 4, 6].iter().cloned().collect();
        let (matching, non_matching) = partition_set(all_evens, |&x| x % 2 == 0);
        assert_eq!(matching.len(), 3);
        assert!(non_matching.is_empty());
    }

    #[test]
    fn test_person_hash_and_equality() {
        let person1 = Person::new("John".to_string(), 30, "john@email.com".to_string());
        let person2 = Person::new("Johnny".to_string(), 31, "john@email.com".to_string()); // Same email
        let person3 = Person::new("John".to_string(), 30, "john2@email.com".to_string()); // Different email
        
        let mut set = HashSet::new();
        set.insert(person1.clone());
        
        // person2 should be considered equal due to same email
        assert!(!set.insert(person2)); // Should return false (already exists)
        assert_eq!(set.len(), 1);
        
        // person3 should be different due to different email
        assert!(set.insert(person3)); // Should return true (new addition)
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_person_set_operations() {
        let set = person_set_operations();
        assert!(!set.is_empty());
        
        // Test that the function demonstrates email-based uniqueness
        // by having multiple Person objects with same email but different other fields
        let people: Vec<_> = set.iter().collect();
        
        // All people in set should have unique emails
        let emails: HashSet<_> = people.iter().map(|p| &p.email).collect();
        assert_eq!(emails.len(), set.len());
    }

    #[test]
    fn test_spell_check() {
        let dictionary: HashSet<String> = ["hello", "world", "rust", "programming"]
            .iter().map(|s| s.to_string()).collect();
        
        let text = "hello wrold rust programing test";
        let errors = spell_check(text, &dictionary);
        
        let expected_errors: HashSet<String> = ["wrold", "programing", "test"]
            .iter().map(|s| s.to_string()).collect();
        
        assert_eq!(errors, expected_errors);
        
        // Test with no errors
        let correct_text = "hello world rust programming";
        let no_errors = spell_check(correct_text, &dictionary);
        assert!(no_errors.is_empty());
    }

    #[test]
    fn test_find_connected_components() {
        let edges = vec![
            (1, 2), (2, 3), // Component 1: {1, 2, 3}
            (4, 5),         // Component 2: {4, 5}
            (6, 7), (7, 8), (8, 6), // Component 3: {6, 7, 8}
        ];
        
        let components = find_connected_components(edges);
        assert_eq!(components.len(), 3);
        
        // Check that all nodes are covered
        let all_nodes: HashSet<i32> = components.iter()
            .flat_map(|comp| comp.iter())
            .cloned()
            .collect();
        let expected_nodes: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8].iter().cloned().collect();
        assert_eq!(all_nodes, expected_nodes);
        
        // Check component sizes
        let sizes: Vec<usize> = components.iter().map(|comp| comp.len()).collect();
        let mut sizes = sizes;
        sizes.sort();
        assert_eq!(sizes, vec![2, 3, 3]);
    }

    #[test]
    fn test_analyze_duplicates() {
        let collections = vec![
            vec!['a', 'b', 'c', 'a'],
            vec!['b', 'c', 'd', 'b'],
            vec!['c', 'd', 'e', 'c'],
        ];
        
        let (duplicates, frequencies) = analyze_duplicates(collections);
        
        let expected_duplicates: HashSet<char> = ['a', 'b', 'c', 'd'].iter().cloned().collect();
        assert_eq!(duplicates, expected_duplicates);
        
        assert_eq!(frequencies.get(&'a'), Some(&2)); // appears twice in first collection
        assert_eq!(frequencies.get(&'b'), Some(&4)); // appears in collections 0 and 1, twice each
        assert_eq!(frequencies.get(&'c'), Some(&6)); // appears in all collections, twice each
        assert_eq!(frequencies.get(&'d'), Some(&4)); // appears in collections 1 and 2, twice each
        assert_eq!(frequencies.get(&'e'), Some(&1)); // appears once in last collection
    }

    #[test]
    fn test_power_set() {
        let set: HashSet<i32> = [1, 2].iter().cloned().collect();
        let power = power_set(set);
        
        assert_eq!(power.len(), 4); // 2^2 = 4 subsets
        
        let empty_set = HashSet::new();
        let set1: HashSet<i32> = [1].iter().cloned().collect();
        let set2: HashSet<i32> = [2].iter().cloned().collect();
        let full_set: HashSet<i32> = [1, 2].iter().cloned().collect();
        
        // Check that all expected subsets are present
        assert!(power.iter().any(|s| s == &empty_set));
        assert!(power.iter().any(|s| s == &set1));
        assert!(power.iter().any(|s| s == &set2));
        assert!(power.iter().any(|s| s == &full_set));
        
        // Test with empty set
        let empty_input = HashSet::<i32>::new();
        let empty_power = power_set(empty_input);
        assert_eq!(empty_power.len(), 1); // Only contains empty set
        assert!(empty_power.iter().any(|s| s == &HashSet::new()));
    }

    #[test]
    fn test_jaccard_similarity() {
        let set_a: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        let set_b: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
        
        let similarity = jaccard_similarity(&set_a, &set_b);
        // Intersection: {3, 4} (size 2)
        // Union: {1, 2, 3, 4, 5, 6} (size 6)
        // Similarity: 2/6 = 1/3 ≈ 0.333
        assert!((similarity - 1.0/3.0).abs() < 1e-10);
        
        // Test identical sets
        let identical_similarity = jaccard_similarity(&set_a, &set_a);
        assert!((identical_similarity - 1.0).abs() < 1e-10);
        
        // Test disjoint sets
        let set_c: HashSet<i32> = [7, 8, 9].iter().cloned().collect();
        let disjoint_similarity = jaccard_similarity(&set_a, &set_c);
        assert!((disjoint_similarity - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_set_tracker() {
        let mut tracker = SetTracker::new();
        
        // Test insertions
        assert!(tracker.insert("a"));
        assert!(!tracker.insert("a")); // Duplicate
        assert!(tracker.insert("b"));
        
        assert_eq!(tracker.addition_history(), &["a", "b"]);
        assert!(tracker.removal_history().is_empty());
        assert_eq!(tracker.current_set().len(), 2);
        
        // Test removals
        assert!(tracker.remove(&"a"));
        assert!(!tracker.remove(&"a")); // Already removed
        
        assert_eq!(tracker.removal_history(), &["a"]);
        assert_eq!(tracker.current_set().len(), 1);
        assert!(tracker.contains(&"b"));
        assert!(!tracker.contains(&"a"));
    }

    #[test]
    fn test_find_mutual_friends() {
        let mut friendships = HashMap::new();
        
        friendships.insert("Alice".to_string(), 
            ["Bob", "Charlie", "David"].iter().map(|s| s.to_string()).collect());
        friendships.insert("Bob".to_string(), 
            ["Alice", "Charlie", "Eve"].iter().map(|s| s.to_string()).collect());
        friendships.insert("Charlie".to_string(), 
            ["Alice", "Bob", "David"].iter().map(|s| s.to_string()).collect());
        
        let mutual = find_mutual_friends(&friendships, "Alice", "Bob");
        let expected: HashSet<String> = ["Charlie"].iter().map(|s| s.to_string()).collect();
        
        assert_eq!(mutual, expected);
        
        // Test with no mutual friends
        friendships.insert("Frank".to_string(), 
            ["Eve"].iter().map(|s| s.to_string()).collect());
        let no_mutual = find_mutual_friends(&friendships, "Alice", "Frank");
        assert!(no_mutual.is_empty());
    }

    #[test]
    fn test_dedup_preserve_order() {
        let items = vec!["a", "b", "a", "c", "b", "d", "a"];
        let result = dedup_preserve_order(items);
        
        assert_eq!(result, vec!["a", "b", "c", "d"]);
        
        // Test with no duplicates
        let no_dups = vec!["x", "y", "z"];
        let result = dedup_preserve_order(no_dups.clone());
        assert_eq!(result, no_dups);
        
        // Test with all duplicates
        let all_dups = vec!["same", "same", "same"];
        let result = dedup_preserve_order(all_dups);
        assert_eq!(result, vec!["same"]);
    }

    #[test]
    fn test_difference_chain() {
        let set1: HashSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        let set2: HashSet<i32> = [2, 3, 4, 5].iter().cloned().collect();
        let set3: HashSet<i32> = [3, 4, 5, 6].iter().cloned().collect();
        let set4: HashSet<i32> = [4, 5, 6, 7].iter().cloned().collect();
        
        let chain = difference_chain(vec![set1, set2, set3, set4]);
        
        assert_eq!(chain.len(), 3);
        
        // set1 - set2 = {1}
        let expected1: HashSet<i32> = [1].iter().cloned().collect();
        assert_eq!(chain[0], expected1);
        
        // set2 - set3 = {2}
        let expected2: HashSet<i32> = [2].iter().cloned().collect();
        assert_eq!(chain[1], expected2);
        
        // set3 - set4 = {3}
        let expected3: HashSet<i32> = [3].iter().cloned().collect();
        assert_eq!(chain[2], expected3);
    }

    #[test]
    fn test_edge_cases() {
        // Test operations with empty sets
        let empty: HashSet<i32> = HashSet::new();
        assert!(union_all_sets(vec![empty.clone()]).is_empty());
        assert!(intersect_all_sets(vec![empty.clone()]).is_empty());
        
        // Test single element sets
        let single: HashSet<i32> = [42].iter().cloned().collect();
        assert_eq!(union_all_sets(vec![single.clone()]), single);
        assert_eq!(intersect_all_sets(vec![single.clone()]), single);
        
        // Test Jaccard similarity edge cases
        let empty_set: HashSet<i32> = HashSet::new();
        let non_empty: HashSet<i32> = [1].iter().cloned().collect();
        
        // Similarity between empty sets should be 1.0 (both are identical)
        assert!((jaccard_similarity(&empty_set, &empty_set) - 1.0).abs() < 1e-10);
        
        // Similarity between empty and non-empty should be 0.0
        assert!((jaccard_similarity(&empty_set, &non_empty) - 0.0).abs() < 1e-10);
    }
}
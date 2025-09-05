// Jaccard Similarity with HashSet Practice
//
// Learning objectives:
// - Calculate Jaccard similarity coefficient between sets
// - Use set intersection and union operations
// - Handle edge cases in similarity calculations
//
// Run with: cargo test jaccard_similarity

/// Implement Jaccard similarity coefficient between two sets.
/// Jaccard similarity = |A ∩ B| / |A ∪ B|
pub fn jaccard_similarity<T>(
    set_a: &std::collections::HashSet<T>,
    set_b: &std::collections::HashSet<T>,
) -> f64
where
    T: std::hash::Hash + Eq,
{
    todo!("Implement Jaccard similarity calculation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
    fn test_jaccard_similarity_empty_sets() {
        let empty_set: HashSet<i32> = HashSet::new();
        let non_empty: HashSet<i32> = [1].iter().cloned().collect();
        
        // Similarity between empty sets should be 1.0 (both are identical)
        assert!((jaccard_similarity(&empty_set, &empty_set) - 1.0).abs() < 1e-10);
        
        // Similarity between empty and non-empty should be 0.0
        assert!((jaccard_similarity(&empty_set, &non_empty) - 0.0).abs() < 1e-10);
        assert!((jaccard_similarity(&non_empty, &empty_set) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_complete_overlap() {
        let set1: HashSet<char> = ['a', 'b', 'c'].iter().cloned().collect();
        let set2: HashSet<char> = ['a', 'b', 'c'].iter().cloned().collect();
        
        let similarity = jaccard_similarity(&set1, &set2);
        assert!((similarity - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_subset() {
        let larger: HashSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
        let smaller: HashSet<i32> = [2, 3, 4].iter().cloned().collect();
        
        let similarity = jaccard_similarity(&larger, &smaller);
        // Intersection: {2, 3, 4} (size 3)
        // Union: {1, 2, 3, 4, 5} (size 5)
        // Similarity: 3/5 = 0.6
        assert!((similarity - 0.6).abs() < 1e-10);
        
        // Should be symmetric
        let reverse_similarity = jaccard_similarity(&smaller, &larger);
        assert!((similarity - reverse_similarity).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_single_element() {
        let single1: HashSet<&str> = ["test"].iter().cloned().collect();
        let single2: HashSet<&str> = ["test"].iter().cloned().collect();
        let different: HashSet<&str> = ["other"].iter().cloned().collect();
        
        // Same single element
        assert!((jaccard_similarity(&single1, &single2) - 1.0).abs() < 1e-10);
        
        // Different single elements
        assert!((jaccard_similarity(&single1, &different) - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_strings() {
        let words1: HashSet<String> = ["hello", "world", "rust"]
            .iter().map(|s| s.to_string()).collect();
        let words2: HashSet<String> = ["hello", "rust", "programming"]
            .iter().map(|s| s.to_string()).collect();
        
        let similarity = jaccard_similarity(&words1, &words2);
        // Intersection: {"hello", "rust"} (size 2)
        // Union: {"hello", "world", "rust", "programming"} (size 4)
        // Similarity: 2/4 = 0.5
        assert!((similarity - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_partial_overlap() {
        let set_a: HashSet<i32> = [1, 2, 3, 4, 5, 6].iter().cloned().collect();
        let set_b: HashSet<i32> = [4, 5, 6, 7, 8, 9].iter().cloned().collect();
        
        let similarity = jaccard_similarity(&set_a, &set_b);
        // Intersection: {4, 5, 6} (size 3)
        // Union: {1, 2, 3, 4, 5, 6, 7, 8, 9} (size 9)
        // Similarity: 3/9 = 1/3 ≈ 0.333
        assert!((similarity - 1.0/3.0).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_large_sets() {
        let large1: HashSet<i32> = (1..=100).collect();
        let large2: HashSet<i32> = (50..=150).collect();
        
        let similarity = jaccard_similarity(&large1, &large2);
        // Intersection: {50..=100} (size 51)
        // Union: {1..=150} (size 150)
        // Similarity: 51/150 = 0.34
        assert!((similarity - 51.0/150.0).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_symmetry() {
        let set_a: HashSet<char> = ['a', 'b', 'c', 'd'].iter().cloned().collect();
        let set_b: HashSet<char> = ['c', 'd', 'e', 'f'].iter().cloned().collect();
        
        let similarity_ab = jaccard_similarity(&set_a, &set_b);
        let similarity_ba = jaccard_similarity(&set_b, &set_a);
        
        // Jaccard similarity should be symmetric
        assert!((similarity_ab - similarity_ba).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_different_sizes() {
        let small: HashSet<i32> = [1, 2].iter().cloned().collect();
        let large: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10].iter().cloned().collect();
        
        let similarity = jaccard_similarity(&small, &large);
        // Intersection: {1, 2} (size 2)
        // Union: {1, 2, 3, 4, 5, 6, 7, 8, 9, 10} (size 10)
        // Similarity: 2/10 = 0.2
        assert!((similarity - 0.2).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_negative_numbers() {
        let positive: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let negative: HashSet<i32> = [-3, -2, -1].iter().cloned().collect();
        let mixed: HashSet<i32> = [-1, 0, 1].iter().cloned().collect();
        
        // Completely disjoint
        assert!((jaccard_similarity(&positive, &negative) - 0.0).abs() < 1e-10);
        
        // Mixed overlap
        let similarity = jaccard_similarity(&positive, &mixed);
        // Intersection: {1} (size 1)
        // Union: {-1, 0, 1, 2, 3} (size 5)
        // Similarity: 1/5 = 0.2
        assert!((similarity - 0.2).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_precision() {
        // Test with sets that create specific fractions
        let set_a: HashSet<i32> = [1, 2, 3, 4, 5, 6, 7, 8].iter().cloned().collect();
        let set_b: HashSet<i32> = [5, 6, 7, 8, 9, 10, 11, 12].iter().cloned().collect();
        
        let similarity = jaccard_similarity(&set_a, &set_b);
        // Intersection: {5, 6, 7, 8} (size 4)
        // Union: {1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12} (size 12)
        // Similarity: 4/12 = 1/3
        assert!((similarity - 1.0/3.0).abs() < 1e-10);
    }
}
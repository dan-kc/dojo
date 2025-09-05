// Power Set Generation with HashSet Practice
//
// Learning objectives:
// - Generate all possible subsets of a set
// - Use recursive or iterative approaches with sets
// - Understand exponential complexity with sets
//
// Run with: cargo test power_set

/// Power set generation - return all possible subsets of a given set.
/// Use HashSet to represent each subset and return Vec of HashSets.
pub fn power_set<T>(
    set: std::collections::HashSet<T>,
) -> Vec<std::collections::HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!("Implement power set generation")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
    fn test_power_set_single_element() {
        let set: HashSet<char> = ['a'].iter().cloned().collect();
        let power = power_set(set);
        
        assert_eq!(power.len(), 2); // 2^1 = 2 subsets
        
        let empty_set: HashSet<char> = HashSet::new();
        let single_set: HashSet<char> = ['a'].iter().cloned().collect();
        
        assert!(power.iter().any(|s| s == &empty_set));
        assert!(power.iter().any(|s| s == &single_set));
    }

    #[test]
    fn test_power_set_three_elements() {
        let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let power = power_set(set);
        
        assert_eq!(power.len(), 8); // 2^3 = 8 subsets
        
        // Check a few specific subsets
        let empty: HashSet<i32> = HashSet::new();
        let full: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let pair12: HashSet<i32> = [1, 2].iter().cloned().collect();
        let single3: HashSet<i32> = [3].iter().cloned().collect();
        
        assert!(power.iter().any(|s| s == &empty));
        assert!(power.iter().any(|s| s == &full));
        assert!(power.iter().any(|s| s == &pair12));
        assert!(power.iter().any(|s| s == &single3));
        
        // Verify all subsets are unique
        let mut unique_check = HashSet::new();
        for subset in &power {
            // Convert subset to a sorted vector for comparison
            let mut sorted: Vec<i32> = subset.iter().cloned().collect();
            sorted.sort();
            assert!(unique_check.insert(sorted));
        }
    }

    #[test]
    fn test_power_set_empty() {
        let empty_set: HashSet<String> = HashSet::new();
        let power = power_set(empty_set);
        
        assert_eq!(power.len(), 1); // 2^0 = 1 subset (empty set only)
        assert!(power[0].is_empty());
    }

    #[test]
    fn test_power_set_strings() {
        let set: HashSet<String> = ["hello", "world"].iter().map(|s| s.to_string()).collect();
        let power = power_set(set);
        
        assert_eq!(power.len(), 4); // 2^2 = 4 subsets
        
        let empty: HashSet<String> = HashSet::new();
        let hello: HashSet<String> = ["hello"].iter().map(|s| s.to_string()).collect();
        let world: HashSet<String> = ["world"].iter().map(|s| s.to_string()).collect();
        let both: HashSet<String> = ["hello", "world"].iter().map(|s| s.to_string()).collect();
        
        assert!(power.iter().any(|s| s == &empty));
        assert!(power.iter().any(|s| s == &hello));
        assert!(power.iter().any(|s| s == &world));
        assert!(power.iter().any(|s| s == &both));
    }

    #[test]
    fn test_power_set_property_based() {
        // Test that the power set has the correct size property
        for size in 0..=4 {
            let set: HashSet<i32> = (0..size).collect();
            let power = power_set(set);
            assert_eq!(power.len(), 1 << size); // 2^size
        }
    }

    #[test]
    fn test_power_set_contains_original() {
        let original: HashSet<i32> = [10, 20, 30].iter().cloned().collect();
        let power = power_set(original.clone());
        
        // The power set should contain the original set
        assert!(power.iter().any(|s| s == &original));
        
        // The power set should contain the empty set
        let empty: HashSet<i32> = HashSet::new();
        assert!(power.iter().any(|s| s == &empty));
    }

    #[test]
    fn test_power_set_subset_property() {
        let set: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let power = power_set(set.clone());
        
        // Every element in the power set should be a subset of the original set
        for subset in &power {
            for element in subset {
                assert!(set.contains(element));
            }
        }
    }

    #[test]
    fn test_power_set_negative_numbers() {
        let set: HashSet<i32> = [-1, 0, 1].iter().cloned().collect();
        let power = power_set(set);
        
        assert_eq!(power.len(), 8); // 2^3 = 8 subsets
        
        // Check that negative numbers are handled correctly
        let negative_only: HashSet<i32> = [-1].iter().cloned().collect();
        let mixed: HashSet<i32> = [-1, 1].iter().cloned().collect();
        
        assert!(power.iter().any(|s| s == &negative_only));
        assert!(power.iter().any(|s| s == &mixed));
    }

    #[test]
    fn test_power_set_characters() {
        let set: HashSet<char> = ['x', 'y'].iter().cloned().collect();
        let power = power_set(set);
        
        assert_eq!(power.len(), 4);
        
        // Check specific character combinations
        let x_only: HashSet<char> = ['x'].iter().cloned().collect();
        let y_only: HashSet<char> = ['y'].iter().cloned().collect();
        let both: HashSet<char> = ['x', 'y'].iter().cloned().collect();
        let empty: HashSet<char> = HashSet::new();
        
        assert!(power.iter().any(|s| s == &x_only));
        assert!(power.iter().any(|s| s == &y_only));
        assert!(power.iter().any(|s| s == &both));
        assert!(power.iter().any(|s| s == &empty));
    }
}
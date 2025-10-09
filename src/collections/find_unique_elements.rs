// cargo test find_unique_elements

/// Find elements that are unique to each set (appear in exactly one set).
/// Return a HashMap mapping each unique element to the set index it came from.
#[allow(unused_variables)]
pub fn find_unique_elements<T>(
    sets: Vec<std::collections::HashSet<T>>,
) -> std::collections::HashMap<T, usize>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

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
    fn test_find_unique_elements_no_overlap() {
        let set1: HashSet<i32> = [1, 2].iter().cloned().collect();
        let set2: HashSet<i32> = [3, 4].iter().cloned().collect();
        let set3: HashSet<i32> = [5, 6].iter().cloned().collect();

        let result = find_unique_elements(vec![set1, set2, set3]);

        // All elements should be unique since there's no overlap
        assert_eq!(result.len(), 6);
        assert_eq!(result.get(&1), Some(&0));
        assert_eq!(result.get(&2), Some(&0));
        assert_eq!(result.get(&3), Some(&1));
        assert_eq!(result.get(&4), Some(&1));
        assert_eq!(result.get(&5), Some(&2));
        assert_eq!(result.get(&6), Some(&2));
    }

    #[test]
    fn test_find_unique_elements_all_overlap() {
        let set1: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set2: HashSet<i32> = [1, 2, 3].iter().cloned().collect();
        let set3: HashSet<i32> = [1, 2, 3].iter().cloned().collect();

        let result = find_unique_elements(vec![set1, set2, set3]);

        // No elements should be unique since all appear in all sets
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_unique_elements_empty_sets() {
        let result = find_unique_elements(vec![HashSet::<i32>::new(), HashSet::<i32>::new()]);
        assert!(result.is_empty());
    }

    #[test]
    fn test_find_unique_elements_single_set() {
        let set1: HashSet<&str> = ["hello", "world"].iter().cloned().collect();
        let result = find_unique_elements(vec![set1]);

        // All elements should be unique in a single set
        assert_eq!(result.len(), 2);
        assert_eq!(result.get(&"hello"), Some(&0));
        assert_eq!(result.get(&"world"), Some(&0));
    }
}

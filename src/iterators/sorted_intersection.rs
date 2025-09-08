// Sorted Intersection
//
// Learning objectives:
// - Finding common elements between collections
// - Using HashSet for efficient lookups
// - Deduplication and sorting with iterators
//
// cargo test --bin sorted_intersection

/// Create a function that finds the intersection of two sorted vectors
/// (elements that appear in both) and returns them as a sorted vector
/// without duplicates.
pub fn sorted_intersection(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    todo!("Use iterator methods to find common elements efficiently")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sorted_intersection() {
        let first = vec![1, 2, 3, 4, 5];
        let second = vec![3, 4, 5, 6, 7];
        let result = sorted_intersection(first, second);
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_sorted_intersection_no_common() {
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];
        let result = sorted_intersection(first, second);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_sorted_intersection_with_duplicates() {
        let first = vec![1, 1, 2, 3, 3];
        let second = vec![1, 3, 3, 4, 4];
        let result = sorted_intersection(first, second);
        assert_eq!(result, vec![1, 3]);
    }

    #[test]
    fn test_sorted_intersection_empty_first() {
        let first = vec![];
        let second = vec![1, 2, 3];
        let result = sorted_intersection(first, second);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_sorted_intersection_empty_second() {
        let first = vec![1, 2, 3];
        let second = vec![];
        let result = sorted_intersection(first, second);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_sorted_intersection_all_common() {
        let first = vec![1, 2, 3];
        let second = vec![1, 2, 3];
        let result = sorted_intersection(first, second);
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_sorted_intersection_unsorted_input() {
        let first = vec![5, 1, 3, 2, 4];
        let second = vec![3, 1, 6, 4, 7];
        let result = sorted_intersection(first, second);
        assert_eq!(result, vec![1, 3, 4]);
    }
}

fn main() {
    println!("Run tests with: cargo test --bin sorted_intersection");
}
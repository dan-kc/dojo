// Elements Above Average
//
// Learning objectives:
// - Processing nested data structures with iterators
// - Using flat_map for flattening operations
// - Efficient average calculation and filtering
//
// cargo test --bin elements_above_average

/// Create a function that efficiently processes nested data structures.
/// Given a vector of vectors, find all elements that are greater than
/// the average of their respective inner vector.
pub fn elements_above_average(data: &[Vec<i32>]) -> Vec<i32> {
    todo!("Use flat_map and efficient average calculation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_elements_above_average() {
        let data = vec![
            vec![1, 2, 3, 4, 5], // avg = 3.0, above avg: 4, 5
            vec![10, 20],        // avg = 15.0, above avg: 20
            vec![7, 8, 9],       // avg = 8.0, above avg: 9
        ];
        let mut result = elements_above_average(&data);
        result.sort(); // Sort for consistent comparison
        assert_eq!(result, vec![4, 5, 9, 20]);
    }

    #[test]
    fn test_elements_above_average_no_elements() {
        let data = vec![
            vec![1, 1, 1], // avg = 1.0, no elements above
            vec![5, 5],    // avg = 5.0, no elements above
        ];
        let result = elements_above_average(&data);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_elements_above_average_empty_data() {
        let data: Vec<Vec<i32>> = vec![];
        let result = elements_above_average(&data);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_elements_above_average_with_empty_inner() {
        let data = vec![
            vec![1, 2, 3],
            vec![], // Empty inner vector
            vec![4, 5, 6],
        ];
        let mut result = elements_above_average(&data);
        result.sort();
        assert_eq!(result, vec![3, 6]); // 3 > 2.0 and 6 > 5.0
    }

    #[test]
    fn test_elements_above_average_single_element() {
        let data = vec![
            vec![42], // Single element, avg = 42.0, nothing above
        ];
        let result = elements_above_average(&data);
        assert_eq!(result, Vec::<i32>::new());
    }
}

fn main() {
    println!("Run tests with: cargo test --bin elements_above_average");
}
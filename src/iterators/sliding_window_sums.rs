// Sliding Window Sums
//
// Learning objectives:
// - Using windows() for overlapping slices
// - Understanding the difference between chunks() and windows()
// - Efficient sliding window computations
//
// cargo test --bin sliding_window_sums

/// Given a vector of integers, create windows of size 3 and return
/// the sum of each window. For example, [1,2,3,4,5] would create
/// windows [1,2,3], [2,3,4], [3,4,5] with sums [6, 9, 12].
pub fn sliding_window_sums(numbers: Vec<i32>) -> Vec<i32> {
    todo!("Use windows() method and map to sum each window")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sliding_window_sums() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, vec![6, 9, 12]); // [1+2+3, 2+3+4, 3+4+5]
    }

    #[test]
    fn test_sliding_window_sums_insufficient_elements() {
        let numbers = vec![1, 2];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_sliding_window_sums_exact_window_size() {
        let numbers = vec![10, 20, 30];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, vec![60]);
    }

    #[test]
    fn test_sliding_window_sums_negative_numbers() {
        let numbers = vec![-1, 0, 1, 2, 3];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, vec![0, 3, 6]); // [-1+0+1, 0+1+2, 1+2+3]
    }

    #[test]
    fn test_sliding_window_sums_empty() {
        let numbers = vec![];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, Vec::<i32>::new());
    }

    #[test]
    fn test_sliding_window_sums_large_numbers() {
        let numbers = vec![100, 200, 300, 400];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, vec![600, 900]); // [100+200+300, 200+300+400]
    }
}

fn main() {
    println!("Run tests with: cargo test --bin sliding_window_sums");
}
// Chunk Averages
//
// Learning objectives:
// - Efficient batch processing with chunks()
// - Computing aggregates over slices
// - Understanding iterator methods for windowing operations
//
// cargo test --bin chunk_averages

/// Create a function that processes a large dataset in chunks to demonstrate
/// efficient batch processing. Calculate the average of each chunk and return
/// a vector of chunk averages.
pub fn chunk_averages(numbers: &[f64], chunk_size: usize) -> Vec<f64> {
    todo!("Use chunks() iterator method for efficient batch processing")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chunk_averages() {
        let numbers = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0];
        let result = chunk_averages(&numbers, 3);
        assert_eq!(result, vec![2.0, 5.0, 7.5]); // [1,2,3]=2.0, [4,5,6]=5.0, [7,8]=7.5
    }

    #[test]
    fn test_chunk_averages_exact_chunks() {
        let numbers = [10.0, 20.0, 30.0, 40.0];
        let result = chunk_averages(&numbers, 2);
        assert_eq!(result, vec![15.0, 35.0]); // [10,20]=15.0, [30,40]=35.0
    }

    #[test]
    fn test_chunk_averages_single_element_chunks() {
        let numbers = [1.0, 2.0, 3.0];
        let result = chunk_averages(&numbers, 1);
        assert_eq!(result, vec![1.0, 2.0, 3.0]);
    }

    #[test]
    fn test_chunk_averages_empty() {
        let numbers = [];
        let result = chunk_averages(&numbers, 3);
        assert_eq!(result, Vec::<f64>::new());
    }

    #[test]
    fn test_chunk_averages_chunk_size_zero() {
        let numbers = [1.0, 2.0, 3.0];
        let result = chunk_averages(&numbers, 0);
        assert_eq!(result, Vec::<f64>::new()); // chunks(0) returns empty iterator
    }
}

fn main() {
    println!("Run tests with: cargo test --bin chunk_averages");
}

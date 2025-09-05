// Zip Vectors
//
// Learning objectives:
// - Understanding zip() for parallel iteration
// - Working with generic types in iterators
// - Handling vectors of different lengths
//
// cargo test --bin zip_vectors

/// Create a function that takes two vectors of equal length and returns
/// a vector of tuples pairing elements at the same indices.
/// If vectors have different lengths, pair up to the shorter length.
pub fn zip_vectors<T, U>(first: Vec<T>, second: Vec<U>) -> Vec<(T, U)>
where
    T: Clone,
    U: Clone,
{
    todo!("Use zip() to combine two vectors")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_vectors_equal_length() {
        let first = vec![1, 2, 3];
        let second = vec!["a", "b", "c"];
        let result = zip_vectors(first, second);
        assert_eq!(result, vec![(1, "a"), (2, "b"), (3, "c")]);
    }

    #[test]
    fn test_zip_vectors_different_lengths() {
        let first = vec![1, 2, 3, 4, 5];
        let second = vec!["a", "b"];
        let result = zip_vectors(first, second);
        assert_eq!(result, vec![(1, "a"), (2, "b")]);
    }

    #[test]
    fn test_zip_vectors_empty() {
        let first: Vec<i32> = vec![];
        let second = vec!["a", "b"];
        let result = zip_vectors(first, second);
        assert_eq!(result, Vec::<(i32, &str)>::new());
    }

    #[test]
    fn test_zip_vectors_both_empty() {
        let first: Vec<i32> = vec![];
        let second: Vec<String> = vec![];
        let result = zip_vectors(first, second);
        assert_eq!(result, Vec::<(i32, String)>::new());
    }

    #[test]
    fn test_zip_vectors_strings() {
        let first = vec!["hello".to_string(), "world".to_string()];
        let second = vec![1, 2, 3];
        let result = zip_vectors(first, second);
        assert_eq!(result, vec![
            ("hello".to_string(), 1),
            ("world".to_string(), 2)
        ]);
    }
}

fn main() {
    println!("Run tests with: cargo test --bin zip_vectors");
}
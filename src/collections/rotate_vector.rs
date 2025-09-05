// Vector Rotation Practice
//
// Learning objectives:
// - Using Vec rotate_left() and rotate_right() methods
// - Handling positive/negative rotation values
// - Understanding modular arithmetic for rotations
//
// Run with: cargo test rotate_vector

/// Rotate vector elements efficiently using Vec operations.
/// Positive n rotates right, negative n rotates left.
pub fn rotate_vector<T>(mut vec: Vec<T>, n: isize) -> Vec<T> {
    todo!("Implement vector rotation")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_vector() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = rotate_vector(vec.clone(), 2);
        assert_eq!(result, vec![4, 5, 1, 2, 3]);
        
        let result = rotate_vector(vec.clone(), -2);
        assert_eq!(result, vec![3, 4, 5, 1, 2]);
        
        let result = rotate_vector(vec.clone(), 0);
        assert_eq!(result, vec);
        
        let result = rotate_vector(vec.clone(), 5); // Full rotation
        assert_eq!(result, vec);
    }

    #[test]
    fn test_rotate_edge_cases() {
        // Empty vector
        let empty: Vec<i32> = vec![];
        let result = rotate_vector(empty.clone(), 3);
        assert_eq!(result, empty);

        // Single element
        let single = vec![42];
        let result = rotate_vector(single.clone(), 1);
        assert_eq!(result, single);

        // Large rotation values
        let vec = vec![1, 2, 3, 4];
        let result = rotate_vector(vec.clone(), 7); // 7 % 4 = 3, same as rotating by 3
        assert_eq!(result, vec![2, 3, 4, 1]);
    }

    #[test]
    fn test_rotate_negative() {
        let vec = vec!['a', 'b', 'c', 'd', 'e'];
        let result = rotate_vector(vec.clone(), -3);
        assert_eq!(result, vec!['d', 'e', 'a', 'b', 'c']);
        
        let result = rotate_vector(vec.clone(), -7); // -7 % 5 = -2
        assert_eq!(result, vec!['d', 'e', 'a', 'b', 'c']);
    }
}
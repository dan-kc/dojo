// cargo test --lib collections::rotate_vector::

//! Positive n rotates right and negative n rotates left, modulo length. Empty input is unchanged. Support isize::MIN and preserve the vector allocation without cloning elements.

/// Rotate vector elements efficiently using Vec operations.
/// Positive n rotates right, negative n rotates left.
#[allow(unused_variables, unused_mut)]
pub fn rotate_vector<T>(mut vec: Vec<T>, n: isize) -> Vec<T> {
    todo!()
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

        let result = rotate_vector(vec.clone(), -8); // -8 % 5 = -3
        assert_eq!(result, vec!['d', 'e', 'a', 'b', 'c']);
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    #[test]
    fn minimum_rotation_reuses_allocation() {
        let v = vec![0, 1, 2, 3, 4];
        let p = v.as_ptr();
        let mut expected = v.clone();
        expected.rotate_right(isize::MIN.rem_euclid(5) as usize);
        let result = rotate_vector(v, isize::MIN);
        assert_eq!(result, expected);
        assert_eq!(result.as_ptr(), p);
    }
}

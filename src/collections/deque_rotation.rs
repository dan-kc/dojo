// cargo test --lib collections::deque_rotation::

//! Rotate left for positive n and right for negative n, modulo length. Empty input is unchanged. Support isize::MIN without overflow. Preserve all elements without cloning.
/// Rotate left by n positions (negative n rotates right).
#[allow(unused_variables)]
#[allow(unused_mut)]
pub fn rotate_deque<T>(
    mut deque: std::collections::VecDeque<T>,
    n: isize,
) -> std::collections::VecDeque<T> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::VecDeque;

    #[test]
    fn test_rotate_deque() {
        let mut deque = VecDeque::new();
        deque.extend([1, 2, 3, 4, 5]);

        // Rotate left by 2
        let rotated = rotate_deque(deque.clone(), 2);
        let expected: VecDeque<_> = [3, 4, 5, 1, 2].iter().cloned().collect();
        assert_eq!(rotated, expected);

        // Rotate right by 2 (negative)
        let rotated = rotate_deque(deque.clone(), -2);
        let expected: VecDeque<_> = [4, 5, 1, 2, 3].iter().cloned().collect();
        assert_eq!(rotated, expected);

        // Rotate left by 7
        let rotated = rotate_deque(deque.clone(), 7);
        let expected: VecDeque<_> = [3, 4, 5, 1, 2].iter().cloned().collect();
        assert_eq!(rotated, expected);

        // Rotate right by 7 (negative)
        let rotated = rotate_deque(deque.clone(), -7);
        let expected: VecDeque<_> = [4, 5, 1, 2, 3].iter().cloned().collect();
        assert_eq!(rotated, expected);

        // No rotation
        let rotated = rotate_deque(deque.clone(), 0);
        assert_eq!(rotated, deque);

        // Rotation by full length
        let rotated = rotate_deque(deque.clone(), 5);
        assert_eq!(rotated, deque);
    }

    #[test]
    fn test_edge_cases() {
        // Test empty deque
        let empty_deque: VecDeque<i32> = VecDeque::new();
        let rotated = rotate_deque(empty_deque.clone(), 5);
        assert_eq!(rotated, empty_deque);

        // Test single element deque
        let single_deque: VecDeque<_> = [42].iter().cloned().collect();
        let rotated = rotate_deque(single_deque.clone(), 1);
        assert_eq!(rotated, single_deque);
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    #[test]
    fn minimum_rotation() {
        let d = std::collections::VecDeque::from([0, 1, 2, 3, 4]);
        let mut expected = d.clone();
        expected.rotate_left(isize::MIN.rem_euclid(5) as usize);
        assert_eq!(rotate_deque(d, isize::MIN), expected);
    }
}

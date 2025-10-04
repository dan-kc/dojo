// Run with: cargo test deque_rotation

/// Use VecDeque to implement efficient rotation of elements.
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

// Circular Buffer Practice
//
// Learning Objectives:
// - Implement a circular buffer using VecDeque
// - Fixed-size buffer that overwrites oldest elements when full
// - Practice with bounded collections and overflow handling
// - Understand circular data structure patterns
//
// Run with: cargo test --bin circular_buffer

/// Implement a circular buffer using VecDeque.
/// Fixed-size buffer that overwrites oldest elements when full.
pub struct CircularBuffer<T> {
    buffer: std::collections::VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        todo!("Create new circular buffer")
    }

    pub fn push(&mut self, item: T) {
        todo!("Add item, removing oldest if at capacity")
    }

    pub fn get(&self, index: usize) -> Option<&T> {
        todo!("Get item at index (0 is newest)")
    }

    pub fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        todo!("Return iterator from newest to oldest")
    }

    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    pub fn is_full(&self) -> bool {
        self.buffer.len() == self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circular_buffer() {
        let mut buffer = CircularBuffer::new(3);
        
        buffer.push("a");
        buffer.push("b");
        buffer.push("c");
        assert_eq!(buffer.len(), 3);
        assert!(buffer.is_full());
        
        // Should have [a, b, c] with c being newest
        assert_eq!(buffer.get(0), Some(&"c")); // newest
        assert_eq!(buffer.get(1), Some(&"b"));
        assert_eq!(buffer.get(2), Some(&"a")); // oldest
        
        // Adding another should overwrite oldest
        buffer.push("d");
        assert_eq!(buffer.len(), 3);
        
        // Should now have [b, c, d] with d being newest
        assert_eq!(buffer.get(0), Some(&"d")); // newest
        assert_eq!(buffer.get(1), Some(&"c"));
        assert_eq!(buffer.get(2), Some(&"b")); // oldest (a was evicted)
        
        // Test iterator
        let items: Vec<_> = buffer.iter().collect();
        assert_eq!(items, vec![&"d", &"c", &"b"]);
    }

    #[test]
    fn test_capacity_one() {
        let mut buffer = CircularBuffer::new(1);
        buffer.push("only");
        assert!(buffer.is_full());
        buffer.push("replacement");
        assert_eq!(buffer.get(0), Some(&"replacement"));
        assert_eq!(buffer.len(), 1);
    }
}
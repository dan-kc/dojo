# iterators/custom_iterator

Reference implementation. Read after attempting the exercise.

```rust
pub struct FibonacciIterator {
    current: Option<u64>,
    next: Option<u64>,
    max_value: u64,
}
impl FibonacciIterator {
    pub fn new(max_value: u64) -> Self {
        Self {
            current: Some(0),
            next: Some(1),
            max_value,
        }
    }
}
impl Iterator for FibonacciIterator {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let value = self.current?;
        if value > self.max_value {
            self.current = None;
            return None;
        }
        self.current = self.next;
        self.next = self.next.and_then(|n| value.checked_add(n));
        Some(value)
    }
}
```

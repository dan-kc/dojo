# Custom Iterator - Solution

## Solution

```rust
pub struct FibonacciIterator {
    current: u64,
    next: u64,
    max_value: u64,
}

impl FibonacciIterator {
    pub fn new(max_value: u64) -> Self {
        Self {
            current: 0,
            next: 1,
            max_value,
        }
    }
}

impl std::iter::Iterator for FibonacciIterator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max_value {
            return None;
        }

        let result = self.current;
        let new_next = self.current + self.next;
        self.current = self.next;
        self.next = new_next;
        
        Some(result)
    }
}

pub struct EveryNth<I> {
    iter: I,
    n: usize,
    current_position: usize,
}

impl<I> EveryNth<I>
where
    I: std::iter::Iterator,
{
    pub fn new(iter: I, n: usize) -> Self {
        Self {
            iter,
            n,
            current_position: 0,
        }
    }
}

impl<I> std::iter::Iterator for EveryNth<I>
where
    I: std::iter::Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(item) => {
                    if self.current_position % self.n == 0 {
                        self.current_position += 1;
                        return Some(item);
                    }
                    self.current_position += 1;
                }
                None => return None,
            }
        }
    }
}

pub struct FloatRange {
    current: f64,
    end: f64,
    step: f64,
}

impl FloatRange {
    pub fn new(start: f64, end: f64, step: f64) -> Self {
        Self {
            current: start,
            end,
            step,
        }
    }
}

impl std::iter::Iterator for FloatRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end {
            let result = self.current;
            self.current += self.step;
            Some(result)
        } else {
            None
        }
    }
}

pub struct CycleOwned<T> {
    items: Vec<T>,
    current_index: usize,
}

impl<T> CycleOwned<T>
where
    T: Clone,
{
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            current_index: 0,
        }
    }
}

impl<T> std::iter::Iterator for CycleOwned<T>
where
    T: Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.items.is_empty() {
            return None;
        }

        let item = self.items[self.current_index].clone();
        self.current_index = (self.current_index + 1) % self.items.len();
        Some(item)
    }
}
```

## Explanation

### Iterator Trait Implementation

**Core Requirements:**
- Define `type Item` to specify what the iterator yields
- Implement `next(&mut self) -> Option<Self::Item>`
- Maintain internal state between calls to `next()`

**State Management Patterns:**

1. **Sequential Generation (Fibonacci):**
   - Track current and next values
   - Update state for each iteration
   - Check bounds before yielding

2. **Filtering with Position (EveryNth):**
   - Wrap an existing iterator
   - Track position counter
   - Skip elements that don't match criteria

3. **Range Generation (FloatRange):**
   - Track current position and increment
   - Compare against end condition
   - Handle floating-point precision considerations

4. **Cycling (CycleOwned):**
   - Store data and current index
   - Use modulo arithmetic for cycling
   - Handle empty collections gracefully

### Design Patterns

**Generic Iterator Wrappers:**
`EveryNth<I>` demonstrates wrapping any iterator type using generics and trait bounds.

**Owned Data Iteration:**
`CycleOwned<T>` shows how to create iterators that own their data, useful when you need infinite iteration over finite data.

**Mathematical Sequences:**
`FibonacciIterator` and `FloatRange` show different approaches to generating mathematical sequences with proper termination conditions.

### Performance Considerations

**Memory Efficiency:**
- `FibonacciIterator`: O(1) memory, generates on-demand
- `EveryNth`: O(1) additional memory, wraps existing iterator
- `FloatRange`: O(1) memory, pure computation
- `CycleOwned`: O(n) memory for data storage

**Computational Efficiency:**
- Iterator methods are zero-cost abstractions
- `Clone` trait bound allows efficient cycling
- Lazy evaluation means work only happens when `next()` is called

**Common Pitfalls:**
- Floating-point precision in `FloatRange`
- Integer overflow in `FibonacciIterator` (consider using checked arithmetic)
- Empty collections in `CycleOwned` require special handling
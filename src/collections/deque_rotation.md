# VecDeque Rotation Solution

## Implementation

```rust
pub fn rotate_deque<T>(
    mut deque: std::collections::VecDeque<T>,
    n: isize,
) -> std::collections::VecDeque<T> {
    if deque.len() == 0 {
        return deque;
    };
    // -7
    // [1,2,3,4,5]
    let n = n % deque.len() as isize;

    let left_rotation = if n < 0 {
        n + deque.len() as isize
    } else {
        n as isize
    } as usize;

    deque.rotate_left(left_rotation);
    deque
}
```

## Explanation

This solution implements efficient deque rotation using VecDeque's double-ended capabilities:

1. **Edge cases**: Handle empty deque or zero rotation
2. **Normalization**: Convert negative rotations to positive equivalents
3. **Bounds checking**: Use modulo to handle rotations larger than deque length
4. **Efficient rotation**: Move elements from back to front using deque operations

The rotation is performed by repeatedly moving the last element to the front.

## Key Learning Points

- **VecDeque efficiency**: O(1) operations at both ends enable efficient rotation
- **Rotation normalization**: Handle negative and large rotations properly
- **Modulo arithmetic**: Normalize rotation values to valid range
- **Double-ended operations**: Leverage push_front and pop_back for rotation

## Rust Concepts Demonstrated

- VecDeque for double-ended queue operations
- Pattern matching with `if let Some()`
- Modulo arithmetic for cyclic operations
- Handling signed integer operations safely
- Efficient algorithm design with specialized collections

# VecDeque Rotation Solution

## Implementation

```rust
pub fn rotate_deque<T>(mut deque: std::collections::VecDeque<T>, n: isize) -> std::collections::VecDeque<T> {
    if deque.is_empty() || n == 0 {
        return deque;
    }
    
    let len = deque.len() as isize;
    
    // Normalize rotation to be within bounds
    let effective_rotation = n % len;
    let effective_rotation = if effective_rotation < 0 {
        effective_rotation + len
    } else {
        effective_rotation
    };
    
    // Perform rotation by moving elements from back to front
    for _ in 0..effective_rotation {
        if let Some(item) = deque.pop_back() {
            deque.push_front(item);
        }
    }
    
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
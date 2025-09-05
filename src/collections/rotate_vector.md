# Vector Rotation Solution

## Implementation

```rust
pub fn rotate_vector<T>(mut vec: Vec<T>, n: isize) -> Vec<T> {
    if vec.is_empty() {
        return vec;
    }
    
    let len = vec.len() as isize;
    let normalized_n = ((n % len) + len) % len;
    let rotate_amount = normalized_n as usize;
    
    if rotate_amount == 0 {
        return vec;
    }
    
    // Positive n rotates right, so we rotate_left by (len - n)
    vec.rotate_left((vec.len() - rotate_amount) % vec.len());
    vec
}
```

## Explanation

This solution implements vector rotation by:

1. **Edge case handling**: Empty vectors and zero rotations are handled early
2. **Normalization**: The rotation amount is normalized to be within [0, len) using modular arithmetic
3. **Direction mapping**: Positive n (right rotation) is converted to left rotation amount
4. **Efficient rotation**: Uses Vec's built-in `rotate_left()` method for O(n) performance

## Mathematical Details

- **Modular arithmetic**: `((n % len) + len) % len` handles negative numbers correctly
- **Direction conversion**: Right rotation by `n` equals left rotation by `len - n`
- **Overflow protection**: All calculations are done with proper bounds checking

## Alternative Implementation (Manual)

```rust
pub fn rotate_vector<T>(mut vec: Vec<T>, n: isize) -> Vec<T> {
    if vec.is_empty() {
        return vec;
    }
    
    let len = vec.len() as isize;
    let normalized_n = ((n % len) + len) % len;
    
    if normalized_n == 0 {
        return vec;
    }
    
    // Split and rejoin for right rotation
    let split_point = (len - normalized_n) as usize;
    let mut result = vec.split_off(split_point);
    result.extend(vec);
    result
}
```

## Key Learning Points

- **Modular arithmetic**: Essential for handling negative rotations and large values
- **Vec rotation methods**: `rotate_left()` and `rotate_right()` are efficient built-in operations
- **Direction semantics**: Understanding the relationship between left and right rotations
- **Edge case handling**: Empty vectors and zero rotations need special consideration

## Rust Concepts Demonstrated

- Mutable vector operations
- Mathematical operations with type casting
- Efficient in-place algorithms
- Modular arithmetic for cyclic operations
- Vector splitting and joining techniques
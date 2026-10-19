# collections/rotate_vector

Reference implementation. Read after attempting the exercise.

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

    vec.rotate_right(rotate_amount);
    vec
}
```

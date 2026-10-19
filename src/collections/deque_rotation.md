# collections/deque_rotation

Reference implementation. Read after attempting the exercise.

```rust
pub fn rotate_deque<T>(
    mut deque: std::collections::VecDeque<T>,
    n: isize,
) -> std::collections::VecDeque<T> {
    if deque.len() == 0 {
        return deque;
    };
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

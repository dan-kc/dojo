# smart_pointers/try_append

Reference implementation. Read after attempting the exercise.

```rust
pub fn try_append<T>(values: &std::cell::RefCell<Vec<T>>, value: T) -> Result<(), T> {
    match values.try_borrow_mut() {
        Ok(mut values) => {
            values.push(value);
            Ok(())
        }
        Err(_) => Err(value),
    }
}
```

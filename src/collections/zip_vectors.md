# collections/zip_vectors

Reference implementation. Read after attempting the exercise.

```rust
pub fn zip_vectors<T>(vec1: Vec<T>, vec2: Vec<T>) -> Vec<T> {
    let mut result = Vec::with_capacity(vec1.len() + vec2.len());
    let mut iter1 = vec1.into_iter();
    let mut iter2 = vec2.into_iter();

    loop {
        match (iter1.next(), iter2.next()) {
            (Some(a), Some(b)) => {
                result.push(a);
                result.push(b);
            }
            (Some(a), None) => {
                result.push(a);
                result.extend(iter1);
                break;
            }
            (None, Some(b)) => {
                result.push(b);
                result.extend(iter2);
                break;
            }
            (None, None) => break,
        }
    }

    result
}
```

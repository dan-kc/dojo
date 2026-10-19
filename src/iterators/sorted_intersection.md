# iterators/sorted_intersection

Reference implementation. Read after attempting the exercise.

```rust
pub fn sorted_intersection(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let (mut i, mut j) = (0, 0);
    let mut out = Vec::new();
    while i < first.len() && j < second.len() {
        match first[i].cmp(&second[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                if out.last() != Some(&first[i]) {
                    out.push(first[i]);
                }
                i += 1;
                j += 1;
            }
        }
    }
    out
}
```

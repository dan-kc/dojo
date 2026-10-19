# collections/btree_custom_ordering

Reference implementation. Read after attempting the exercise.

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct LengthFirstString(String);

impl Ord for LengthFirstString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.len().cmp(&other.0.len()) {
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            other => other,
        }
    }
}

impl PartialOrd for LengthFirstString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn custom_ordered_set() -> std::collections::BTreeSet<LengthFirstString> {
    let mut set = std::collections::BTreeSet::new();

    set.insert(LengthFirstString("zebra".to_string()));
    set.insert(LengthFirstString("a".to_string()));
    set.insert(LengthFirstString("cat".to_string()));
    set.insert(LengthFirstString("dog".to_string()));
    set.insert(LengthFirstString("elephant".to_string()));
    set.insert(LengthFirstString("ab".to_string()));
    set.insert(LengthFirstString("aa".to_string()));

    set
}
```

# collections/btree_simple_index

Reference implementation. Read after attempting the exercise.

```rust
struct SimpleIndex {
    index: std::collections::BTreeMap<String, std::collections::BTreeSet<u32>>,
}

impl SimpleIndex {
    fn new() -> Self {
        Self {
            index: std::collections::BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: String, row_id: u32) {
        self.index
            .entry(key)
            .or_insert_with(|| std::collections::BTreeSet::new())
            .insert(row_id);
    }

    fn find_exact(&self, key: &str) -> Vec<u32> {
        self.index
            .get(key)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_else(Vec::new)
    }

    fn find_range(&self, start: &str, end: &str) -> Vec<u32> {
        if start > end {
            return Vec::new();
        }
        let mut result = std::collections::BTreeSet::new();

        for (_, row_ids) in self.index.range(start.to_string()..=end.to_string()) {
            result.extend(row_ids.iter().cloned());
        }

        result.into_iter().collect()
    }

    fn find_prefix(&self, prefix: &str) -> Vec<u32> {
        let mut result = std::collections::BTreeSet::new();

        let end = format!("{}{}", prefix, char::MAX);

        for (key, row_ids) in self.index.range(prefix.to_string()..) {
            if key.starts_with(prefix) {
                result.extend(row_ids.iter().cloned());
            } else {
                break; // No more matches possible
            }
        }

        result.into_iter().collect()
    }

    fn remove(&mut self, key: &str, row_id: u32) {
        if let Some(row_ids) = self.index.get_mut(key) {
            row_ids.remove(&row_id);

            if row_ids.is_empty() {
                self.index.remove(key);
            }
        }
    }
}
```

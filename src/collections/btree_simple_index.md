# BTree Simple Database Index Solution

## Implementation

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
        let mut result = std::collections::BTreeSet::new();
        
        for (_, row_ids) in self.index.range(start.to_string()..end.to_string()) {
            result.extend(row_ids.iter().cloned());
        }
        
        result.into_iter().collect()
    }

    fn find_prefix(&self, prefix: &str) -> Vec<u32> {
        let mut result = std::collections::BTreeSet::new();
        
        // Create end bound for prefix search
        let end = format!("{}{}", prefix, char::MAX);
        
        for (key, row_ids) in self.index.range(prefix.to_string()..=end) {
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
            
            // Remove key if no more row_ids
            if row_ids.is_empty() {
                self.index.remove(key);
            }
        }
    }
}
```

## Explanation

This solution implements a database-like index using nested BTree collections:

1. **Index structure**: BTreeMap maps keys to BTreeSet of row IDs
2. **Insertion**: Creates set if needed, adds row ID to set
3. **Exact search**: Direct lookup returning all associated row IDs
4. **Range queries**: Uses BTreeMap's range method for efficient filtering
5. **Prefix search**: Range query with prefix bounds and validation
6. **Deletion**: Removes row ID, cleans up empty keys

## Key Learning Points

- **Nested collections**: BTreeMap containing BTreeSets for multi-value indexing
- **Range queries**: Efficient ordered access to key ranges
- **Prefix bounds**: Using char::MAX to create upper bound for prefix
- **Entry API**: Efficient insert-or-update patterns
- **Cleanup logic**: Removing empty entries to maintain index integrity

## Rust Concepts Demonstrated

- BTreeMap and BTreeSet for ordered storage
- Range method with string bounds
- Entry API with or_insert_with
- Iterator extension and collection
- Option handling with map and unwrap_or_else
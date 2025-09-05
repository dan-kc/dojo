# BTree Map Splitting Solution

## Implementation

```rust
fn split_btree_map<K, V>(
    mut map: std::collections::BTreeMap<K, V>,
    split_key: &K,
) -> (std::collections::BTreeMap<K, V>, std::collections::BTreeMap<K, V>)
where
    K: Ord + Clone,
{
    // Split off the right part (keys >= split_key)
    let right = map.split_off(split_key);
    
    // The original map now contains keys < split_key
    let left = map;
    
    (left, right)
}
```

## Explanation

This solution efficiently splits a BTreeMap using the split_off method:

1. **split_off method**: Removes and returns all entries with keys >= split_key
2. **In-place modification**: Original map retains keys < split_key
3. **Ownership transfer**: Both parts are owned after split
4. **Efficient operation**: O(log n) time complexity
5. **No cloning needed**: Values are moved, not copied

## Key Learning Points

- **split_off method**: Built-in BTreeMap method for efficient partitioning
- **Ownership semantics**: Consuming original map, producing two owned maps
- **Mutation pattern**: Original map is modified and becomes left partition
- **Key comparison**: Split point determines partition boundary

## Rust Concepts Demonstrated

- BTreeMap split_off method
- Move semantics and ownership transfer
- Mutable binding for in-place operations
- Generic constraints for ordered keys
- Efficient tree splitting algorithms
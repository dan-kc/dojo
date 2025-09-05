# BTree Range Query Solution

## Implementation

```rust
fn range_query<K, V>(
    map: &std::collections::BTreeMap<K, V>,
    start: &K,
    end: &K,
) -> Vec<(K, V)>
where
    K: Ord + Clone,
    V: Clone,
{
    map.range(start..=end)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}
```

## Alternative Implementation (More Explicit Range)

```rust
fn range_query<K, V>(
    map: &std::collections::BTreeMap<K, V>,
    start: &K,
    end: &K,
) -> Vec<(K, V)>
where
    K: Ord + Clone,
    V: Clone,
{
    use std::ops::Bound::{Included, Unbounded};
    
    map.range((Included(start), Included(end)))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}
```

## Explanation

This solution implements efficient range queries using BTreeMap's ordered structure:

1. **Range method**: Uses `range(start..=end)` for inclusive range queries
2. **Ordered iteration**: BTreeMap maintains keys in sorted order
3. **Efficient lookup**: Range queries are O(log n + k) where k is result size
4. **Clone and collect**: Creates owned pairs from borrowed references

## Key Learning Points

- **BTreeMap range queries**: Leveraging ordered structure for efficient range operations
- **Inclusive ranges**: Using `..=` syntax for inclusive bounds
- **Iterator transformation**: Mapping borrowed references to owned values
- **Logarithmic complexity**: Range queries are much faster than linear scans

## Range Bound Options

```rust
use std::ops::Bound::{Included, Excluded, Unbounded};

// Inclusive on both ends: [start, end]
map.range((Included(start), Included(end)))

// Exclusive on both ends: (start, end)
map.range((Excluded(start), Excluded(end)))

// Mixed bounds: [start, end)
map.range((Included(start), Excluded(end)))

// Unbounded ranges: [start, ∞)
map.range((Included(start), Unbounded))
```

## Use Cases

- **Time series queries**: Finding records within date ranges
- **Numerical analysis**: Querying data within value ranges
- **Database operations**: Range scans over indexed data
- **Geospatial queries**: Finding points within coordinate ranges

## Performance Characteristics

- **Time**: O(log n + k) where n is map size, k is result size
- **Space**: O(k) for storing results
- **Memory**: Efficient iteration without loading entire map

## Rust Concepts Demonstrated

- BTreeMap range operations and ordered iteration
- Range syntax and bound specifications
- Iterator transformation and collection
- Owned vs borrowed data handling
- Logarithmic algorithm complexity
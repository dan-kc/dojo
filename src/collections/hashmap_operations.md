# HashMap Operations - Complete Solution

## Solutions

### 1. Character Frequency Counting

```rust
fn count_char_frequencies(text: &str) -> std::collections::HashMap<char, usize> {
    let mut frequencies = std::collections::HashMap::new();
    
    for ch in text.chars() {
        *frequencies.entry(ch).or_insert(0) += 1;
    }
    
    frequencies
}
```

### 2. HashMap Merging with Value Combination

```rust
fn merge_hashmaps<K, V, F>(
    maps: Vec<std::collections::HashMap<K, V>>,
    combine_fn: F,
) -> std::collections::HashMap<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
    F: Fn(V, V) -> V,
{
    let mut result = std::collections::HashMap::new();
    
    for map in maps {
        for (key, value) in map {
            result.entry(key.clone())
                .and_modify(|existing| *existing = combine_fn(existing.clone(), value.clone()))
                .or_insert(value);
        }
    }
    
    result
}
```

### 3. HashMap Key-Value Transformation

```rust
fn transform_hashmap<K1, V1, K2, V2, FK, FV, FC>(
    map: std::collections::HashMap<K1, V1>,
    key_fn: FK,
    value_fn: FV,
    combine_fn: FC,
) -> std::collections::HashMap<K2, V2>
where
    K2: std::hash::Hash + Eq,
    FK: Fn(K1) -> K2,
    FV: Fn(V1) -> V2,
    FC: Fn(V2, V2) -> V2,
{
    let mut result = std::collections::HashMap::new();
    
    for (key, value) in map {
        let new_key = key_fn(key);
        let new_value = value_fn(value);
        
        result.entry(new_key)
            .and_modify(|existing| *existing = combine_fn(existing.clone(), new_value.clone()))
            .or_insert(new_value);
    }
    
    result
}
```

### 4. Grouping Elements by Key Function

```rust
fn group_by<T, K, F>(items: Vec<T>, key_fn: F) -> std::collections::HashMap<K, Vec<T>>
where
    K: std::hash::Hash + Eq,
    F: Fn(&T) -> K,
{
    let mut groups = std::collections::HashMap::new();
    
    for item in items {
        let key = key_fn(&item);
        groups.entry(key).or_insert_with(Vec::new).push(item);
    }
    
    groups
}
```

### 5. Limited Cache Implementation

```rust
impl<K, V> LimitedCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
{
    fn new(capacity: usize) -> Self {
        Self {
            map: std::collections::HashMap::new(),
            insertion_order: Vec::new(),
            capacity,
        }
    }

    fn insert(&mut self, key: K, value: V) -> Option<V> {
        // Check if key already exists
        if let Some(existing_value) = self.map.insert(key.clone(), value) {
            // Key existed, return old value without changing order
            return Some(existing_value);
        }
        
        // New key inserted
        self.insertion_order.push(key.clone());
        
        // Check if we exceeded capacity
        if self.map.len() > self.capacity && self.capacity > 0 {
            // Remove oldest entry (FIFO)
            if let Some(oldest_key) = self.insertion_order.remove(0) {
                self.map.remove(&oldest_key);
            }
        } else if self.capacity == 0 {
            // Capacity 0 means no storage
            self.map.remove(&key);
            self.insertion_order.pop();
        }
        
        None
    }

    fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    fn remove(&mut self, key: &K) -> Option<V> {
        if let Some(value) = self.map.remove(key) {
            // Remove from insertion order
            self.insertion_order.retain(|k| k != key);
            Some(value)
        } else {
            None
        }
    }
}
```

### 6. Compound Key Operations

```rust
fn compound_key_operations() -> std::collections::HashMap<CompoundKey, String> {
    let mut map = std::collections::HashMap::new();
    
    // Create various compound keys to demonstrate usage
    let key1 = CompoundKey::new("user".to_string(), 1, vec![true, false]);
    let key2 = CompoundKey::new("admin".to_string(), 2, vec![false, true]);
    let key3 = CompoundKey::new("user".to_string(), 3, vec![true, true]);
    let key4 = CompoundKey::new("guest".to_string(), 1, vec![false, false]);
    
    map.insert(key1, "User account 1".to_string());
    map.insert(key2, "Admin account 2".to_string());
    map.insert(key3, "User account 3".to_string());
    map.insert(key4, "Guest account 1".to_string());
    
    // Demonstrate querying by compound keys
    let query_key = CompoundKey::new("user".to_string(), 1, vec![true, false]);
    if let Some(value) = map.get(&query_key) {
        println!("Found: {}", value);
    }
    
    map
}
```

### 7. HashMap Intersection

```rust
fn intersect_hashmaps<K, V>(
    map1: std::collections::HashMap<K, V>,
    map2: &std::collections::HashMap<K, V>,
) -> std::collections::HashMap<K, V>
where
    K: std::hash::Hash + Eq + Clone,
    V: Clone,
{
    let mut result = std::collections::HashMap::new();
    
    for (key, value) in map1 {
        if map2.contains_key(&key) {
            result.insert(key, value);
        }
    }
    
    result
}
```

### 8. Word Position Index

```rust
fn build_word_index(text: &str) -> std::collections::HashMap<String, std::collections::HashSet<usize>> {
    let mut index = std::collections::HashMap::new();
    
    for (position, word) in text.split_whitespace().enumerate() {
        index.entry(word.to_string())
            .or_insert_with(std::collections::HashSet::new)
            .insert(position);
    }
    
    index
}
```

### 9. Memoizer Implementation

```rust
impl<K, V> Memoizer<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    fn new() -> Self {
        Self {
            cache: std::cell::RefCell::new(std::collections::HashMap::new()),
        }
    }

    fn compute<F>(&self, key: K, compute_fn: F) -> V
    where
        F: FnOnce(&K) -> V,
    {
        let mut cache = self.cache.borrow_mut();
        
        if let Some(cached_value) = cache.get(&key) {
            cached_value.clone()
        } else {
            let computed_value = compute_fn(&key);
            cache.insert(key, computed_value.clone());
            computed_value
        }
    }

    fn clear_cache(&self) {
        self.cache.borrow_mut().clear();
    }

    fn cache_size(&self) -> usize {
        self.cache.borrow().len()
    }
}
```

### 10. Performance Comparison

```rust
fn performance_comparison(data_size: usize) -> (std::time::Duration, std::time::Duration) {
    use std::time::Instant;
    
    // Create test data
    let test_keys: Vec<String> = (0..data_size).map(|i| format!("key_{}", i)).collect();
    let search_keys: Vec<String> = (0..100).map(|i| format!("key_{}", i * data_size / 100)).collect();
    
    // HashMap performance test
    let mut hashmap = std::collections::HashMap::new();
    for (i, key) in test_keys.iter().enumerate() {
        hashmap.insert(key.clone(), i);
    }
    
    let hashmap_start = Instant::now();
    for key in &search_keys {
        let _ = hashmap.get(key);
    }
    let hashmap_time = hashmap_start.elapsed();
    
    // Vec performance test (linear search)
    let vec_data: Vec<(String, usize)> = test_keys.into_iter().enumerate().map(|(i, k)| (k, i)).collect();
    
    let vec_start = Instant::now();
    for key in &search_keys {
        let _ = vec_data.iter().find(|(k, _)| k == key);
    }
    let vec_time = vec_start.elapsed();
    
    (hashmap_time, vec_time)
}
```

## Detailed Explanation

### Key Concepts Demonstrated

#### 1. Entry API Mastery
The **Entry API** is the cornerstone of efficient HashMap operations:
- **`or_insert()`**: Insert if key doesn't exist
- **`and_modify()`**: Modify existing value if key exists  
- **`or_insert_with()`**: Insert with computed value if key doesn't exist
- **`or_default()`**: Insert default value if key doesn't exist

This API prevents double hash lookups that would occur with separate `contains_key()` and `insert()` operations.

#### 2. Generic Programming Patterns
The solutions demonstrate advanced generic programming:
- **Multiple type parameters**: Functions that transform between different key and value types
- **Closure parameters**: Flexible behavior through function parameters
- **Trait bounds**: Ensuring types have necessary capabilities (Hash, Eq, Clone)

#### 3. Interior Mutability with RefCell
The `Memoizer` demonstrates **interior mutability**:
- Allows mutation of HashMap inside an immutable context
- Uses `RefCell` for runtime borrow checking
- Enables caching within immutable function calls

#### 4. Complex Key Types
The `CompoundKey` example shows:
- Custom types as HashMap keys require `Hash + Eq + PartialEq` traits
- Derive macros automatically implement these traits
- Complex keys enable sophisticated data organization

#### 5. Collection Performance Characteristics

**HashMap advantages:**
- **O(1) average case** for insert, lookup, and remove operations
- **Constant time performance** regardless of collection size
- **Excellent for key-based access patterns**

**HashMap considerations:**
- **Memory overhead** for hash table structure
- **Hash computation cost** for complex key types
- **Potential hash collisions** in worst-case scenarios

### Advanced Patterns

#### 1. FIFO Cache Implementation
The `LimitedCache` demonstrates:
- **Capacity management**: Automatic eviction when full
- **Insertion order tracking**: Separate Vec to track order
- **FIFO eviction**: Remove oldest entries first

#### 2. Data Aggregation
Multiple functions show aggregation patterns:
- **Frequency counting**: Accumulating occurrences
- **Grouping**: Organizing items by computed keys
- **Merging**: Combining multiple data sources

#### 3. Type Transformation
The `transform_hashmap` function shows:
- **Type-level transformations**: Converting key and value types
- **Collision handling**: Resolving conflicts during transformation
- **Functional programming**: Using closures for transformation logic

### Performance Insights

#### Time Complexities
- **Character counting**: O(n) where n is string length
- **HashMap merging**: O(m) where m is total entries across all maps
- **Grouping**: O(n) where n is number of items
- **Intersection**: O(min(|map1|, |map2|))
- **Word indexing**: O(n) where n is number of words

#### Space Complexities
- Most operations: O(k) where k is number of unique keys
- **Cache**: O(min(capacity, total_insertions))
- **Memoizer**: O(unique_computations)

### Best Practices

1. **Use Entry API**: Prefer `entry()` over separate `contains_key()` and `insert()`
2. **Consider capacity**: Pre-allocate HashMap with `with_capacity()` for known sizes
3. **Handle panics**: Use `try_borrow()` with RefCell in production code
4. **Custom key types**: Ensure good hash distribution for performance
5. **Memory efficiency**: Consider `IndexMap` for insertion-order preservation with better memory usage than separate Vec tracking

### Common Pitfalls

1. **Double lookups**: Using `get()` followed by `insert()` instead of Entry API
2. **Unnecessary cloning**: Clone only when ownership transfer is needed
3. **RefCell panics**: Runtime borrow checker can panic on conflicting borrows
4. **Poor hash functions**: Custom key types with bad hash distribution hurt performance

This comprehensive solution demonstrates HashMap mastery through practical examples that cover the most important patterns and use cases in Rust programming.
# Compound Key Operations Solution

## Implementation

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompoundKey {
    category: String,
    id: u32,
    flags: Vec<bool>,
}

impl CompoundKey {
    pub fn new(category: String, id: u32, flags: Vec<bool>) -> Self {
        Self { category, id, flags }
    }
}

pub fn compound_key_operations() -> std::collections::HashMap<CompoundKey, String> {
    let mut map = std::collections::HashMap::new();
    
    // Create various compound keys to demonstrate usage
    let key1 = CompoundKey::new("category1".to_string(), 1, vec![true, false]);
    let key2 = CompoundKey::new("category1".to_string(), 2, vec![false, true]);
    let key3 = CompoundKey::new("category2".to_string(), 1, vec![true, false]);
    let key4 = CompoundKey::new("category1".to_string(), 1, vec![true, true]); // Different flags
    
    map.insert(key1, "First item in category1".to_string());
    map.insert(key2, "Second item in category1".to_string());
    map.insert(key3, "First item in category2".to_string());
    map.insert(key4, "Category1 with different flags".to_string());
    
    // Demonstrate compound key with complex flags
    let complex_key = CompoundKey::new(
        "analytics".to_string(), 
        42, 
        vec![true, false, true, false, true]
    );
    map.insert(complex_key, "Analytics data with complex flags".to_string());
    
    map
}
```

## Explanation

This solution demonstrates working with complex keys in HashMap:

1. **Derive traits**: `Hash`, `PartialEq`, `Eq` are required for HashMap keys
2. **Compound structure**: Multiple fields combine to form a unique key
3. **Hash implementation**: All fields contribute to the hash calculation
4. **Equality semantics**: Two keys are equal only if all fields match exactly

## Key Learning Points

- **Trait derivation**: `Hash`, `PartialEq`, and `Eq` must all be implemented for HashMap keys
- **Composite keys**: Multiple data fields can combine to create unique identifiers
- **Hash consistency**: The derived Hash implementation ensures consistent hashing
- **Memory considerations**: Vec<bool> in keys means heap allocation for each key

## Advanced Usage Patterns

```rust
// Querying by partial key information
pub fn find_by_category(
    map: &std::collections::HashMap<CompoundKey, String>,
    category: &str
) -> Vec<(CompoundKey, String)> {
    map.iter()
        .filter(|(key, _)| key.category == category)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

// Building keys programmatically
pub fn build_analytics_key(experiment_id: u32, feature_flags: &[bool]) -> CompoundKey {
    CompoundKey::new(
        "analytics".to_string(),
        experiment_id,
        feature_flags.to_vec()
    )
}
```

## Performance Considerations

- **Hash performance**: Vec<bool> hashing can be slower than primitive types
- **Memory usage**: Each key allocates heap memory for String and Vec
- **Equality checks**: All fields must be compared for equality

## Rust Concepts Demonstrated

- Custom types as HashMap keys with required trait derivations
- Composite data structures and their hash behavior
- String and Vec ownership in struct fields
- Pattern matching and querying with complex keys
- Generic collection operations with custom types
# Power Set Generation Solution

## Implementation

```rust
pub fn power_set<T>(
    set: std::collections::HashSet<T>,
) -> Vec<std::collections::HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    let elements: Vec<T> = set.into_iter().collect();
    let n = elements.len();
    let mut result = Vec::new();
    
    // Generate all 2^n subsets using bit manipulation
    for i in 0..(1 << n) {
        let mut subset = std::collections::HashSet::new();
        
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                subset.insert(elements[j].clone());
            }
        }
        
        result.push(subset);
    }
    
    result
}
```

## Recursive Implementation

```rust
pub fn power_set<T>(
    set: std::collections::HashSet<T>,
) -> Vec<std::collections::HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    let elements: Vec<T> = set.into_iter().collect();
    power_set_recursive(&elements)
}

fn power_set_recursive<T>(
    elements: &[T],
) -> Vec<std::collections::HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    if elements.is_empty() {
        return vec![std::collections::HashSet::new()];
    }
    
    let (first, rest) = elements.split_first().unwrap();
    let rest_power_set = power_set_recursive(rest);
    
    let mut result = rest_power_set.clone();
    
    for mut subset in rest_power_set {
        subset.insert(first.clone());
        result.push(subset);
    }
    
    result
}
```

## Explanation

This solution generates all possible subsets of a given set:

1. **Bit manipulation approach**: Uses binary representation where each bit indicates element inclusion
2. **Exponential generation**: Creates 2^n subsets for n elements
3. **Systematic enumeration**: Each number from 0 to 2^n-1 represents a unique subset
4. **HashSet construction**: Builds each subset as a HashSet for efficient operations

## Key Learning Points

- **Combinatorial generation**: Creating all possible combinations
- **Bit manipulation**: Using binary representation for subset selection
- **Exponential complexity**: Understanding 2^n growth rate
- **Recursive decomposition**: Alternative divide-and-conquer approach

## Algorithm Complexity

- **Time**: O(n × 2^n) - generating and populating 2^n subsets
- **Space**: O(2^n) - storing all subsets in result
- **Exponential growth**: Quickly becomes impractical for large sets

## Rust Concepts Demonstrated

- Bit manipulation techniques
- Vec and HashSet construction patterns
- Recursive algorithm implementation
- Iterator processing and collection
- Exponential algorithm complexity handling
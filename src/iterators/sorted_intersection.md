# Sorted Intersection

## Solution

```rust
pub fn sorted_intersection(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    
    let set: HashSet<i32> = second.into_iter().collect();
    
    let mut result: Vec<i32> = first.into_iter()
        .filter(|x| set.contains(x))
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    
    result.sort();
    result
}
```

## Explanation

This solution efficiently finds the intersection of two vectors:

1. **HashSet Creation**: Converts the second vector into a HashSet for O(1) lookups
2. **Filtering**: Keeps only elements from the first vector that exist in the set
3. **Deduplication**: Collects into a HashSet to remove duplicates
4. **Sorting**: Sorts the final result to ensure consistent ordering

Key concepts:
- **Time Complexity**: O(n + m) for building set and filtering, O(k log k) for sorting
- **Space Complexity**: O(n + k) where k is the size of intersection
- **Deduplication Strategy**: Using HashSet automatically removes duplicates
- **Efficiency**: HashSet lookup is much faster than nested loops

Alternative approaches:
```rust
// For already sorted vectors, can use two-pointer technique:
pub fn sorted_intersection_two_pointer(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut result = Vec::new();
    let mut last = None;
    
    while i < first.len() && j < second.len() {
        match first[i].cmp(&second[j]) {
            std::cmp::Ordering::Less => i += 1,
            std::cmp::Ordering::Greater => j += 1,
            std::cmp::Ordering::Equal => {
                if Some(first[i]) != last {
                    result.push(first[i]);
                    last = Some(first[i]);
                }
                i += 1;
                j += 1;
            }
        }
    }
    result
}
```

The HashSet approach is generally more robust as it handles unsorted input correctly.
# Format Adults

## Solution

```rust
pub fn format_adults(mut people: Vec<Person>) -> Vec<String> {
    people.sort_by(|a, b| b.age.cmp(&a.age));
    
    people.into_iter()
        .filter(|person| person.age > 18)
        .map(|person| format!("Name: {}, Age: {}", person.name, person.age))
        .collect()
}
```

## Explanation

This solution demonstrates working with custom structs and multiple transformations:

1. **In-place Sort**: Sorts the vector by age in descending order before processing
2. **Filter**: Selects only people over 18 years old
3. **Map**: Formats each person into the required string format
4. **Collect**: Gathers results into the final vector

Key concepts:
- **Mutable Operations**: Sorting requires a mutable vector
- **Sort Stability**: Rust's sort is stable, preserving original order for equal elements
- **Ownership**: `into_iter()` consumes the sorted vector
- **Format Macro**: Creates formatted strings efficiently

Alternative approaches:
```rust
// Without mutating the original:
pub fn format_adults(people: Vec<Person>) -> Vec<String> {
    let mut adults: Vec<_> = people.into_iter()
        .filter(|p| p.age > 18)
        .collect();
    adults.sort_by(|a, b| b.age.cmp(&a.age));
    adults.into_iter()
        .map(|p| format!("Name: {}, Age: {}", p.name, p.age))
        .collect()
}
```

The solution demonstrates:
- Efficient data transformation pipelines
- Working with structured data
- Combining imperative (sort) and functional (filter/map) styles
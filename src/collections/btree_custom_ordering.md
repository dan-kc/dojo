# BTree Custom Ordering Solution

## Implementation

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
struct LengthFirstString(String);

impl Ord for LengthFirstString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare by length
        match self.0.len().cmp(&other.0.len()) {
            std::cmp::Ordering::Equal => {
                // If lengths are equal, compare lexicographically
                self.0.cmp(&other.0)
            }
            other => other
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
    
    // Add various strings with different lengths
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

## Explanation

This solution implements custom ordering for BTreeSet elements:

1. **Wrapper type**: LengthFirstString wraps String with custom ordering
2. **Two-level comparison**: First by length, then lexicographically
3. **Ord implementation**: Defines complete ordering for the type
4. **PartialOrd delegation**: Simply delegates to Ord implementation
5. **BTreeSet usage**: Automatically maintains elements in custom order

## Key Learning Points

- **Custom ordering**: Implementing Ord trait for custom sort logic
- **Wrapper pattern**: Using newtype wrapper to add behavior to existing types
- **Multi-criteria sorting**: Combining multiple comparison criteria
- **BTreeSet invariants**: Elements automatically sorted by Ord implementation

## Rust Concepts Demonstrated

- Ord and PartialOrd trait implementations
- Newtype pattern for custom behavior
- Pattern matching on Ordering enum
- BTreeSet with custom ordered types
- Method chaining for comparison logic
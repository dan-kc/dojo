# Find First N Numbers Divisible by Three

## Solution

```rust
pub fn first_five_divisible_by_three(start: i32, end: i32) -> Vec<i32> {
    (start..end)
        .filter(|&x| x % 3 == 0)
        .take(5)
        .collect()
}
```

## Explanation

This solution demonstrates **range iteration** with **lazy evaluation** and **result limiting**:

### Key Concepts Demonstrated:

1. **Range Creation**:
   - `(start..end)` creates a `Range<i32>` iterator
   - Represents integers from `start` (inclusive) to `end` (exclusive)
   - Lazy - numbers are generated on demand, not stored in memory

2. **Filtering with Mathematical Predicates**:
   - `filter(|&x| x % 3 == 0)` tests divisibility by 3
   - Pattern matching `|&x|` destructures the reference to get the value
   - Only values where remainder is 0 pass through the filter

3. **Result Limitation**:
   - `take(5)` limits the output to at most 5 elements
   - Short-circuits iteration once 5 elements are found
   - Handles cases where fewer than 5 numbers exist in the range

4. **Lazy Evaluation Benefits**:
   - If 5 divisible numbers are found early, iteration stops immediately
   - No unnecessary computation of remaining range values
   - Memory efficient - no intermediate collections

### Iterator Chain Analysis:

```rust
(start..end)                    // Range<i32>
  .filter(|&x| x % 3 == 0)     // Filter<Range<i32>, Closure>
  .take(5)                     // Take<Filter<Range<i32>, Closure>>
  .collect()                   // Vec<i32>
```

### How Lazy Evaluation Works:

```rust
// Example: first_five_divisible_by_three(1, 20)
// Iterator produces: 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19
// Filter keeps:                 3,       6,       9,        12,        15
// Take(5) stops here: [3, 6, 9, 12, 15] - never evaluates 16, 17, 18, 19
```

### Edge Cases Handled:

1. **Empty Range**: `(10, 10)` or `(10, 5)` produces empty iterator
2. **No Matches**: Range with no numbers divisible by 3 returns empty vector  
3. **Fewer Than 5 Matches**: Returns whatever matches exist
4. **Negative Numbers**: Works correctly with negative ranges

### Range Variants:

```rust
// Inclusive range (includes end):
(start..=end).filter(|&x| x % 3 == 0).take(5).collect()

// Different range types:
std::ops::Range { start, end }           // start..end
std::ops::RangeInclusive::new(start, end) // start..=end
std::ops::RangeFrom { start }            // start.. (infinite)
std::ops::RangeTo { end }                // ..end
std::ops::RangeFull                      // ..
```

### Performance Characteristics:

- **Time Complexity**: O(min(n, 5k)) where n is range size, k is avg distance between multiples
- **Space Complexity**: O(1) during iteration, O(min(5, matches)) for result
- **Early Termination**: Stops as soon as 5 matches are found
- **No Allocation**: Until `collect()` is called

### Alternative Implementations:

```rust
// Using while loop (imperative style):
pub fn first_five_divisible_by_three_manual(start: i32, end: i32) -> Vec<i32> {
    let mut result = Vec::new();
    let mut current = start;
    
    while current < end && result.len() < 5 {
        if current % 3 == 0 {
            result.push(current);
        }
        current += 1;
    }
    result
}

// Using step_by for more efficient scanning:
pub fn first_five_divisible_by_three_optimized(start: i32, end: i32) -> Vec<i32> {
    let first_multiple = if start % 3 == 0 {
        start
    } else {
        start + (3 - start % 3)
    };
    
    (first_multiple..end)
        .step_by(3)
        .take(5)
        .collect()
}
```

### Mathematical Efficiency:

The optimized version calculates the first multiple and then steps by 3:
- Reduces iterations from `O(n)` to `O(n/3)`
- More efficient for large ranges
- Same correctness guarantees

### Handling Negative Numbers:

```rust
// Works correctly with negative ranges:
first_five_divisible_by_three(-15, 0)  // [-15, -12, -9, -6, -3]

// Note: Rust's % operator preserves sign of left operand:
assert_eq!(-9 % 3, 0);   // true
assert_eq!(-8 % 3, -2);  // not 1
```

### Iterator Composition:

```rust
// Can be easily extended:
(start..end)
    .filter(|&x| x % 3 == 0)
    .filter(|&x| x > 0)        // Additional filtering
    .map(|x| x * x)            // Transformation  
    .take(5)
    .collect()
```

### Real-World Applications:

- **Pagination**: Getting first N results from a filtered dataset
- **Batch Processing**: Processing limited chunks of data
- **Resource Limiting**: Preventing unbounded computation
- **Sampling**: Taking representative samples from large datasets

### Zero-Cost Abstraction:

The iterator chain compiles to efficient code equivalent to:
```rust
let mut result = Vec::new();
for i in start..end {
    if i % 3 == 0 {
        result.push(i);
        if result.len() == 5 {
            break;
        }
    }
}
result
```

This example demonstrates how Rust's iterator system provides both expressiveness and performance, allowing complex data processing pipelines to be built from simple, composable operations.
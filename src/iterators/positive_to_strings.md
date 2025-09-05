# Convert Positive Numbers to Strings

## Solution

```rust
pub fn positive_numbers_to_strings(numbers: Vec<i32>) -> Vec<String> {
    numbers
        .into_iter()
        .filter(|&x| x > 0)
        .map(|x| x.to_string())
        .collect()
}
```

## Explanation

This solution demonstrates **chaining filter() and map() operations** for transformation pipelines:

### Key Concepts Demonstrated:

1. **Iterator Method Chaining**:
   - Combines filtering and transformation in a single, readable pipeline  
   - Each method returns a new iterator type, enabling composition
   - Functional programming style that's both expressive and efficient

2. **Predicate Filtering**:
   - `filter(|&x| x > 0)` removes zero and negative numbers
   - Pattern matching `|&x|` destructures `&i32` to `i32` for comparison
   - Only positive numbers (> 0) continue through the pipeline

3. **Type Transformation**:
   - `map(|x| x.to_string())` converts each `i32` to `String`
   - No pattern matching needed here since `into_iter()` produces owned values
   - Creates owned string data suitable for the return type

4. **Ownership Through the Pipeline**:
   - `into_iter()` takes ownership of the vector and its elements
   - Each stage passes owned values to the next
   - No unnecessary borrowing or cloning

### Iterator Type Evolution:

```rust
Vec<i32>                        // Input vector
  .into_iter()               → IntoIter<i32>      (owns i32 values)
  .filter(|&x| x > 0)        → Filter<IntoIter<i32>, Closure>
  .map(|x| x.to_string())    → Map<Filter<..., Closure>, Closure> 
  .collect()                 → Vec<String>        (final result)
```

### Pattern Matching vs Direct Access:

```rust
// Our approach (pattern matching in filter):
.filter(|&x| x > 0)        // Destructure &i32 to i32
.map(|x| x.to_string())    // x is already i32

// Alternative (dereferencing in filter):  
.filter(|x| *x > 0)        // Explicit dereference
.map(|x| x.to_string())    // x is i32

// If using iter() instead of into_iter():
.filter(|&&x| x > 0)       // Destructure &&i32 to i32
.map(|&x| x.to_string())   // Destructure &i32 to i32
```

### Why This Pipeline Is Efficient:

1. **Single Pass**: Data flows through all transformations in one iteration
2. **Lazy Evaluation**: No work done until `collect()` is called
3. **No Intermediate Collections**: Filter and map don't allocate temporary vectors
4. **Optimal Memory Usage**: Only the final result vector is allocated

### Handling Different Number Types:

```rust
// The pattern works with any Display type:
pub fn positive_floats_to_strings(numbers: Vec<f64>) -> Vec<String> {
    numbers
        .into_iter()
        .filter(|&x| x > 0.0)
        .map(|x| x.to_string())
        .collect()
}

// Custom formatting:
pub fn positive_numbers_formatted(numbers: Vec<i32>) -> Vec<String> {
    numbers
        .into_iter()
        .filter(|&x| x > 0)
        .map(|x| format!("Number: {}", x))
        .collect()
}
```

### Zero and Negative Number Handling:

```rust
// Test cases covered:
assert_eq!(positive_numbers_to_strings(vec![]), vec![] as Vec<String>);
assert_eq!(positive_numbers_to_strings(vec![-5, -1, 0]), vec![] as Vec<String>);
assert_eq!(positive_numbers_to_strings(vec![1, 2, 3]), vec!["1", "2", "3"]);
assert_eq!(positive_numbers_to_strings(vec![-2, -1, 0, 1, 2]), vec!["1", "2"]);
```

### Alternative Implementations:

```rust
// Using filter_map for combined filter + transform:
pub fn positive_numbers_to_strings_v2(numbers: Vec<i32>) -> Vec<String> {
    numbers
        .into_iter()
        .filter_map(|x| if x > 0 { Some(x.to_string()) } else { None })
        .collect()
}

// Using retain + map (if input was mutable):
pub fn positive_numbers_to_strings_v3(mut numbers: Vec<i32>) -> Vec<String> {
    numbers.retain(|&x| x > 0);
    numbers.into_iter().map(|x| x.to_string()).collect()
}

// Manual approach for comparison:
pub fn positive_numbers_to_strings_manual(numbers: Vec<i32>) -> Vec<String> {
    let mut result = Vec::new();
    for number in numbers {
        if number > 0 {
            result.push(number.to_string());
        }
    }
    result
}
```

### Performance Considerations:

- **String Allocation**: Each `to_string()` call allocates memory
- **Vector Growth**: Result vector may need to resize during collection
- **CPU Usage**: Number-to-string conversion has computational cost
- **Memory Efficiency**: Input vector is consumed, no extra copies

### Optimization Opportunities:

```rust
// Pre-allocate result capacity if input size is known:
pub fn positive_numbers_to_strings_optimized(numbers: Vec<i32>) -> Vec<String> {
    let positive_count = numbers.iter().filter(|&&x| x > 0).count();
    let mut result = Vec::with_capacity(positive_count);
    
    numbers.into_iter()
        .filter(|&x| x > 0)
        .map(|x| x.to_string())
        .collect_into(&mut result);  // hypothetical method
    
    result
}
```

### Real-World Applications:

- **Data Validation**: Converting valid numeric inputs to display strings
- **Log Processing**: Formatting positive metrics for reporting
- **User Interfaces**: Displaying filtered numeric data
- **APIs**: Converting internal numeric IDs to string responses

### Memory Layout:

```
Input:  Vec<i32>    [heap] → [-2, -1, 0, 1, 2, 3]
                                      ↓ (filter & map)
Output: Vec<String> [heap] → [String("1"), String("2"), String("3")]
                             [ptr,len,cap] [ptr,len,cap] [ptr,len,cap]
                             ↓             ↓             ↓
                             "1"          "2"          "3" (heap allocated strings)
```

### Type Safety Benefits:

The pipeline ensures:
- **Compile-time checks**: Type mismatches caught at compile time
- **No runtime errors**: No panics from invalid conversions
- **Clear intent**: Code clearly expresses the transformation logic

This example showcases how iterator chaining creates readable, efficient data transformation pipelines while maintaining Rust's safety guarantees and zero-cost abstraction principles.
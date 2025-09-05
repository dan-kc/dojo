# Sum Even Numbers Using Iterators

## Solution

```rust
pub fn sum_evens(numbers: Vec<i32>) -> i32 {
    numbers
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .sum()
}
```

## Explanation

This solution demonstrates **iterator chaining** and **functional programming patterns** in Rust:

### Key Concepts Demonstrated:

1. **Iterator Creation**:
   - `into_iter()` creates an owning iterator that consumes the vector
   - Takes ownership of each element, avoiding unnecessary borrowing
   - More efficient than `iter()` when the original vector isn't needed

2. **Filtering with Predicates**:
   - `filter()` applies a predicate function to each element
   - `|&x| x % 2 == 0` uses pattern matching to destructure the reference
   - Only elements that return `true` are passed to the next stage

3. **Aggregation**:
   - `sum()` is a consuming iterator adapter that reduces all elements
   - Returns `0` for empty iterators (identity element for addition)
   - Works with any type that implements the `Sum` trait

4. **Method Chaining**:
   - Iterator methods are chained together for readable, functional-style code
   - Each method transforms the iterator without allocating intermediate collections
   - Lazy evaluation means no work is done until `sum()` is called

### Why This Approach Is Efficient:

```rust
// Functional approach (our solution):
numbers.into_iter().filter(|&x| x % 2 == 0).sum()

// Equivalent imperative approach:
let mut total = 0;
for number in numbers {
    if number % 2 == 0 {
        total += number;
    }
}
total
```

Both approaches have similar performance, but the functional approach is:
- **More readable**: Intent is clear from the method names
- **Less error-prone**: No manual loop management or mutable variables
- **Composable**: Easy to add additional transformations

### Iterator Adapter Chain:

1. `Vec<i32>` → `into_iter()` → `IntoIter<i32>`
2. `IntoIter<i32>` → `filter(...)` → `Filter<IntoIter<i32>, Closure>`
3. `Filter<...>` → `sum()` → `i32`

### Pattern Matching in Closures:

```rust
.filter(|&x| x % 2 == 0)
//      ^^^ pattern matching destructures &i32 to i32

// Alternative syntaxes:
.filter(|x| *x % 2 == 0)   // explicit dereferencing
.filter(|x| x % 2 == 0)    // if using iter() instead of into_iter()
```

### Zero-Cost Abstractions:

Rust's iterators are **zero-cost abstractions**:
- The functional code compiles to the same assembly as a manual loop
- No heap allocations for intermediate results
- Compiler optimizes the iterator chain into efficient machine code

### Handling Edge Cases:

- **Empty vector**: `sum()` returns `0` (correct behavior)
- **No even numbers**: Filter produces empty iterator, `sum()` returns `0`
- **Negative numbers**: Modulo works correctly with negatives in Rust
- **Large vectors**: Streaming processing, no memory overhead

### Alternative Implementations:

```rust
// Using fold instead of sum:
numbers.into_iter()
    .filter(|&x| x % 2 == 0)
    .fold(0, |acc, x| acc + x)

// Using reduce (Rust 1.51+):
numbers.into_iter()
    .filter(|&x| x % 2 == 0)
    .reduce(|acc, x| acc + x)
    .unwrap_or(0)

// Using for_each with external accumulator:
let mut sum = 0;
numbers.into_iter()
    .filter(|&x| x % 2 == 0)
    .for_each(|x| sum += x);
```

### Performance Characteristics:

- **Time Complexity**: O(n) - single pass through the vector
- **Space Complexity**: O(1) - constant space, no intermediate collections
- **Memory Usage**: Moves elements from vector (no copying)

This example showcases how Rust's iterator system provides both high performance and excellent readability, making functional programming patterns natural and efficient.
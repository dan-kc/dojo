# Iterator Performance - Solution

## Solution

```rust
pub fn sum_squares_evens_iterator(numbers: &[i32]) -> i64 {
    numbers
        .iter()
        .filter(|&&n| n % 2 == 0)
        .map(|&n| (n as i64) * (n as i64))
        .sum()
}

pub fn first_n_primes(n: usize) -> Vec<u32> {
    (2u32..)
        .filter(|&num| is_prime(num))
        .take(n)
        .collect()
}

pub fn top_k_frequent_words(text: &str, k: usize) -> Vec<String> {
    use std::collections::HashMap;
    
    let mut word_counts: HashMap<&str, usize> = std::collections::HashMap::new();
    
    text.split_whitespace()
        .for_each(|word| {
            *word_counts.entry(word).or_insert(0) += 1;
        });
    
    let mut word_freq: Vec<_> = word_counts.into_iter().collect();
    word_freq.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(b.0)));
    
    word_freq
        .into_iter()
        .take(k)
        .map(|(word, _)| word.to_string())
        .collect()
}

pub fn chunk_averages(numbers: &[f64], chunk_size: usize) -> Vec<f64> {
    numbers
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().sum::<f64>() / chunk.len() as f64)
        .collect()
}

pub fn dot_product(a: &[f64], b: &[f64]) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(&x, &y)| x * y)
        .sum()
}

pub fn elements_above_average(data: &[Vec<i32>]) -> Vec<i32> {
    data.iter()
        .flat_map(|inner_vec| {
            let average = inner_vec.iter().sum::<i32>() as f64 / inner_vec.len() as f64;
            inner_vec
                .iter()
                .filter(move |&&x| x as f64 > average)
                .cloned()
        })
        .collect()
}
```

## Explanation

### Performance Optimization Patterns

**Iterator Chains vs Loops:**
- Iterator chains like `filter().map().sum()` are zero-cost abstractions
- Compiler can optimize iterator chains better than manual loops
- Iterator methods convey intent more clearly than index manipulation

**Lazy Evaluation Benefits:**
```rust
(2u32..)                    // Infinite iterator - no memory allocation
    .filter(|&num| is_prime(num))  // Only checks primality when needed
    .take(n)                // Stops after n elements
    .collect()              // Only then allocates memory for n elements
```

**Memory Efficient Processing:**
- `chunks()` processes data in fixed-size windows without additional allocation
- `flat_map()` flattens nested structures without intermediate collections
- `zip()` pairs elements lazily without creating tuple collections

### Advanced Iterator Techniques

**Method Chaining Strategies:**
1. **Filter Early:** Apply filters before expensive transformations
2. **Use Specialized Methods:** `sum()` is more efficient than `fold(0, |acc, x| acc + x)`
3. **Minimize Allocations:** Use `for_each()` instead of `collect()` when possible

**Efficient Data Structures:**
```rust
// HashMap for O(1) lookups in word frequency counting
let mut word_counts: HashMap<&str, usize> = HashMap::new();

// Vec for sorting by frequency
let mut word_freq: Vec<_> = word_counts.into_iter().collect();
```

**Parallel-Style Operations:**
- `zip()` enables vectorized operations like dot product
- Processing corresponding elements without index arithmetic
- Compiler can optimize these patterns effectively

### Performance Considerations

**Memory Access Patterns:**
- Sequential access through iterators is cache-friendly
- `chunks()` maintains spatial locality
- Avoiding random access improves performance

**Computational Efficiency:**
- Iterator fusion eliminates intermediate collections
- Lazy evaluation prevents unnecessary computation
- Specialized iterator methods (like `sum()`) use optimized implementations

**Avoiding Common Pitfalls:**
1. **Don't collect unnecessarily** - use iterator methods that consume directly
2. **Consider `iter()` vs `into_iter()`** - borrowing is often sufficient
3. **Use appropriate numeric types** - avoid unnecessary conversions
4. **Leverage iterator adaptors** - they're designed for composition

**Benchmarking Insights:**
The iterator version often outperforms manual loops due to:
- Better compiler optimizations
- Elimination of bounds checking in some cases
- Vectorization opportunities
- Reduced branching and improved predictability
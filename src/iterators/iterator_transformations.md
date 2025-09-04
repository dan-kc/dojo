# Iterator Transformations - Solution

## Solution

```rust
pub fn zip_vectors<T, U>(first: Vec<T>, second: Vec<U>) -> Vec<(T, U)>
where
    T: Clone,
    U: Clone,
{
    first
        .into_iter()
        .zip(second.into_iter())
        .collect()
}

pub fn enumerate_long_words(words: Vec<String>) -> Vec<(usize, String, usize)> {
    words
        .into_iter()
        .skip(2)
        .enumerate()
        .filter(|(_, word)| word.len() > 4)
        .map(|(index, word)| {
            let char_count = word.len();
            (index, word, char_count)
        })
        .collect()
}

pub fn format_adults(mut people: Vec<Person>) -> Vec<String> {
    people.sort_by(|a, b| b.age.cmp(&a.age)); // Sort by age descending
    people
        .into_iter()
        .filter(|person| person.age > 18)
        .map(|person| format!("Name: {}, Age: {}", person.name, person.age))
        .collect()
}

pub fn sliding_window_sums(numbers: Vec<i32>) -> Vec<i32> {
    numbers
        .windows(3)
        .map(|window| window.iter().sum())
        .collect()
}

pub fn sorted_intersection(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    use std::collections::HashSet;
    
    let first_set: HashSet<i32> = first.into_iter().collect();
    let mut intersection: Vec<i32> = second
        .into_iter()
        .filter(|item| first_set.contains(item))
        .collect();
    
    intersection.sort();
    intersection.dedup();
    intersection
}
```

## Explanation

### Advanced Iterator Methods

**zip():**
- Combines two iterators element-wise into tuples
- Stops when the shorter iterator is exhausted
- Perfect for pairing related data from different sources

**enumerate():**
- Adds index information to iterator elements
- Returns `(index, item)` tuples
- Index starts from 0 and increments for each element

**skip():**
- Skips the first N elements of an iterator
- Useful for pagination or removing headers/metadata
- Combining with enumerate() affects the starting index

**windows():**
- Creates overlapping sub-slices of specified size
- Only available on slices, not general iterators
- Returns an iterator over `&[T]` windows
- Perfect for sliding window algorithms

### Complex Transformations

**Multi-step Pipelines:**
The `enumerate_long_words` function demonstrates chaining multiple operations:
1. `skip(2)` - removes first 2 elements
2. `enumerate()` - adds indices starting from 0
3. `filter()` - keeps only long words
4. `map()` - transforms to desired tuple format

**Sorting and Filtering:**
In `format_adults`, we sort first, then filter and map. Note that sorting requires mutable access or collecting into a new vector.

**Window Processing:**
`sliding_window_sums` shows how to process overlapping subsequences efficiently using `windows()`.

### Performance Considerations

**Iterator vs Vec Operations:**
- Use iterators for chained transformations
- Collect only when necessary (end of pipeline)
- `windows()` provides efficient sliding window without allocation

**Memory Efficiency:**
- `zip()` processes elements lazily
- `HashSet` for intersection provides O(1) lookup
- `dedup()` requires sorted data but is efficient for removing duplicates

**Trade-offs:**
- Sorting before filtering vs after: depends on data size and filter selectivity
- Using `HashSet` for intersection trades memory for time complexity
- Iterator chains are zero-cost abstractions when optimized
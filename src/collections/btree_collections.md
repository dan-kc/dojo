# BTree Collections Solutions

## Solutions

```rust
use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};
use std::time::{Duration, Instant};

fn range_query<K, V>(
    map: &BTreeMap<K, V>,
    start: &K,
    end: &K,
) -> Vec<(K, V)>
where
    K: Ord + Clone,
    V: Clone,
{
    map.range(start..=end)
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}

fn split_btree_map<K, V>(
    mut map: BTreeMap<K, V>,
    split_key: &K,
) -> (BTreeMap<K, V>, BTreeMap<K, V>)
where
    K: Ord + Clone,
{
    let right = map.split_off(split_key);
    (map, right)
}

fn find_k_extremes<T>(
    set: &BTreeSet<T>,
    k: usize,
) -> (Vec<T>, Vec<T>)
where
    T: Ord + Clone,
{
    let smallest: Vec<T> = set.iter().take(k).cloned().collect();
    let largest: Vec<T> = set.iter().rev().take(k).cloned().collect();
    let largest: Vec<T> = largest.into_iter().rev().collect(); // Restore ascending order
    
    (smallest, largest)
}

struct SlidingWindowTracker {
    window: BTreeMap<i32, usize>,
    k: usize,
    current_window: VecDeque<i32>,
}

impl SlidingWindowTracker {
    fn new(k: usize) -> Self {
        Self {
            window: BTreeMap::new(),
            k,
            current_window: VecDeque::new(),
        }
    }

    fn add(&mut self, value: i32) {
        // Add new value
        *self.window.entry(value).or_insert(0) += 1;
        self.current_window.push_back(value);

        // Remove old value if window exceeds size k
        if self.current_window.len() > self.k {
            let old_value = self.current_window.pop_front().unwrap();
            let count = self.window.get_mut(&old_value).unwrap();
            *count -= 1;
            if *count == 0 {
                self.window.remove(&old_value);
            }
        }
    }

    fn get_min(&self) -> Option<i32> {
        self.window.keys().next().copied()
    }

    fn get_max(&self) -> Option<i32> {
        self.window.keys().next_back().copied()
    }

    fn window_size(&self) -> usize {
        self.current_window.len()
    }
}

fn merge_sorted_btreemaps<K, V, F>(
    maps: Vec<BTreeMap<K, V>>,
    combine_fn: F,
) -> BTreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
    F: Fn(V, V) -> V,
{
    let mut result = BTreeMap::new();
    
    for map in maps {
        for (key, value) in map {
            result.entry(key.clone())
                .and_modify(|existing| *existing = combine_fn(existing.clone(), value.clone()))
                .or_insert(value);
        }
    }
    
    result
}

fn ordered_set_operations<T>(
    set_a: &BTreeSet<T>,
    set_b: &BTreeSet<T>,
) -> (BTreeSet<T>, BTreeSet<T>, BTreeSet<T>)
where
    T: Ord + Clone,
{
    let union: BTreeSet<T> = set_a.union(set_b).cloned().collect();
    let intersection: BTreeSet<T> = set_a.intersection(set_b).cloned().collect();
    let difference: BTreeSet<T> = set_a.difference(set_b).cloned().collect();
    
    (union, intersection, difference)
}

fn find_consecutive_ranges<T>(
    set: &BTreeSet<T>,
) -> Vec<(T, T)>
where
    T: Ord + Clone + std::ops::Add<Output = T> + From<u8> + PartialEq,
{
    let mut ranges = Vec::new();
    let values: Vec<_> = set.iter().cloned().collect();
    
    if values.is_empty() {
        return ranges;
    }
    
    let mut start = values[0].clone();
    let mut end = values[0].clone();
    let one = T::from(1);
    
    for i in 1..values.len() {
        if values[i] == end.clone() + one.clone() {
            end = values[i].clone();
        } else {
            ranges.push((start.clone(), end.clone()));
            start = values[i].clone();
            end = values[i].clone();
        }
    }
    
    ranges.push((start, end));
    ranges
}

#[derive(Debug, Clone, PartialEq)]
struct Event {
    id: u32,
    description: String,
    duration: u32,
}

struct Timeline {
    events: BTreeMap<u64, Vec<Event>>,
}

impl Timeline {
    fn new() -> Self {
        Self {
            events: BTreeMap::new(),
        }
    }

    fn add_event(&mut self, timestamp: u64, event: Event) {
        self.events.entry(timestamp).or_insert_with(Vec::new).push(event);
    }

    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<(u64, &Event)> {
        self.events
            .range(start..=end)
            .flat_map(|(timestamp, events)| {
                events.iter().map(move |event| (*timestamp, event))
            })
            .collect()
    }

    fn get_next_event(&self, after: u64) -> Option<(u64, &Event)> {
        self.events
            .range((after + 1)..)
            .flat_map(|(timestamp, events)| {
                events.iter().map(move |event| (*timestamp, event))
            })
            .next()
    }

    fn remove_events_before(&mut self, timestamp: u64) {
        let keys_to_remove: Vec<_> = self.events
            .range(..timestamp)
            .map(|(k, _)| *k)
            .collect();
        
        for key in keys_to_remove {
            self.events.remove(&key);
        }
    }

    fn get_overlapping_events(&self, timestamp: u64, duration: u32) -> Vec<(u64, &Event)> {
        let end_time = timestamp + duration as u64;
        
        self.events
            .range(..)
            .flat_map(|(event_time, events)| {
                events.iter().filter_map(move |event| {
                    let event_end = event_time + event.duration as u64;
                    if *event_time <= end_time && event_end >= timestamp {
                        Some((*event_time, event))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
}

fn performance_comparison(size: usize) -> (
    Duration, Duration, Duration, Duration, Duration, Duration,
) {
    let mut btree_map = BTreeMap::new();
    let mut hash_map = HashMap::new();
    
    // BTreeMap insertion
    let start = Instant::now();
    for i in 0..size {
        btree_map.insert(i, i * 2);
    }
    let btree_insert = start.elapsed();
    
    // HashMap insertion
    let start = Instant::now();
    for i in 0..size {
        hash_map.insert(i, i * 2);
    }
    let hash_insert = start.elapsed();
    
    // BTreeMap lookup
    let start = Instant::now();
    for i in 0..size {
        btree_map.get(&i);
    }
    let btree_lookup = start.elapsed();
    
    // HashMap lookup
    let start = Instant::now();
    for i in 0..size {
        hash_map.get(&i);
    }
    let hash_lookup = start.elapsed();
    
    // BTreeMap ordered iteration
    let start = Instant::now();
    for (_, _) in &btree_map {}
    let btree_iter = start.elapsed();
    
    // HashMap unordered iteration
    let start = Instant::now();
    for (_, _) in &hash_map {}
    let hash_iter = start.elapsed();
    
    (btree_insert, hash_insert, btree_lookup, hash_lookup, btree_iter, hash_iter)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct LengthFirstString(String);

impl Ord for LengthFirstString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.0.len().cmp(&other.0.len()) {
            std::cmp::Ordering::Equal => self.0.cmp(&other.0),
            other => other,
        }
    }
}

impl PartialOrd for LengthFirstString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn custom_ordered_set() -> BTreeSet<LengthFirstString> {
    let mut set = BTreeSet::new();
    set.insert(LengthFirstString("zoo".to_string()));
    set.insert(LengthFirstString("a".to_string()));
    set.insert(LengthFirstString("apple".to_string()));
    set.insert(LengthFirstString("an".to_string()));
    set.insert(LengthFirstString("at".to_string()));
    set
}

fn find_predecessor_successor<T>(
    set: &BTreeSet<T>,
    target: &T,
) -> (Option<T>, Option<T>)
where
    T: Ord + Clone,
{
    let predecessor = set.range(..target).next_back().cloned();
    let successor = set.range((std::ops::Bound::Excluded(target), std::ops::Bound::Unbounded))
        .next().cloned();
    
    (predecessor, successor)
}

struct SimpleIndex {
    index: BTreeMap<String, BTreeSet<u32>>,
}

impl SimpleIndex {
    fn new() -> Self {
        Self {
            index: BTreeMap::new(),
        }
    }

    fn insert(&mut self, key: String, row_id: u32) {
        self.index.entry(key).or_insert_with(BTreeSet::new).insert(row_id);
    }

    fn find_exact(&self, key: &str) -> Vec<u32> {
        self.index.get(key)
            .map(|set| set.iter().cloned().collect())
            .unwrap_or_else(Vec::new)
    }

    fn find_range(&self, start: &str, end: &str) -> Vec<u32> {
        self.index
            .range(start.to_string()..=end.to_string())
            .flat_map(|(_, row_ids)| row_ids.iter().cloned())
            .collect()
    }

    fn find_prefix(&self, prefix: &str) -> Vec<u32> {
        let end = format!("{}~", prefix); // Use tilde as end boundary
        self.index
            .range(prefix.to_string()..end)
            .filter(|(key, _)| key.starts_with(prefix))
            .flat_map(|(_, row_ids)| row_ids.iter().cloned())
            .collect()
    }

    fn remove(&mut self, key: &str, row_id: u32) {
        if let Some(row_ids) = self.index.get_mut(key) {
            row_ids.remove(&row_id);
            if row_ids.is_empty() {
                self.index.remove(key);
            }
        }
    }
}

fn stepped_union<T>(
    sets: Vec<BTreeSet<T>>,
) -> Vec<BTreeSet<T>>
where
    T: Ord + Clone,
{
    let mut steps = Vec::new();
    let mut current_union = BTreeSet::new();
    
    for set in sets {
        current_union = current_union.union(&set).cloned().collect();
        steps.push(current_union.clone());
    }
    
    steps
}
```

## Explanation

This comprehensive solution demonstrates **advanced BTree collection operations** and **ordered data structures**:

### Key Concepts Demonstrated:

1. **BTreeMap Range Operations**:
   - `range()` method enables efficient range queries
   - Ordered iteration over specific key ranges
   - O(log n + k) complexity where k is number of results

2. **BTreeMap Splitting**:
   - `split_off()` efficiently divides a map at a key boundary
   - Maintains ordering in both resulting maps
   - Useful for partitioning data or implementing database operations

3. **Sliding Window with Ordered Data**:
   - BTreeMap tracks value frequencies in O(log n) time
   - VecDeque maintains insertion order for window management
   - Enables efficient min/max queries in sliding windows

4. **Timeline Data Structure**:
   - Demonstrates time-based event storage and querying
   - Range queries for events in time windows
   - Overlap detection using interval arithmetic

5. **Custom Ordering**:
   - `LengthFirstString` shows how to implement custom sort orders
   - Demonstrates compound ordering (length first, then lexicographic)
   - BTreeSet respects custom Ord implementations

### Performance Characteristics:

**BTreeMap vs HashMap:**
- **Insertion**: HashMap typically faster (O(1) vs O(log n))
- **Lookup**: HashMap typically faster (O(1) vs O(log n))
- **Iteration**: BTreeMap provides ordered iteration
- **Memory**: BTreeMap has better cache locality for range operations

### Advanced Patterns:

1. **Predecessor/Successor Queries**:
   - Efficient nearest neighbor searches
   - Uses range operations with bounds
   - Common in database index implementations

2. **Simple Database Index**:
   - Inverted index pattern using BTreeMap
   - Supports exact, range, and prefix queries
   - Demonstrates how to build search indices

3. **Stepped Union Operations**:
   - Shows intermediate results of set operations
   - Useful for visualizing algorithm progress
   - Demonstrates iterative set building

### Real-World Applications:

- **Database Systems**: B-tree indices for ordered data access
- **Time Series**: Event storage and temporal queries  
- **Search Systems**: Ordered indices for range and prefix searches
- **Resource Scheduling**: Timeline-based event management
- **Analytics**: Sliding window computations over ordered data

This solution showcases when to choose BTree collections over HashMap/HashSet and how to leverage their ordering properties for efficient algorithms.
# Specialized Collections Solutions

## Solutions

```rust
use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::cmp::Reverse;
use std::time::{Duration, Instant};

fn is_palindrome_deque(s: &str) -> bool {
    let mut chars: VecDeque<char> = s.chars().filter(|c| !c.is_whitespace()).collect();
    
    while chars.len() > 1 {
        if chars.pop_front().unwrap().to_ascii_lowercase() != 
           chars.pop_back().unwrap().to_ascii_lowercase() {
            return false;
        }
    }
    true
}

fn rotate_deque<T>(mut deque: VecDeque<T>, n: isize) -> VecDeque<T> {
    if deque.is_empty() {
        return deque;
    }
    
    let len = deque.len() as isize;
    let n = n % len; // Handle rotations larger than length
    
    if n > 0 {
        // Rotate left: move front elements to back
        for _ in 0..n {
            if let Some(item) = deque.pop_front() {
                deque.push_back(item);
            }
        }
    } else if n < 0 {
        // Rotate right: move back elements to front
        for _ in 0..(-n) {
            if let Some(item) = deque.pop_back() {
                deque.push_front(item);
            }
        }
    }
    
    deque
}

fn sliding_window_maximum(nums: Vec<i32>, k: usize) -> Vec<i32> {
    if nums.is_empty() || k == 0 {
        return Vec::new();
    }
    
    let mut result = Vec::new();
    let mut deque = VecDeque::new(); // Store indices
    
    for i in 0..nums.len() {
        // Remove indices outside window
        while let Some(&front) = deque.front() {
            if front + k <= i {
                deque.pop_front();
            } else {
                break;
            }
        }
        
        // Remove indices with smaller values (they can't be maximum)
        while let Some(&back) = deque.back() {
            if nums[back] <= nums[i] {
                deque.pop_back();
            } else {
                break;
            }
        }
        
        deque.push_back(i);
        
        // Window is fully formed
        if i >= k - 1 {
            result.push(nums[*deque.front().unwrap()]);
        }
    }
    
    result
}

fn dijkstra_shortest_paths(
    graph: &HashMap<usize, Vec<(usize, u32)>>,
    start: usize,
    num_nodes: usize,
) -> Vec<Option<u32>> {
    let mut distances = vec![None; num_nodes];
    let mut heap = BinaryHeap::new();
    
    distances[start] = Some(0);
    heap.push(Reverse((0, start))); // Use Reverse for min-heap behavior
    
    while let Some(Reverse((dist, node))) = heap.pop() {
        if Some(dist) > distances[node] {
            continue; // Already found shorter path
        }
        
        if let Some(neighbors) = graph.get(&node) {
            for &(neighbor, weight) in neighbors {
                let new_distance = dist + weight;
                
                if distances[neighbor].map_or(true, |d| new_distance < d) {
                    distances[neighbor] = Some(new_distance);
                    heap.push(Reverse((new_distance, neighbor)));
                }
            }
        }
    }
    
    distances
}

#[derive(Eq, PartialEq)]
struct HeapItem<T> {
    value: T,
    iterator_id: usize,
}

impl<T: Ord> Ord for HeapItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering for min-heap behavior
        other.value.cmp(&self.value)
            .then_with(|| other.iterator_id.cmp(&self.iterator_id))
    }
}

impl<T: Ord> PartialOrd for HeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct KWayMerge<T> {
    heap: BinaryHeap<HeapItem<T>>,
    iterators: Vec<std::iter::Peekable<std::vec::IntoIter<T>>>,
}

impl<T: Ord + Clone> KWayMerge<T> {
    fn new(sorted_vecs: Vec<Vec<T>>) -> Self {
        let mut heap = BinaryHeap::new();
        let mut iterators: Vec<_> = sorted_vecs.into_iter()
            .map(|v| v.into_iter().peekable())
            .collect();
        
        // Initialize heap with first element from each non-empty iterator
        for (id, iter) in iterators.iter_mut().enumerate() {
            if let Some(value) = iter.next() {
                heap.push(HeapItem { value, iterator_id: id });
            }
        }
        
        Self { heap, iterators }
    }
}

impl<T: Ord + Clone> Iterator for KWayMerge<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(HeapItem { value, iterator_id }) = self.heap.pop() {
            // Add next element from the same iterator if available
            if let Some(next_value) = self.iterators[iterator_id].next() {
                self.heap.push(HeapItem { 
                    value: next_value, 
                    iterator_id 
                });
            }
            
            Some(value)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Task {
    id: u32,
    priority: u32,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Higher priority first, then by ID for stable ordering
        self.priority.cmp(&other.priority)
            .then_with(|| self.id.cmp(&other.id))
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct TaskScheduler {
    heap: BinaryHeap<Task>,
}

impl TaskScheduler {
    fn new() -> Self {
        Self {
            heap: BinaryHeap::new(),
        }
    }

    fn add_task(&mut self, task: Task) {
        self.heap.push(task);
    }

    fn get_next_task(&mut self) -> Option<Task> {
        self.heap.pop()
    }

    fn peek_next_task(&self) -> Option<&Task> {
        self.heap.peek()
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn len(&self) -> usize {
        self.heap.len()
    }
}

struct MedianTracker {
    lower_half: BinaryHeap<i32>,                     // Max heap
    upper_half: BinaryHeap<Reverse<i32>>,           // Min heap (using Reverse)
}

impl MedianTracker {
    fn new() -> Self {
        Self {
            lower_half: BinaryHeap::new(),
            upper_half: BinaryHeap::new(),
        }
    }

    fn add(&mut self, value: i32) {
        // Add to appropriate heap
        if self.lower_half.is_empty() || value <= *self.lower_half.peek().unwrap() {
            self.lower_half.push(value);
        } else {
            self.upper_half.push(Reverse(value));
        }
        
        // Rebalance heaps to maintain size invariant
        if self.lower_half.len() > self.upper_half.len() + 1 {
            let value = self.lower_half.pop().unwrap();
            self.upper_half.push(Reverse(value));
        } else if self.upper_half.len() > self.lower_half.len() + 1 {
            let Reverse(value) = self.upper_half.pop().unwrap();
            self.lower_half.push(value);
        }
    }

    fn get_median(&self) -> Option<f64> {
        match (self.lower_half.len(), self.upper_half.len()) {
            (0, 0) => None,
            (l, u) if l == u => {
                let lower_max = *self.lower_half.peek().unwrap() as f64;
                let upper_min = self.upper_half.peek().unwrap().0 as f64;
                Some((lower_max + upper_min) / 2.0)
            }
            (l, u) if l > u => Some(*self.lower_half.peek().unwrap() as f64),
            _ => Some(self.upper_half.peek().unwrap().0 as f64),
        }
    }

    fn len(&self) -> usize {
        self.lower_half.len() + self.upper_half.len()
    }
}

struct SimpleLRUCache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    access_order: VecDeque<K>,
}

impl<K, V> SimpleLRUCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        Self {
            capacity,
            map: HashMap::new(),
            access_order: VecDeque::new(),
        }
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            // Move to front (most recently used)
            self.access_order.retain(|k| k != key);
            self.access_order.push_back(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            // Update existing key
            self.access_order.retain(|k| k != &key);
            self.access_order.push_back(key.clone());
            self.map.insert(key, value);
        } else {
            // Add new key
            if self.map.len() >= self.capacity {
                // Evict least recently used
                if let Some(lru_key) = self.access_order.pop_front() {
                    self.map.remove(&lru_key);
                }
            }
            
            self.map.insert(key.clone(), value);
            self.access_order.push_back(key);
        }
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

struct CircularBuffer<T> {
    buffer: VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    fn new(capacity: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            capacity,
        }
    }

    fn push(&mut self, item: T) {
        if self.buffer.len() >= self.capacity {
            self.buffer.pop_back(); // Remove oldest (back)
        }
        self.buffer.push_front(item); // Add newest to front
    }

    fn get(&self, index: usize) -> Option<&T> {
        self.buffer.get(index)
    }

    fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        self.buffer.iter()
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn is_full(&self) -> bool {
        self.buffer.len() == self.capacity
    }
}

struct CardDeck {
    cards: VecDeque<String>,
}

impl CardDeck {
    fn new() -> Self {
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];
        
        let mut cards = VecDeque::new();
        for suit in &suits {
            for rank in &ranks {
                cards.push_back(format!("{} of {}", rank, suit));
            }
        }
        
        Self { cards }
    }

    fn shuffle(&mut self) {
        use std::collections::VecDeque;
        let mut temp: Vec<_> = self.cards.drain(..).collect();
        
        // Simple Fisher-Yates shuffle
        for i in (1..temp.len()).rev() {
            let j = i % (i + 1); // Simple pseudo-random for demo
            temp.swap(i, j);
        }
        
        self.cards = temp.into_iter().collect();
    }

    fn deal_from_top(&mut self) -> Option<String> {
        self.cards.pop_front()
    }

    fn deal_from_bottom(&mut self) -> Option<String> {
        self.cards.pop_back()
    }

    fn insert_at(&mut self, index: usize, card: String) {
        self.cards.insert(index, card);
    }

    fn peek_top(&self) -> Option<&String> {
        self.cards.front()
    }

    fn peek_bottom(&self) -> Option<&String> {
        self.cards.back()
    }

    fn remaining_cards(&self) -> usize {
        self.cards.len()
    }
}

struct WorkStealingDeque<T> {
    deque: VecDeque<T>,
}

impl<T> WorkStealingDeque<T> {
    fn new() -> Self {
        Self {
            deque: VecDeque::new(),
        }
    }

    fn push_task(&mut self, task: T) {
        self.deque.push_back(task);
    }

    fn pop_task(&mut self) -> Option<T> {
        self.deque.pop_back() // Owner pops from same end (LIFO)
    }

    fn steal_task(&mut self) -> Option<T> {
        self.deque.pop_front() // Thief steals from other end (FIFO)
    }

    fn len(&self) -> usize {
        self.deque.len()
    }

    fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
}

fn collection_performance_comparison() -> (
    Duration, Duration, Duration, Duration, Duration, Duration,
) {
    let size = 10000;
    
    // Vec front insertion
    let start = Instant::now();
    let mut vec = Vec::new();
    for i in 0..size {
        vec.insert(0, i);
    }
    let vec_front = start.elapsed();
    
    // VecDeque front insertion
    let start = Instant::now();
    let mut deque = VecDeque::new();
    for i in 0..size {
        deque.push_front(i);
    }
    let deque_front = start.elapsed();
    
    // Vec back insertion
    let start = Instant::now();
    let mut vec = Vec::new();
    for i in 0..size {
        vec.push(i);
    }
    let vec_back = start.elapsed();
    
    // VecDeque back insertion
    let start = Instant::now();
    let mut deque = VecDeque::new();
    for i in 0..size {
        deque.push_back(i);
    }
    let deque_back = start.elapsed();
    
    // Vec random access
    let vec: Vec<_> = (0..size).collect();
    let start = Instant::now();
    for i in 0..size {
        let _ = vec[i];
    }
    let vec_random = start.elapsed();
    
    // VecDeque random access
    let deque: VecDeque<_> = (0..size).collect();
    let start = Instant::now();
    for i in 0..size {
        let _ = deque[i];
    }
    let deque_random = start.elapsed();
    
    (vec_front, deque_front, vec_back, deque_back, vec_random, deque_random)
}
```

## Explanation

This comprehensive solution demonstrates **specialized collection types** and their optimal use cases:

### Key Concepts Demonstrated:

1. **VecDeque for Double-Ended Operations**:
   - Efficient insertion/removal at both ends
   - Palindrome checking by removing from both sides
   - Rotation operations using front/back manipulation
   - Sliding window maximum using deque as index tracker

2. **BinaryHeap for Priority Operations**:
   - Dijkstra's algorithm using min-heap with `Reverse`
   - Task scheduling with priority queues
   - Median tracking using two heaps (max + min)
   - K-way merge using heap to track minimum elements

3. **Custom Ordering in Heaps**:
   - `HeapItem` demonstrates custom `Ord` implementation
   - `Reverse` wrapper for min-heap behavior
   - Stable ordering using secondary keys

4. **Advanced Data Structure Patterns**:
   - LRU Cache combining HashMap + VecDeque for O(1) operations
   - Circular Buffer using VecDeque with capacity management
   - Work-stealing deque with LIFO/FIFO access patterns

### Performance Characteristics:

**VecDeque vs Vec:**
- **Front operations**: VecDeque O(1), Vec O(n)
- **Back operations**: Both O(1) amortized
- **Random access**: Vec O(1), VecDeque O(1) but with higher constant factor
- **Memory**: VecDeque uses ring buffer, may have unused capacity

**BinaryHeap Operations:**
- **Insert/Remove**: O(log n)
- **Peek**: O(1)
- **Heapify**: O(n)

### Algorithmic Applications:

1. **Sliding Window Maximum**:
   - Uses deque to maintain candidates in decreasing order
   - O(n) time complexity despite nested loops
   - Each element added/removed at most once

2. **Dijkstra's Algorithm**:
   - BinaryHeap provides efficient priority queue
   - `Reverse` wrapper creates min-heap behavior
   - Demonstrates graph algorithms with Rust collections

3. **Median Tracking**:
   - Two-heap approach maintains running median
   - Max heap for lower half, min heap for upper half
   - O(log n) insertion, O(1) median query

### Real-World Applications:

- **System Programming**: Work-stealing queues in thread pools
- **Games**: Circular buffers for game state history
- **Databases**: Priority queues for query scheduling
- **Web Servers**: LRU caches for content caching
- **Algorithms**: Heap-based algorithms like A* pathfinding

### Collection Selection Guide:

| Operation | Best Collection | Reason |
|-----------|----------------|---------|
| Double-ended queue | VecDeque | O(1) at both ends |
| Priority queue | BinaryHeap | O(log n) priority operations |
| Sliding window | VecDeque | Efficient candidate tracking |
| LRU cache | HashMap + VecDeque | O(1) access + ordering |
| Card deck | VecDeque | Dealing from both ends |

This solution showcases how choosing the right specialized collection can dramatically improve both performance and code clarity for specific algorithmic patterns.
// Specialized Collections Practice
//
// Learning Objectives:
// - Master VecDeque for double-ended queue operations
// - Use BinaryHeap for priority queue algorithms
// - Practice LinkedList for specific use cases
// - Understand when to choose specialized collections
// - Work with collection-specific performance characteristics
// - Implement algorithms that leverage specialized collection strengths
//
// Run with: cargo test --bin specialized_collections

/// Implement a palindrome checker using VecDeque's double-ended capabilities.
/// Remove characters from both ends and compare for efficiency.
fn is_palindrome_deque(s: &str) -> bool {
    todo!("Implement palindrome checking using VecDeque")
}

/// Use VecDeque to implement efficient rotation of elements.
/// Rotate left by n positions (negative n rotates right).
fn rotate_deque<T>(mut deque: std::collections::VecDeque<T>, n: isize) -> std::collections::VecDeque<T> {
    todo!("Implement efficient deque rotation")
}

/// Implement a sliding window maximum using VecDeque to maintain candidates.
/// For each window of size k, find the maximum element efficiently.
fn sliding_window_maximum(nums: Vec<i32>, k: usize) -> Vec<i32> {
    todo!("Implement sliding window maximum using VecDeque")
}

/// Use BinaryHeap to implement Dijkstra's shortest path algorithm.
/// Return the shortest distances from start node to all other nodes.
fn dijkstra_shortest_paths(
    graph: &std::collections::HashMap<usize, Vec<(usize, u32)>>, // node -> [(neighbor, weight)]
    start: usize,
    num_nodes: usize,
) -> Vec<Option<u32>> {
    todo!("Implement Dijkstra's algorithm using BinaryHeap")
}

/// Implement k-way merge using BinaryHeap.
/// Merge k sorted iterators into a single sorted iterator.
struct KWayMerge<T> {
    heap: std::collections::BinaryHeap<HeapItem<T>>,
    iterators: Vec<std::iter::Peekable<std::vec::IntoIter<T>>>,
}

#[derive(Eq, PartialEq)]
struct HeapItem<T> {
    value: T,
    iterator_id: usize,
}

impl<T: Ord> Ord for HeapItem<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!("Implement ordering for min-heap (reverse of natural order)")
    }
}

impl<T: Ord> PartialOrd for HeapItem<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Ord + Clone> KWayMerge<T> {
    fn new(sorted_vecs: Vec<Vec<T>>) -> Self {
        todo!("Initialize k-way merge with sorted vectors")
    }
}

impl<T: Ord + Clone> Iterator for KWayMerge<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        todo!("Implement iterator that returns elements in sorted order")
    }
}

/// Use BinaryHeap to implement a task scheduler with priorities.
/// Higher priority tasks (larger numbers) should be executed first.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Task {
    id: u32,
    priority: u32,
    description: String,
}

impl Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!("Implement task ordering by priority (higher priority first)")
    }
}

impl PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct TaskScheduler {
    heap: std::collections::BinaryHeap<Task>,
}

impl TaskScheduler {
    fn new() -> Self {
        todo!("Create new task scheduler")
    }

    fn add_task(&mut self, task: Task) {
        todo!("Add task to scheduler")
    }

    fn get_next_task(&mut self) -> Option<Task> {
        todo!("Get highest priority task")
    }

    fn peek_next_task(&self) -> Option<&Task> {
        todo!("Peek at highest priority task without removing")
    }

    fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    fn len(&self) -> usize {
        self.heap.len()
    }
}

/// Implement a median tracker using two heaps (BinaryHeap).
/// Efficiently maintain running median as elements are added.
struct MedianTracker {
    lower_half: std::collections::BinaryHeap<i32>, // max heap
    upper_half: std::collections::BinaryHeap<std::cmp::Reverse<i32>>, // min heap
}

impl MedianTracker {
    fn new() -> Self {
        todo!("Create new median tracker")
    }

    fn add(&mut self, value: i32) {
        todo!("Add value while maintaining median property")
    }

    fn get_median(&self) -> Option<f64> {
        todo!("Get current median")
    }

    fn len(&self) -> usize {
        self.lower_half.len() + self.upper_half.len()
    }
}

/// Simple LRU cache implementation using HashMap and insertion order tracking.
/// This is a simplified version that doesn't use raw pointers.
struct SimpleLRUCache<K, V> {
    capacity: usize,
    map: std::collections::HashMap<K, V>,
    access_order: Vec<K>,
}

impl<K, V> SimpleLRUCache<K, V>
where
    K: Clone + std::hash::Hash + Eq,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        todo!("Create new simple LRU cache with given capacity")
    }

    fn get(&mut self, key: &K) -> Option<&V> {
        todo!("Get value and mark as recently used")
    }

    fn put(&mut self, key: K, value: V) {
        todo!("Insert key-value pair, evicting LRU if necessary")
    }

    fn len(&self) -> usize {
        self.map.len()
    }

    fn capacity(&self) -> usize {
        self.capacity
    }
}

/// Implement a circular buffer using VecDeque.
/// Fixed-size buffer that overwrites oldest elements when full.
struct CircularBuffer<T> {
    buffer: std::collections::VecDeque<T>,
    capacity: usize,
}

impl<T> CircularBuffer<T> {
    fn new(capacity: usize) -> Self {
        todo!("Create new circular buffer")
    }

    fn push(&mut self, item: T) {
        todo!("Add item, removing oldest if at capacity")
    }

    fn get(&self, index: usize) -> Option<&T> {
        todo!("Get item at index (0 is newest)")
    }

    fn iter(&self) -> std::collections::vec_deque::Iter<'_, T> {
        todo!("Return iterator from newest to oldest")
    }

    fn len(&self) -> usize {
        self.buffer.len()
    }

    fn is_full(&self) -> bool {
        self.buffer.len() == self.capacity
    }
}

/// Use VecDeque to implement a deck of cards with efficient shuffling.
/// Support dealing from both ends and inserting at arbitrary positions.
struct CardDeck {
    cards: std::collections::VecDeque<String>,
}

impl CardDeck {
    fn new() -> Self {
        todo!("Create standard 52-card deck")
    }

    fn shuffle(&mut self) {
        todo!("Shuffle deck using random swaps")
    }

    fn deal_from_top(&mut self) -> Option<String> {
        todo!("Deal card from top (front)")
    }

    fn deal_from_bottom(&mut self) -> Option<String> {
        todo!("Deal card from bottom (back)")
    }

    fn insert_at(&mut self, index: usize, card: String) {
        todo!("Insert card at specific position")
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

/// Implement a work-stealing deque using VecDeque.
/// Workers can steal work from both ends for load balancing.
struct WorkStealingDeque<T> {
    deque: std::collections::VecDeque<T>,
}

impl<T> WorkStealingDeque<T> {
    fn new() -> Self {
        todo!("Create new work-stealing deque")
    }

    fn push_task(&mut self, task: T) {
        todo!("Add task to own end (back)")
    }

    fn pop_task(&mut self) -> Option<T> {
        todo!("Remove task from own end (back)")
    }

    fn steal_task(&mut self) -> Option<T> {
        todo!("Steal task from other end (front)")
    }

    fn len(&self) -> usize {
        self.deque.len()
    }

    fn is_empty(&self) -> bool {
        self.deque.is_empty()
    }
}

/// Performance comparison between different collections for specific use cases.
/// Compare Vec, VecDeque, and LinkedList for different operation patterns.
fn collection_performance_comparison() -> (
    std::time::Duration, // Vec front insertion
    std::time::Duration, // VecDeque front insertion
    std::time::Duration, // Vec back insertion  
    std::time::Duration, // VecDeque back insertion
    std::time::Duration, // Vec random access
    std::time::Duration, // VecDeque random access
) {
    todo!("Compare collection performance for different operations")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BinaryHeap, HashMap, VecDeque};

    #[test]
    fn test_is_palindrome_deque() {
        assert!(is_palindrome_deque("racecar"));
        assert!(is_palindrome_deque("level"));
        assert!(!is_palindrome_deque("hello"));
        assert!(is_palindrome_deque(""));
        assert!(is_palindrome_deque("a"));
        assert!(is_palindrome_deque("aa"));
        assert!(!is_palindrome_deque("ab"));
        
        // Test with spaces and punctuation
        assert!(is_palindrome_deque("a man a plan a canal panama"));
    }

    #[test]
    fn test_rotate_deque() {
        let mut deque = VecDeque::new();
        deque.extend([1, 2, 3, 4, 5]);
        
        // Rotate left by 2
        let rotated = rotate_deque(deque.clone(), 2);
        let expected: VecDeque<_> = [3, 4, 5, 1, 2].iter().cloned().collect();
        assert_eq!(rotated, expected);
        
        // Rotate right by 2 (negative)
        let rotated = rotate_deque(deque.clone(), -2);
        let expected: VecDeque<_> = [4, 5, 1, 2, 3].iter().cloned().collect();
        assert_eq!(rotated, expected);
        
        // No rotation
        let rotated = rotate_deque(deque.clone(), 0);
        assert_eq!(rotated, deque);
        
        // Rotation by full length
        let rotated = rotate_deque(deque.clone(), 5);
        assert_eq!(rotated, deque);
    }

    #[test]
    fn test_sliding_window_maximum() {
        let nums = vec![1, 3, -1, -3, 5, 3, 6, 7];
        let result = sliding_window_maximum(nums, 3);
        assert_eq!(result, vec![3, 3, 5, 5, 6, 7]);
        
        let nums = vec![1];
        let result = sliding_window_maximum(nums, 1);
        assert_eq!(result, vec![1]);
        
        let nums = vec![1, -1];
        let result = sliding_window_maximum(nums, 1);
        assert_eq!(result, vec![1, -1]);
        
        let nums = vec![9, 11];
        let result = sliding_window_maximum(nums, 2);
        assert_eq!(result, vec![11]);
    }

    #[test]
    fn test_dijkstra_shortest_paths() {
        let mut graph = HashMap::new();
        graph.insert(0, vec![(1, 4), (2, 1)]);
        graph.insert(1, vec![(3, 1)]);
        graph.insert(2, vec![(1, 2), (3, 5)]);
        graph.insert(3, vec![]);
        
        let distances = dijkstra_shortest_paths(&graph, 0, 4);
        
        assert_eq!(distances[0], Some(0));  // Distance to self
        assert_eq!(distances[1], Some(3));  // 0->2->1 (cost 1+2=3) is shorter than 0->1 (cost 4)
        assert_eq!(distances[2], Some(1));  // Direct edge 0->2
        assert_eq!(distances[3], Some(4));  // 0->2->1->3 (cost 1+2+1=4)
    }

    #[test]
    fn test_k_way_merge() {
        let vec1 = vec![1, 4, 7];
        let vec2 = vec![2, 5, 8];
        let vec3 = vec![3, 6, 9];
        
        let merge = KWayMerge::new(vec![vec1, vec2, vec3]);
        let result: Vec<_> = merge.collect();
        
        assert_eq!(result, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
        
        // Test with empty vectors
        let vec1 = vec![];
        let vec2 = vec![1, 3];
        let vec3 = vec![2];
        
        let merge = KWayMerge::new(vec![vec1, vec2, vec3]);
        let result: Vec<_> = merge.collect();
        
        assert_eq!(result, vec![1, 2, 3]);
    }

    #[test]
    fn test_task_scheduler() {
        let mut scheduler = TaskScheduler::new();
        
        let task1 = Task { id: 1, priority: 3, description: "Low priority".to_string() };
        let task2 = Task { id: 2, priority: 5, description: "High priority".to_string() };
        let task3 = Task { id: 3, priority: 1, description: "Lowest priority".to_string() };
        
        scheduler.add_task(task1);
        scheduler.add_task(task2.clone());
        scheduler.add_task(task3);
        
        assert_eq!(scheduler.len(), 3);
        
        // Should get highest priority task first
        let next = scheduler.get_next_task().unwrap();
        assert_eq!(next.id, 2);
        assert_eq!(next.priority, 5);
        
        assert_eq!(scheduler.len(), 2);
        
        // Peek should not remove task
        let peeked = scheduler.peek_next_task().unwrap();
        assert_eq!(peeked.priority, 3); // Next highest
        assert_eq!(scheduler.len(), 2); // Still 2 tasks
    }

    #[test]
    fn test_median_tracker() {
        let mut tracker = MedianTracker::new();
        
        tracker.add(1);
        assert_eq!(tracker.get_median(), Some(1.0));
        
        tracker.add(2);
        assert_eq!(tracker.get_median(), Some(1.5)); // (1+2)/2
        
        tracker.add(3);
        assert_eq!(tracker.get_median(), Some(2.0)); // middle of [1,2,3]
        
        tracker.add(4);
        assert_eq!(tracker.get_median(), Some(2.5)); // (2+3)/2 for [1,2,3,4]
        
        tracker.add(5);
        assert_eq!(tracker.get_median(), Some(3.0)); // middle of [1,2,3,4,5]
        
        assert_eq!(tracker.len(), 5);
    }

    #[test]
    fn test_circular_buffer() {
        let mut buffer = CircularBuffer::new(3);
        
        buffer.push("a");
        buffer.push("b");
        buffer.push("c");
        assert_eq!(buffer.len(), 3);
        assert!(buffer.is_full());
        
        // Should have [a, b, c] with c being newest
        assert_eq!(buffer.get(0), Some(&"c")); // newest
        assert_eq!(buffer.get(1), Some(&"b"));
        assert_eq!(buffer.get(2), Some(&"a")); // oldest
        
        // Adding another should overwrite oldest
        buffer.push("d");
        assert_eq!(buffer.len(), 3);
        
        // Should now have [b, c, d] with d being newest
        assert_eq!(buffer.get(0), Some(&"d")); // newest
        assert_eq!(buffer.get(1), Some(&"c"));
        assert_eq!(buffer.get(2), Some(&"b")); // oldest (a was evicted)
        
        // Test iterator
        let items: Vec<_> = buffer.iter().collect();
        assert_eq!(items, vec![&"d", &"c", &"b"]);
    }

    #[test]
    fn test_card_deck() {
        let mut deck = CardDeck::new();
        assert_eq!(deck.remaining_cards(), 52);
        
        let top_card = deck.peek_top().unwrap().clone();
        let bottom_card = deck.peek_bottom().unwrap().clone();
        
        // Deal from top
        let dealt_top = deck.deal_from_top().unwrap();
        assert_eq!(dealt_top, top_card);
        assert_eq!(deck.remaining_cards(), 51);
        
        // Deal from bottom
        let dealt_bottom = deck.deal_from_bottom().unwrap();
        assert_eq!(dealt_bottom, bottom_card);
        assert_eq!(deck.remaining_cards(), 50);
        
        // Insert a card
        deck.insert_at(0, "Joker".to_string());
        assert_eq!(deck.remaining_cards(), 51);
        assert_eq!(deck.peek_top(), Some(&"Joker".to_string()));
        
        // Test shuffle (just verify it doesn't crash and deck size remains same)
        deck.shuffle();
        assert_eq!(deck.remaining_cards(), 51);
    }

    #[test]
    fn test_work_stealing_deque() {
        let mut deque = WorkStealingDeque::new();
        assert!(deque.is_empty());
        
        // Add some tasks
        deque.push_task("task1");
        deque.push_task("task2");
        deque.push_task("task3");
        assert_eq!(deque.len(), 3);
        
        // Owner pops from back (LIFO)
        let task = deque.pop_task();
        assert_eq!(task, Some("task3"));
        assert_eq!(deque.len(), 2);
        
        // Thief steals from front (FIFO)
        let stolen = deque.steal_task();
        assert_eq!(stolen, Some("task1"));
        assert_eq!(deque.len(), 1);
        
        // Remaining task
        let last = deque.pop_task();
        assert_eq!(last, Some("task2"));
        assert!(deque.is_empty());
        
        // Stealing from empty deque
        assert_eq!(deque.steal_task(), None);
    }

    #[test]
    fn test_simple_lru_cache() {
        let mut cache = SimpleLRUCache::new(2);
        
        cache.put("key1", "value1");
        cache.put("key2", "value2");
        assert_eq!(cache.len(), 2);
        assert_eq!(cache.capacity(), 2);
        
        assert_eq!(cache.get(&"key1"), Some(&"value1"));
        assert_eq!(cache.get(&"key2"), Some(&"value2"));
        
        // Adding third item should evict least recently used
        cache.put("key3", "value3");
        assert_eq!(cache.len(), 2);
        
        // Test that cache respects capacity
        assert!(cache.len() <= cache.capacity());
    }

    #[test]
    fn test_collection_performance_comparison() {
        let (vec_front, deque_front, vec_back, deque_back, vec_random, deque_random) 
            = collection_performance_comparison();
        
        // All operations should complete in reasonable time
        assert!(vec_front < std::time::Duration::from_secs(1));
        assert!(deque_front < std::time::Duration::from_secs(1));
        assert!(vec_back < std::time::Duration::from_secs(1));
        assert!(deque_back < std::time::Duration::from_secs(1));
        assert!(vec_random < std::time::Duration::from_secs(1));
        assert!(deque_random < std::time::Duration::from_secs(1));
        
        println!("Vec front insertion: {:?}", vec_front);
        println!("VecDeque front insertion: {:?}", deque_front);
        println!("Vec back insertion: {:?}", vec_back);
        println!("VecDeque back insertion: {:?}", deque_back);
        println!("Vec random access: {:?}", vec_random);
        println!("VecDeque random access: {:?}", deque_random);
        
        // VecDeque should be significantly faster for front operations
        // Vec should be faster for random access
        // These are educational assertions
        assert!(deque_front <= vec_front); // VecDeque should be better for front ops
        assert!(vec_random <= deque_random); // Vec should be better for random access
    }

    #[test]
    fn test_heap_item_ordering() {
        let mut heap = BinaryHeap::new();
        
        heap.push(HeapItem { value: 3, iterator_id: 0 });
        heap.push(HeapItem { value: 1, iterator_id: 1 });
        heap.push(HeapItem { value: 2, iterator_id: 2 });
        
        // Should come out in min-heap order: 1, 2, 3
        assert_eq!(heap.pop().unwrap().value, 1);
        assert_eq!(heap.pop().unwrap().value, 2);
        assert_eq!(heap.pop().unwrap().value, 3);
    }

    #[test]
    fn test_task_ordering() {
        let mut heap = BinaryHeap::new();
        
        let task1 = Task { id: 1, priority: 3, description: "Medium".to_string() };
        let task2 = Task { id: 2, priority: 5, description: "High".to_string() };
        let task3 = Task { id: 3, priority: 1, description: "Low".to_string() };
        
        heap.push(task1);
        heap.push(task2.clone());
        heap.push(task3);
        
        // Should come out in priority order: 5, 3, 1
        assert_eq!(heap.pop().unwrap().priority, 5);
        assert_eq!(heap.pop().unwrap().priority, 3);
        assert_eq!(heap.pop().unwrap().priority, 1);
    }

    #[test]
    fn test_edge_cases() {
        // Test empty collections
        assert!(is_palindrome_deque(""));
        
        let empty_deque: VecDeque<i32> = VecDeque::new();
        let rotated = rotate_deque(empty_deque.clone(), 5);
        assert_eq!(rotated, empty_deque);
        
        // Test sliding window with empty input
        let result = sliding_window_maximum(vec![], 1);
        assert!(result.is_empty());
        
        // Test single element cases
        assert!(is_palindrome_deque("x"));
        
        let single_deque: VecDeque<_> = [42].iter().cloned().collect();
        let rotated = rotate_deque(single_deque.clone(), 1);
        assert_eq!(rotated, single_deque);
        
        // Test median tracker with single element
        let mut tracker = MedianTracker::new();
        tracker.add(42);
        assert_eq!(tracker.get_median(), Some(42.0));
        
        // Test circular buffer with capacity 1
        let mut buffer = CircularBuffer::new(1);
        buffer.push("only");
        assert!(buffer.is_full());
        buffer.push("replacement");
        assert_eq!(buffer.get(0), Some(&"replacement"));
        assert_eq!(buffer.len(), 1);
    }
}
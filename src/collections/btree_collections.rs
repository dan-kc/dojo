// BTree Collections Practice
//
// Learning Objectives:
// - Master BTreeMap and BTreeSet for ordered collections
// - Use range queries and ordered iteration
// - Practice split operations and ordered manipulation
// - Compare performance with HashMap/HashSet
// - Work with custom ordering and sorted data
// - Understand when to choose BTree vs Hash collections
//
// Run with: cargo test --bin btree_collections

/// Implement range queries on a BTreeMap to find all entries within a key range.
/// Return both keys and values within the specified range (inclusive).
fn range_query<K, V>(
    map: &std::collections::BTreeMap<K, V>,
    start: &K,
    end: &K,
) -> Vec<(K, V)>
where
    K: Ord + Clone,
    V: Clone,
{
    todo!("Implement range query using BTreeMap range methods")
}

/// Split a BTreeMap at a given key, returning two maps: one with keys < split_key,
/// one with keys >= split_key. Original map should be consumed.
fn split_btree_map<K, V>(
    mut map: std::collections::BTreeMap<K, V>,
    split_key: &K,
) -> (std::collections::BTreeMap<K, V>, std::collections::BTreeMap<K, V>)
where
    K: Ord + Clone,
{
    todo!("Implement BTreeMap splitting")
}

/// Find the k smallest and k largest elements from a BTreeSet.
/// Return as two separate vectors in sorted order.
fn find_k_extremes<T>(
    set: &std::collections::BTreeSet<T>,
    k: usize,
) -> (Vec<T>, Vec<T>)
where
    T: Ord + Clone,
{
    todo!("Implement finding k smallest and largest elements")
}

/// Implement a sliding window minimum/maximum tracker using BTreeMap.
/// Efficiently maintain min/max in a sliding window of size k.
struct SlidingWindowTracker {
    window: std::collections::BTreeMap<i32, usize>, // value -> count
    k: usize,
    current_window: std::collections::VecDeque<i32>,
}

impl SlidingWindowTracker {
    fn new(k: usize) -> Self {
        todo!("Implement new sliding window tracker")
    }

    fn add(&mut self, value: i32) {
        todo!("Add value to sliding window")
    }

    fn get_min(&self) -> Option<i32> {
        todo!("Get minimum value in current window")
    }

    fn get_max(&self) -> Option<i32> {
        todo!("Get maximum value in current window")
    }

    fn window_size(&self) -> usize {
        self.current_window.len()
    }
}

/// Merge multiple sorted BTreeMaps, combining values for duplicate keys.
/// Use a combining function to handle value conflicts.
fn merge_sorted_btreemaps<K, V, F>(
    maps: Vec<std::collections::BTreeMap<K, V>>,
    combine_fn: F,
) -> std::collections::BTreeMap<K, V>
where
    K: Ord + Clone,
    V: Clone,
    F: Fn(V, V) -> V,
{
    todo!("Implement merging of sorted BTreeMaps")
}

/// Implement ordered set operations: ordered union, intersection, difference.
/// Return results as BTreeSets to maintain ordering.
fn ordered_set_operations<T>(
    set_a: &std::collections::BTreeSet<T>,
    set_b: &std::collections::BTreeSet<T>,
) -> (std::collections::BTreeSet<T>, std::collections::BTreeSet<T>, std::collections::BTreeSet<T>)
where
    T: Ord + Clone,
{
    todo!("Implement ordered union, intersection, and difference")
}

/// Find all subranges in a BTreeSet where consecutive elements differ by exactly 1.
/// Return the ranges as (start, end) pairs.
fn find_consecutive_ranges<T>(
    set: &std::collections::BTreeSet<T>,
) -> Vec<(T, T)>
where
    T: Ord + Clone + std::ops::Add<Output = T> + From<u8> + PartialEq,
{
    todo!("Implement consecutive range finding")
}

/// Implement a timeline data structure using BTreeMap for event scheduling.
/// Events have timestamps and can be queried by time ranges.
#[derive(Debug, Clone, PartialEq)]
struct Event {
    id: u32,
    description: String,
    duration: u32, // in minutes
}

struct Timeline {
    events: std::collections::BTreeMap<u64, Vec<Event>>, // timestamp -> events
}

impl Timeline {
    fn new() -> Self {
        todo!("Implement new timeline")
    }

    fn add_event(&mut self, timestamp: u64, event: Event) {
        todo!("Add event at timestamp")
    }

    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<(u64, &Event)> {
        todo!("Get all events within time range")
    }

    fn get_next_event(&self, after: u64) -> Option<(u64, &Event)> {
        todo!("Get next event after given timestamp")
    }

    fn remove_events_before(&mut self, timestamp: u64) {
        todo!("Remove all events before given timestamp")
    }

    fn get_overlapping_events(&self, timestamp: u64, duration: u32) -> Vec<(u64, &Event)> {
        todo!("Find events that overlap with given time period")
    }
}

/// Compare performance between BTreeMap and HashMap for different operations.
/// Return timing results for insertion, lookup, and iteration.
fn performance_comparison(
    size: usize,
) -> (
    std::time::Duration, // BTreeMap insert
    std::time::Duration, // HashMap insert  
    std::time::Duration, // BTreeMap lookup
    std::time::Duration, // HashMap lookup
    std::time::Duration, // BTreeMap ordered iteration
    std::time::Duration, // HashMap unordered iteration
) {
    todo!("Implement performance comparison between BTreeMap and HashMap")
}

/// Implement a custom ordering for BTreeSet using a wrapper type.
/// Sort strings by length first, then lexicographically.
#[derive(Debug, Clone, PartialEq, Eq)]
struct LengthFirstString(String);

impl Ord for LengthFirstString {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        todo!("Implement length-first ordering")
    }
}

impl PartialOrd for LengthFirstString {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// Create and demonstrate BTreeSet with custom ordering.
fn custom_ordered_set() -> std::collections::BTreeSet<LengthFirstString> {
    todo!("Create BTreeSet with length-first string ordering")
}

/// Implement efficient predecessor/successor queries in BTreeSet.
/// Find the largest element < target and smallest element > target.
fn find_predecessor_successor<T>(
    set: &std::collections::BTreeSet<T>,
    target: &T,
) -> (Option<T>, Option<T>)
where
    T: Ord + Clone,
{
    todo!("Implement predecessor/successor finding")
}

/// Use BTreeMap to implement a simple database index.
/// Support range queries, prefix searches, and ordered iteration.
struct SimpleIndex {
    index: std::collections::BTreeMap<String, std::collections::BTreeSet<u32>>, // key -> row_ids
}

impl SimpleIndex {
    fn new() -> Self {
        todo!("Implement new index")
    }

    fn insert(&mut self, key: String, row_id: u32) {
        todo!("Insert key-row_id pair")
    }

    fn find_exact(&self, key: &str) -> Vec<u32> {
        todo!("Find exact key matches")
    }

    fn find_range(&self, start: &str, end: &str) -> Vec<u32> {
        todo!("Find all row_ids for keys in range")
    }

    fn find_prefix(&self, prefix: &str) -> Vec<u32> {
        todo!("Find all row_ids for keys with given prefix")
    }

    fn remove(&mut self, key: &str, row_id: u32) {
        todo!("Remove specific key-row_id pair")
    }
}

/// Implement set operations that maintain order and return intermediate steps.
/// Return each step of the union operation as it builds the result.
fn stepped_union<T>(
    sets: Vec<std::collections::BTreeSet<T>>,
) -> Vec<std::collections::BTreeSet<T>>
where
    T: Ord + Clone,
{
    todo!("Implement union with intermediate steps")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{BTreeMap, BTreeSet, HashMap, VecDeque};

    #[test]
    fn test_range_query() {
        let mut map = BTreeMap::new();
        map.insert(1, "one");
        map.insert(3, "three");
        map.insert(5, "five");
        map.insert(7, "seven");
        map.insert(9, "nine");
        
        let result = range_query(&map, &3, &7);
        assert_eq!(result, vec![(3, "three"), (5, "five"), (7, "seven")]);
        
        // Test empty range
        let empty_result = range_query(&map, &2, &2);
        assert!(empty_result.is_empty());
        
        // Test range with no matches
        let no_matches = range_query(&map, &10, &20);
        assert!(no_matches.is_empty());
    }

    #[test]
    fn test_split_btree_map() {
        let mut map = BTreeMap::new();
        map.insert(1, "one");
        map.insert(2, "two");
        map.insert(3, "three");
        map.insert(4, "four");
        map.insert(5, "five");
        
        let (left, right) = split_btree_map(map, &3);
        
        // Left should contain keys < 3
        assert_eq!(left.len(), 2);
        assert_eq!(left.get(&1), Some(&"one"));
        assert_eq!(left.get(&2), Some(&"two"));
        
        // Right should contain keys >= 3
        assert_eq!(right.len(), 3);
        assert_eq!(right.get(&3), Some(&"three"));
        assert_eq!(right.get(&4), Some(&"four"));
        assert_eq!(right.get(&5), Some(&"five"));
    }

    #[test]
    fn test_find_k_extremes() {
        let set: BTreeSet<i32> = [1, 3, 5, 7, 9, 11, 13, 15, 17, 19].iter().cloned().collect();
        
        let (smallest, largest) = find_k_extremes(&set, 3);
        
        assert_eq!(smallest, vec![1, 3, 5]);
        assert_eq!(largest, vec![15, 17, 19]); // 3 largest in ascending order
        
        // Test with k larger than set size
        let (small_all, large_all) = find_k_extremes(&set, 20);
        assert_eq!(small_all.len(), set.len());
        assert_eq!(large_all.len(), set.len());
        
        // Test with k = 0
        let (empty_small, empty_large) = find_k_extremes(&set, 0);
        assert!(empty_small.is_empty());
        assert!(empty_large.is_empty());
    }

    #[test]
    fn test_sliding_window_tracker() {
        let mut tracker = SlidingWindowTracker::new(3);
        
        // Initially empty
        assert_eq!(tracker.get_min(), None);
        assert_eq!(tracker.get_max(), None);
        
        tracker.add(5);
        assert_eq!(tracker.get_min(), Some(5));
        assert_eq!(tracker.get_max(), Some(5));
        assert_eq!(tracker.window_size(), 1);
        
        tracker.add(2);
        tracker.add(8);
        assert_eq!(tracker.get_min(), Some(2));
        assert_eq!(tracker.get_max(), Some(8));
        assert_eq!(tracker.window_size(), 3);
        
        // Adding 4th element should evict first element (5)
        tracker.add(1);
        assert_eq!(tracker.get_min(), Some(1)); // min of [2, 8, 1]
        assert_eq!(tracker.get_max(), Some(8)); // max of [2, 8, 1]
        assert_eq!(tracker.window_size(), 3);
        
        tracker.add(10);
        assert_eq!(tracker.get_min(), Some(1)); // min of [8, 1, 10]
        assert_eq!(tracker.get_max(), Some(10)); // max of [8, 1, 10]
    }

    #[test]
    fn test_merge_sorted_btreemaps() {
        let mut map1 = BTreeMap::new();
        map1.insert("a", 1);
        map1.insert("c", 3);
        
        let mut map2 = BTreeMap::new();
        map2.insert("b", 2);
        map2.insert("c", 5); // Conflict with map1
        
        let mut map3 = BTreeMap::new();
        map3.insert("d", 4);
        map3.insert("c", 7); // Another conflict
        
        let merged = merge_sorted_btreemaps(vec![map1, map2, map3], |v1, v2| v1 + v2);
        
        assert_eq!(merged.get("a"), Some(&1));
        assert_eq!(merged.get("b"), Some(&2));
        assert_eq!(merged.get("c"), Some(&15)); // 3 + 5 + 7
        assert_eq!(merged.get("d"), Some(&4));
        
        // Verify ordering is maintained
        let keys: Vec<_> = merged.keys().collect();
        assert_eq!(keys, vec![&"a", &"b", &"c", &"d"]);
    }

    #[test]
    fn test_ordered_set_operations() {
        let set_a: BTreeSet<i32> = [1, 3, 5, 7].iter().cloned().collect();
        let set_b: BTreeSet<i32> = [3, 5, 7, 9].iter().cloned().collect();
        
        let (union, intersection, difference) = ordered_set_operations(&set_a, &set_b);
        
        let expected_union: BTreeSet<i32> = [1, 3, 5, 7, 9].iter().cloned().collect();
        let expected_intersection: BTreeSet<i32> = [3, 5, 7].iter().cloned().collect();
        let expected_difference: BTreeSet<i32> = [1].iter().cloned().collect();
        
        assert_eq!(union, expected_union);
        assert_eq!(intersection, expected_intersection);
        assert_eq!(difference, expected_difference);
    }

    #[test]
    fn test_find_consecutive_ranges() {
        let set: BTreeSet<i32> = [1, 2, 3, 5, 6, 10, 11, 12, 13, 20].iter().cloned().collect();
        let ranges = find_consecutive_ranges(&set);
        
        // Expected ranges: [1,3], [5,6], [10,13], and [20,20]
        assert_eq!(ranges.len(), 4);
        assert!(ranges.contains(&(1, 3)));
        assert!(ranges.contains(&(5, 6)));
        assert!(ranges.contains(&(10, 13)));
        assert!(ranges.contains(&(20, 20)));
    }

    #[test]
    fn test_timeline() {
        let mut timeline = Timeline::new();
        
        let event1 = Event { id: 1, description: "Meeting".to_string(), duration: 60 };
        let event2 = Event { id: 2, description: "Call".to_string(), duration: 30 };
        let event3 = Event { id: 3, description: "Lunch".to_string(), duration: 90 };
        
        timeline.add_event(1000, event1.clone());
        timeline.add_event(1030, event2.clone());
        timeline.add_event(1200, event3.clone());
        
        // Test range queries
        let events_in_range = timeline.get_events_in_range(1000, 1100);
        assert_eq!(events_in_range.len(), 2);
        
        // Test next event
        let next = timeline.get_next_event(1050);
        assert!(next.is_some());
        assert_eq!(next.unwrap().1.id, 3);
        
        // Test overlapping events (meeting from 1000-1060 overlaps with call at 1030-1060)
        let overlapping = timeline.get_overlapping_events(1020, 40); // 1020-1060
        assert!(!overlapping.is_empty());
        
        // Test removal
        timeline.remove_events_before(1100);
        let remaining = timeline.get_events_in_range(0, 2000);
        assert_eq!(remaining.len(), 1); // Only lunch should remain
    }

    #[test]
    fn test_performance_comparison() {
        let (btree_insert, hash_insert, btree_lookup, hash_lookup, btree_iter, hash_iter) 
            = performance_comparison(1000);
        
        // All operations should complete reasonably quickly
        assert!(btree_insert < std::time::Duration::from_secs(1));
        assert!(hash_insert < std::time::Duration::from_secs(1));
        assert!(btree_lookup < std::time::Duration::from_secs(1));
        assert!(hash_lookup < std::time::Duration::from_secs(1));
        assert!(btree_iter < std::time::Duration::from_secs(1));
        assert!(hash_iter < std::time::Duration::from_secs(1));
        
        println!("BTreeMap insert: {:?}", btree_insert);
        println!("HashMap insert: {:?}", hash_insert);
        println!("BTreeMap lookup: {:?}", btree_lookup);
        println!("HashMap lookup: {:?}", hash_lookup);
        println!("BTreeMap iteration: {:?}", btree_iter);
        println!("HashMap iteration: {:?}", hash_iter);
    }

    #[test]
    fn test_custom_ordered_set() {
        let set = custom_ordered_set();
        assert!(!set.is_empty());
        
        // Convert to vector to test ordering
        let ordered: Vec<_> = set.iter().cloned().collect();
        
        // Verify that strings are ordered by length first, then lexicographically
        for i in 1..ordered.len() {
            let prev = &ordered[i-1];
            let curr = &ordered[i];
            
            if prev.0.len() == curr.0.len() {
                // Same length, should be lexicographically ordered
                assert!(prev.0 <= curr.0);
            } else {
                // Different lengths, shorter should come first
                assert!(prev.0.len() < curr.0.len());
            }
        }
    }

    #[test]
    fn test_length_first_string_ordering() {
        let mut set = BTreeSet::new();
        
        set.insert(LengthFirstString("zoo".to_string()));      // length 3
        set.insert(LengthFirstString("a".to_string()));        // length 1
        set.insert(LengthFirstString("apple".to_string()));    // length 5
        set.insert(LengthFirstString("an".to_string()));       // length 2
        set.insert(LengthFirstString("at".to_string()));       // length 2
        
        let ordered: Vec<_> = set.iter().map(|s| &s.0).collect();
        
        // Should be: "a" (len 1), "an", "at" (len 2, alphabetical), "zoo" (len 3), "apple" (len 5)
        assert_eq!(ordered, vec!["a", "an", "at", "zoo", "apple"]);
    }

    #[test]
    fn test_find_predecessor_successor() {
        let set: BTreeSet<i32> = [1, 3, 5, 7, 9, 11].iter().cloned().collect();
        
        // Target in set
        let (pred, succ) = find_predecessor_successor(&set, &5);
        assert_eq!(pred, Some(3));
        assert_eq!(succ, Some(7));
        
        // Target not in set
        let (pred, succ) = find_predecessor_successor(&set, &6);
        assert_eq!(pred, Some(5));
        assert_eq!(succ, Some(7));
        
        // Target smaller than all elements
        let (pred, succ) = find_predecessor_successor(&set, &0);
        assert_eq!(pred, None);
        assert_eq!(succ, Some(1));
        
        // Target larger than all elements
        let (pred, succ) = find_predecessor_successor(&set, &20);
        assert_eq!(pred, Some(11));
        assert_eq!(succ, None);
    }

    #[test]
    fn test_simple_index() {
        let mut index = SimpleIndex::new();
        
        index.insert("apple".to_string(), 1);
        index.insert("application".to_string(), 2);
        index.insert("apply".to_string(), 3);
        index.insert("banana".to_string(), 4);
        index.insert("band".to_string(), 5);
        
        // Test exact match
        let apple_rows = index.find_exact("apple");
        assert_eq!(apple_rows, vec![1]);
        
        // Test range query
        let app_range = index.find_range("app", "apq");
        assert!(app_range.contains(&1)); // apple
        assert!(app_range.contains(&2)); // application
        assert!(app_range.contains(&3)); // apply
        assert!(!app_range.contains(&4)); // banana not in range
        
        // Test prefix search
        let app_prefix = index.find_prefix("app");
        assert!(app_prefix.contains(&1)); // apple
        assert!(app_prefix.contains(&2)); // application
        assert!(app_prefix.contains(&3)); // apply
        
        let ban_prefix = index.find_prefix("ban");
        assert!(ban_prefix.contains(&4)); // banana
        assert!(ban_prefix.contains(&5)); // band
        
        // Test removal
        index.remove("apple", 1);
        let apple_after_remove = index.find_exact("apple");
        assert!(apple_after_remove.is_empty());
    }

    #[test]
    fn test_stepped_union() {
        let set1: BTreeSet<i32> = [1, 3].iter().cloned().collect();
        let set2: BTreeSet<i32> = [2, 4].iter().cloned().collect();
        let set3: BTreeSet<i32> = [3, 5].iter().cloned().collect();
        
        let steps = stepped_union(vec![set1, set2, set3]);
        
        assert_eq!(steps.len(), 3);
        
        // Step 0: just set1
        let expected_step0: BTreeSet<i32> = [1, 3].iter().cloned().collect();
        assert_eq!(steps[0], expected_step0);
        
        // Step 1: set1 ∪ set2
        let expected_step1: BTreeSet<i32> = [1, 2, 3, 4].iter().cloned().collect();
        assert_eq!(steps[1], expected_step1);
        
        // Step 2: set1 ∪ set2 ∪ set3
        let expected_step2: BTreeSet<i32> = [1, 2, 3, 4, 5].iter().cloned().collect();
        assert_eq!(steps[2], expected_step2);
    }

    #[test]
    fn test_edge_cases() {
        // Test with empty BTreeMap
        let empty_map: BTreeMap<i32, &str> = BTreeMap::new();
        let empty_range = range_query(&empty_map, &1, &10);
        assert!(empty_range.is_empty());
        
        // Test splitting empty map
        let (left, right) = split_btree_map(empty_map, &5);
        assert!(left.is_empty());
        assert!(right.is_empty());
        
        // Test with empty BTreeSet
        let empty_set: BTreeSet<i32> = BTreeSet::new();
        let (empty_small, empty_large) = find_k_extremes(&empty_set, 3);
        assert!(empty_small.is_empty());
        assert!(empty_large.is_empty());
        
        // Test consecutive ranges with single elements
        let single_set: BTreeSet<i32> = [5].iter().cloned().collect();
        let single_ranges = find_consecutive_ranges(&single_set);
        assert_eq!(single_ranges, vec![(5, 5)]);
        
        // Test predecessor/successor with empty set
        let empty_set: BTreeSet<i32> = BTreeSet::new();
        let (no_pred, no_succ) = find_predecessor_successor(&empty_set, &5);
        assert_eq!(no_pred, None);
        assert_eq!(no_succ, None);
    }
}
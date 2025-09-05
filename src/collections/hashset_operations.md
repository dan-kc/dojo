# HashSet Operations Solutions

## Solutions

```rust
use std::collections::{HashMap, HashSet};
use std::thread;

fn union_all_sets<T>(sets: Vec<HashSet<T>>) -> HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    sets.into_iter()
        .fold(HashSet::new(), |mut acc, set| {
            acc.extend(set);
            acc
        })
}

fn intersect_all_sets<T>(sets: Vec<HashSet<T>>) -> HashSet<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    if sets.is_empty() {
        return HashSet::new();
    }
    
    sets.into_iter()
        .reduce(|acc, set| acc.intersection(&set).cloned().collect())
        .unwrap_or_else(HashSet::new)
}

fn find_unique_elements<T>(
    sets: Vec<HashSet<T>>,
) -> HashMap<T, usize>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut element_counts: HashMap<T, Vec<usize>> = HashMap::new();
    
    // Track which sets each element appears in
    for (set_index, set) in sets.iter().enumerate() {
        for element in set {
            element_counts
                .entry(element.clone())
                .or_insert_with(Vec::new)
                .push(set_index);
        }
    }
    
    // Return elements that appear in exactly one set
    element_counts
        .into_iter()
        .filter_map(|(element, set_indices)| {
            if set_indices.len() == 1 {
                Some((element, set_indices[0]))
            } else {
                None
            }
        })
        .collect()
}

fn partition_set<T, F>(
    set: HashSet<T>,
    predicate: F,
) -> (HashSet<T>, HashSet<T>)
where
    T: Clone + std::hash::Hash + Eq,
    F: Fn(&T) -> bool,
{
    let mut matching = HashSet::new();
    let mut non_matching = HashSet::new();
    
    for element in set {
        if predicate(&element) {
            matching.insert(element);
        } else {
            non_matching.insert(element);
        }
    }
    
    (matching, non_matching)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Person {
    name: String,
    age: u32,
    email: String,
}

impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash based on email only (treating email as unique identifier)
        self.email.hash(state);
    }
}

impl Person {
    fn new(name: String, age: u32, email: String) -> Self {
        Self { name, age, email }
    }
}

fn person_set_operations() -> HashSet<Person> {
    let mut set = HashSet::new();
    
    // These people have the same email, so only one will be kept in the set
    let person1 = Person::new("John".to_string(), 30, "john@example.com".to_string());
    let person2 = Person::new("Johnny".to_string(), 31, "john@example.com".to_string()); // Same email
    let person3 = Person::new("Jane".to_string(), 25, "jane@example.com".to_string());
    
    set.insert(person1);
    set.insert(person2); // Won't be inserted due to same email
    set.insert(person3);
    
    set
}

fn spell_check(
    text: &str,
    dictionary: &HashSet<String>,
) -> HashSet<String> {
    text.split_whitespace()
        .map(|word| word.to_lowercase().trim_matches(|c: char| !c.is_alphabetic()).to_string())
        .filter(|word| !word.is_empty() && !dictionary.contains(word))
        .collect()
}

fn find_connected_components(
    edges: Vec<(i32, i32)>,
) -> Vec<HashSet<i32>> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    let mut all_nodes = HashSet::new();
    
    // Build adjacency list
    for (u, v) in edges {
        graph.entry(u).or_insert_with(HashSet::new).insert(v);
        graph.entry(v).or_insert_with(HashSet::new).insert(u);
        all_nodes.insert(u);
        all_nodes.insert(v);
    }
    
    let mut visited = HashSet::new();
    let mut components = Vec::new();
    
    // DFS to find connected components
    fn dfs(
        node: i32,
        graph: &HashMap<i32, HashSet<i32>>,
        visited: &mut HashSet<i32>,
        component: &mut HashSet<i32>,
    ) {
        visited.insert(node);
        component.insert(node);
        
        if let Some(neighbors) = graph.get(&node) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    dfs(neighbor, graph, visited, component);
                }
            }
        }
    }
    
    for &node in &all_nodes {
        if !visited.contains(&node) {
            let mut component = HashSet::new();
            dfs(node, &graph, &mut visited, &mut component);
            components.push(component);
        }
    }
    
    components
}

fn analyze_duplicates<T>(
    collections: Vec<Vec<T>>,
) -> (HashSet<T>, HashMap<T, usize>)
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut frequencies = HashMap::new();
    let mut duplicates = HashSet::new();
    
    for collection in collections {
        for item in collection {
            let count = frequencies.entry(item.clone()).or_insert(0);
            *count += 1;
            
            if *count > 1 {
                duplicates.insert(item);
            }
        }
    }
    
    (duplicates, frequencies)
}

fn power_set<T>(
    set: HashSet<T>,
) -> Vec<HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    let elements: Vec<T> = set.into_iter().collect();
    let mut power_set = Vec::new();
    let n = elements.len();
    
    // Generate all 2^n subsets using binary representation
    for i in 0..(1 << n) {
        let mut subset = HashSet::new();
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                subset.insert(elements[j].clone());
            }
        }
        power_set.push(subset);
    }
    
    power_set
}

fn jaccard_similarity<T>(
    set_a: &HashSet<T>,
    set_b: &HashSet<T>,
) -> f64
where
    T: std::hash::Hash + Eq,
{
    if set_a.is_empty() && set_b.is_empty() {
        return 1.0; // Both empty sets are identical
    }
    
    let intersection_size = set_a.intersection(set_b).count();
    let union_size = set_a.union(set_b).count();
    
    if union_size == 0 {
        0.0
    } else {
        intersection_size as f64 / union_size as f64
    }
}

struct SetTracker<T> {
    current_set: HashSet<T>,
    additions: Vec<T>,
    removals: Vec<T>,
}

impl<T> SetTracker<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    fn new() -> Self {
        Self {
            current_set: HashSet::new(),
            additions: Vec::new(),
            removals: Vec::new(),
        }
    }

    fn insert(&mut self, item: T) -> bool {
        let was_new = self.current_set.insert(item.clone());
        if was_new {
            self.additions.push(item);
        }
        was_new
    }

    fn remove(&mut self, item: &T) -> bool {
        let was_present = self.current_set.remove(item);
        if was_present {
            self.removals.push(item.clone());
        }
        was_present
    }

    fn contains(&self, item: &T) -> bool {
        self.current_set.contains(item)
    }

    fn addition_history(&self) -> &[T] {
        &self.additions
    }

    fn removal_history(&self) -> &[T] {
        &self.removals
    }

    fn current_set(&self) -> &HashSet<T> {
        &self.current_set
    }
}

fn find_mutual_friends(
    friendships: &HashMap<String, HashSet<String>>,
    person1: &str,
    person2: &str,
) -> HashSet<String> {
    let friends1 = friendships.get(person1);
    let friends2 = friendships.get(person2);
    
    match (friends1, friends2) {
        (Some(f1), Some(f2)) => f1.intersection(f2).cloned().collect(),
        _ => HashSet::new(),
    }
}

fn dedup_preserve_order<T>(items: Vec<T>) -> Vec<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut seen = HashSet::new();
    let mut result = Vec::new();
    
    for item in items {
        if seen.insert(item.clone()) {
            result.push(item);
        }
    }
    
    result
}

fn difference_chain<T>(
    sets: Vec<HashSet<T>>,
) -> Vec<HashSet<T>>
where
    T: Clone + std::hash::Hash + Eq,
{
    let mut result = Vec::new();
    
    for i in 0..sets.len().saturating_sub(1) {
        let difference: HashSet<T> = sets[i].difference(&sets[i + 1]).cloned().collect();
        result.push(difference);
    }
    
    result
}
```

## Explanation

This comprehensive solution demonstrates **advanced HashSet operations** and **set-based algorithms**:

### Key Concepts Demonstrated:

1. **Set Operations**:
   - **Union**: Combines all unique elements from multiple sets
   - **Intersection**: Finds common elements across all sets
   - **Difference**: Elements in one set but not another
   - **Symmetric Difference**: Elements in either set but not both

2. **Custom Hash Implementation**:
   - `Person` struct uses email-only hashing for unique identification
   - Demonstrates how hash and equality can be customized
   - Shows that objects can be equal even with different field values

3. **Set-Based Algorithms**:
   - **Connected Components**: Graph traversal using DFS with visited tracking
   - **Spell Checking**: Word validation against dictionary sets
   - **Duplicate Analysis**: Frequency counting with set tracking

4. **Advanced Data Structures**:
   - **SetTracker**: Maintains history of set operations
   - **Power Set Generation**: Creates all possible subsets using bit manipulation
   - **Jaccard Similarity**: Statistical measure of set similarity

### Performance Characteristics:

**HashSet Operations:**
- **Insert/Remove/Contains**: O(1) average, O(n) worst case
- **Set Operations**: O(min(|A|, |B|)) for intersection, O(|A| + |B|) for union
- **Memory**: Hash table overhead, load factor affects performance

### Algorithmic Patterns:

1. **Graph Connected Components**:
   - Uses HashSet for visited tracking in DFS
   - Adjacency list representation with HashSet for neighbors
   - O(V + E) time complexity for DFS traversal

2. **Power Set Generation**:
   - Binary representation to enumerate all subsets
   - Each bit position represents including/excluding an element
   - Generates 2^n subsets for n elements

3. **Order-Preserving Deduplication**:
   - HashSet tracks seen elements in O(1) time
   - Preserves first occurrence order unlike HashSet dedup
   - Common pattern in data processing pipelines

### Hash Function Design:

```rust
impl Hash for Person {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.email.hash(state); // Only email determines uniqueness
    }
}

// This means:
Person { name: "John", age: 30, email: "john@email.com" }
Person { name: "Johnny", age: 25, email: "john@email.com" }
// Are considered the same person in HashSet operations
```

### Real-World Applications:

- **Social Networks**: Finding mutual friends, connected components
- **Data Deduplication**: Removing duplicates while preserving order  
- **Text Processing**: Spell checking, word frequency analysis
- **Recommendation Systems**: Jaccard similarity for user preferences
- **Database Operations**: Set-based queries and joins

### Memory and Performance Trade-offs:

1. **Space Efficiency**: HashSet uses more memory than sorted vectors
2. **Time Complexity**: O(1) operations vs O(log n) for BTreeSet
3. **Hash Quality**: Poor hash functions can degrade to O(n) performance
4. **Load Factor**: HashSet automatically resizes to maintain performance

### Set Theory Applications:

- **Venn Diagrams**: Union, intersection, difference operations
- **Boolean Logic**: Set operations correspond to logical operations
- **Database Joins**: Set intersection for inner joins
- **Permissions**: Set operations for role-based access control

This solution showcases HashSet as a fundamental data structure for efficient membership testing, deduplication, and set-based algorithms, while demonstrating how to customize hash behavior for domain-specific requirements.
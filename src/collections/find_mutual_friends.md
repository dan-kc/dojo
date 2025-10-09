# Social Network Mutual Friends Solution

## Implementation

```rust
pub fn find_mutual_friends(
    friendships: &std::collections::HashMap<String, std::collections::HashSet<String>>,
    person1: &str,
    person2: &str,
) -> std::collections::HashSet<String> {
    if let (Some(friends), Some(friends_2)) = (friendships.get(person1), friendships.get(person2)) {
        return friends.intersection(&friends_2).cloned().collect();
    };
    std::collections::HashSet::new()
}
```

## Alternative Implementation (Handle Same Person)

```rust
pub fn find_mutual_friends(
    friendships: &std::collections::HashMap<String, std::collections::HashSet<String>>,
    person1: &str,
    person2: &str,
) -> std::collections::HashSet<String> {
    // Handle edge case: same person
    if person1 == person2 {
        return std::collections::HashSet::new();
    }
    
    let friends1 = friendships.get(person1);
    let friends2 = friendships.get(person2);
    
    match (friends1, friends2) {
        (Some(f1), Some(f2)) => {
            f1.intersection(f2).cloned().collect()
        }
        _ => std::collections::HashSet::new(),
    }
}
```

## Explanation

This solution finds common friends between two people:

1. **Data lookup**: Retrieves friend lists from the friendship HashMap
2. **Missing data handling**: Returns empty set if either person is not found
3. **Set intersection**: Uses HashSet intersection to find common friends
4. **Result collection**: Clones and collects intersection results

## Key Learning Points

- **Social network algorithms**: Modeling relationships with HashMaps and HashSets
- **Set intersection**: Finding common elements between two sets
- **Option handling**: Graceful handling of missing data
- **Graph traversal patterns**: Social networks as graph data structures

## Enhanced Implementation

```rust
pub fn find_mutual_friends_detailed(
    friendships: &std::collections::HashMap<String, std::collections::HashSet<String>>,
    person1: &str,
    person2: &str,
) -> (std::collections::HashSet<String>, usize, usize) {
    let empty_set = std::collections::HashSet::new();
    
    let friends1 = friendships.get(person1).unwrap_or(&empty_set);
    let friends2 = friendships.get(person2).unwrap_or(&empty_set);
    
    let mutual_friends: std::collections::HashSet<String> = 
        friends1.intersection(friends2).cloned().collect();
    
    (mutual_friends, friends1.len(), friends2.len())
}
```

## Use Cases

- **Social media platforms**: Suggesting mutual connections
- **Professional networks**: Finding common business contacts
- **Recommendation systems**: Friend-of-friend suggestions
- **Community analysis**: Identifying shared group memberships

## Algorithm Complexity

- **Time**: O(min(|F1|, |F2|)) where F1, F2 are friend sets
- **Space**: O(|intersection|) for storing mutual friends
- **Lookup**: O(1) average case for HashMap access

## Rust Concepts Demonstrated

- HashMap and HashSet composition for graph modeling
- Set intersection operations
- Option handling with pattern matching
- Iterator processing with `cloned()` and `collect()`
- Social network data structure patterns

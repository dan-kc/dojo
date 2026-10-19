# collections/find_mutual_friends

Reference implementation. Read after attempting the exercise.

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

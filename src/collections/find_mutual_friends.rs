// cargo test find_mutual_friends

/// Implement set-based algorithm for finding common friends in a social network.
/// Given friendship connections, find mutual friends between two people.
pub fn find_mutual_friends(
    friendships: &std::collections::HashMap<String, std::collections::HashSet<String>>,
    person1: &str,
    person2: &str,
) -> std::collections::HashSet<String> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashMap, HashSet};

    #[test]
    fn test_find_mutual_friends() {
        let mut friendships = HashMap::new();

        friendships.insert(
            "Alice".to_string(),
            ["Bob", "Charlie", "David"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        friendships.insert(
            "Bob".to_string(),
            ["Alice", "Charlie", "Eve"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        friendships.insert(
            "Charlie".to_string(),
            ["Alice", "Bob", "David"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        let mutual = find_mutual_friends(&friendships, "Alice", "Bob");
        let expected: HashSet<String> = ["Charlie"].iter().map(|s| s.to_string()).collect();

        assert_eq!(mutual, expected);

        // Test with no mutual friends
        friendships.insert(
            "Frank".to_string(),
            ["Eve"].iter().map(|s| s.to_string()).collect(),
        );
        let no_mutual = find_mutual_friends(&friendships, "Alice", "Frank");
        assert!(no_mutual.is_empty());
    }

    #[test]
    fn test_find_mutual_friends_no_friends() {
        let mut friendships = HashMap::new();

        // Person with empty friend list
        friendships.insert("Lonely".to_string(), HashSet::new());
        friendships.insert(
            "Alice".to_string(),
            ["Bob"].iter().map(|s| s.to_string()).collect(),
        );

        let mutual = find_mutual_friends(&friendships, "Lonely", "Alice");
        assert!(mutual.is_empty());
    }

    #[test]
    fn test_find_mutual_friends_person_not_found() {
        let mut friendships = HashMap::new();

        friendships.insert(
            "Alice".to_string(),
            ["Bob", "Charlie"].iter().map(|s| s.to_string()).collect(),
        );

        // One person not in the system
        let mutual = find_mutual_friends(&friendships, "Alice", "Unknown");
        assert!(mutual.is_empty());

        // Both persons not in the system
        let mutual = find_mutual_friends(&friendships, "Unknown1", "Unknown2");
        assert!(mutual.is_empty());
    }

    #[test]
    fn test_find_mutual_friends_same_person() {
        let mut friendships = HashMap::new();

        friendships.insert(
            "Alice".to_string(),
            ["Bob", "Charlie", "David"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        // Mutual friends with oneself should return all friends or empty set
        let mutual = find_mutual_friends(&friendships, "Alice", "Alice");
        // This could be empty (no mutual friends with self) or all friends (depending on implementation)
        // Test that it doesn't crash and returns a valid HashSet
        assert!(mutual.len() <= 3);
    }

    #[test]
    fn test_find_mutual_friends_multiple_mutual() {
        let mut friendships = HashMap::new();

        friendships.insert(
            "Alice".to_string(),
            ["Bob", "Charlie", "David", "Eve"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        friendships.insert(
            "Frank".to_string(),
            ["Bob", "Charlie", "Grace", "Henry"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        let mutual = find_mutual_friends(&friendships, "Alice", "Frank");
        let expected: HashSet<String> = ["Bob", "Charlie"].iter().map(|s| s.to_string()).collect();

        assert_eq!(mutual, expected);
    }

    #[test]
    fn test_find_mutual_friends_all_same_friends() {
        let mut friendships = HashMap::new();

        let common_friends: HashSet<String> = ["Friend1", "Friend2", "Friend3"]
            .iter()
            .map(|s| s.to_string())
            .collect();

        friendships.insert("Alice".to_string(), common_friends.clone());
        friendships.insert("Bob".to_string(), common_friends.clone());

        let mutual = find_mutual_friends(&friendships, "Alice", "Bob");
        assert_eq!(mutual, common_friends);
    }

    #[test]
    fn test_find_mutual_friends_asymmetric_friendship() {
        let mut friendships = HashMap::new();

        // Alice considers Bob a friend, but Bob's friends don't include Alice
        friendships.insert(
            "Alice".to_string(),
            ["Bob", "Charlie", "David"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        friendships.insert(
            "Bob".to_string(),
            ["Charlie", "Eve", "Frank"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        let mutual = find_mutual_friends(&friendships, "Alice", "Bob");
        let expected: HashSet<String> = ["Charlie"].iter().map(|s| s.to_string()).collect();

        assert_eq!(mutual, expected);
    }

    #[test]
    fn test_find_mutual_friends_large_network() {
        let mut friendships = HashMap::new();

        // Create a larger network
        let person1_friends: HashSet<String> = (1..=50).map(|i| format!("Friend{}", i)).collect();
        let person2_friends: HashSet<String> = (25..=75).map(|i| format!("Friend{}", i)).collect();

        friendships.insert("Person1".to_string(), person1_friends);
        friendships.insert("Person2".to_string(), person2_friends);

        let mutual = find_mutual_friends(&friendships, "Person1", "Person2");

        // Should have friends 25-50 in common
        let expected: HashSet<String> = (25..=50).map(|i| format!("Friend{}", i)).collect();
        assert_eq!(mutual, expected);
        assert_eq!(mutual.len(), 26);
    }

    #[test]
    fn test_find_mutual_friends_single_mutual() {
        let mut friendships = HashMap::new();

        friendships.insert(
            "Alice".to_string(),
            ["SharedFriend", "AliceUnique1", "AliceUnique2"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        friendships.insert(
            "Bob".to_string(),
            ["SharedFriend", "BobUnique1", "BobUnique2"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        let mutual = find_mutual_friends(&friendships, "Alice", "Bob");
        let expected: HashSet<String> = ["SharedFriend"].iter().map(|s| s.to_string()).collect();

        assert_eq!(mutual, expected);
        assert_eq!(mutual.len(), 1);
    }

    #[test]
    fn test_find_mutual_friends_circular_relationships() {
        let mut friendships = HashMap::new();

        // Create a circular friend relationship
        friendships.insert(
            "A".to_string(),
            ["B", "C"].iter().map(|s| s.to_string()).collect(),
        );
        friendships.insert(
            "B".to_string(),
            ["A", "C"].iter().map(|s| s.to_string()).collect(),
        );
        friendships.insert(
            "C".to_string(),
            ["A", "B"].iter().map(|s| s.to_string()).collect(),
        );

        let mutual_ab = find_mutual_friends(&friendships, "A", "B");
        let expected: HashSet<String> = ["C"].iter().map(|s| s.to_string()).collect();

        assert_eq!(mutual_ab, expected);

        let mutual_bc = find_mutual_friends(&friendships, "B", "C");
        let expected: HashSet<String> = ["A"].iter().map(|s| s.to_string()).collect();

        assert_eq!(mutual_bc, expected);
    }

    #[test]
    fn test_find_mutual_friends_empty_network() {
        let friendships = HashMap::new();

        let mutual = find_mutual_friends(&friendships, "Anyone", "Someone");
        assert!(mutual.is_empty());
    }

    #[test]
    fn test_find_mutual_friends_order_independence() {
        let mut friendships = HashMap::new();

        friendships.insert(
            "Alice".to_string(),
            ["Friend1", "Friend2", "Friend3"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );
        friendships.insert(
            "Bob".to_string(),
            ["Friend2", "Friend3", "Friend4"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        );

        let mutual1 = find_mutual_friends(&friendships, "Alice", "Bob");
        let mutual2 = find_mutual_friends(&friendships, "Bob", "Alice");

        // Results should be the same regardless of parameter order
        assert_eq!(mutual1, mutual2);

        let expected: HashSet<String> = ["Friend2", "Friend3"]
            .iter()
            .map(|s| s.to_string())
            .collect();
        assert_eq!(mutual1, expected);
    }
}

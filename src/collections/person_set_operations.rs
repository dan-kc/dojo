// cargo test person_set_operations

/// Custom hashable type for testing HashSet with complex objects.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, #[allow(unused_variables)] state: &mut H) {
        todo!()
    }
}

impl Person {
    pub fn new(name: String, age: u32, email: String) -> Self {
        Self { name, age, email }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn test_person_hash_and_equality() {
        let person1 = Person::new("John".to_string(), 30, "john@email.com".to_string());
        let person2 = Person::new("Johnny".to_string(), 31, "john@email.com".to_string()); // Same email
        let person3 = Person::new("John".to_string(), 30, "john2@email.com".to_string()); // Different email

        let mut set = HashSet::new();
        set.insert(person1.clone());

        // person2 should be considered equal due to same email
        assert!(!set.insert(person2)); // Should return false (already exists)
        assert_eq!(set.len(), 1);

        // person3 should be different due to different email
        assert!(set.insert(person3)); // Should return true (new addition)
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_person_email_uniqueness() {
        let mut set = HashSet::new();

        // Add persons with same email but different names and ages
        let alice1 = Person::new("Alice".to_string(), 25, "alice@company.com".to_string());
        let alice2 = Person::new("Alicia".to_string(), 26, "alice@company.com".to_string());
        let bob = Person::new("Bob".to_string(), 30, "bob@company.com".to_string());

        assert!(set.insert(alice1));
        assert!(!set.insert(alice2)); // Should fail - same email
        assert!(set.insert(bob));

        assert_eq!(set.len(), 2);

        // Verify the right person is in the set (first one added)
        let alice_in_set = set.iter().find(|p| p.email == "alice@company.com").unwrap();
        assert_eq!(alice_in_set.name, "Alice");
        assert_eq!(alice_in_set.age, 25);
    }

    #[test]
    fn test_person_contains_lookup() {
        let mut set = HashSet::new();

        let person = Person::new("Test".to_string(), 20, "test@email.com".to_string());
        set.insert(person);

        // Should find person with same email but different other attributes
        let lookup_person = Person::new(
            "Different Name".to_string(),
            99,
            "test@email.com".to_string(),
        );
        assert!(set.contains(&lookup_person));

        // Should not find person with different email
        let different_person =
            Person::new("Test".to_string(), 20, "different@email.com".to_string());
        assert!(!set.contains(&different_person));
    }

    #[test]
    fn test_person_removal() {
        let mut set = HashSet::new();

        let person = Person::new("Remove Me".to_string(), 40, "remove@test.com".to_string());
        set.insert(person);
        assert_eq!(set.len(), 1);

        // Remove using a person with same email but different other fields
        let removal_key = Person::new("Different".to_string(), 999, "remove@test.com".to_string());
        assert!(set.remove(&removal_key));
        assert!(set.is_empty());
    }

    #[test]
    fn test_person_edge_cases() {
        let mut set = HashSet::new();

        // Test with empty email
        let empty_email1 = Person::new("Person1".to_string(), 25, "".to_string());
        let empty_email2 = Person::new("Person2".to_string(), 30, "".to_string());

        assert!(set.insert(empty_email1));
        assert!(!set.insert(empty_email2)); // Same empty email
        assert_eq!(set.len(), 1);

        // Test with very long email
        let long_email = "a".repeat(1000) + "@domain.com";
        let long_person = Person::new("Long".to_string(), 35, long_email.clone());
        let long_person2 = Person::new("Also Long".to_string(), 40, long_email);

        assert!(set.insert(long_person));
        assert!(!set.insert(long_person2)); // Same long email
        assert_eq!(set.len(), 2);
    }
}

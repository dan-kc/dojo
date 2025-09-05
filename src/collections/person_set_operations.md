# Custom HashSet with Person Type Solution

## Implementation

```rust
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Hash based on email only - treating email as unique identifier
        self.email.hash(state);
    }
}

impl Person {
    pub fn new(name: String, age: u32, email: String) -> Self {
        Self { name, age, email }
    }
}

pub fn person_set_operations() -> std::collections::HashSet<Person> {
    let mut people_set = std::collections::HashSet::new();
    
    // Demonstrate email-based uniqueness
    let alice1 = Person::new("Alice Smith".to_string(), 25, "alice@company.com".to_string());
    let alice2 = Person::new("Alicia Smith".to_string(), 26, "alice@company.com".to_string()); // Same email, different details
    let bob = Person::new("Bob Jones".to_string(), 30, "bob@company.com".to_string());
    let charlie = Person::new("Charlie Brown".to_string(), 28, "charlie@company.com".to_string());
    let alice3 = Person::new("Alice Johnson".to_string(), 24, "alice@company.com".to_string()); // Same email again
    
    // Insert people - duplicates by email should be rejected
    people_set.insert(alice1);  // Should succeed
    people_set.insert(alice2);  // Should fail - same email as alice1
    people_set.insert(bob);     // Should succeed
    people_set.insert(charlie); // Should succeed
    people_set.insert(alice3);  // Should fail - same email as alice1
    
    people_set
}
```

## Key Implementation Details

### Custom Hash Implementation

The crucial aspect is implementing `Hash` to use only the email field:

```rust
impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}
```

### Equality Consistency

Since `PartialEq` and `Eq` are derived, they compare all fields. However, for HashSet consistency, the hash implementation using only email means:

- Two `Person` objects with same email but different name/age will hash to same value
- They might not be equal by `==`, but HashSet will treat them as duplicates
- This demonstrates the importance of hash/equality consistency in practice

## Explanation

This solution demonstrates using custom types as HashSet keys:

1. **Email-based hashing**: Only the email field contributes to the hash
2. **Unique identifier pattern**: Treating email as the primary key
3. **Hash consistency**: Hash function determines set membership, not equality
4. **Practical uniqueness**: Models real-world scenarios where email is unique

## Key Learning Points

- **Custom Hash traits**: Implementing hash based on specific fields
- **Identity vs Equality**: Hash determines membership, equality determines replacement
- **Unique identifiers**: Using specific fields as primary keys
- **HashSet behavior**: Understanding when elements are considered duplicates

## Design Considerations

```rust
// Alternative: Hash and Equality both based on email only
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.email == other.email
    }
}

impl std::hash::Hash for Person {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.email.hash(state);
    }
}
```

## Rust Concepts Demonstrated

- Custom Hash trait implementation
- HashSet with complex custom types  
- Trait derivation vs manual implementation
- Primary key patterns in data structures
- Collection uniqueness semantics
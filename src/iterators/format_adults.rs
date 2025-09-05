// Format Adults
//
// Learning objectives:
// - Working with custom structs in iterators
// - Combining filter, sort, and map operations
// - Understanding mutable iterator operations
//
// cargo test --bin format_adults

#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

/// Transform a vector of Person structs into a vector of formatted strings
/// containing "Name: {name}, Age: {age}" but only for people over 18,
/// sorted by age in descending order.
pub fn format_adults(people: Vec<Person>) -> Vec<String> {
    todo!("Chain filter(), sort operations, and map()")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_adults() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
                email: "alice@example.com".to_string(),
            },
            Person {
                name: "Bob".to_string(),
                age: 17,
                email: "bob@example.com".to_string(),
            },
            Person {
                name: "Charlie".to_string(),
                age: 30,
                email: "charlie@example.com".to_string(),
            },
            Person {
                name: "Diana".to_string(),
                age: 22,
                email: "diana@example.com".to_string(),
            },
        ];
        let result = format_adults(people);
        assert_eq!(result, vec![
            "Name: Charlie, Age: 30",
            "Name: Alice, Age: 25",
            "Name: Diana, Age: 22",
        ]);
    }

    #[test]
    fn test_format_adults_no_adults() {
        let people = vec![
            Person {
                name: "Kid1".to_string(),
                age: 15,
                email: "kid1@example.com".to_string(),
            },
            Person {
                name: "Kid2".to_string(),
                age: 12,
                email: "kid2@example.com".to_string(),
            },
        ];
        let result = format_adults(people);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_format_adults_empty() {
        let people = vec![];
        let result = format_adults(people);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_format_adults_same_age() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
                email: "alice@example.com".to_string(),
            },
            Person {
                name: "Bob".to_string(),
                age: 25,
                email: "bob@example.com".to_string(),
            },
        ];
        let result = format_adults(people);
        // When ages are equal, maintain original order (stable sort)
        assert_eq!(result, vec![
            "Name: Alice, Age: 25",
            "Name: Bob, Age: 25",
        ]);
    }

    #[test]
    fn test_format_adults_exactly_18() {
        let people = vec![
            Person {
                name: "Just18".to_string(),
                age: 18,
                email: "just18@example.com".to_string(),
            },
        ];
        let result = format_adults(people);
        assert_eq!(result, vec![]); // 18 is not "over 18"
    }
}

fn main() {
    println!("Run tests with: cargo test --bin format_adults");
}
// String Formatting and Transformation Practice
//
// Learning Objectives:
// - Format data into structured string representations
// - Use Rust's formatting macros and string interpolation
// - Handle optional data and provide sensible defaults
// - Practice with string building and memory efficiency
// - Implement custom display logic for data structures
//
// Run tests with: cargo test --lib string_manipulation::string_formatter

/// Represents a person with optional contact information
#[derive(Debug)]
pub struct PersonData {
    pub first_name: String,
    pub last_name: String,
    pub age: Option<u32>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub address: Option<String>,
}

/// Formats person data into a human-readable string representation.
///
/// The format should follow these rules:
/// - Always include first and last name: "FirstName LastName"
/// - If age is provided, add it in parentheses: "FirstName LastName (age: 25)"
/// - Add contact information on separate lines if available:
///   - "Email: email@example.com"
///   - "Phone: +1-555-0123"
///   - "Address: 123 Main St, City, State"
/// - If no contact information is available, add "No contact information available"
/// - Trim whitespace and handle empty optional fields gracefully
///
/// # Arguments
/// * `person` - The PersonData to format
///
/// # Returns
/// A formatted string representation of the person
///
/// # Examples
/// ```
/// let person = PersonData {
///     first_name: "John".to_string(),
///     last_name: "Doe".to_string(),
///     age: Some(30),
///     email: Some("john@example.com".to_string()),
///     phone: None,
///     address: None,
/// };
/// let formatted = format_person_data(&person);
/// // Should contain name, age, email, but indicate missing phone/address
/// ```
pub fn format_person_data(person: &PersonData) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complete_person_data() {
        let person = PersonData {
            first_name: "John".to_string(),
            last_name: "Doe".to_string(),
            age: Some(30),
            email: Some("john.doe@example.com".to_string()),
            phone: Some("+1-555-0123".to_string()),
            address: Some("123 Main St, Springfield, IL".to_string()),
        };

        let result = format_person_data(&person);

        assert!(result.contains("John Doe (age: 30)"));
        assert!(result.contains("Email: john.doe@example.com"));
        assert!(result.contains("Phone: +1-555-0123"));
        assert!(result.contains("Address: 123 Main St, Springfield, IL"));
        assert!(!result.contains("No contact information"));
    }

    #[test]
    fn test_minimal_person_data() {
        let person = PersonData {
            first_name: "Jane".to_string(),
            last_name: "Smith".to_string(),
            age: None,
            email: None,
            phone: None,
            address: None,
        };

        let result = format_person_data(&person);

        assert!(result.contains("Jane Smith"));
        assert!(!result.contains("age:"));
        assert!(result.contains("No contact information available"));
    }

    #[test]
    fn test_partial_contact_information() {
        let person = PersonData {
            first_name: "Alice".to_string(),
            last_name: "Johnson".to_string(),
            age: Some(25),
            email: Some("alice@example.com".to_string()),
            phone: None,
            address: Some("456 Oak Ave, Boston, MA".to_string()),
        };

        let result = format_person_data(&person);

        assert!(result.contains("Alice Johnson (age: 25)"));
        assert!(result.contains("Email: alice@example.com"));
        assert!(result.contains("Address: 456 Oak Ave, Boston, MA"));
        assert!(!result.contains("Phone:"));
        assert!(!result.contains("No contact information"));
    }

    #[test]
    fn test_whitespace_handling() {
        let person = PersonData {
            first_name: "  Bob  ".to_string(),
            last_name: "  Wilson  ".to_string(),
            age: Some(35),
            email: Some("  bob@example.com  ".to_string()),
            phone: Some("".to_string()), // Empty string should be treated as None
            address: Some("   ".to_string()), // Whitespace-only should be treated as None
        };

        let result = format_person_data(&person);

        assert!(result.contains("Bob Wilson (age: 35)"));
        assert!(result.contains("Email: bob@example.com"));
        assert!(!result.contains("Phone:"));
        assert!(!result.contains("Address:"));
    }

    #[test]
    fn test_zero_age() {
        let person = PersonData {
            first_name: "Baby".to_string(),
            last_name: "Doe".to_string(),
            age: Some(0),
            email: None,
            phone: None,
            address: None,
        };

        let result = format_person_data(&person);

        assert!(result.contains("Baby Doe (age: 0)"));
    }

    #[test]
    fn test_formatting_consistency() {
        let person = PersonData {
            first_name: "Test".to_string(),
            last_name: "User".to_string(),
            age: Some(42),
            email: Some("test@example.com".to_string()),
            phone: Some("555-1234".to_string()),
            address: None,
        };

        let result = format_person_data(&person);

        // Check that the format is consistent and readable
        let lines: Vec<&str> = result.lines().collect();
        assert!(lines.len() >= 3); // Name line + at least 2 contact lines

        // First line should be the name and age
        assert!(lines[0].contains("Test User (age: 42)"));

        // Contact information should be on separate lines
        assert!(lines
            .iter()
            .any(|line| line.contains("Email: test@example.com")));
        assert!(lines.iter().any(|line| line.contains("Phone: 555-1234")));
    }
}

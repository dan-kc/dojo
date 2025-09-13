// Move Semantics and Ownership Transfer Practice
//
// Learning Objectives:
// - Understand when moves occur in Rust
// - Practice ownership transfer patterns
// - Handle moved values and borrowing conflicts
// - Work with partial moves and field access
// - Implement functions that take ownership vs borrow
//
// Run with: cargo test --bin move_semantics

/// Implement a function that takes ownership of a String and returns its length
/// after performing some transformations. The original string should be moved.
fn process_owned_string(s: String) -> usize {
    todo!("Implement process_owned_string")
}

/// Create a function that takes a vector by value, processes it, and returns
/// both the processed result and gives back ownership of a modified version.
fn transform_and_return(mut vec: Vec<i32>, multiplier: i32) -> (Vec<i32>, i32) {
    todo!("Implement transform_and_return")
}

/// Demonstrate partial moves with a struct containing different field types.
#[derive(Debug)]
struct PersonData {
    name: String,
    age: u32,
    hobbies: Vec<String>,
}

impl PersonData {
    fn new(name: String, age: u32, hobbies: Vec<String>) -> Self {
        Self { name, age, hobbies }
    }

    /// Extract the name, leaving the rest of the struct intact.
    /// This should demonstrate partial moves.
    fn extract_name(self) -> (String, u32, Vec<String>) {
        todo!("Implement extract_name")
    }

    /// Move out hobbies while keeping other fields accessible.
    fn take_hobbies(&mut self) -> Vec<String> {
        todo!("Implement take_hobbies")
    }
}

/// Implement a swap function that exchanges ownership of two values.
fn swap_ownership<T>(a: T, b: T) -> (T, T) {
    todo!("Implement swap_ownership")
}

/// Create a function that conditionally moves a value based on a predicate.
/// If the predicate is true, return Some(value), otherwise return None
/// and the value is effectively "consumed."
fn conditional_move<T, P>(value: T, predicate: P) -> Option<T>
where
    P: FnOnce(&T) -> bool,
{
    todo!("Implement conditional_move")
}

/// Implement a builder pattern that transfers ownership through method chaining.
struct ConfigBuilder {
    name: Option<String>,
    timeout: Option<u64>,
    retries: Option<u32>,
}

impl ConfigBuilder {
    fn new() -> Self {
        todo!("Implement new")
    }

    /// Each method should take self by value and return Self
    fn name(self, name: String) -> Self {
        todo!("Implement name")
    }

    fn timeout(self, timeout: u64) -> Self {
        todo!("Implement timeout")
    }

    fn retries(self, retries: u32) -> Self {
        todo!("Implement retries")
    }

    /// Consume the builder and produce the final config
    fn build(self) -> Result<Config, &'static str> {
        todo!("Implement build")
    }
}

#[derive(Debug, PartialEq)]
struct Config {
    name: String,
    timeout: u64,
    retries: u32,
}

/// Implement a function that takes ownership of multiple values and processes them.
/// Use tuple destructuring to handle the ownership transfer.
fn process_multiple_owned(data: (String, Vec<i32>, Option<u64>)) -> String {
    todo!("Implement process_multiple_owned")
}

/// Create a move closure that captures variables by value.
/// Return a closure that owns its captured environment.
fn create_move_closure(prefix: String, suffix: String) -> impl Fn(&str) -> String {
    move |middle: &str| {
        format!("{} {} {}", prefix, middle, suffix)
    }
}

/// Demonstrate ownership transfer in iterators.
/// Take a vector, transform it using iterators with moves, and collect results.
fn iterator_with_moves(strings: Vec<String>, transform: fn(String) -> String) -> Vec<String> {
    todo!("Implement iterator_with_moves")
}

/// Handle ownership in error scenarios using Result.
/// If processing succeeds, return the processed value. If it fails, 
/// return the original value in the error.
fn try_process_or_return(value: String, should_succeed: bool) -> Result<String, String> {
    todo!("Implement try_process_or_return")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_owned_string() {
        let s = String::from("Hello, World!");
        let length = process_owned_string(s);
        
        // String should be moved, so this line would not compile:
        // println!("{}", s); // Error: value moved
        
        assert!(length > 0);
    }

    #[test]
    fn test_transform_and_return() {
        let vec = vec![1, 2, 3, 4, 5];
        let (processed, sum) = transform_and_return(vec, 2);
        
        // Original vec is moved, can't use it anymore
        assert_eq!(processed.len(), 5);
        assert!(sum > 0);
    }

    #[test]
    fn test_person_data_moves() {
        let person = PersonData::new(
            "Alice".to_string(),
            25,
            vec!["reading".to_string(), "hiking".to_string()],
        );
        
        let (name, age, hobbies) = person.extract_name();
        assert_eq!(name, "Alice");
        assert_eq!(age, 25);
        assert_eq!(hobbies.len(), 2);
        
        // person is now moved and cannot be used
    }

    #[test]
    fn test_partial_move_hobbies() {
        let mut person = PersonData::new(
            "Bob".to_string(),
            30,
            vec!["gaming".to_string(), "cooking".to_string()],
        );
        
        let hobbies = person.take_hobbies();
        assert_eq!(hobbies.len(), 2);
        
        // Can still access other fields
        assert_eq!(person.name, "Bob");
        assert_eq!(person.age, 30);
        
        // But hobbies field is no longer accessible (moved out)
        // This would not compile: println!("{:?}", person.hobbies);
    }

    #[test]
    fn test_swap_ownership() {
        let a = String::from("first");
        let b = String::from("second");
        
        let (new_a, new_b) = swap_ownership(a, b);
        
        assert_eq!(new_a, "second");
        assert_eq!(new_b, "first");
    }

    #[test]
    fn test_conditional_move() {
        let value1 = String::from("keep");
        let result1 = conditional_move(value1, |s| s.len() > 3);
        assert_eq!(result1, Some("keep".to_string()));
        
        let value2 = String::from("drop");
        let result2 = conditional_move(value2, |s| s.len() < 3);
        assert_eq!(result2, None);
    }

    #[test]
    fn test_process_multiple_owned() {
        let data = (
            "prefix".to_string(),
            vec![1, 2, 3],
            Some(42),
        );
        
        let result = process_multiple_owned(data);
        assert!(!result.is_empty());
    }

    #[test]
    fn test_move_closure() {
        let prefix = "Hello".to_string();
        let suffix = "World".to_string();
        
        let closure = create_move_closure(prefix, suffix);
        
        // prefix and suffix are moved into closure
        let result = closure("Beautiful");
        assert!(result.contains("Hello"));
        assert!(result.contains("World"));
        assert!(result.contains("Beautiful"));
    }

    #[test]
    fn test_iterator_with_moves() {
        let strings = vec![
            "hello".to_string(),
            "world".to_string(),
            "rust".to_string(),
        ];
        
        let transformed = iterator_with_moves(strings, |s| s.to_uppercase());
        
        assert_eq!(transformed, vec!["HELLO", "WORLD", "RUST"]);
    }

    #[test]
    fn test_try_process_or_return() {
        let success_case = "process me".to_string();
        let result = try_process_or_return(success_case, true);
        assert!(result.is_ok());
        
        let failure_case = "return me".to_string();
        let result = try_process_or_return(failure_case, false);
        assert_eq!(result, Err("return me".to_string()));
    }

    #[test]
    fn test_move_semantics_in_loops() {
        let mut strings = vec![
            "first".to_string(),
            "second".to_string(),
            "third".to_string(),
        ];
        
        let mut processed = Vec::new();
        
        // Move each string out of the vector during iteration
        for s in strings.drain(..) {
            processed.push(process_owned_string(s));
        }
        
        assert_eq!(processed.len(), 3);
        assert!(strings.is_empty()); // All elements moved out
    }

    #[test]
    fn test_ownership_in_match() {
        let option_string = Some("test".to_string());
        
        let result = match option_string {
            Some(s) => process_owned_string(s), // s is moved here
            None => 0,
        };
        
        assert!(result > 0);
        // option_string is now None due to partial move
    }

    #[test]
    fn test_return_ownership() {
        fn create_and_return() -> String {
            let s = String::from("created inside");
            s // Ownership transferred to caller
        }
        
        let owned_string = create_and_return();
        assert_eq!(owned_string, "created inside");
    }
}

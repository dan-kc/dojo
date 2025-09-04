// Lifetime Structs Practice
//
// Learning objectives:
// - Complex struct lifetime relationships
// - Multiple lifetime parameters
// - Lifetime bounds and constraints
// - Associated lifetimes in traits
//
// cargo test --lib lifetimes::lifetime_structs

/// Implement a struct that holds references to both a key and value from different sources.
/// This demonstrates multiple lifetime parameters in a single struct.
pub struct KeyValuePair {
    todo!("Add lifetime parameters and fields for key: &str and value: &str")
}

impl KeyValuePair {
    /// Create a new KeyValuePair with potentially different lifetimes for key and value
    pub fn new(key: &str, value: &str) -> KeyValuePair {
        todo!("Implement with appropriate lifetime annotations")
    }
    
    /// Get the key reference
    pub fn key(&self) -> &str {
        todo!("Return key with correct lifetime")
    }
    
    /// Get the value reference  
    pub fn value(&self) -> &str {
        todo!("Return value with correct lifetime")
    }
    
    /// Create a formatted string showing key=value
    pub fn format(&self) -> String {
        todo!("Create owned String representation")
    }
}

/// Create a struct that represents a parser holding a reference to input text
/// and tracking current position. This demonstrates self-referential lifetime patterns.
pub struct Parser {
    todo!("Add lifetime parameter and fields: input: &str, position: usize")
}

impl Parser {
    /// Create a new parser for the given input
    pub fn new(input: &str) -> Parser {
        todo!("Initialize parser with input and position 0")
    }
    
    /// Get the remaining unparsed input
    pub fn remaining(&self) -> &str {
        todo!("Return slice from current position to end")
    }
    
    /// Parse the next word (sequence of non-whitespace characters)
    /// Returns Some(word) and advances position, or None if at end
    pub fn next_word(&mut self) -> Option<&str> {
        todo!("Parse next word and advance position")
    }
    
    /// Peek at the next word without advancing position
    pub fn peek_word(&self) -> Option<&str> {
        todo!("Return next word without modifying parser state")
    }
}

/// Implement a struct that can hold references to items with different lifetimes
/// but ensures they all live long enough. This demonstrates lifetime bounds.
pub struct MultiRef {
    todo!("Add lifetime parameters and fields for refs: Vec<&str>")
}

impl MultiRef {
    /// Create a new MultiRef (empty)
    pub fn new() -> MultiRef {
        todo!("Create empty MultiRef")
    }
    
    /// Add a reference to the collection
    /// The reference must live at least as long as the struct
    pub fn add_ref(&mut self, r: &str) {
        todo!("Add reference to internal vector")
    }
    
    /// Get all references as a slice
    pub fn get_refs(&self) -> &[&str] {
        todo!("Return slice of all stored references")
    }
    
    /// Find the longest reference in the collection
    pub fn find_longest(&self) -> Option<&str> {
        todo!("Return longest string reference")
    }
}

/// Create a trait for types that can provide a string representation
/// with lifetime parameters for the returned reference
pub trait AsStrRef {
    todo!("Define trait with lifetime parameter and method returning &str")
}

/// Implement a struct that implements AsStrRef
pub struct NamedItem {
    todo!("Add lifetime parameter and name field")
}

impl NamedItem {
    pub fn new(name: &str) -> NamedItem {
        todo!("Create new NamedItem")
    }
}

impl AsStrRef for NamedItem {
    todo!("Implement AsStrRef trait")
}

/// A struct that demonstrates lifetime relationships between fields
/// This struct holds both owned and borrowed data
pub struct MixedData {
    todo!("Add lifetime parameter and fields: owned: String, borrowed: &str")
}

impl MixedData {
    /// Create MixedData with both owned and borrowed components
    pub fn new(owned: String, borrowed: &str) -> MixedData {
        todo!("Initialize with owned and borrowed data")
    }
    
    /// Get the owned data as a string slice
    pub fn owned_as_str(&self) -> &str {
        todo!("Return reference to owned string")
    }
    
    /// Get the borrowed data
    pub fn borrowed(&self) -> &str {
        todo!("Return the borrowed string")
    }
    
    /// Create a new string combining owned and borrowed data
    pub fn combine(&self) -> String {
        todo!("Combine owned and borrowed into new String")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_value_pair_basic() {
        let key = "name";
        let value = "Alice";
        let kv = KeyValuePair::new(key, value);
        assert_eq!(kv.key(), "name");
        assert_eq!(kv.value(), "Alice");
        assert_eq!(kv.format(), "name=Alice");
    }

    #[test]
    fn test_key_value_pair_different_lifetimes() {
        let key = "status";
        let kv = {
            let value = String::from("active");
            KeyValuePair::new(key, &value)
        };
        // This should work because key outlives the KeyValuePair
        assert_eq!(kv.key(), "status");
    }

    #[test]
    fn test_parser_basic() {
        let input = "hello world rust";
        let mut parser = Parser::new(input);
        
        assert_eq!(parser.remaining(), "hello world rust");
        assert_eq!(parser.next_word(), Some("hello"));
        assert_eq!(parser.remaining(), "world rust");
        assert_eq!(parser.next_word(), Some("world"));
        assert_eq!(parser.next_word(), Some("rust"));
        assert_eq!(parser.next_word(), None);
    }

    #[test]
    fn test_parser_peek() {
        let input = "one two";
        let mut parser = Parser::new(input);
        
        assert_eq!(parser.peek_word(), Some("one"));
        assert_eq!(parser.remaining(), "one two"); // Position unchanged
        assert_eq!(parser.next_word(), Some("one"));
        assert_eq!(parser.peek_word(), Some("two"));
        assert_eq!(parser.next_word(), Some("two"));
        assert_eq!(parser.peek_word(), None);
    }

    #[test]
    fn test_parser_empty_input() {
        let input = "";
        let mut parser = Parser::new(input);
        assert_eq!(parser.remaining(), "");
        assert_eq!(parser.next_word(), None);
        assert_eq!(parser.peek_word(), None);
    }

    #[test]
    fn test_parser_whitespace_handling() {
        let input = "  hello   world  ";
        let mut parser = Parser::new(input);
        assert_eq!(parser.next_word(), Some("hello"));
        assert_eq!(parser.next_word(), Some("world"));
        assert_eq!(parser.next_word(), None);
    }

    #[test]
    fn test_multi_ref_basic() {
        let s1 = "hello";
        let s2 = "world";
        let mut multi = MultiRef::new();
        
        multi.add_ref(s1);
        multi.add_ref(s2);
        
        let refs = multi.get_refs();
        assert_eq!(refs, &["hello", "world"]);
        assert_eq!(multi.find_longest(), Some("hello")); // Both same length, returns first
    }

    #[test]
    fn test_multi_ref_find_longest() {
        let s1 = "hi";
        let s2 = "programming";
        let s3 = "rust";
        let mut multi = MultiRef::new();
        
        multi.add_ref(s1);
        multi.add_ref(s2);
        multi.add_ref(s3);
        
        assert_eq!(multi.find_longest(), Some("programming"));
    }

    #[test]
    fn test_multi_ref_empty() {
        let multi = MultiRef::new();
        assert_eq!(multi.get_refs(), &[]);
        assert_eq!(multi.find_longest(), None);
    }

    #[test]
    fn test_named_item_as_str_ref() {
        let name = "test_item";
        let item = NamedItem::new(name);
        assert_eq!(item.as_str_ref(), "test_item");
    }

    #[test]
    fn test_mixed_data_basic() {
        let owned = String::from("owned_text");
        let borrowed = "borrowed_text";
        let mixed = MixedData::new(owned, borrowed);
        
        assert_eq!(mixed.owned_as_str(), "owned_text");
        assert_eq!(mixed.borrowed(), "borrowed_text");
        assert_eq!(mixed.combine(), "owned_textborrowed_text");
    }

    #[test]
    fn test_mixed_data_owned_outlives_borrowed() {
        let owned = String::from("persistent");
        let mixed = {
            let borrowed = String::from("temporary");
            MixedData::new(owned, &borrowed)
        };
        // This tests that owned data remains accessible
        assert_eq!(mixed.owned_as_str(), "persistent");
    }
}
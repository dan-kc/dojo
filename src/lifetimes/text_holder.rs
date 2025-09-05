// Text Holder
//
// Learning objectives:
// - Structs with lifetime parameters
// - Methods returning references with lifetimes
// - Lifetime relationships in struct implementations
//
// cargo test --bin text_holder

/// Fix this struct definition and its method to properly handle lifetimes.
/// The struct should hold a reference to a string slice.
pub struct TextHolder {
    todo!("Add lifetime parameter and field")
}

impl TextHolder {
    /// Create a new TextHolder with the given text reference
    pub fn new(text: &str) -> TextHolder {
        todo!("Implement constructor with proper lifetime handling")
    }
    
    /// Return the held text
    pub fn get_text(&self) -> &str {
        todo!("Return the stored text reference")
    }
    
    /// Return the first n characters of the held text
    pub fn get_prefix(&self, n: usize) -> &str {
        todo!("Return prefix with correct lifetime relationship")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_holder_basic_usage() {
        let text = "hello world";
        let holder = TextHolder::new(text);
        assert_eq!(holder.get_text(), "hello world");
    }

    #[test]
    fn test_text_holder_prefix() {
        let text = "hello world";
        let holder = TextHolder::new(text);
        assert_eq!(holder.get_prefix(5), "hello");
        assert_eq!(holder.get_prefix(20), "hello world"); // Beyond length
        assert_eq!(holder.get_prefix(0), "");
    }

    #[test]
    fn test_text_holder_unicode() {
        let text = "hello 世界";
        let holder = TextHolder::new(text);
        assert_eq!(holder.get_text(), "hello 世界");
        // Note: prefix by bytes, not chars - be careful with UTF-8
        assert_eq!(holder.get_prefix(5), "hello");
    }

    #[test]
    fn test_text_holder_empty() {
        let text = "";
        let holder = TextHolder::new(text);
        assert_eq!(holder.get_text(), "");
        assert_eq!(holder.get_prefix(5), "");
    }

    #[test]
    fn test_text_holder_lifetime_scope() {
        let holder = {
            let text = String::from("temporary");
            TextHolder::new(&text)
        }; // text goes out of scope here
        // This test verifies the struct compiles correctly with lifetime annotations
        // In actual usage, the above would cause a compilation error
        assert_eq!(holder.get_text().len(), 9);
    }
}

fn main() {
    println!("Run tests with: cargo test --bin text_holder");
}
// Lifetime Generics Practice
//
// Learning objectives:
// - Combining lifetimes with generic type parameters
// - Higher-ranked trait bounds (HRTB)
// - Lifetime bounds and where clauses
// - Complex generic lifetime relationships
//
// cargo test --lib lifetimes::lifetime_generics

use std::fmt::Debug;

/// Implement a generic container that holds a reference to some data
/// along with a transformation function that can be applied to the data.
pub struct DataProcessor {
    todo!("Add lifetime and generic type parameters, fields: data: &T, func: F")
}

impl DataProcessor {
    /// Create a new DataProcessor with data and a transformation function
    pub fn new(data: &T, func: F) -> DataProcessor
    where
        F: Fn(&T) -> R,
    {
        todo!("Implement constructor with appropriate lifetime and generic constraints")
    }
    
    /// Apply the transformation function to the stored data
    pub fn process(&self) -> R
    where
        F: Fn(&T) -> R,
    {
        todo!("Apply function to data reference")
    }
    
    /// Get a reference to the stored data
    pub fn get_data(&self) -> &T {
        todo!("Return reference to stored data")
    }
}

/// Create a function that takes a slice of any type and a predicate function,
/// returning an iterator that filters elements and maps them to string representations.
/// This demonstrates higher-ranked trait bounds.
pub fn filter_and_format<T, F>(slice: &[T], predicate: F) -> impl Iterator<Item = String> + '_
where
    T: Debug,
    F: Fn(&T) -> bool,
{
    todo!("Return iterator that filters by predicate and formats items using Debug")
}

/// Implement a struct that can hold different types of references
/// and provides methods that work generically across the types.
pub struct GenericRefHolder {
    todo!("Add lifetime parameter and generic type parameter for field: item: &T")
}

impl GenericRefHolder {
    /// Create a new GenericRefHolder
    pub fn new(item: &T) -> GenericRefHolder {
        todo!("Create new holder with generic item")
    }
    
    /// Apply a function to the held item and return the result
    pub fn map_item<R, F>(&self, func: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        todo!("Apply function to held item")
    }
    
    /// Compare held item with another item of the same type
    pub fn equals(&self, other: &T) -> bool
    where
        T: PartialEq,
    {
        todo!("Compare held item with other")
    }
    
    /// Create a debug representation of the held item
    pub fn debug_string(&self) -> String
    where
        T: Debug,
    {
        todo!("Format held item using Debug trait")
    }
}

/// Create a trait for types that can be borrowed with specific lifetime constraints
pub trait BorrowWithLifetime {
    type Target;
    
    todo!("Define method: fn borrow_for_lifetime(&self) -> &Self::Target;")
}

/// Implement a generic function that works with the BorrowWithLifetime trait
/// and applies a transformation to the borrowed data.
pub fn transform_borrowed<T, R, F>(borrower: &T, transformer: F) -> R
where
    T: BorrowWithLifetime,
    F: todo!("Add appropriate trait bounds for function that takes borrowed data"),
{
    todo!("Implement function that borrows data and applies transformation")
}

/// Implement a struct that demonstrates lifetime bounds with multiple generic parameters
pub struct ComplexContainer {
    todo!("Add lifetime and generic parameters for: items: Vec<&T>, processor: P")
}

impl ComplexContainer
where
    P: Fn(&T) -> String,
    T: Clone + Debug,
{
    /// Create a new ComplexContainer
    pub fn new(processor: P) -> ComplexContainer {
        todo!("Initialize with processor and empty items vector")
    }
    
    /// Add an item reference to the container
    pub fn add_item(&mut self, item: &T) {
        todo!("Add item reference to items vector")
    }
    
    /// Process all items and return results
    pub fn process_all(&self) -> Vec<String> {
        todo!("Apply processor to all items and collect results")
    }
    
    /// Find the first item that matches a predicate
    pub fn find_item<F>(&self, predicate: F) -> Option<&T>
    where
        F: Fn(&T) -> bool,
    {
        todo!("Find first item matching predicate")
    }
}

/// A trait that demonstrates higher-ranked trait bounds
pub trait AsyncProcessor<T> {
    todo!("Define method that takes a higher-ranked function: fn process_with_callback<F>(&self, callback: F) where F: for<'a> Fn(&'a T) -> String;")
}

/// Implement the AsyncProcessor trait for a simple type
pub struct SimpleProcessor;

impl<T> AsyncProcessor<T> for SimpleProcessor
where
    T: Debug,
{
    todo!("Implement process_with_callback using the provided callback with any lifetime")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_data_processor_basic() {
        let data = 42i32;
        let processor = DataProcessor::new(&data, |x| x * 2);
        
        assert_eq!(processor.process(), 84);
        assert_eq!(processor.get_data(), &42);
    }

    #[test]
    fn test_data_processor_string() {
        let data = "hello".to_string();
        let processor = DataProcessor::new(&data, |s| s.len());
        
        assert_eq!(processor.process(), 5);
        assert_eq!(processor.get_data(), "hello");
    }

    #[test]
    fn test_filter_and_format() {
        let numbers = [1, 2, 3, 4, 5, 6];
        let result: Vec<String> = filter_and_format(&numbers, |x| *x % 2 == 0).collect();
        
        assert_eq!(result, vec!["2", "4", "6"]);
    }

    #[test]
    fn test_filter_and_format_strings() {
        let words = ["hello", "hi", "world", "rust"];
        let result: Vec<String> = filter_and_format(&words, |s| s.len() > 3).collect();
        
        assert_eq!(result, vec!["\"hello\"", "\"world\"", "\"rust\""]);
    }

    #[test]
    fn test_generic_ref_holder() {
        let value = 100;
        let holder = GenericRefHolder::new(&value);
        
        assert_eq!(holder.map_item(|x| x * 3), 300);
        assert!(holder.equals(&100));
        assert!(!holder.equals(&200));
        
        let debug_str = holder.debug_string();
        assert_eq!(debug_str, "100");
    }

    #[test]
    fn test_generic_ref_holder_string() {
        let text = "testing";
        let holder = GenericRefHolder::new(&text);
        
        assert_eq!(holder.map_item(|s| s.len()), 7);
        assert!(holder.equals(&"testing"));
        
        let debug_str = holder.debug_string();
        assert_eq!(debug_str, "\"testing\"");
    }

    // Helper struct for BorrowWithLifetime tests
    struct StringWrapper(String);
    
    impl BorrowWithLifetime for StringWrapper {
        type Target = str;
        
        fn borrow_for_lifetime(&self) -> &str {
            &self.0
        }
    }

    #[test]
    fn test_transform_borrowed() {
        let wrapper = StringWrapper("hello world".to_string());
        let result = transform_borrowed(&wrapper, |s: &str| s.len());
        
        assert_eq!(result, 11);
    }

    #[test]
    fn test_transform_borrowed_uppercase() {
        let wrapper = StringWrapper("hello".to_string());
        let result = transform_borrowed(&wrapper, |s: &str| s.to_uppercase());
        
        assert_eq!(result, "HELLO");
    }

    #[test]
    fn test_complex_container() {
        let mut container = ComplexContainer::new(|x: &i32| format!("Number: {}", x));
        
        let a = 10;
        let b = 20;
        let c = 30;
        
        container.add_item(&a);
        container.add_item(&b);
        container.add_item(&c);
        
        let results = container.process_all();
        assert_eq!(results, vec!["Number: 10", "Number: 20", "Number: 30"]);
        
        let found = container.find_item(|x| **x > 15);
        assert_eq!(found, Some(&20));
        
        let not_found = container.find_item(|x| **x > 50);
        assert_eq!(not_found, None);
    }

    #[test]
    fn test_simple_processor() {
        let processor = SimpleProcessor;
        let value = 42;
        
        processor.process_with_callback(&value, |x| format!("Processed: {:?}", x));
    }

    #[test]
    fn test_simple_processor_with_string() {
        let processor = SimpleProcessor;
        let text = "hello".to_string();
        
        processor.process_with_callback(&text, |s| format!("Text: {:?}", s));
    }
}
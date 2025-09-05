// Non-Send Container Practice
//
// Learning Objectives:
// - Handle non-Send data safely across threads
// - Use Mutex to make containers Send even with non-Send contents
// - Understand the difference between container and content Send properties
//
// Run with: cargo test --bin non_send_container

use std::sync::{Arc, Mutex};
use std::thread;
use std::rc::Rc;

/// A struct that contains non-Send data but can still be used safely
/// across threads through careful encapsulation
pub struct NonSendContainer {
    // This contains Rc which is not Send, but we'll make the container Send
    data: Mutex<Option<Rc<String>>>,
}

impl NonSendContainer {
    pub fn new() -> Self {
        todo!("Implement new")
    }

    pub fn set_data(&self, data: String) {
        todo!("Implement set_data")
    }

    pub fn get_data(&self) -> Option<String> {
        todo!("Implement get_data")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_non_send_container() {
    //     // This test demonstrates the challenge of making non-Send data work across threads
    //     // The Rc<String> inside the Mutex still prevents the container from being Send
    //     let container = Arc::new(NonSendContainer::new());
    //     let container_clone = container.clone();

    //     let handle = thread::spawn(move || {
    //         container_clone.set_data("Thread data".to_string());
    //         container_clone.get_data()
    //     });

    //     let result = handle.join().unwrap();
    //     assert_eq!(result, Some("Thread data".to_string()));
    // }

    #[test]
    fn test_compile_time_safety() {
        // These should compile (Send + Sync types)
        let _: Arc<NonSendContainer> = Arc::new(NonSendContainer::new());
        
        // These demonstrate the compiler preventing unsafe sharing
        // Uncommenting these would cause compilation errors:
        
        // let rc = Rc::new(5);  // Rc is not Send
        // thread::spawn(move || println!("{}", rc)); // Would fail to compile
    }
}
// Borrow Checking Demo Practice
//
// Learning objectives:
// - Understanding RefCell runtime borrow checking
// - Handling potential runtime panics gracefully
// - Learning when RefCell borrows can conflict
//
// Run with: cargo test borrow_checking_demo

use std::cell::RefCell;

/// Demonstrate potential runtime panics with RefCell borrow checking
pub fn demonstrate_borrow_checking() -> Result<String, &'static str> {
    todo!("Create RefCell, try to create conflicting borrows, handle the panic gracefully")
}

/// Safe wrapper that attempts operations and reports success/failure
pub fn safe_refcell_operations() -> Vec<String> {
    todo!("Perform various RefCell operations and report which ones succeed/fail")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::panic;

    #[test]
    fn test_successful_borrow_checking() {
        // This should work fine - sequential borrows
        let cell = RefCell::new(42);
        
        {
            let borrow1 = cell.borrow();
            assert_eq!(*borrow1, 42);
            // borrow1 goes out of scope here
        }
        
        {
            let mut borrow2 = cell.borrow_mut();
            *borrow2 = 100;
            // borrow2 goes out of scope here
        }
        
        assert_eq!(*cell.borrow(), 100);
    }

    #[test]
    fn test_demonstrate_borrow_checking() {
        let result = demonstrate_borrow_checking();
        
        // The function should handle panics gracefully
        match result {
            Ok(msg) => {
                assert!(msg.contains("borrow") || msg.contains("conflict"));
            }
            Err(err) => {
                assert!(err.contains("borrow") || err.contains("panic"));
            }
        }
    }

    #[test] 
    fn test_safe_operations() {
        let results = safe_refcell_operations();
        assert!(!results.is_empty());
        
        // Should contain reports of various operations
        let joined = results.join(" ");
        assert!(joined.contains("success") || joined.contains("fail"));
    }

    #[test]
    #[should_panic]
    fn test_conflicting_borrows_panic() {
        let cell = RefCell::new(42);
        
        let _immutable = cell.borrow();
        let _mutable = cell.borrow_mut(); // This should panic!
    }
}
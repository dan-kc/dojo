// Publisher-Subscriber System Practice
//
// Learning objectives:
// - Implementing the observer pattern with smart pointers
// - Using Weak<T> references to avoid memory leaks
// - Managing dynamic collections of subscribers
// - Understanding trait objects and dynamic dispatch
// - Handling subscriber lifecycle and cleanup
//
// Run with: cargo test publisher_subscriber

use std::rc::{Rc, Weak};
use std::cell::RefCell;

pub trait Subscriber<T> {
    fn notify(&self, message: &T);
}

/// Implement a publish-subscribe system using smart pointers.
/// Publishers can notify multiple subscribers, subscribers can be dropped independently.
pub struct Publisher<T> {
    subscribers: RefCell<Vec<Weak<dyn Subscriber<T>>>>,
}

impl<T> Publisher<T>
where
    T: Clone,
{
    /// Create a new publisher
    pub fn new() -> Self {
        todo!("Initialize with empty subscribers list")
    }
    
    /// Subscribe to this publisher
    pub fn subscribe(&self, subscriber: Rc<dyn Subscriber<T>>) {
        todo!("Add weak reference to subscriber to the list")
    }
    
    /// Publish a message to all active subscribers
    pub fn publish(&self, message: T) {
        todo!("Notify all subscribers that can still be upgraded from weak references")
    }
    
    /// Clean up dropped subscribers
    pub fn cleanup_subscribers(&self) {
        todo!("Remove weak references that can no longer be upgraded")
    }
    
    /// Get the number of active subscribers
    pub fn active_subscriber_count(&self) -> usize {
        todo!("Count weak references that can still be upgraded")
    }
}

/// Example subscriber implementation
pub struct LoggingSubscriber {
    name: String,
    logs: RefCell<Vec<String>>,
}

impl LoggingSubscriber {
    pub fn new(name: String) -> Rc<Self> {
        todo!("Create subscriber wrapped in Rc")
    }
    
    pub fn get_logs(&self) -> Vec<String> {
        todo!("Return cloned logs")
    }
}

impl Subscriber<String> for LoggingSubscriber {
    fn notify(&self, message: &String) {
        todo!("Log the message with subscriber name")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publisher_subscriber_basic() {
        let publisher = Publisher::new();
        let sub1 = LoggingSubscriber::new("sub1".to_string());
        let sub2 = LoggingSubscriber::new("sub2".to_string());
        
        publisher.subscribe(sub1.clone());
        publisher.subscribe(sub2.clone());
        
        assert_eq!(publisher.active_subscriber_count(), 2);
        
        publisher.publish("Hello".to_string());
        
        let logs1 = sub1.get_logs();
        let logs2 = sub2.get_logs();
        
        assert_eq!(logs1.len(), 1);
        assert_eq!(logs2.len(), 1);
        assert!(logs1[0].contains("Hello"));
        assert!(logs2[0].contains("Hello"));
    }

    #[test]
    fn test_publisher_subscriber_cleanup() {
        let publisher = Publisher::new();
        let sub1 = LoggingSubscriber::new("temp".to_string());
        
        publisher.subscribe(sub1.clone());
        assert_eq!(publisher.active_subscriber_count(), 1);
        
        drop(sub1); // Drop the subscriber
        
        publisher.cleanup_subscribers();
        assert_eq!(publisher.active_subscriber_count(), 0);
    }

    #[test]
    fn test_publisher_multiple_messages() {
        let publisher = Publisher::new();
        let sub = LoggingSubscriber::new("test".to_string());
        
        publisher.subscribe(sub.clone());
        
        publisher.publish("Message 1".to_string());
        publisher.publish("Message 2".to_string());
        publisher.publish("Message 3".to_string());
        
        let logs = sub.get_logs();
        assert_eq!(logs.len(), 3);
        assert!(logs[0].contains("Message 1"));
        assert!(logs[1].contains("Message 2"));
        assert!(logs[2].contains("Message 3"));
    }

    #[test]
    fn test_publisher_subscriber_partial_drop() {
        let publisher = Publisher::new();
        let sub1 = LoggingSubscriber::new("persistent".to_string());
        let sub2 = LoggingSubscriber::new("temporary".to_string());
        
        publisher.subscribe(sub1.clone());
        publisher.subscribe(sub2.clone());
        
        assert_eq!(publisher.active_subscriber_count(), 2);
        
        drop(sub2); // Drop one subscriber
        
        publisher.publish("After drop".to_string());
        
        // Only the remaining subscriber should receive the message
        let logs1 = sub1.get_logs();
        assert_eq!(logs1.len(), 1);
        assert!(logs1[0].contains("After drop"));
        
        // Active count should automatically exclude dropped subscribers
        assert_eq!(publisher.active_subscriber_count(), 1);
    }

    #[test]
    fn test_publisher_no_subscribers() {
        let publisher = Publisher::<String>::new();
        
        // Should not panic when publishing with no subscribers
        publisher.publish("Hello empty world".to_string());
        
        assert_eq!(publisher.active_subscriber_count(), 0);
    }

    #[test]
    fn test_logging_subscriber_name_in_logs() {
        let sub = LoggingSubscriber::new("TestLogger".to_string());
        let publisher = Publisher::new();
        
        publisher.subscribe(sub.clone());
        publisher.publish("Test message".to_string());
        
        let logs = sub.get_logs();
        assert_eq!(logs.len(), 1);
        assert!(logs[0].contains("TestLogger"));
        assert!(logs[0].contains("Test message"));
    }

    // Custom subscriber for testing
    struct CountingSubscriber {
        count: RefCell<usize>,
    }

    impl CountingSubscriber {
        fn new() -> Rc<Self> {
            Rc::new(Self {
                count: RefCell::new(0),
            })
        }

        fn get_count(&self) -> usize {
            *self.count.borrow()
        }
    }

    impl Subscriber<String> for CountingSubscriber {
        fn notify(&self, _message: &String) {
            *self.count.borrow_mut() += 1;
        }
    }

    #[test]
    fn test_custom_subscriber() {
        let publisher = Publisher::new();
        let counter = CountingSubscriber::new();
        
        publisher.subscribe(counter.clone());
        
        publisher.publish("msg1".to_string());
        publisher.publish("msg2".to_string());
        publisher.publish("msg3".to_string());
        
        assert_eq!(counter.get_count(), 3);
    }
}
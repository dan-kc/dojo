// cargo test --lib smart_pointers::publisher_subscriber::

//! Keep non-owning subscriber registrations and notify currently live subscribers in registration order.
//! Take a snapshot of live subscribers before invoking callbacks, allowing callbacks to add subscriptions without a borrow panic.
//! Registrations added during delivery apply to later publications. Each registration receives one callback (duplicates count).
//! Cleanup removes expired registrations; active count excludes them even before cleanup. LoggingSubscriber records exactly name: message.

pub trait Subscriber<T> {
    fn notify(&self, message: &T);
}
pub struct Publisher<T> {
    subscribers: std::cell::RefCell<Vec<std::rc::Weak<dyn Subscriber<T>>>>,
}
impl<T> Publisher<T> {
    pub fn new() -> Self {
        todo!()
    }
    pub fn subscribe(&self, subscriber: std::rc::Rc<dyn Subscriber<T>>) {
        todo!()
    }
    pub fn publish(&self, message: T) {
        todo!()
    }
    pub fn cleanup_subscribers(&self) {
        todo!()
    }
    pub fn active_subscriber_count(&self) -> usize {
        todo!()
    }
}
pub struct LoggingSubscriber {
    name: String,
    logs: std::cell::RefCell<Vec<String>>,
}
impl LoggingSubscriber {
    pub fn new(name: String) -> std::rc::Rc<Self> {
        todo!()
    }
    pub fn get_logs(&self) -> Vec<String> {
        todo!()
    }
}
impl Subscriber<String> for LoggingSubscriber {
    fn notify(&self, message: &String) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::rc::Rc;

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

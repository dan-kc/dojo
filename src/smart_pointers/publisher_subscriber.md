# smart_pointers/publisher_subscriber

Reference implementation. Read after attempting the exercise.

```rust
pub trait Subscriber<T> {
    fn notify(&self, message: &T);
}
pub struct Publisher<T> {
    subscribers: std::cell::RefCell<Vec<std::rc::Weak<dyn Subscriber<T>>>>,
}
impl<T> Publisher<T> {
    pub fn new() -> Self {
        Self {
            subscribers: std::cell::RefCell::new(vec![]),
        }
    }
    pub fn subscribe(&self, subscriber: std::rc::Rc<dyn Subscriber<T>>) {
        self.subscribers
            .borrow_mut()
            .push(std::rc::Rc::downgrade(&subscriber));
    }
    pub fn publish(&self, message: T) {
        let live: Vec<_> = self
            .subscribers
            .borrow()
            .iter()
            .filter_map(std::rc::Weak::upgrade)
            .collect();
        for subscriber in live {
            subscriber.notify(&message);
        }
    }
    pub fn cleanup_subscribers(&self) {
        self.subscribers
            .borrow_mut()
            .retain(|s| s.strong_count() > 0);
    }
    pub fn active_subscriber_count(&self) -> usize {
        self.subscribers
            .borrow()
            .iter()
            .filter(|s| s.strong_count() > 0)
            .count()
    }
}
pub struct LoggingSubscriber {
    name: String,
    logs: std::cell::RefCell<Vec<String>>,
}
impl LoggingSubscriber {
    pub fn new(name: String) -> std::rc::Rc<Self> {
        std::rc::Rc::new(Self {
            name,
            logs: std::cell::RefCell::new(vec![]),
        })
    }
    pub fn get_logs(&self) -> Vec<String> {
        self.logs.borrow().clone()
    }
}
impl Subscriber<String> for LoggingSubscriber {
    fn notify(&self, message: &String) {
        self.logs
            .borrow_mut()
            .push(format!("{}: {}", self.name, message));
    }
}
```

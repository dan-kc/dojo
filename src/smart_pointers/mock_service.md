# smart_pointers/mock_service

Reference implementation. Read after attempting the exercise.

```rust
pub struct MockService {
    calls: std::cell::RefCell<Vec<(String, String)>>,
}
impl MockService {
    pub fn new() -> Self {
        Self {
            calls: std::cell::RefCell::new(vec![]),
        }
    }
    pub fn call_method(&self, method: &str, args: &str) -> String {
        self.calls.borrow_mut().push((method.into(), args.into()));
        format!("ok: {method}")
    }
    pub fn call_count(&self) -> usize {
        self.calls.borrow().len()
    }
    pub fn get_call_log(&self) -> Vec<String> {
        self.calls
            .borrow()
            .iter()
            .map(|(m, a)| format!("{m}({a})"))
            .collect()
    }
    pub fn reset(&self) {
        self.calls.borrow_mut().clear();
    }
    pub fn was_called(&self, method: &str) -> bool {
        self.calls.borrow().iter().any(|(m, _)| m == method)
    }
}
```

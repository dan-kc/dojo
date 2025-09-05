# Mock Service - Solution

## Solution

```rust
use std::cell::RefCell;

/// Create a mock object that tracks method calls using interior mutability.
pub struct MockService {
    call_log: RefCell<Vec<String>>,
}

impl MockService {
    /// Create a new mock service
    pub fn new() -> Self {
        MockService {
            call_log: RefCell::new(Vec::new()),
        }
    }
    
    /// Simulate a method call, logging it internally
    pub fn call_method(&self, method_name: &str, args: &str) -> String {
        let call_entry = format!("{}({})", method_name, args);
        self.call_log.borrow_mut().push(call_entry);
        format!("Mock response for {}", method_name)
    }
    
    /// Get the number of method calls made
    pub fn call_count(&self) -> usize {
        self.call_log.borrow().len()
    }
    
    /// Get all method calls made (for verification in tests)
    pub fn get_call_log(&self) -> Vec<String> {
        self.call_log.borrow().clone()
    }
    
    /// Clear the call log
    pub fn reset(&self) {
        self.call_log.borrow_mut().clear();
    }
    
    /// Check if a specific method was called
    pub fn was_called(&self, method_name: &str) -> bool {
        self.call_log
            .borrow()
            .iter()
            .any(|call| call.contains(method_name))
    }
}
```

## Explanation

### Mock Objects and Interior Mutability

**The Testing Problem:**
```rust
// Traditional approach - requires mutable access for tracking
struct TraditionalMock {
    call_log: Vec<String>,
}

impl TraditionalMock {
    fn call_method(&mut self, method: &str) -> String {
        //     ^^^^^ Requires mutable reference
        self.call_log.push(method.to_string());
        "response".to_string()
    }
}

// Problem: In tests, the mock often needs to be shared immutably
fn test_service(service: &dyn Service) {  // Immutable reference
    service.do_something();  // But we want to track this call!
}
```

**Interior Mutability Solution:**
```rust
// MockService can track calls through immutable references
impl MockService {
    pub fn call_method(&self, method_name: &str, args: &str) -> String {
        //              ^^^^^ Only needs &self
        // Can still track calls internally via RefCell
        self.call_log.borrow_mut().push(format!("{}({})", method_name, args));
        format!("Mock response for {}", method_name)
    }
}
```

**Key Insight:** Mock objects need to appear stateless to the code under test (accepting `&self`), but internally track interactions for verification. RefCell<T> perfectly enables this hidden state pattern.

### Call Logging Implementation

**Structured Call Tracking:**
```rust
pub fn call_method(&self, method_name: &str, args: &str) -> String {
    let call_entry = format!("{}({})", method_name, args);
    self.call_log.borrow_mut().push(call_entry);
    format!("Mock response for {}", method_name)
}

// Creates entries like:
// "login(user=alice,password=****)"  
// "get_data(id=123)"
// "delete(id=456,force=true)"
```

**Why This Format:**
- **Parseable:** Easy to extract method name and arguments later
- **Human readable:** Great for debugging test failures
- **Searchable:** `was_called()` can use simple string contains
- **Comprehensive:** Captures both method and its parameters

### Advanced Mock Patterns

**Return Value Customization:**
```rust
use std::collections::HashMap;

struct AdvancedMock {
    call_log: RefCell<Vec<String>>,
    responses: RefCell<HashMap<String, String>>,
}

impl AdvancedMock {
    fn set_response(&self, method: &str, response: &str) {
        self.responses.borrow_mut().insert(method.to_string(), response.to_string());
    }
    
    fn call_method(&self, method_name: &str, args: &str) -> String {
        // Log the call
        let call_entry = format!("{}({})", method_name, args);
        self.call_log.borrow_mut().push(call_entry);
        
        // Return custom or default response
        self.responses
            .borrow()
            .get(method_name)
            .cloned()
            .unwrap_or_else(|| format!("Mock response for {}", method_name))
    }
}
```

**Call Sequence Tracking:**
```rust
struct SequenceMock {
    call_log: RefCell<Vec<(String, std::time::Instant)>>,
}

impl SequenceMock {
    fn call_method(&self, method_name: &str, args: &str) -> String {
        let call_entry = (
            format!("{}({})", method_name, args),
            std::time::Instant::now()
        );
        self.call_log.borrow_mut().push(call_entry);
        format!("Mock response for {}", method_name)
    }
    
    fn get_call_sequence(&self) -> Vec<String> {
        let mut calls = self.call_log.borrow().clone();
        calls.sort_by_key(|(_, timestamp)| *timestamp);
        calls.into_iter().map(|(call, _)| call).collect()
    }
}
```

### Test Verification Patterns

**Basic Call Verification:**
```rust
#[test]
fn test_service_calls_login() {
    let mock = MockService::new();
    let service = ServiceUnderTest::new(&mock);
    
    // Execute the behavior
    service.authenticate("alice", "password123");
    
    // Verify the mock was called correctly
    assert_eq!(mock.call_count(), 1);
    assert!(mock.was_called("login"));
    
    // Verify call details
    let calls = mock.get_call_log();
    assert_eq!(calls[0], "login(user=alice,password=password123)");
}
```

**Complex Interaction Testing:**
```rust
#[test]
fn test_service_workflow() {
    let mock = MockService::new();
    let service = ServiceUnderTest::new(&mock);
    
    // Execute complex workflow
    service.complete_user_registration("alice", "alice@example.com");
    
    // Verify call sequence and parameters
    let calls = mock.get_call_log();
    assert_eq!(calls.len(), 3);
    assert!(calls[0].contains("validate_email"));
    assert!(calls[1].contains("create_user"));
    assert!(calls[2].contains("send_welcome_email"));
    
    // Verify specific parameters
    assert!(calls[0].contains("alice@example.com"));
    assert!(calls[1].contains("alice"));
}
```

**Negative Testing:**
```rust
#[test] 
fn test_service_doesnt_call_delete() {
    let mock = MockService::new();
    let service = ServiceUnderTest::new(&mock);
    
    // Execute read-only operation
    service.get_user_profile("alice");
    
    // Verify no destructive operations were called
    assert!(!mock.was_called("delete"));
    assert!(!mock.was_called("update"));
    assert!(!mock.was_called("create"));
}
```

### Memory Management in Mocks

**Automatic Cleanup:**
```rust
impl MockService {
    /// Clear old entries to prevent memory buildup in long tests
    pub fn trim_log(&self, max_entries: usize) {
        let mut log = self.call_log.borrow_mut();
        if log.len() > max_entries {
            let excess = log.len() - max_entries;
            log.drain(0..excess);
        }
    }
    
    /// Get recent calls only
    pub fn get_recent_calls(&self, count: usize) -> Vec<String> {
        let log = self.call_log.borrow();
        let start = log.len().saturating_sub(count);
        log[start..].to_vec()
    }
}
```

### Thread Safety Considerations

**Single-Threaded Mocks:**
```rust
// MockService with RefCell is NOT thread-safe
let mock = MockService::new();

// This won't compile:
// std::thread::spawn(move || {
//     mock.call_method("test", "args");  // ERROR: RefCell not Send
// });
```

**Thread-Safe Mock Alternative:**
```rust
use std::sync::{Arc, Mutex};

struct ThreadSafeMock {
    call_log: Arc<Mutex<Vec<String>>>,
}

impl ThreadSafeMock {
    fn new() -> Self {
        ThreadSafeMock {
            call_log: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn call_method(&self, method_name: &str, args: &str) -> String {
        let call_entry = format!("{}({})", method_name, args);
        self.call_log.lock().unwrap().push(call_entry);
        format!("Mock response for {}", method_name)
    }
    
    fn clone_handle(&self) -> Self {
        ThreadSafeMock {
            call_log: Arc::clone(&self.call_log),
        }
    }
}
```

### Integration with Test Frameworks

**With Custom Traits:**
```rust
trait UserService {
    fn login(&self, username: &str, password: &str) -> Result<String, String>;
    fn get_profile(&self, user_id: &str) -> Option<UserProfile>;
}

impl UserService for MockService {
    fn login(&self, username: &str, password: &str) -> Result<String, String> {
        let response = self.call_method("login", &format!("username={},password=****", username));
        
        // Mock can return different responses based on input
        if username == "valid_user" {
            Ok("session_token_123".to_string())
        } else {
            Err("Invalid credentials".to_string())
        }
    }
    
    fn get_profile(&self, user_id: &str) -> Option<UserProfile> {
        self.call_method("get_profile", &format!("user_id={}", user_id));
        
        // Return mock profile data
        Some(UserProfile {
            id: user_id.to_string(),
            name: "Mock User".to_string(),
        })
    }
}
```

**Assertion Helpers:**
```rust
impl MockService {
    /// Assert that a method was called with specific arguments
    pub fn assert_called_with(&self, method: &str, args: &str) {
        let expected = format!("{}({})", method, args);
        let calls = self.get_call_log();
        assert!(
            calls.contains(&expected),
            "Expected call '{}' not found in calls: {:?}",
            expected, calls
        );
    }
    
    /// Assert call count for specific method
    pub fn assert_method_call_count(&self, method: &str, expected_count: usize) {
        let actual_count = self.call_log
            .borrow()
            .iter()
            .filter(|call| call.contains(method))
            .count();
        assert_eq!(
            actual_count, expected_count,
            "Expected {} calls to '{}', but found {}",
            expected_count, method, actual_count
        );
    }
}
```

### Best Practices for Mock Objects

**Design Principles:**
1. **Keep mocks simple:** Focus on call tracking, not complex behavior
2. **Use consistent formatting:** Standardize how calls are logged
3. **Provide rich verification:** Offer multiple ways to inspect mock state
4. **Reset between tests:** Ensure test isolation with `reset()` method

**Testing Anti-Patterns to Avoid:**
```rust
// BAD: Testing mock implementation instead of behavior
#[test]
fn test_mock_logs_calls() {  // This tests the mock, not your code!
    let mock = MockService::new();
    mock.call_method("test", "args");
    assert_eq!(mock.call_count(), 1);
}

// GOOD: Testing actual service behavior via mock
#[test] 
fn test_service_authenticates_user() {  // This tests your service logic
    let mock = MockService::new();
    let service = AuthService::new(&mock);
    
    service.authenticate_user("alice", "password");
    
    // Verify service called the right mock methods
    assert!(mock.was_called("validate_credentials"));
}
```

**Mock State Management:**
```rust
// Use in test setup/teardown
impl MockService {
    fn setup_for_test() -> Self {
        let mock = MockService::new();
        // Pre-populate with expected responses
        mock
    }
    
    fn verify_and_reset(&self) {
        // Verify any remaining expectations
        // Clear state for next test
        self.reset();
    }
}
```

Mock objects demonstrate how RefCell<T> enables elegant testing patterns by allowing objects to maintain hidden state while presenting immutable interfaces to the code under test. This pattern is essential for effective unit testing in Rust.
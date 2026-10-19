// cargo test --lib smart_pointers::mock_service::

//! Track calls through shared references. Each call appends exactly method(args) to the log and returns exactly ok: method.
//! was_called matches the complete method name, never a substring of another method or its arguments.
//! get_call_log returns an independent snapshot; reset clears calls. Method names contain no parentheses.

pub struct MockService {
    calls: std::cell::RefCell<Vec<(String, String)>>,
}
impl MockService {
    pub fn new() -> Self {
        todo!()
    }
    pub fn call_method(&self, method: &str, args: &str) -> String {
        todo!()
    }
    pub fn call_count(&self) -> usize {
        todo!()
    }
    pub fn get_call_log(&self) -> Vec<String> {
        todo!()
    }
    pub fn reset(&self) {
        todo!()
    }
    pub fn was_called(&self, method: &str) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_service_basic() {
        let mock = MockService::new();
        assert_eq!(mock.call_count(), 0);

        let response1 = mock.call_method("login", "user=alice");
        let response2 = mock.call_method("get_data", "id=123");

        assert_eq!(mock.call_count(), 2);
        assert!(mock.was_called("login"));
        assert!(mock.was_called("get_data"));
        assert!(!mock.was_called("delete"));
    }

    #[test]
    fn test_mock_service_call_log() {
        let mock = MockService::new();

        mock.call_method("method1", "arg1");
        mock.call_method("method2", "arg2");

        let log = mock.get_call_log();
        assert_eq!(log.len(), 2);
        assert!(log[0].contains("method1"));
        assert!(log[1].contains("method2"));
    }

    #[test]
    fn test_mock_service_reset() {
        let mock = MockService::new();

        mock.call_method("test", "args");
        assert_eq!(mock.call_count(), 1);

        mock.reset();
        assert_eq!(mock.call_count(), 0);
        assert!(!mock.was_called("test"));
    }
}
#[cfg(test)]
mod exact_tests {
    use super::*;
    #[test]
    fn exact_matching_and_snapshot() {
        let m = MockService::new();
        assert_eq!(m.call_method("login_extra", "login"), "ok: login_extra");
        assert!(!m.was_called("login"));
        assert_eq!(m.get_call_log(), ["login_extra(login)"]);
        let log = m.get_call_log();
        m.reset();
        assert_eq!(log.len(), 1);
    }
}

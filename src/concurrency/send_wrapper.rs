// cargo test --lib concurrency::send_wrapper::

//! Provide synchronized closure-based access to an owned T through shared references.
//! with reads and with_mut modifies the value while excluding concurrent access. Propagate poisoning as a panic.
//! The wrapper should be Send + Sync when T: Send, including T that is not Sync. Use safe Rust and automatic trait derivation;
//! it must not make a non-Send T transferable. The type name is historical; this is synchronized access, not an unsafe Send escape hatch.

pub struct SendWrapper<T> {
    value: std::sync::Mutex<T>,
}
impl<T> SendWrapper<T> {
    pub fn new(value: T) -> Self {
        todo!()
    }
    pub fn with<F: FnOnce(&T) -> R, R>(&self, f: F) -> R {
        todo!()
    }
    pub fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_send_wrapper() {
        let wrapper = Arc::new(SendWrapper::new(String::from("Hello, World!")));
        let wrapper_clone = wrapper.clone();

        let handle = thread::spawn(move || {
            wrapper_clone.with(|s| {
                assert_eq!(s, "Hello, World!");
                s.len()
            })
        });

        let length = handle.join().unwrap();
        assert_eq!(length, 13);

        // Test mutable access
        wrapper.with_mut(|s| {
            s.push_str(" Modified");
        });

        wrapper.with(|s| {
            assert_eq!(s, "Hello, World! Modified");
        });
    }
}
#[cfg(test)]
mod trait_tests {
    use super::*;
    #[test]
    fn send_but_not_sync_payload() {
        fn check<T: Send + Sync>() {}
        check::<SendWrapper<std::cell::Cell<u32>>>();
        let w = SendWrapper::new(std::cell::Cell::new(1));
        w.with(|c| c.set(2));
        assert_eq!(w.with(|c| c.get()), 2);
    }
}

// cargo test --lib smart_pointers::try_append::

//! Append an owned value through a shared reference to a RefCell<Vec<T>>.
//! If any conflicting borrow is alive, return the original value as Err without changing the vector or panicking.
//! Support non-Clone values. After the conflicting borrow is released, appending must work normally.

pub fn try_append<T>(values: &std::cell::RefCell<Vec<T>>, value: T) -> Result<(), T> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn preserves_rejected_value() {
        #[derive(Debug)]
        struct Item(u32);
        let values = std::cell::RefCell::new(vec![]);
        let guard = values.borrow();
        let item = try_append(&values, Item(7)).unwrap_err();
        assert_eq!(item.0, 7);
        drop(guard);
        try_append(&values, item).unwrap();
        assert_eq!(values.borrow()[0].0, 7);
    }
    #[test]
    fn exclusive_borrow_conflicts() {
        let values = std::cell::RefCell::new(vec![1]);
        let guard = values.borrow_mut();
        assert_eq!(try_append(&values, 2), Err(2));
        drop(guard);
        assert_eq!(*values.borrow(), [1]);
    }
}

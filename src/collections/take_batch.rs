// cargo test --lib collections::take_batch::

//! Remove and return the first min(limit, pending.len()) items, preserving their order.
//! Retain the original vector's allocation for reuse and preserve the order of its remaining items.
//! Items need not implement Clone. Zero limit leaves pending unchanged.

pub fn take_batch<T>(pending: &mut Vec<T>, limit: usize) -> Vec<T> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn removes_prefix_and_reuses_buffer() {
        let mut v = Vec::with_capacity(20);
        v.extend([1, 2, 3, 4]);
        let ptr = v.as_ptr();
        let cap = v.capacity();
        assert_eq!(take_batch(&mut v, 2), [1, 2]);
        assert_eq!(v, [3, 4]);
        assert_eq!(v.as_ptr(), ptr);
        assert_eq!(v.capacity(), cap);
    }
    #[test]
    fn handles_limits() {
        let mut v = vec![1, 2];
        assert!(take_batch(&mut v, 0).is_empty());
        assert_eq!(v, [1, 2]);
        assert_eq!(take_batch(&mut v, 99), [1, 2]);
        assert!(take_batch(&mut v, 1).is_empty());
    }
    #[test]
    fn moves_nonclone_items() {
        struct Item(u8);
        let mut v = vec![Item(7), Item(9)];
        assert_eq!(take_batch(&mut v, 1)[0].0, 7);
        assert_eq!(v[0].0, 9);
    }
}

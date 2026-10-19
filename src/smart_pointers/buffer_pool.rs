// cargo test --lib smart_pointers::buffer_pool::

//! Maintain reusable byte buffers. new(count, capacity) preallocates count empty buffers with at least capacity bytes each.
//! acquire returns a uniquely owned lease; if no buffer is available, allocate one with the configured capacity.
//! The lease dereferences mutably to Vec<u8>. On drop it clears and returns its buffer to the pool, preserving allocation.
//! Multiple leases may coexist. A lease can outlive the pool handle. available counts only returned buffers.
//! Single-threaded use is sufficient. Do not use unsafe code.

pub struct BufferPool {
    available: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
    capacity: usize,
}
pub struct BufferLease {
    pool: std::rc::Rc<std::cell::RefCell<Vec<Vec<u8>>>>,
    buffer: Option<Vec<u8>>,
}
impl BufferPool {
    pub fn new(count: usize, capacity: usize) -> Self {
        todo!()
    }
    pub fn acquire(&self) -> BufferLease {
        todo!()
    }
    pub fn available(&self) -> usize {
        todo!()
    }
}
impl std::ops::Deref for BufferLease {
    type Target = Vec<u8>;
    fn deref(&self) -> &Vec<u8> {
        todo!()
    }
}
impl std::ops::DerefMut for BufferLease {
    fn deref_mut(&mut self) -> &mut Vec<u8> {
        todo!()
    }
}
impl Drop for BufferLease {
    fn drop(&mut self) {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn actually_reuses_allocation() {
        let pool = BufferPool::new(1, 64);
        let mut lease = pool.acquire();
        lease.extend([1, 2, 3]);
        let ptr = lease.as_ptr();
        let cap = lease.capacity();
        assert_eq!(pool.available(), 0);
        drop(lease);
        assert_eq!(pool.available(), 1);
        let lease = pool.acquire();
        assert!(lease.is_empty());
        assert_eq!(lease.as_ptr(), ptr);
        assert_eq!(lease.capacity(), cap);
    }
    #[test]
    fn independent_leases_and_pool_lifetime() {
        let pool = BufferPool::new(0, 16);
        let mut a = pool.acquire();
        let mut b = pool.acquire();
        a.push(1);
        b.push(2);
        assert_eq!(&**a, &[1]);
        assert_eq!(&**b, &[2]);
        drop(pool);
        a.push(3);
        assert_eq!(&**a, &[1, 3]);
    }
}

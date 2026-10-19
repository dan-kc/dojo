// cargo test --lib tokio::bounded_map::

//! Process owned inputs with at most limit asynchronous jobs in flight. Preserve input order in the returned vector.
//! Call the supplied operation once per item; it may borrow caller-owned data and need not be Send or static.
//! Do not spawn detached tasks. If the caller drops the returned future, all in-flight operations must be dropped.
//! Do not eagerly create a future for every input: memory for in-flight futures must be O(limit).

pub async fn bounded_map<T, U, F, Fut>(
    items: Vec<T>,
    limit: std::num::NonZeroUsize,
    operation: F,
) -> Vec<U>
where
    F: FnMut(T) -> Fut,
    Fut: std::future::Future<Output = U>,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroUsize;
    #[tokio::test(start_paused = true)]
    async fn bound_order_and_overlap() {
        use std::cell::Cell;
        use std::rc::Rc;
        let active = Rc::new(Cell::new(0));
        let max = Rc::new(Cell::new(0));
        let output = bounded_map(vec![3, 1, 2, 4], NonZeroUsize::new(2).unwrap(), |n| {
            let active = active.clone();
            let max = max.clone();
            async move {
                active.set(active.get() + 1);
                max.set(max.get().max(active.get()));
                tokio::time::sleep(std::time::Duration::from_secs(n)).await;
                active.set(active.get() - 1);
                n * 2
            }
        })
        .await;
        assert_eq!(output, [6, 2, 4, 8]);
        assert_eq!(max.get(), 2);
        assert_eq!(active.get(), 0);
    }
    #[tokio::test]
    async fn empty() {
        assert!(bounded_map(
            Vec::<u8>::new(),
            NonZeroUsize::new(1).unwrap(),
            |n| async move { n }
        )
        .await
        .is_empty());
    }
    #[tokio::test]
    async fn cancellation_drops_work_and_limits_creation() {
        use std::cell::Cell;
        use std::future::Future;
        use std::rc::Rc;
        struct Guard(Rc<Cell<usize>>);
        impl Drop for Guard {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }
        let created = Cell::new(0);
        let dropped = Rc::new(Cell::new(0));
        let mut work = Box::pin(bounded_map(
            vec![1, 2, 3, 4],
            NonZeroUsize::new(2).unwrap(),
            |_| {
                created.set(created.get() + 1);
                let guard = Guard(dropped.clone());
                async move {
                    let _guard = guard;
                    std::future::pending::<()>().await;
                }
            },
        ));
        assert!(futures::poll!(&mut work).is_pending());
        assert_eq!(created.get(), 2);
        drop(work);
        assert_eq!(dropped.get(), 2);
    }
}

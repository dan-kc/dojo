// cargo test --lib tokio::async_closure_usage::

//! Apply an asynchronous fallible operation sequentially to owned inputs, preserving order.
//! Stop on the first error and do not invoke the operation for later items. Empty input returns an empty vector.
//! The operation may capture borrowed or non-Send data; do not spawn tasks. Move values without Clone.

pub async fn async_closure_usage<T, U, E, F, Fut>(
    items: Vec<T>,
    mut operation: F,
) -> Result<Vec<U>, E>
where
    F: FnMut(T) -> Fut,
    Fut: std::future::Future<Output = Result<U, E>>,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn sequential_error_stops_calls() {
        let called = std::cell::RefCell::new(vec![]);
        let out = async_closure_usage(vec![1, 2, 3], |n| {
            called.borrow_mut().push(n);
            async move {
                if n == 2 {
                    Err("failed")
                } else {
                    Ok(n * 2)
                }
            }
        })
        .await;
        assert_eq!(out, Err("failed"));
        assert_eq!(*called.borrow(), [1, 2]);
    }
    #[tokio::test]
    async fn borrowed_capture_and_empty() {
        let suffix = String::from("!");
        let out = async_closure_usage(vec!["a", "b"], |s| {
            let suffix = &suffix;
            async move { Ok::<_, ()>(format!("{s}{suffix}")) }
        })
        .await
        .unwrap();
        assert_eq!(out, ["a!", "b!"]);
        assert_eq!(
            async_closure_usage(Vec::<u8>::new(), |n| async move { Ok::<_, ()>(n) }).await,
            Ok(vec![])
        );
    }
}

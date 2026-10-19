// cargo test --lib concurrency::timed_message_collector::

//! Collect messages until a fixed deadline or channel disconnection, preserving receive order.
//! An already-expired deadline returns immediately without consuming queued messages. New arrivals do not
//! extend the deadline. Own no producer thread: the caller supplies the channel. Return collected messages.

pub fn timed_message_collector<T>(
    receiver: &std::sync::mpsc::Receiver<T>,
    deadline: std::time::Instant,
) -> Vec<T> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;
    use std::time::{Duration, Instant};
    #[test]
    fn queued_then_disconnected() {
        let (tx, rx) = mpsc::channel();
        tx.send(2).unwrap();
        tx.send(1).unwrap();
        drop(tx);
        assert_eq!(
            timed_message_collector(&rx, Instant::now() + Duration::from_secs(3)),
            [2, 1]
        );
    }
    #[test]
    fn expired_does_not_consume() {
        let (tx, rx) = mpsc::channel();
        tx.send(8).unwrap();
        assert!(timed_message_collector(&rx, Instant::now()).is_empty());
        assert_eq!(rx.try_recv(), Ok(8));
    }
    #[test]
    fn open_empty_channel_times_out() {
        let (_tx, rx) = mpsc::channel::<u8>();
        assert!(
            timed_message_collector(&rx, Instant::now() + Duration::from_millis(20)).is_empty()
        );
    }
}

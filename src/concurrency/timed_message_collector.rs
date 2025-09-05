// Timed Message Collection Practice
//
// Learning Objectives:
// - Use channel recv_timeout for time-based operations
// - Handle timeout scenarios in channel communication
// - Work with time-bounded message collection
//
// cargo test --bin timed_message_collector

/// Implement a timeout-based message receiver that collects messages
/// for a specified duration and then returns all received messages.
/// Use channel recv_timeout for this implementation.
fn timed_message_collector(
    messages: Vec<String>,
    send_interval_ms: u64,
    collect_duration_ms: u64,
) -> Vec<String> {
    todo!("Implement timed message collection with recv_timeout")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timed_message_collector() {
        let messages = vec!["hello".to_string(), "world".to_string(), "test".to_string()];
        let result = timed_message_collector(messages.clone(), 50, 200);
        
        // Should collect all messages sent within 200ms window
        assert!(result.len() >= 2); // At least some messages should be collected
        
        // Test timeout behavior
        let messages = vec!["slow".to_string()];
        let result = timed_message_collector(messages, 300, 100);
        assert_eq!(result.len(), 0); // No messages should be collected due to timeout
    }
}
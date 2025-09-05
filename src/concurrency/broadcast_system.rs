// Broadcast System with Channel Relay Practice
//
// Learning Objectives:
// - Implement broadcast patterns with single-consumer channels
// - Create relay systems for message distribution
// - Work with multiple receivers pattern
//
// cargo test --bin broadcast_system

/// Create a broadcast system where one sender sends messages to multiple
/// receivers. Since mpsc doesn't support multiple consumers directly,
/// implement a relay system that distributes messages to multiple channels.
fn broadcast_system(message: String, num_receivers: usize) -> Vec<String> {
    todo!("Implement broadcast system using channel relay")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_broadcast_system() {
        let message = "broadcast test".to_string();
        let results = broadcast_system(message.clone(), 3);
        
        assert_eq!(results.len(), 3);
        for result in results {
            assert_eq!(result, message);
        }
        
        // Test with single receiver
        let results = broadcast_system("single".to_string(), 1);
        assert_eq!(results, vec!["single".to_string()]);
    }
}
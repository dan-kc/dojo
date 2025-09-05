# BTree Timeline Solution

## Implementation

```rust
#[derive(Debug, Clone, PartialEq)]
struct Event {
    id: u32,
    description: String,
    duration: u32,
}

struct Timeline {
    events: std::collections::BTreeMap<u64, Vec<Event>>,
}

impl Timeline {
    fn new() -> Self {
        Self {
            events: std::collections::BTreeMap::new(),
        }
    }

    fn add_event(&mut self, timestamp: u64, event: Event) {
        self.events
            .entry(timestamp)
            .or_insert_with(Vec::new)
            .push(event);
    }

    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<(u64, &Event)> {
        let mut results = Vec::new();
        
        for (timestamp, events) in self.events.range(start..=end) {
            for event in events {
                results.push((*timestamp, event));
            }
        }
        
        results
    }

    fn get_next_event(&self, after: u64) -> Option<(u64, &Event)> {
        // Find first timestamp > after
        for (timestamp, events) in self.events.range((std::ops::Bound::Excluded(after), std::ops::Bound::Unbounded)) {
            if !events.is_empty() {
                return Some((*timestamp, &events[0]));
            }
        }
        None
    }

    fn remove_events_before(&mut self, timestamp: u64) {
        // Collect keys to remove
        let keys_to_remove: Vec<u64> = self.events
            .range(..timestamp)
            .map(|(k, _)| *k)
            .collect();
        
        for key in keys_to_remove {
            self.events.remove(&key);
        }
    }

    fn get_overlapping_events(&self, timestamp: u64, duration: u32) -> Vec<(u64, &Event)> {
        let mut results = Vec::new();
        let period_end = timestamp + duration as u64;
        
        // Check all events that might overlap
        for (event_start, events) in &self.events {
            for event in events {
                let event_end = event_start + event.duration as u64;
                
                // Check if events overlap
                // Event overlaps if: event_start < period_end && event_end > timestamp
                if *event_start < period_end && event_end > timestamp {
                    results.push((*event_start, event));
                }
            }
        }
        
        results
    }
}
```

## Explanation

This solution implements a timeline data structure for event scheduling:

1. **Event storage**: BTreeMap maps timestamps to vectors of events
2. **Multiple events**: Supports multiple events at same timestamp
3. **Range queries**: Efficiently finds events within time periods
4. **Next event**: Uses range with excluded bound to find future events
5. **Overlap detection**: Checks interval intersection for all events
6. **Bulk removal**: Removes historical events before cutoff

## Key Learning Points

- **Temporal indexing**: Using BTreeMap for time-based ordering
- **Range operations**: Efficient queries over time intervals
- **Overlap algorithm**: Interval intersection logic
- **Multiple values**: Handling multiple events at same timestamp
- **Bound types**: Using Excluded bound for "greater than" queries

## Rust Concepts Demonstrated

- BTreeMap for temporal ordering
- Range queries with various bound types
- Entry API for vector initialization
- Interval overlap detection logic
- Collecting keys for bulk removal
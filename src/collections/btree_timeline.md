# BTree Timeline Solution

## Implementation

```rust
#[derive(Debug, Clone, PartialEq)]
struct Event {
    id: u32,
    description: String,
    duration: u32,
}

use std::collections;
use std::ops;
impl Timeline {
    fn new() -> Self {
        return Self {
            events: collections::BTreeMap::new(),
        };
    }

    fn add_event(&mut self, timestamp: u64, event: Event) {
        self.events.entry(timestamp).or_default().push(event);
    }

    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<(u64, &Event)> {
        let mut res = vec![];
        for (k, v) in self.events.range(start..end) {
            for event in v {
                res.push((k.clone(), event))
            }
        }
        return res;
    }

    fn get_next_event(&self, after: u64) -> Option<(u64, &Event)> {
        let (timestamp, events) = self
            .events
            .range((ops::Bound::Excluded(after), ops::Bound::Unbounded))
            .next()?;
        return Some((*timestamp, events.first()?));
    }

    fn remove_events_before(&mut self, timestamp: u64) {
        let x: Vec<u64> = self
            .events
            .range(..timestamp)
            .map(|(k, _)| return k.clone())
            .collect();

        for k in x {
            self.events.remove(&k);
        }
    }

    fn get_overlapping_events(&self, timestamp: u64, duration: u32) -> Vec<(u64, &Event)> {
        let mut res = vec![];
        let end_range = duration as u64 + timestamp;

        for (event_start_time, events) in self.events.iter() {
            for event in events {
                let event_end_time = event_start_time + event.duration as u64;
                if event_end_time < timestamp || *event_start_time > end_range {
                    continue;
                }

                res.push((*event_start_time, event))
            }
        }

        return res;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timeline() {
        let mut timeline = Timeline::new();

        let event1 = Event {
            id: 1,
            description: "Meeting".to_string(),
            duration: 60,
        };
        let event2 = Event {
            id: 2,
            description: "Call".to_string(),
            duration: 30,
        };
        let event3 = Event {
            id: 3,
            description: "Lunch".to_string(),
            duration: 90,
        };

        timeline.add_event(1000, event1.clone());
        timeline.add_event(1030, event2.clone());
        timeline.add_event(1200, event3.clone());

        // Test range queries
        let events_in_range = timeline.get_events_in_range(1000, 1100);
        assert_eq!(events_in_range.len(), 2);

        // Test next event
        let next = timeline.get_next_event(1050);
        assert!(next.is_some());
        assert_eq!(next.unwrap().1.id, 3);

        // Test overlapping events (meeting from 1000-1060 overlaps with call at 1030-1060)
        let overlapping = timeline.get_overlapping_events(1020, 40); // 1020-1060
        assert!(!overlapping.is_empty());

        // Test removal
        timeline.remove_events_before(1100);
        let remaining = timeline.get_events_in_range(0, 2000);
        assert_eq!(remaining.len(), 1); // Only lunch should remain
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

// BTree Timeline Practice
//
// Learning Objectives:
// - Use BTreeMap for time-based event scheduling
// - Implement range queries for time periods
// - Practice event overlap detection algorithms
// - Handle multiple events at same timestamp
//
// Run with: cargo test btree_timeline

/// Implement a timeline data structure using BTreeMap for event scheduling.
/// Events have timestamps and can be queried by time ranges.
#[derive(Debug, Clone, PartialEq)]
struct Event {
    id: u32,
    description: String,
    duration: u32, // in minutes
}

struct Timeline {
    events: std::collections::BTreeMap<u64, Vec<Event>>, // timestamp -> events
}

impl Timeline {
    fn new() -> Self {
        todo!()
    }

    fn add_event(&mut self, timestamp: u64, event: Event) {
        todo!()
    }

    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<(u64, &Event)> {
        todo!()
    }

    fn get_next_event(&self, after: u64) -> Option<(u64, &Event)> {
        todo!()
    }

    fn remove_events_before(&mut self, timestamp: u64) {
        todo!()
    }

    fn get_overlapping_events(&self, timestamp: u64, duration: u32) -> Vec<(u64, &Event)> {
        todo!()
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

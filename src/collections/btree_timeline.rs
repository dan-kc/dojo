// cargo test --lib collections::btree_timeline::

//! Timestamps and durations use the same unit (minutes). Range queries include both endpoints and sort by timestamp, then insertion order. Next event means strictly after the argument. Overlap uses half-open intervals; zero-duration events overlap nothing. Compute interval endpoints without overflow. Removing before a timestamp is strict.

/// Implement a timeline data structure using BTreeMap for event scheduling.
/// Events have timestamps and can be queried by time ranges.
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)]
struct Event {
    id: u32,
    description: String,
    duration: u32, // in minutes
}

#[allow(dead_code)]
struct Timeline {
    events: std::collections::BTreeMap<u64, Vec<Event>>, // timestamp -> events
}

#[allow(dead_code)]
impl Timeline {
    fn new() -> Self {
        todo!()
    }

    #[allow(unused_variables)]
    fn add_event(&mut self, timestamp: u64, event: Event) {
        todo!()
    }

    #[allow(unused_variables)]
    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<(u64, &Event)> {
        todo!()
    }

    #[allow(unused_variables)]
    fn get_next_event(&self, after: u64) -> Option<(u64, &Event)> {
        todo!()
    }

    #[allow(unused_variables)]
    fn remove_events_before(&mut self, timestamp: u64) {
        todo!()
    }

    #[allow(unused_variables)]
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

#[cfg(test)]
mod boundary_tests {
    use super::*;
    #[test]
    fn boundaries_and_overflow() {
        let mut t = Timeline::new();
        let e = |id, duration| Event {
            id,
            description: String::new(),
            duration,
        };
        t.add_event(10, e(1, 5));
        t.add_event(15, e(2, 0));
        t.add_event(u64::MAX, e(3, 2));
        assert_eq!(t.get_events_in_range(10, 10).len(), 1);
        assert!(t.get_events_in_range(20, 10).is_empty());
        assert!(t.get_overlapping_events(15, 1).is_empty());
        assert!(t.get_overlapping_events(10, 0).is_empty());
        assert_eq!(t.get_overlapping_events(u64::MAX, 1)[0].1.id, 3);
        t.remove_events_before(10);
        assert_eq!(t.get_events_in_range(10, 10).len(), 1);
    }
}

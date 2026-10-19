# collections/btree_timeline

Reference implementation. Read after attempting the exercise.

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
        self.events.entry(timestamp).or_default().push(event);
    }
    fn get_events_in_range(&self, start: u64, end: u64) -> Vec<(u64, &Event)> {
        if start > end {
            return vec![];
        }
        self.events
            .range(start..=end)
            .flat_map(|(&t, es)| es.iter().map(move |e| (t, e)))
            .collect()
    }
    fn get_next_event(&self, after: u64) -> Option<(u64, &Event)> {
        self.events
            .range((std::ops::Bound::Excluded(after), std::ops::Bound::Unbounded))
            .next()
            .and_then(|(&t, es)| es.first().map(|e| (t, e)))
    }
    fn remove_events_before(&mut self, timestamp: u64) {
        self.events.retain(|&t, _| t >= timestamp);
    }
    fn get_overlapping_events(&self, timestamp: u64, duration: u32) -> Vec<(u64, &Event)> {
        if duration == 0 {
            return vec![];
        }
        let end = u128::from(timestamp) + u128::from(duration);
        self.events
            .iter()
            .flat_map(|(&t, es)| es.iter().map(move |e| (t, e)))
            .filter(|(t, e)| {
                e.duration > 0
                    && u128::from(*t) < end
                    && u128::from(timestamp) < u128::from(*t) + u128::from(e.duration)
            })
            .collect()
    }
}
```

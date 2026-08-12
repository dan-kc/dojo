// cargo test median_tracker

/// Implement a median tracker for i32.
/// Efficiently maintain running median as elements are added.

#[derive(Debug)]
pub struct MedianTracker {
    // TODO
}

impl MedianTracker {
    pub fn new() -> Self {
        todo!()
    }

    pub fn add(&mut self, value: i32) {
        todo!()
    }

    pub fn get_median(&self) -> Option<f64> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_tracker_has_no_median() {
        let tracker = MedianTracker::new();

        assert_eq!(tracker.get_median(), None);
    }

    #[test]
    fn single_element_is_the_median() {
        let mut tracker = MedianTracker::new();

        tracker.add(42);

        assert_eq!(tracker.get_median(), Some(42.0));
    }

    #[test]
    fn odd_number_of_elements_returns_the_middle_value() {
        let mut tracker = MedianTracker::new();

        tracker.add(1);
        tracker.add(2);
        tracker.add(3);

        assert_eq!(tracker.get_median(), Some(2.0));
    }

    #[test]
    fn even_number_of_elements_returns_the_mean_of_the_middle_values() {
        let mut tracker = MedianTracker::new();

        tracker.add(1);
        tracker.add(2);
        tracker.add(3);
        tracker.add(4);

        assert_eq!(tracker.get_median(), Some(2.5));
    }

    #[test]
    fn insertion_order_does_not_affect_the_median() {
        let mut tracker = MedianTracker::new();

        tracker.add(9);
        tracker.add(1);
        tracker.add(7);
        tracker.add(3);
        tracker.add(5);

        assert_eq!(tracker.get_median(), Some(5.0));
    }

    #[test]
    fn duplicate_values_are_included() {
        let mut tracker = MedianTracker::new();

        tracker.add(2);
        tracker.add(2);
        tracker.add(2);
        tracker.add(10);

        assert_eq!(tracker.get_median(), Some(2.0));
    }

    #[test]
    fn negative_values_are_ordered_correctly() {
        let mut tracker = MedianTracker::new();

        tracker.add(-10);
        tracker.add(-4);
        tracker.add(-2);

        assert_eq!(tracker.get_median(), Some(-4.0));
    }

    #[test]
    fn median_can_fall_between_negative_and_positive_values() {
        let mut tracker = MedianTracker::new();

        tracker.add(-5);
        tracker.add(2);

        assert_eq!(tracker.get_median(), Some(-1.5));
    }

    #[test]
    fn median_updates_after_each_insertion() {
        let mut tracker = MedianTracker::new();

        tracker.add(5);
        assert_eq!(tracker.get_median(), Some(5.0));

        tracker.add(1);
        assert_eq!(tracker.get_median(), Some(3.0));

        tracker.add(9);
        assert_eq!(tracker.get_median(), Some(5.0));

        tracker.add(2);
        assert_eq!(tracker.get_median(), Some(3.5));

        tracker.add(8);
        assert_eq!(tracker.get_median(), Some(5.0));
    }

    #[test]
    fn descending_values_are_rebalanced() {
        let mut tracker = MedianTracker::new();

        tracker.add(5);
        tracker = dbg!(tracker);
        tracker.add(4);
        tracker = dbg!(tracker);
        tracker.add(3);
        tracker = dbg!(tracker);
        tracker.add(2);
        tracker = dbg!(tracker);
        tracker.add(1);
        tracker = dbg!(tracker);

        assert_eq!(tracker.get_median(), Some(3.0));
        //          left: Some(1.0)
        // right: Some(3.0)
    }

    #[test]
    fn averaging_integer_boundaries_does_not_overflow() {
        let mut tracker = MedianTracker::new();

        tracker.add(i32::MIN);
        tracker.add(i32::MAX);

        assert_eq!(tracker.get_median(), Some(-0.5));
    }
}

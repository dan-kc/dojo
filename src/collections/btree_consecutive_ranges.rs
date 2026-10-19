// cargo test --lib collections::btree_consecutive_ranges::

//! Return maximal inclusive runs of consecutive i32 values in ascending order, including singleton runs.
//! Empty input returns no runs. Handle i32::MIN and i32::MAX without overflow.

fn find_consecutive_ranges(set: &std::collections::BTreeSet<i32>) -> Vec<(i32, i32)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn test_find_consecutive_ranges() {
        let set: BTreeSet<i32> = [1, 2, 3, 5, 6, 10, 11, 12, 13, 20].into_iter().collect();
        let ranges = find_consecutive_ranges(&set);

        // Expected ranges: [1,3], [5,6], [10,13], and [20,20]
        assert_eq!(ranges.len(), 4);
        assert!(ranges.contains(&(1, 3)));
        assert!(ranges.contains(&(5, 6)));
        assert!(ranges.contains(&(10, 13)));
        assert!(ranges.contains(&(20, 20)));
    }

    #[test]
    fn test_edge_cases() {
        // Test consecutive ranges with single elements
        let single_set: BTreeSet<i32> = [5].into_iter().collect();
        let single_ranges = find_consecutive_ranges(&single_set);
        assert_eq!(single_ranges, vec![(5, 5)]);
    }
}

#[cfg(test)]
mod boundary_tests {
    use super::*;
    #[test]
    fn integer_limits() {
        assert_eq!(
            find_consecutive_ranges(&[i32::MIN, i32::MIN + 1, i32::MAX].into_iter().collect()),
            [(i32::MIN, i32::MIN + 1), (i32::MAX, i32::MAX)]
        );
    }
}

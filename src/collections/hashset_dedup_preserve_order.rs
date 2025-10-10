// cargo test hashset_dedup_preserve_order

/// Return Vec with duplicates removed, maintaining first occurrence order.
#[allow(unused_variables)]
pub fn dedup_preserve_order<T>(items: Vec<T>) -> Vec<T>
where
    T: Clone + std::hash::Hash + Eq,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup_preserve_order() {
        let items = vec!["a", "b", "a", "c", "b", "d", "a"];
        let result = dedup_preserve_order(items);

        assert_eq!(result, vec!["a", "b", "c", "d"]);

        // Test with no duplicates
        let no_dups = vec!["x", "y", "z"];
        let result = dedup_preserve_order(no_dups.clone());
        assert_eq!(result, no_dups);

        // Test with all duplicates
        let all_dups = vec!["same", "same", "same"];
        let result = dedup_preserve_order(all_dups);
        assert_eq!(result, vec!["same"]);
    }

    #[test]
    fn test_dedup_preserve_order_numbers() {
        let items = vec![1, 2, 3, 2, 4, 1, 5, 3];
        let result = dedup_preserve_order(items);

        assert_eq!(result, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_dedup_preserve_order_empty() {
        let empty: Vec<i32> = vec![];
        let result = dedup_preserve_order(empty);

        assert!(result.is_empty());
    }

    #[test]
    fn test_dedup_preserve_order_single_element() {
        let single = vec![42];
        let result = dedup_preserve_order(single);

        assert_eq!(result, vec![42]);
    }

    #[test]
    fn test_dedup_preserve_order_strings() {
        let items = vec![
            "hello".to_string(),
            "world".to_string(),
            "hello".to_string(),
            "rust".to_string(),
            "world".to_string(),
            "programming".to_string(),
        ];

        let result = dedup_preserve_order(items);

        let expected = vec![
            "hello".to_string(),
            "world".to_string(),
            "rust".to_string(),
            "programming".to_string(),
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_dedup_preserve_order_maintains_first_occurrence() {
        let items = vec![1, 2, 3, 1, 4, 2, 5];
        let result = dedup_preserve_order(items);

        // Should keep the first occurrence of each element
        assert_eq!(result, vec![1, 2, 3, 4, 5]);

        // Verify order is based on first occurrence
        let items2 = vec![5, 4, 3, 2, 1, 5, 4, 3, 2, 1];
        let result2 = dedup_preserve_order(items2);
        assert_eq!(result2, vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn test_dedup_preserve_order_large_input() {
        // Create a large vector with many duplicates
        let mut items = Vec::new();
        for i in 0..1000 {
            items.push(i % 10); // Will create many duplicates
        }

        let result = dedup_preserve_order(items);

        // Should only contain 0-9 in order
        assert_eq!(result, vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_dedup_preserve_order_complex_duplicates() {
        let items = vec![
            'a', 'b', 'c', 'a', 'b', 'c', 'a', 'b', 'c', 'd', 'e', 'f', 'd', 'e', 'f',
        ];

        let result = dedup_preserve_order(items);

        assert_eq!(result, vec!['a', 'b', 'c', 'd', 'e', 'f']);
    }

    #[test]
    fn test_dedup_preserve_order_alternating() {
        let items = vec![1, 2, 1, 2, 1, 2, 3, 4, 3, 4];
        let result = dedup_preserve_order(items);

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_dedup_preserve_order_adjacent_duplicates() {
        let items = vec![1, 1, 1, 2, 2, 3, 3, 3, 3, 4];
        let result = dedup_preserve_order(items);

        assert_eq!(result, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_dedup_preserve_order_negative_numbers() {
        let items = vec![-1, 0, -1, 1, 0, -2, 1, -2];
        let result = dedup_preserve_order(items);

        assert_eq!(result, vec![-1, 0, 1, -2]);
    }

    #[test]
    fn test_dedup_preserve_order_mixed_types() {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        struct TestItem {
            id: i32,
            name: String,
        }

        let items = vec![
            TestItem {
                id: 1,
                name: "One".to_string(),
            },
            TestItem {
                id: 2,
                name: "Two".to_string(),
            },
            TestItem {
                id: 1,
                name: "One".to_string(),
            }, // Duplicate
            TestItem {
                id: 3,
                name: "Three".to_string(),
            },
            TestItem {
                id: 2,
                name: "Two".to_string(),
            }, // Duplicate
        ];

        let result = dedup_preserve_order(items);

        let expected = vec![
            TestItem {
                id: 1,
                name: "One".to_string(),
            },
            TestItem {
                id: 2,
                name: "Two".to_string(),
            },
            TestItem {
                id: 3,
                name: "Three".to_string(),
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn test_dedup_preserve_order_performance_characteristic() {
        // This test verifies that the function can handle reasonable sizes efficiently
        let size = 10000;
        let mut items = Vec::new();

        // Create a vector where every element appears twice
        for i in 0..size {
            items.push(i);
        }
        for i in 0..size {
            items.push(i);
        }

        let result = dedup_preserve_order(items);

        // Should contain each number exactly once
        assert_eq!(result.len(), size);
        for i in 0..size {
            assert_eq!(result[i], i); //          left: 4806 right: 0
        }
    }

    // Note: This test is commented out because floating point numbers don't implement Hash or Eq
    // #[test]
    // fn test_dedup_preserve_order_floating_point() {
    //     let items = vec![1.0, 2.5, 1.0, 3.14, 2.5, 4.0, 3.14];
    //     let result = dedup_preserve_order(items);
    //
    //     assert_eq!(result, vec![1.0, 2.5, 3.14, 4.0]);
    // }
}

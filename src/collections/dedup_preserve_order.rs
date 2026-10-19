// cargo test --lib collections::dedup_preserve_order::

//! Preserve the first occurrence of each equal value in input order. Aim for expected O(n) time. Empty input returns empty output.

/// Implement an efficient deduplication that preserves order of first occurrence.
#[allow(unused_variables)]
pub fn dedup_preserve_order<T>(vec: Vec<T>) -> Vec<T>
where
    T: Clone + PartialEq + Eq + std::hash::Hash,
{
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dedup_preserve_order() {
        let vec = vec![1, 2, 2, 3, 1, 4, 3, 5];
        let result = dedup_preserve_order(vec);
        assert_eq!(result, vec![1, 2, 3, 4, 5]);

        let vec = vec![1, 1, 1];
        let result = dedup_preserve_order(vec);
        assert_eq!(result, vec![1]);

        let empty: Vec<i32> = vec![];
        let result = dedup_preserve_order(empty);
        assert!(result.is_empty());
    }

    #[test]
    fn test_dedup_strings() {
        let vec = vec![
            "a".to_string(),
            "b".to_string(),
            "a".to_string(),
            "c".to_string(),
            "b".to_string(),
        ];
        let result = dedup_preserve_order(vec);
        assert_eq!(
            result,
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        );
    }

    #[test]
    fn test_dedup_already_unique() {
        let vec = vec![1, 2, 3, 4, 5];
        let result = dedup_preserve_order(vec.clone());
        assert_eq!(result, vec);
    }
}

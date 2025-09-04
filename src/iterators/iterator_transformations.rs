// Iterator Transformations Practice
//
// Learning objectives:
// - Advanced iterator methods: enumerate, zip, rev, skip
// - Chaining multiple transformations
// - Working with complex data structures
// - Iterator performance patterns
//
// cargo test --lib iterators::iterator_transformations

#[derive(Debug, PartialEq, Clone)]
pub struct Person {
    pub name: String,
    pub age: u32,
    pub email: String,
}

/// Create a function that takes two vectors of equal length and returns
/// a vector of tuples pairing elements at the same indices.
/// If vectors have different lengths, pair up to the shorter length.
pub fn zip_vectors<T, U>(first: Vec<T>, second: Vec<U>) -> Vec<(T, U)>
where
    T: Clone,
    U: Clone,
{
    todo!("Use zip() to combine two vectors")
}

/// Given a vector of words, return a vector of tuples containing
/// (index, word, character_count) for words longer than 4 characters.
/// Skip the first 2 words in the original vector.
pub fn enumerate_long_words(words: Vec<String>) -> Vec<(usize, String, usize)> {
    todo!("Combine skip(), enumerate(), filter(), and map()")
}

/// Transform a vector of Person structs into a vector of formatted strings
/// containing "Name: {name}, Age: {age}" but only for people over 18,
/// sorted by age in descending order.
pub fn format_adults(people: Vec<Person>) -> Vec<String> {
    todo!("Chain filter(), sort operations, and map()")
}

/// Given a vector of integers, create windows of size 3 and return
/// the sum of each window. For example, [1,2,3,4,5] would create
/// windows [1,2,3], [2,3,4], [3,4,5] with sums [6, 9, 12].
pub fn sliding_window_sums(numbers: Vec<i32>) -> Vec<i32> {
    todo!("Use windows() method and map to sum each window")
}

/// Create a function that finds the intersection of two sorted vectors
/// (elements that appear in both) and returns them as a sorted vector
/// without duplicates.
pub fn sorted_intersection(first: Vec<i32>, second: Vec<i32>) -> Vec<i32> {
    todo!("Use iterator methods to find common elements efficiently")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zip_vectors_equal_length() {
        let first = vec![1, 2, 3];
        let second = vec!["a", "b", "c"];
        let result = zip_vectors(first, second);
        assert_eq!(result, vec![(1, "a"), (2, "b"), (3, "c")]);
    }

    #[test]
    fn test_zip_vectors_different_lengths() {
        let first = vec![1, 2, 3, 4, 5];
        let second = vec!["a", "b"];
        let result = zip_vectors(first, second);
        assert_eq!(result, vec![(1, "a"), (2, "b")]);
    }

    #[test]
    fn test_zip_vectors_empty() {
        let first: Vec<i32> = vec![];
        let second = vec!["a", "b"];
        let result = zip_vectors(first, second);
        assert_eq!(result, Vec::<(i32, &str)>::new());
    }

    #[test]
    fn test_enumerate_long_words() {
        let words = vec![
            "hi".to_string(),
            "go".to_string(),
            "hello".to_string(),
            "world".to_string(),
            "programming".to_string(),
            "rust".to_string(),
        ];
        let result = enumerate_long_words(words);
        // Skips "hi", "go", then enumerates from index 0 for remaining
        // Filters for words > 4 chars: "hello"(5), "world"(5), "programming"(11)
        assert_eq!(result, vec![
            (0, "hello".to_string(), 5),
            (1, "world".to_string(), 5),
            (2, "programming".to_string(), 11),
        ]);
    }

    #[test]
    fn test_enumerate_long_words_insufficient_data() {
        let words = vec!["hi".to_string()];
        let result = enumerate_long_words(words);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_format_adults() {
        let people = vec![
            Person {
                name: "Alice".to_string(),
                age: 25,
                email: "alice@example.com".to_string(),
            },
            Person {
                name: "Bob".to_string(),
                age: 17,
                email: "bob@example.com".to_string(),
            },
            Person {
                name: "Charlie".to_string(),
                age: 30,
                email: "charlie@example.com".to_string(),
            },
            Person {
                name: "Diana".to_string(),
                age: 22,
                email: "diana@example.com".to_string(),
            },
        ];
        let result = format_adults(people);
        assert_eq!(result, vec![
            "Name: Charlie, Age: 30",
            "Name: Alice, Age: 25",
            "Name: Diana, Age: 22",
        ]);
    }

    #[test]
    fn test_format_adults_no_adults() {
        let people = vec![
            Person {
                name: "Kid1".to_string(),
                age: 15,
                email: "kid1@example.com".to_string(),
            },
            Person {
                name: "Kid2".to_string(),
                age: 12,
                email: "kid2@example.com".to_string(),
            },
        ];
        let result = format_adults(people);
        assert_eq!(result, Vec::<String>::new());
    }

    #[test]
    fn test_sliding_window_sums() {
        let numbers = vec![1, 2, 3, 4, 5];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, vec![6, 9, 12]); // [1+2+3, 2+3+4, 3+4+5]
    }

    #[test]
    fn test_sliding_window_sums_insufficient_elements() {
        let numbers = vec![1, 2];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_sliding_window_sums_exact_window_size() {
        let numbers = vec![10, 20, 30];
        let result = sliding_window_sums(numbers);
        assert_eq!(result, vec![60]);
    }

    #[test]
    fn test_sorted_intersection() {
        let first = vec![1, 2, 3, 4, 5];
        let second = vec![3, 4, 5, 6, 7];
        let result = sorted_intersection(first, second);
        assert_eq!(result, vec![3, 4, 5]);
    }

    #[test]
    fn test_sorted_intersection_no_common() {
        let first = vec![1, 2, 3];
        let second = vec![4, 5, 6];
        let result = sorted_intersection(first, second);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_sorted_intersection_with_duplicates() {
        let first = vec![1, 1, 2, 3, 3];
        let second = vec![1, 3, 3, 4, 4];
        let result = sorted_intersection(first, second);
        assert_eq!(result, vec![1, 3]);
    }
}
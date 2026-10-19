// cargo test --lib iterators::enumerate_long_words::

//! Return (original_index, owned_word, Unicode_scalar_count) for words longer than min_chars,
//! after skipping the first skip input words. Preserve order and move strings without cloning.
//! Indices refer to the original vector, including skipped and filtered entries.

pub fn enumerate_long_words(
    words: Vec<String>,
    skip: usize,
    min_chars: usize,
) -> Vec<(usize, String, usize)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_positions() {
        assert_eq!(
            enumerate_long_words(
                ["skip", "hello", "a", "world"].map(str::to_owned).to_vec(),
                1,
                4
            ),
            vec![(1, "hello".into(), 5), (3, "world".into(), 5)]
        );
    }
    #[test]
    fn unicode_counts_scalars() {
        assert_eq!(
            enumerate_long_words(["🦀", "café", "日本語"].map(str::to_owned).to_vec(), 0, 2),
            vec![(1, "café".into(), 4), (2, "日本語".into(), 3)]
        );
    }
    #[test]
    fn excessive_skip() {
        assert!(enumerate_long_words(vec!["x".into()], 4, 0).is_empty());
    }
}

// cargo test --lib lifetimes::find_longest_string::

//! Return a borrowed longest string by UTF-8 byte length, choosing the first on ties. Empty input returns None.
//! The returned string must be usable after the slice containing the references is dropped, provided its underlying text is still alive.

pub fn find_longest_string<'a>(strings: &[&'a str]) -> Option<&'a str> {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn selection() {
        assert_eq!(find_longest_string(&["hi", "rust", "code"]), Some("rust"));
        assert_eq!(find_longest_string(&[]), None);
    }
    #[test]
    fn result_outlives_reference_array() {
        let owned = "borrowed text".to_owned();
        let selected;
        {
            let refs = [owned.as_str(), "short"];
            selected = find_longest_string(&refs);
        }
        assert_eq!(selected, Some(owned.as_str()));
    }
}

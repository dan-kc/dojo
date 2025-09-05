// Repeat Longer Owned
//
// Learning objectives:
// - Understanding when lifetime annotations aren't needed
// - Working with owned data vs references
// - String manipulation and ownership
//
// cargo test --bin repeat_longer_owned

/// Create a function that takes two string slices and returns a String
/// containing the concatenation of the longer slice repeated twice.
/// This demonstrates when you don't need lifetime annotations.
pub fn repeat_longer_owned(s1: &str, s2: &str) -> String {
    todo!("Return owned String, no lifetime annotations needed")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repeat_longer_owned() {
        let s1 = "hi";
        let s2 = "hello";
        assert_eq!(repeat_longer_owned(s1, s2), "hellohello");
    }

    #[test]
    fn test_repeat_longer_owned_first_longer() {
        let s1 = "programming";
        let s2 = "code";
        assert_eq!(repeat_longer_owned(s1, s2), "programmingprogramming");
    }

    #[test]
    fn test_repeat_longer_owned_equal_length() {
        let s1 = "rust";
        let s2 = "code";
        // When equal, should use the first one
        assert_eq!(repeat_longer_owned(s1, s2), "rustrust");
    }

    #[test]
    fn test_repeat_longer_owned_empty() {
        let s1 = "";
        let s2 = "hello";
        assert_eq!(repeat_longer_owned(s1, s2), "hellohello");
    }

    #[test]
    fn test_repeat_longer_owned_both_empty() {
        let s1 = "";
        let s2 = "";
        assert_eq!(repeat_longer_owned(s1, s2), "");
    }

    #[test]
    fn test_repeat_longer_owned_unicode() {
        let s1 = "hello";
        let s2 = "世界🌍";
        assert_eq!(repeat_longer_owned(s1, s2), "世界🌍世界🌍");
    }
}

fn main() {
    println!("Run tests with: cargo test --bin repeat_longer_owned");
}
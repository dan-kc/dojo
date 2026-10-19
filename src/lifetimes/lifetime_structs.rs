// cargo test --lib lifetimes::lifetime_structs::

//! Implement a borrowed word parser over Unicode-whitespace-separated input.
//! next_word advances past the next word; peek_word returns it without advancing.
//! remaining returns the exact unconsumed suffix (including whitespace after the last word).
//! Returned words borrow the original input, so earlier words remain usable while advancing the parser.
//! Avoid allocations and repeated scanning of previously consumed text.

pub struct Parser<'a> {
    input: &'a str,
    position: usize,
}
impl<'a> Parser<'a> {
    pub fn new(input: &'a str) -> Self {
        todo!()
    }
    pub fn remaining(&self) -> &'a str {
        todo!()
    }
    pub fn peek_word(&self) -> Option<&'a str> {
        todo!()
    }
    pub fn next_word(&mut self) -> Option<&'a str> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn words_survive_advancement() {
        let text = "  hello	世界  ".to_owned();
        let mut p = Parser::new(&text);
        let a = p.next_word().unwrap();
        assert_eq!(p.peek_word(), Some("世界"));
        assert_eq!(p.peek_word(), Some("世界"));
        let b = p.next_word().unwrap();
        assert_eq!((a, b), ("hello", "世界"));
        assert_eq!(p.remaining(), "  ");
        assert_eq!(p.next_word(), None);
        assert_eq!(p.remaining(), "");
    }
    #[test]
    fn empty_and_unicode_space() {
        let mut p = Parser::new(
            "\u{2003}a
🦀",
        );
        assert_eq!(p.next_word(), Some("a"));
        assert_eq!(p.next_word(), Some("🦀"));
        assert_eq!(p.next_word(), None);
        assert_eq!(Parser::new("").peek_word(), None);
    }
}

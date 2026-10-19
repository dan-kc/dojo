// cargo test --lib lifetimes::text_holder::

//! Hold borrowed text and return its first n Unicode scalar values without allocation.
//! If n exceeds the character count, return the full text. Zero yields an empty prefix.
//! Returned text borrows the original input and may outlive the holder.

pub struct TextHolder<'a> {
    text: &'a str,
}
impl<'a> TextHolder<'a> {
    pub fn new(text: &'a str) -> Self {
        todo!()
    }
    pub fn get_text(&self) -> &'a str {
        todo!()
    }
    pub fn get_prefix(&self, n: usize) -> &'a str {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn utf8_boundaries() {
        let h = TextHolder::new("🦀café");
        assert_eq!(h.get_prefix(1), "🦀");
        assert_eq!(h.get_prefix(3), "🦀ca");
        assert_eq!(h.get_prefix(99), "🦀café");
        assert_eq!(h.get_prefix(0), "");
    }
    #[test]
    fn prefix_outlives_holder() {
        let text = "owned".to_owned();
        let prefix;
        {
            let holder = TextHolder::new(&text);
            prefix = holder.get_prefix(2);
        }
        assert_eq!(prefix, "ow");
    }
}

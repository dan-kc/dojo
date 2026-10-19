// cargo test --lib collections::card_deck::

//! Manage a caller-supplied deck of cards. Top is the front, bottom is the back.
//! Deal and peek return None when empty. insert_at allows index == len and panics beyond len.
//! Shuffle using the caller's random generator; preserve every card and duplicate exactly once.

pub struct CardDeck {
    cards: std::collections::VecDeque<String>,
}
impl CardDeck {
    pub fn new(cards: Vec<String>) -> Self {
        todo!()
    }
    pub fn shuffle(&mut self, rng: &mut impl rand::Rng) {
        todo!()
    }
    pub fn deal_from_top(&mut self) -> Option<String> {
        todo!()
    }
    pub fn deal_from_bottom(&mut self) -> Option<String> {
        todo!()
    }
    pub fn insert_at(&mut self, index: usize, card: String) {
        todo!()
    }
    pub fn peek_top(&self) -> Option<&String> {
        todo!()
    }
    pub fn peek_bottom(&self) -> Option<&String> {
        todo!()
    }
    pub fn remaining_cards(&self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    #[test]
    fn both_ends_and_insertion() {
        let mut d = CardDeck::new(vec!["a".into(), "b".into()]);
        d.insert_at(1, "c".into());
        assert_eq!(d.peek_top().map(String::as_str), Some("a"));
        assert_eq!(d.peek_bottom().map(String::as_str), Some("b"));
        assert_eq!(d.deal_from_top().as_deref(), Some("a"));
        assert_eq!(d.deal_from_bottom().as_deref(), Some("b"));
        assert_eq!(d.deal_from_top().as_deref(), Some("c"));
        assert_eq!(d.deal_from_top(), None);
    }
    #[test]
    fn seeded_shuffle_preserves_multiset() {
        let input: Vec<_> = (0..40).map(|i| format!("{}", i % 7)).collect();
        let mut a = CardDeck::new(input.clone());
        let mut b = CardDeck::new(input.clone());
        a.shuffle(&mut rand::rngs::StdRng::seed_from_u64(17));
        b.shuffle(&mut rand::rngs::StdRng::seed_from_u64(17));
        let mut out = Vec::new();
        while let Some(x) = a.deal_from_top() {
            assert_eq!(Some(x.clone()), b.deal_from_top());
            out.push(x);
        }
        assert_ne!(out, input);
        out.sort();
        let mut expected = input;
        expected.sort();
        assert_eq!(out, expected);
    }
}

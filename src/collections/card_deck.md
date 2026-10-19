# collections/card_deck

Reference implementation. Read after attempting the exercise.

```rust
pub struct CardDeck {
    cards: std::collections::VecDeque<String>,
}
impl CardDeck {
    pub fn new(cards: Vec<String>) -> Self {
        Self {
            cards: cards.into(),
        }
    }
    pub fn shuffle(&mut self, rng: &mut impl rand::Rng) {
        use rand::seq::SliceRandom;
        self.cards.make_contiguous().shuffle(rng);
    }
    pub fn deal_from_top(&mut self) -> Option<String> {
        self.cards.pop_front()
    }
    pub fn deal_from_bottom(&mut self) -> Option<String> {
        self.cards.pop_back()
    }
    pub fn insert_at(&mut self, index: usize, card: String) {
        self.cards.insert(index, card);
    }
    pub fn peek_top(&self) -> Option<&String> {
        self.cards.front()
    }
    pub fn peek_bottom(&self) -> Option<&String> {
        self.cards.back()
    }
    pub fn remaining_cards(&self) -> usize {
        self.cards.len()
    }
}
```

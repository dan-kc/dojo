// Card Deck Practice
//
// Learning Objectives:
// - Use VecDeque to implement a deck of cards with efficient shuffling
// - Support dealing from both ends and inserting at arbitrary positions
// - Practice with double-ended operations for card game mechanics
// - Implement shuffling algorithms using deque operations
//
// Run with: cargo test card_deck

/// Use VecDeque to implement a deck of cards with efficient shuffling.
/// Support dealing from both ends and inserting at arbitrary positions.
pub struct CardDeck {
    cards: std::collections::VecDeque<String>,
}

use rand::seq::SliceRandom;
use std::collections;

impl CardDeck {
    pub fn new() -> Self {
        todo!()
    }

    pub fn shuffle(&mut self) {
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

    #[test]
    fn test_card_deck() {
        let mut deck = CardDeck::new();
        assert_eq!(deck.remaining_cards(), 52);

        let top_card = deck.peek_top().unwrap().clone();
        let bottom_card = deck.peek_bottom().unwrap().clone();

        // Deal from top
        let dealt_top = deck.deal_from_top().unwrap();
        assert_eq!(dealt_top, top_card);
        assert_eq!(deck.remaining_cards(), 51);

        // Deal from bottom
        let dealt_bottom = deck.deal_from_bottom().unwrap();
        assert_eq!(dealt_bottom, bottom_card);
        assert_eq!(deck.remaining_cards(), 50);

        // Insert a card
        deck.insert_at(0, "Joker".to_string());
        assert_eq!(deck.remaining_cards(), 51);
        assert_eq!(deck.peek_top(), Some(&"Joker".to_string()));

        // Test shuffle (just verify it doesn't crash and deck size remains same)
        deck.shuffle();
        assert_eq!(deck.remaining_cards(), 51);
    }
}

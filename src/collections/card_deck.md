# Card Deck Solution

## Implementation

```rust
pub struct CardDeck {
    cards: std::collections::VecDeque<String>,
}

use rand::seq::SliceRandom;
use std::collections;
impl CardDeck {
    pub fn new() -> Self {
        let mut cards = std::collections::VecDeque::with_capacity(52);
        let suits = ["Hearts", "Diamonds", "Clubs", "Spades"];
        let ranks = ["A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K"];

        for suit in &suits {
            for rank in &ranks {
                cards.push_back(format!("{} of {}", rank, suit));
            }
        }

        Self { cards }
    }

    pub fn shuffle(&mut self) {
        use std::collections::VecDeque;

        // Convert to Vec for efficient random access during shuffle
        let mut vec: Vec<String> = self.cards.drain(..).collect();

        // Fisher-Yates shuffle
        let len = vec.len();
        for i in (1..len).rev() {
            let j = (rand::random::<usize>() % (i + 1)) as usize;
            vec.swap(i, j);
        }

        // Convert back to VecDeque
        self.cards = VecDeque::from(vec);
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

## Explanation

This solution implements a card deck using VecDeque for efficient operations:

1. **Deck initialization**: Creates standard 52-card deck with suits and ranks
2. **Double-ended operations**: Deal from top (front) or bottom (back)
3. **Shuffling**: Converts to Vec for random access, applies Fisher-Yates shuffle
4. **Insertion**: Supports inserting cards at arbitrary positions
5. **Peeking**: Non-destructive viewing of top/bottom cards

## Key Learning Points

- **VecDeque benefits**: O(1) operations at both ends
- **Fisher-Yates shuffle**: Unbiased shuffling algorithm
- **Type conversion**: Converting between Vec and VecDeque as needed
- **Card representation**: String formatting for card names

## Rust Concepts Demonstrated

- VecDeque for double-ended queue operations
- pop_front/pop_back for dealing cards
- insert method for arbitrary position insertion
- drain() for efficient collection conversion
- Random number generation for shuffling


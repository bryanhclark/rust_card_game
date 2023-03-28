use crate::card::{Card, CardSuite, CardValue};

use rand::prelude::SliceRandom;
use rand::thread_rng;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut deck = Deck { cards: [].to_vec() };
        deck.create_deck();
        deck
    }

    pub fn create_deck(&mut self) {
        for suite in CardSuite::VALUES.iter().copied() {
            for value in CardValue::VALUES.iter().copied() {
                let card = Card::new(suite, value);
                self.cards.push(card);
            }
        }
        self.shuffle_deck()
    }

    pub fn shuffle_deck(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng)
    }
}

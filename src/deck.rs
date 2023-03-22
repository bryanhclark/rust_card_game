use crate::card::{Card, CardSuite, CardValue};

use rand::thread_rng;
use rand::prelude::SliceRandom;

pub struct Deck {
    pub cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        Deck { cards: [].to_vec() }
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

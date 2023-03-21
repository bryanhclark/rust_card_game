mod card;
mod deck;

use crate::card::Card;
use crate::deck::Deck;

fn main() {
    let mut deck = Deck::new();
    deck.create_deck();
    println!("{:?}", deck.cards.len())

}

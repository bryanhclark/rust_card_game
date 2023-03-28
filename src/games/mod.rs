use crate::{player::Player, card::Card, deck::Deck};

pub mod war;

pub struct CardGame {
    pub players: Vec<Player>,
    num_starting_cards: i32,
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
}

impl CardGame {
    pub fn new() -> CardGame {
        let deck = Deck::new();
        // TODO: deal cards to players
        println!("{:?}", deck.cards[0]);

        let player_1 = Player { hand: [].to_vec() };
        let player_2 = Player { hand: [].to_vec() };
        let players = [player_1, player_2].to_vec();
        CardGame {
            num_starting_cards: deck.cards.len() as i32 / players.len() as i32,
            players,
            draw_pile: [].to_vec(),
            discard_pile: [].to_vec()
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum CardSuite {
    Club,
    Diamond,
    Heart,
    Spade,
}

impl CardSuite {
    pub const VALUES: [Self; 4] = [Self::Club, Self::Diamond, Self::Heart, Self::Spade];
}

#[derive(Debug, Clone, Copy)]
pub enum CardValue {
    Numeric(u8),
    King,
    Queen,
    Jack,
    Ace,
}

impl CardValue {
    pub const VALUES: [Self; 13] = [
        Self::Numeric(2),
        Self::Numeric(3),
        Self::Numeric(4),
        Self::Numeric(5),
        Self::Numeric(6),
        Self::Numeric(7),
        Self::Numeric(8),
        Self::Numeric(9),
        Self::Numeric(10),
        Self::Jack,
        Self::Queen,
        Self::King,
        Self::Ace,
    ];
}

#[derive(Copy, Clone, Debug)]
pub struct Card {
    pub suite: CardSuite,
    pub value: CardValue,
}


impl Card {
    pub fn new(suite: CardSuite, value: CardValue) -> Card {
        Card {
            suite,
            value,
        }
    }
}

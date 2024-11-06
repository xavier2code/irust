use number::CardNumber;
use suit::CardSuit;

mod number;
mod suit;

pub struct Card {
    pub number: CardNumber,
    pub suit: CardSuit,
}

impl Card {
    pub fn new(number: CardNumber, suit: CardSuit) -> Card {
        Card { number, suit }
    }

    pub fn to_suit_string(&self) -> String {
        format!("{}", self.suit.to_string())
    }

    pub fn to_number_string(&self) -> String {
        format!("{}", self.number.to_string())
    }
}

pub fn random_cards(size: usize) -> Vec<Card> {
    let mut cards = Vec::new();
    for _ in 0..size {
        let number = match rand::random::<u8>() % 13 {
            0 => CardNumber::Two,
            1 => CardNumber::Three,
            2 => CardNumber::Four,
            3 => CardNumber::Five,
            4 => CardNumber::Six,
            5 => CardNumber::Seven,
            6 => CardNumber::Eight,
            7 => CardNumber::Nine,
            8 => CardNumber::Ten,
            9 => CardNumber::J,
            10 => CardNumber::Q,
            11 => CardNumber::K,
            _ => CardNumber::A,
        };
        let suit = match rand::random::<u8>() % 4 {
            0 => CardSuit::Spades,
            1 => CardSuit::Hearts,
            2 => CardSuit::Diamonds,
            _ => CardSuit::Clubs,
        };
        cards.push(Card::new(number, suit));
    }
    cards
}

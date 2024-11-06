// card suit enum
pub enum CardSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl CardSuit {
    pub fn to_string(&self) -> String {
        match self {
            CardSuit::Spades => "♠".to_string(),
            CardSuit::Hearts => "♥".to_string(),
            CardSuit::Diamonds => "♦".to_string(),
            CardSuit::Clubs => "♣".to_string(),
        }
    }
}

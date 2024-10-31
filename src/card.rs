// crate a card by println!
pub fn empty_card(card: Card) {
    // green border, red text, white background
    let text_len = card.number.len();
    let fix_len = 4;
    let border_len = if text_len + fix_len > 5 {
        5
    } else {
        text_len + fix_len
    };
    println!("\x1b[32m╭{}╮", "─".repeat(border_len));
    println!("│{}{}│", card.to_text_string(), " ".repeat(border_len - text_len));
    println!("│\x1b[31m{}\x1b[1m{}\x1b[0m{}\x1b[32m│", " ".repeat((border_len - 1)/2)
    , card.to_suit_string(), " ".repeat((border_len - 1)/2));
    println!("│{}│", " ".repeat(border_len));
    println!("╰{}╯", "─".repeat(border_len));
}

// card number enum 1 to 10, J,Q,K,A
pub enum CardNumber {
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Q,
    K,
    A,
}

impl CardNumber {
    fn len(&self) -> usize {
        match self {
            CardNumber::One => "1".len(),
            CardNumber::Two => "2".len(),
            CardNumber::Three => "3".len(),
            CardNumber::Four => "4".len(),
            CardNumber::Five => "5".len(),
            CardNumber::Six => "6".len(),
            CardNumber::Seven => "7".len(),
            CardNumber::Eight => "8".len(),
            CardNumber::Nine => "9".len(),
            CardNumber::Ten => "10".len(),
            CardNumber::J => "J".len(),
            CardNumber::Q => "Q".len(),
            CardNumber::K => "K".len(),
            CardNumber::A => "A".len(),
        }
    }
    fn to_string(&self) -> String {
        match self {
            CardNumber::One => "1".to_string(),
            CardNumber::Two => "2".to_string(),
            CardNumber::Three => "3".to_string(),
            CardNumber::Four => "4".to_string(),
            CardNumber::Five => "5".to_string(),
            CardNumber::Six => "6".to_string(),
            CardNumber::Seven => "7".to_string(),
            CardNumber::Eight => "8".to_string(),
            CardNumber::Nine => "9".to_string(),
            CardNumber::Ten => "10".to_string(),
            CardNumber::J => "J".to_string(),
            CardNumber::Q => "Q".to_string(),
            CardNumber::K => "K".to_string(),
            CardNumber::A => "A".to_string(),
        }
    }
}

// card suit enum
pub enum CardSuit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

impl CardSuit {
    fn to_string(&self) -> String {
        match self {
            CardSuit::Spades => "♠".to_string(),
            CardSuit::Hearts => "♥".to_string(),
            CardSuit::Diamonds => "♦".to_string(),
            CardSuit::Clubs => "♣".to_string(),
        }
    }
}

// card struct
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

    pub fn to_text_string(&self) -> String {
        format!("{}", self.number.to_string())
    }
}
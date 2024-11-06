// card number enum 1 to 10, J,Q,K,A
pub enum CardNumber {
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
    pub fn len(&self) -> usize {
        match self {
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
    pub fn to_string(&self) -> String {
        match self {
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

use std::io;
// crate a card by println!
pub fn new_card(card: Card) {
    // green border, red text, white background
    let text_len = card.number.len();
    let fix_len = 4;
    let border_len = if text_len + fix_len > 5 {
        5
    } else {
        text_len + fix_len
    };
    println!("\x1b[32m╭{}╮", "─".repeat(border_len));
    println!("│\x1b[31m{}\x1b[0m{}│", card.to_text_string(), " ".repeat(border_len - text_len));
    println!("│\x1b[31m{}\x1b[1m{}\x1b[0m{}\x1b[32m│", " ".repeat((border_len - 1)/2), card.to_suit_string(), " ".repeat((border_len - 1)/2));
    println!("│{}│", " ".repeat(border_len));
    println!("╰{}╯", "─".repeat(border_len));
}

// generator random card with card size
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

pub fn heap_up(cards: &Vec<Card>) {
    // generator cards display in oneline
    let cards_len = cards.len();

    // ╭───╭───╭───╭───╭─────╮
    for _ in 0..cards_len {
        print!("\x1b[32m╭───");
    };
    print!("\x1b[32m──╮");
    println!();

    // │1  │2  │3  │10  │k    │
    for c in cards {
        print!("\x1b[32m│\x1b[31m{}{}", c.to_text_string(), " ".repeat(3 - c.number.len()));
    };
    println!("{}\x1b[32m│"," ".repeat(2));

    // │ ♠ │   │   │   │     │
    for c in cards {
        print!("\x1b[32m│\x1b[31m{}{}", " ".repeat(2), c.to_suit_string());
    }
    println!("{}\x1b[32m│"," ".repeat(2));

    // │   │   │   │   │     │
    for _ in 0..cards_len {
        print!("\x1b[32m│{}", " ".repeat(3));
    };
    println!("{}\x1b[32m│"," ".repeat(2));

    // ╰── ╰── ╰── ╰── ╰─────╯
    for _ in 0..cards_len {
        print!("\x1b[32m╰───");
    };
    print!("\x1b[32m──╯");
    println!();

}

pub fn init_game() {
    println!("=> Hi, Please check your cards.");
    let cards = random_cards(2);
    heap_up(&cards);
    let mut buf = String::new();
    'game_loop: loop {
        println!("=> Do you want a card? (yes/no)");
        io::stdin().read_line(&mut buf).expect("Read line error.");
        // if buf equals yes then add a card
        match buf.as_str() {
            "yes" => push_card(&cards),
            "no" => { show_cards(&cards); break 'game_loop;},
            _ => println!("Worng input, please input again."),
        }
    }
}

pub fn show_cards(cards: &Vec<Card>) {
    heap_up(cards);
}

pub fn push_card(cards: &Vec<Card>) {
    // this card is out of init cards
}

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
    fn len(&self) -> usize {
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
    fn to_string(&self) -> String {
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
pub mod card;

fn main() {
    vector();
}

fn vector() {
    let card_a = card::Card::new(card::CardNumber::A, card::CardSuit::Hearts);
    let card_2 = card::Card::new(card::CardNumber::Two, card::CardSuit::Spades);
    let card_10 = card::Card::new(card::CardNumber::K, card::CardSuit::Clubs);
    let card_k = card::Card::new(card::CardNumber::Ten, card::CardSuit::Diamonds);
    let cards = vec![card_2, card_10, card_a, card_k];
    card::heap_up(&cards);
}
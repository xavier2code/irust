pub mod card;


fn main() {
    let number = card::CardNumber::One;
    let suit = card::CardSuit::Spades;
    let card = card::Card::new(number, suit);
    card::new_card(card);

}
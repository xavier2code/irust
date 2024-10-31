pub mod card;


fn main() {
    let number = card::CardNumber::One;
    let suit = card::CardSuit::Spades;
    let card = card::Card::new(number, suit);
    card::empty_card(card);

    //
    let number = card::CardNumber::A;
    let suit = card::CardSuit::Clubs;
    let card = card::Card::new(number, suit);
    card::empty_card(card);

    //
    let number = card::CardNumber::Ten;
    let suit = card::CardSuit::Diamonds;
    let card = card::Card::new(number, suit);
    card::empty_card(card);

    //
    let number = card::CardNumber::Q;
    let suit = card::CardSuit::Hearts;
    let card = card::Card::new(number, suit);
    card::empty_card(card);
}
use crate::card::random_cards;
use crate::display::pack;

mod card;
mod display;

fn main() {
    // start the game
    init_game();
}

fn init_game() {
    // generate numbers of cards.
    let cards = random_cards(12);
    // print cards
    pack(&cards);
}

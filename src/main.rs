use std::io;

use crate::card::random_cards;
use crate::display::{pack, style};

mod card;
mod display;

fn main() -> io::Result<()> {
    // start the game
    init_game();
    // ? style
    style()
}

fn init_game() {
    // generate numbers of cards.
    let cards = random_cards(12);
    // print cards
    pack(&cards);
}

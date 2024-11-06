use crate::card::random_cards;
use crate::display::{heap_up, push_card, show_cards};
use std::io;

mod card;
mod display;

fn main() {
    init_game();
}

fn init_game() {
    println!("=> Hi, Please check your cards.");
    let cards = random_cards(12);
    heap_up(&cards);
    let mut buf = String::new();
    'game_loop: loop {
        println!("=> Do you want a card? (yes/no)");
        io::stdin().read_line(&mut buf).expect("Read line error.");
        // if buf equals yes then add a card
        println!("=> Your input is: {}", buf.as_str());
        match buf.as_str() {
            "yes" => push_card(&cards),
            "y" => push_card(&cards),
            "no" => {
                show_cards(&cards);
                break 'game_loop;
            }
            "n" => {
                show_cards(&cards);
                break 'game_loop;
            }
            "q" => break 'game_loop,
            _ => println!("Worng input, please input again."),
        }
        buf.clear();
    }
}

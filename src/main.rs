use std::io;
use std::io::Read;

use card::init_game;
use crossterm::terminal::enable_raw_mode;
use crossterm::terminal::disable_raw_mode;

pub mod card;
pub mod rectangle;

fn main() {
    // enable_raw_mode().unwrap();
    // init_game();
    // for b in io::stdin().bytes() {
    //     let c = b.unwrap() as char;
    //     if c == 'q' {
    //         disable_raw_mode().unwrap();
    //         break;
    //     }
    //     println!("You pass key : {}", c);
    // }
    rectangle::new_rectangle();
}
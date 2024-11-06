use std::io;

use crate::card::Card;
use crossterm::{
    self,
    event::{read, EnableBracketedPaste, EnableFocusChange, EnableMouseCapture, Event},
    execute,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor, Stylize},
};

mod selector;
// * crate a card by println!
#[allow(dead_code)]
pub fn new_card(card: Card) {
    // green border, red text
    let text_len = card.number.len();
    let fix_len = 4;
    let border_len = if text_len + fix_len > 5 {
        5
    } else {
        text_len + fix_len
    };
    println!("\x1b[32m╭{}╮", "─".repeat(border_len));
    println!(
        "│\x1b[31m{}\x1b[0m{}│",
        card.to_number_string(),
        " ".repeat(border_len - text_len)
    );
    println!(
        "│\x1b[31m{}\x1b[1m{}\x1b[0m{}\x1b[32m│",
        " ".repeat((border_len - 1) / 2),
        card.to_suit_string(),
        " ".repeat((border_len - 1) / 2)
    );
    println!("│{}│", " ".repeat(border_len));
    println!("╰{}╯", "─".repeat(border_len));
}

pub fn pack(cards: &Vec<Card>) {
    // generator cards display in one line
    let cards_len = cards.len();

    // ? sort the cards by number 2..10,j,q,k,A and suit

    // ╭───╭───╭───╭───╭─────╮
    for _ in 0..cards_len {
        print!("\x1b[32m╭───");
    }
    print!("\x1b[32m──╮");
    println!();

    // │1  │2  │3  │10  │k    │
    for c in cards {
        print!(
            "\x1b[32m│\x1b[31m{}{}",
            c.to_number_string(),
            " ".repeat(3 - c.number.len())
        );
    }
    println!("{}\x1b[32m│", " ".repeat(2));

    // │ ♠ │   │   │   │     │
    for c in cards {
        print!("\x1b[32m│\x1b[31m{}{}", " ".repeat(2), c.to_suit_string());
    }
    println!("{}\x1b[32m│", " ".repeat(2));

    // │   │   │   │   │     │
    for _ in 0..cards_len {
        print!("\x1b[32m│{}", " ".repeat(3));
    }
    println!("{}\x1b[32m│", " ".repeat(2));

    // ╰── ╰── ╰── ╰── ╰─────╯
    for _ in 0..cards_len {
        print!("\x1b[32m╰───");
    }
    print!("\x1b[32m──╯");
    println!();
}

#[allow(dead_code, unused_variables)]
pub fn push_card(cards: &Vec<Card>) {}

#[allow(dead_code, unused_variables)]
pub fn show_cards(cards: &Vec<Card>) {}

// * use keyword to move
pub fn move_arrow() -> std::io::Result<()> {
    let var_name = execute!(
        std::io::stdout(),
        EnableBracketedPaste,
        EnableFocusChange,
        EnableMouseCapture
    );
    var_name?;
    loop {
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            Event::Paste(data) => println!("{:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
}

pub fn style() -> io::Result<()> {
    println!("{}", "Red foreground color & blue background.".red().on_white());
    execute!(
        io::stdout(),
        SetForegroundColor(Color::Red),
        SetBackgroundColor(Color::White),
        Print("Red text and white background.\n".to_string()),
        ResetColor
    )
}

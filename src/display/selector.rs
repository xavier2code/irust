use crate::card::Card;

#[allow(dead_code)]
pub struct Selector {
    position: (i32, i32),
    current: Card,
}

// ? to move this selector with keyword
#[allow(dead_code)]
impl Selector {
    pub fn to_move_left() {
        println!("move left: {}", "");
    }

    pub fn to_move_right() {
        println!("move right: {}", "");
    }

    pub fn to_move_up() {
        println!("move up: {}", "");
    }

    pub fn to_move_down() {
        println!("move down: {}", "");
    }

    pub fn init_selector(card: Card) -> Selector {
        let position = (0, 0);
        let current = card;
        Selector { position, current }
    }
}

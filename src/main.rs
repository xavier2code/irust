pub mod card;

fn main() {
    vector();
}

fn vector() {
    card::heap_up(&card::random_cards(12));
}
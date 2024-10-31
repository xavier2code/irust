use std::fmt::Display;

use card::init_game;

pub mod card;

fn main() {
    // init_game();
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
    println!("{:?}", map);
    map.entry(String::from("CCC")).or_insert(String::from("Red"));
    println!("{:?}", map);
    map.entry(String::from("Favorite color")).or_insert(String::from("Red"));
    println!("{:?}", map);

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct CardType<T> {
    x: T,
    y: T,
}

impl <T: Display + PartialOrd> CardType<T> {
    
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
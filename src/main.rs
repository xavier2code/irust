fn main() {
    println!("Hello, world!");
}

/* <================== chapter5 Using Structs to Structure Related Data ===============> */

/* 5.1 */
// Defining a struct called `User`
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

// To use a struct after we’ve defined it, 
// we create an instance of that struct by 
// specifying concrete values for each of the fields.
fn create_user() {
    let mut new_person= User {
        active: true,
        username: String::from("username"),
        email: String::from("xavier@gmail.com"),
        sign_in_count: 1,
    };

    new_person.email = String::from("others@gmail.com");
}

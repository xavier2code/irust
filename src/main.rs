fn main() {
    /* 5.1 */
    create_user();
    /* 5.2 */
    show_area();
}

/* <================== chapter1 Getting Started ===============> */

/* <================== chapter2 Programming Guessing Game ===============> */

/* <================== chapter3 Common Programming Concepts ===============> */

/* <================== chapter4 Understanding Ownership ===============> */

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

    println!("New persion {} : {}/{}/{}", new_person.username, new_person.email
    , new_person.active, new_person.sign_in_count);

    let new_person2 = build_user(new_person.email, new_person.username);
    println!("New persion {} : {}/{}/{}", new_person2.username, new_person2.email

    , new_person2.active, new_person2.sign_in_count);
    let new_person3 = build_user_shorthand(new_person2.email, new_person2.username);

    println!("New persion {} : {}/{}/{}", new_person3.username, new_person3.email
    , new_person3.active, new_person3.sign_in_count);

    update_user(new_person3);

}

// we can construct a new instance of the struct as the 
// last expression in the function body to implicitly 
// return that new instance.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1,
    }
}

// Using the Field Init Shorthand
fn build_user_shorthand(email: String, username: String) -> User {
    User {
        active: false,
        username,
        email,
        sign_in_count: 2,
    }
}

// Creating Instances from Other Instances with Struct Update Syntax
fn update_user(new_person2: User) {
    let new_person3 = User {
        email: String::from("user2@gmail.com"),
        ..new_person2
    };

    println!("New persion {} : {}/{}/{}", new_person3.username, new_person3.email
    , new_person3.active, new_person3.sign_in_count);

    tuple_structs();
}

// Using Tuple Structs Without Named Fields to Create Different Types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn tuple_structs() {
    let black = Color(0, 0, 0);
    println!("Color black : {},{},{}", black.0,black.1,black.2);
    let origin = Point(0,0, 0);
    println!("Point origin: {},{},{}", origin.0,origin.1,origin.2);
}

/* 5.2 */

// The area function is supposed to calculate the area of one rectangle
fn area_v1(width: u32, height: u32) -> u32 {
    width * height
}

// Tuples let us add a bit of structure, and we’re now passing just one argument.
fn area_v2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// We use structs to add meaning by labeling the data. 
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn area_v3(rectangle: &Rectangle) -> u32 {
    rectangle.height * rectangle.width
}

fn show_area() {
    println!("area_v1 : {}", area_v1(1, 2));
    println!("area_v2 : {}", area_v2((1, 2)));
    let scale = 2;
    let rectangle = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };
    dbg!(&rectangle);
    println!("Rectangle is : {rectangle:?}");
    println!("area_v3 : {}", area_v3(&rectangle));

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Rect1 : {}", rect1.area());
    println!("Rect1 : {}", rect1.others(1));
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    let square = Rectangle::square(1);
    println!("Square : {}", square.width);

}

/* 5.3 */

// Defining Methods
// To define the function within the context of Rectangle, 
// we start an impl (implementation) block for Rectangle.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn others(&self, other: u32) -> u32 {
        (self.width + other) * (self.height + other)
    }

    fn width(&self) -> bool {
        self.width > 0
    }
}

// Multiple impl Blocks
// Associated functions that aren’t methods are often used for 
// constructors that will return a new instance of the struct.
impl Rectangle {
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

/*
 * Summary
 * Structs let you create custom types that are meaningful 
 * for your domain. By using structs, you can keep associated 
 * pieces of data connected to each other and name each piece 
 * to make your code clear. In impl blocks, you can define 
 * functions that are associated with your type, and methods 
 * are a kind of associated function that let you specify the 
 * behavior that instances of your structs have.
 * But structs aren’t the only way you can create custom types: 
 * let’s turn to Rust’s enum feature to add another tool to your 
 * toolbox.
 */

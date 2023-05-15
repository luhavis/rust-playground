
fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("user1@gmail.com"),
        sign_in_count: 1,
    };

    let mut user2 = User {
        active: true,
        username: String::from("user2"),
        email: String::from("user2@gmail.com"),
        sign_in_count: 1,
    };

    user2.email = String::from("user2_update@gmail.com");

    println!("{}, {}",user1.username, user1.email);
    
    println!("{}, {}",user2.username, user2.email);

    let user3 = build_user(String::from("user3@gmail.com"), String::from("user3"));

    println!("{}, {}", user3.username, user3.email);


    let blue = Color(0, 255, 255);
    let origin = Color(0, 0, 0);

    println!("{}, {}, {}", blue.0, blue.1, blue.2);
    println!("{}, {}, {}", origin.0, origin.1, origin.2);

    let subject = AlwaysEqual;

    let width = 1920;
    let height = 1080;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height),
    );

    let rect1 = (1920, 1080);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1),
    );

    let rect2 = Rectangle {
        width: 1920,
        height: 1080,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2),
    );

    let rect3 = Rectangle {
        width: 300,
        height: 400,
    };

    println!("rect3 : {:?}", rect3);


    let scale = 3;
    let rect4 = Rectangle {
        width: dbg!(300 * 3),
        height: 400,
    };

    dbg!(&rect4);

}




fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        // username: username,
        username,
        // email: email,
        email,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color (i32, i32, i32);
struct Point (i32, i32, i32);
struct AlwaysEqual;


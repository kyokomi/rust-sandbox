struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

struct Color(i32, i32, i32);

struct Point(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );

    println_user(&user1);

    user1.email = String::from("anotheremail@example.com");

    println_user(&user1);

    let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };

    println_user(&user2);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("{}.{}.{}", black.0, black.1, black.2);
    println!("{}.{}.{}", origin.0, origin.1, origin.2);

    // let width1 = 30;
    // let height1 = 50;
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    let rect4 = Rectangle::square(3);

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area(),
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    println!("{:#?}", rect1);
    println!("{:#?}", rect4);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn println_user(user: &User) {
    println!(
        "{} {} {} {}",
        user.username, user.email, user.sign_in_count, user.active
    );
}

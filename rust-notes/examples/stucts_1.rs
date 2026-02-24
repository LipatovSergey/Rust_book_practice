fn main() {}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn example_1() {
    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 1,
    };
}

fn example_2() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@gmail.com")
}

fn example_3() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{}", user1.active);
    println!("{}", user2.email)
}

fn example_4() {
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    println!("user1.email: {}", user1.email);
    user1.email = String::from("anotheremail@example.com");
    println!("user1.email: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    // user1 is now unusable because user1.username has been moved
    // println!("user2.username: {}", user1.username);
    println!("user2.username: {}", user2.username);

    // Tuple structs
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black.0: {}", black.0);
    println!("origing.0: {}", origin.0);

    // Unit-Like structs
    // These ebcome useful later as containers for traits
    let subject = AlwaysEqual;
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    // Note email and username are just raw names
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

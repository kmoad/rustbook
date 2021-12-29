fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    route(&four);
    route(&six);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    let home = IpAddrData::V4(String::from("127.0.0.1"));
    let loopback = IpAddrData::V6(String::from("::1"));

    let home = IpAddrInt::V4(127, 0, 0, 1);
    let loopback = IpAddrInt::V6(String::from("::1"));

    let msg = Message::Quit;
    msg.call();

    let msg = Message::Move { x: 10, y: 32 };
    msg.call();

    let msg = Message::Write(String::from("hello"));
    msg.call();

    let msg = Message::ChangeColor(50, 60, 70);
    msg.call();

    // Some and None
    let some_number = Some(5);
    let some_string = Some("a string");
    // Have to specify which type of Option::None
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    // This won't work. value is held within the Some
    // let sum = x + y;
    // Compiler forces you to handle Options, but you can confidently
    // assume all other values are not null/nil/None
    // There are myriad ways to convert Option<T> to T

    // Using match
    let p = Coin::Penny;
    value_in_cents(p);
    let q = Coin::Quarter(UsState::Alabama);
    value_in_cents(q);

    //Option<T>
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    match five {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
    match six {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
    match none {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }

    // Match catch-all patterns
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other),
    }
    fn add_fancy_hat() {
        println!("hat on")
    }
    fn remove_fancy_hat() {
        println!("hat off")
    }
    fn move_player(num_spaces: u8) {
        println!("move {}", num_spaces)
    }
    // or use _ if you don't neet to use the variable
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(),
    }
    fn reroll() {
        println!("reroll")
    }
    // nothing happens if dice misses
    // () is the unit value or empty tuple
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => (),
    }

    // if-let
    let config_max = Some(3u8);
    // annoying verbose way
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    // if-let way
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
    // Pattern first, then expression, then action. If no match, no action taken

    // If let accepts and else to take default action
    let mut count = 0;
    let coin = Coin::Quarter(UsState::Alaska);
    let coin = Coin::Dime;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    println!("count: {}", count)
}

enum IpAddrKind {
    V4,
    V6,
}

fn route(ip_kind: &IpAddrKind) {}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// Can attach data directly to enum variants
enum IpAddrData {
    V4(String),
    V6(String),
}

// Variants can have mixed types
enum IpAddrInt {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
// Enums can have methods too
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("To location ({}, {})", x, y),
            Message::Write(s) => println!("Message is: {}", s),
            Message::ChangeColor(r, g, b) => println!("rgb({},{},{})", r, g, b),
        }
    }
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

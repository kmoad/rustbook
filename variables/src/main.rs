fn main() {
    // Assignment
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // Constants are defined at compile time
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("3hr in s: {}", THREE_HOURS_IN_SECONDS);

    // Shadowing and scoping
    let y = 5;
    let y = y + 1;

    {
        let y = y * 2;
        println!("The value of y in the inner scope is: {}", y);
    }
    println!("The value of y is: {}", y);

    // Shadowing allow type changes
    let spaces = "   ";
    let spaces = spaces.len();
    println!("spaces: {}", spaces);

    // Numbers
    let thousand = 1_000;
    println!("{}", thousand);

    let sum = 5 + 10;
    println!("{}", sum);

    let difference = 95.5 - 4.3;
    println!("{}", difference);

    let product = 4 * 30;
    println!("{}", product);

    let quotient = 56.7 / 33.2;
    println!("{}", quotient);
    let floored = 2 / 3;
    println!("{}", floored);

    let remainder = 43 % 5;
    println!("{}", remainder);

    // Booleans
    let t = true;
    let f: bool = false;

    // Character
    let c = 'z';
    let heart_eyed_cat = '😻';
    println!("{}", heart_eyed_cat);

    // Tuple
    let tup_anno: (i32, f64, u8) = (500, 6.4, 1);
    let tup_raw = (501, 6.5, 2);
    let (t1, t2, t3) = tup_raw;
    println!("t2: {}", t2);
    let t1 = tup_raw.0;
    println!("t1: {}", t1);

    // Array
    // fixed size, allocates to the stack
    // There is a variable len type called a vector in the stdlib
    let a = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let b: [u8; 5] = [1, 2, 3, 4, 5];
    let c = [3; 5]; //[3,3,3,3,3]
    let first = a[0];
    println!("first: {}", first);
    // Out-of-bounds array access results in a runtime panic
}

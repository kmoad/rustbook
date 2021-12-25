fn main() {
    println!("Hello, world!");

    another_function(5);

    print_labeled_measuremnts(5, 'h');

    // A statement, it does not return a value
    let _ = 6;
    // A function definition is also a statement

    // The block in {} is an expression, it returns the value
    // of the last line
    let y = {
        let x = 3;
        // This is an expression. Note the lack of semicolon
        x + 1
    };

    println!("y: {}", y);

    let x = five();
    println!("x: {}", x);

    println!("5+1={}",plus_one(5));

    println!("5+2={}",plus_two(5));

}

fn another_function(x: i32) {
    println!("The value of x is: {}", x)
}

fn print_labeled_measuremnts(value: i32, unit_label: char) {
    println!("The measurement is {}{}", value, unit_label);
}

// Functions return the last expression
// You can explicitly use return as well
fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    // can't have a semicolon at the end
    x + 1
}

fn plus_two(x: i32) -> i32 {
    // Can have semicolon with return
    return x + 2;
}
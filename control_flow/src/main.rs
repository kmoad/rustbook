fn main() {
    let number = 3;

    // The condition expression MUST evaluate to a bool
    // No truthy inference
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let number = 6;

    if number % 4 == 0 {
        println!("divisible by 4");
    } else if number % 3 == 0 {
        println!("divisible by 3");
    } else if number % 2 == 0 {
        println!("divisible by 2");
    } else {
        println!("not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("number: {}", number);
    // Expressions must be of same type though
    // let number = if condition { 5 } else { "six" };

    // Loops
    loop {
        println!("only once");
        // loop is infinite by default
        break;
    }

    // This example of using break is confusing, but w/e
    let mut count = 0;
    // Loops may have names
    'counting_up: loop {
        println!("count = {}", count);
        let mut remaining = 10;
        loop {
            println!("remaining = {}", remaining);
            if remaining == 9 {
                // Unlabeled break will apply to inner loop
                break;
            }
            if count == 2 {
                // You may break a loop by name
                // This allows to break the outer loop
                break 'counting_up;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {}", count);

    // Returning values from loops
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("Loop result: {}", result);

    // While loop
    let mut number = 3;
    while number != 0 {
        println!("{}", number);
        number -= 1;
    }
    println!("LIFTOFF!");

    // For loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("current element: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!");

    println!("recur fib(5): {}", fib_recursive(5));
    println!("recur fib(1): {}", fib_recursive(1));
    println!("recur fib(0): {}", fib_recursive(0));
    println!("loop fib(5): {}", fib_loop(5));
    println!("loop fib(1): {}", fib_loop(1));
    println!("loop fib(0): {}", fib_loop(0));

    println!(
        "68 F in C: {}",
        convert_temp(68.0, TemperatureUnit::Farenheit)
    );

    println!(
        "100 C in F: {}",
        convert_temp(100.0, TemperatureUnit::Celcius)
    );
}

// Convert between Celcius and Farenheit
enum TemperatureUnit {
    Celcius,
    Farenheit,
}
fn convert_temp(temp: f64, unit: TemperatureUnit) -> f64 {
    match unit {
        TemperatureUnit::Celcius => temp * 9.0 / 5.0 + 32.0,
        TemperatureUnit::Farenheit => (temp - 32.0) * 5.0 / 9.0,
    }
}

// Fibonacci RECURSIVE
// TODO handle 0 or negative
fn fib_recursive(n: u32) -> u32 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return n + fib_recursive(n - 1);
    }
}

// Fibonacci LOOP
fn fib_loop(n: usize) -> usize {
    let mut res = 0;
    for i in 1..n + 1 {
        res += i;
    }
    res
}

fn main() {
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let x = 5;
    let y = x;
    // Two variables, at two memory locations

    let s1 = String::from("hello");
    let s2 = s1;
    // Haha no copy made
    // The data in "hello" is on the heap, s1 stores a
    // pointer to it. When s2 is made, makes a new var
    // that simply stores the pointer. Stack data is
    // copied, heap data is not.
    // This means that the following is no longer valid
    // println!("{}, world!", s1);
    // So, really s1 is MOVED to s2

    let s3 = s2.clone();
    // This is a deep copy of the heap data
    println!("s2 = {}, s3 = {}", s2, s3);

    println!("x = {}, y = {}", x, y);
    // Wait! this doesn't apply to the integers!
    // Integers are entirely in the stack, and are a known size. 
    // They are copied by default

    // Passing as a param will also move
    takes_ownership(s3);
    // s3 is no longer valid
    // println!("s3: {}", s3);
    makes_copy(y);
    println!("y: {}", y);

    // This is obviously annoying. Enter borrowing

    // References and Borrowing
    let mut s4 = String::from("hello");
    let len = calculate_length(&s4);
    println!("The length of {} is {}", s4, len);

    change(&mut s4);
    println!("s4 after change: {}", s4);

    let mut s5 = String::from("hello");
    // Mutable references
    let r1 = &mut s5;
    // Can't have more than one mutable ref at a time
    // let r2 = &mut s5;
    // println!("r1 {}, r2 {}", r1, r2);
    // Can't mix mutable and immutable refs
    // let r3 = &s5;
    // println!("r1 {}, r3 {}", r1, r3);

    // This works though. bc r1 and r1 ARE NOT USED later
    let r1 = &s5; // no problem
    let r2 = &s5; // no problem
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not be used after this point

    let r3 = &mut s5; // no problem
    println!("{}", r3);

    // Slices
    
}


fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.

fn calculate_length(s: &String) -> usize {
    s.len()
}

// References are immutable by default
// fn change(some_string: &String) {
//     some_string.push_str(", world");
// }

// But you can make them mutable
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
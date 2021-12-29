fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // :? used to pretty-print rect1 using Debug
    println!("rect1 is {:#?}", rect1);
    // Could also use the debug macro
    dbg!(&rect1);
    println!("rect1 area is: {}", area(&rect1));

    // another use of dbg! to inspect an expression
    let scale = 1;
    let rect2 = Rectangle {
        width: dbg!(30 * scale),
        height: 50,
    };

    let rect3 = Rectangle {
        width: 30,
        height: 50,
    };
    println!("rect3.area(): {}", rect3.area());
    if rect3.width() {
        println!("rect3 has width");
    }

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
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
    let square1 = Rectangle::square(100);
    println!("square1: {:#?}", square1);
}

// the derive(Debug) is used for println! and dbg! macros
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
// Define a method with impl block
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    // Method names can overlap struct fields
    fn width(&self) -> bool {
        self.width > 0
    }
    // Methods can take parameters
    fn can_hold(&self, other: &Rectangle) -> bool {
        other.width < self.width && other.height < self.height
    }
    // An Associated Function doens't have &self as first param
    // They are accessed like String::From
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Non-method implementation
fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

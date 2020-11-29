#[derive(Debug)]
struct Rectangle {
    length: i32,
    width: i32
}

fn main() {
    let rect1 = Rectangle {
        length: 5,
        width: 4
    };

    println!("{}", area(&rect1));

    // This does not work because rect1 being a struct does not implement the std::fmt::Display trait
    // Struct has comma, curly bracket etc, so rust won't try to guss for us and that is why this does not work
    // So instead of {} we need to use {:?} (debug trait) and annotation on rectangle #[derive(Debug)]
    println!("{}", rect1);
}

fn area(rectangle: &Rectangle) -> i32 {
    return rectangle.length * rectangle.width;
}
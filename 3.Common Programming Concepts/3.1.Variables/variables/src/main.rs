fn main() {
    // By defaults int's are 32 bit in Rust
    let x = 5;

    // By default variables are immutable in rust
    // This gives you to write your code in a way that takes advantage of the safety and easy concurrency that Rust offers

    println!("The value of x is {}", x);

    // This is invalid as x is immutable
    x = 6;
    println!("The value of x is: {}", x);

    // To change the value of x we need to add `mut` keyword, `let mut x`

    // CONSTANTS
    // Constants are always immutable and are declared using `const` keyword and the type must always
    // be annotated
    const MAX_POINTS: u32 = 100000;

}

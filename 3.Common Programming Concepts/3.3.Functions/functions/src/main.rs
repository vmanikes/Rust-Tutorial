fn main() {
    println!("Hello, world!");

    another_function();
    parameter_function(1, 3.54);
}

fn another_function() {
    println!("another function here")
}

// Parameterized functions
fn parameter_function(x: i32, y:f64) {
    println!("Values of x and y are {} and {}", x, y)
}

// Function with return
fn return_function() -> i32 {
    // This is a statement
    let x = 5;

    // This is an expression, if we want a function to return it should be of type expression
    // Also expressions do not end with ;
    x
}

fn return_function2() -> i32 {
    // This is a statement
    let x = 5;

    // This will return error because of ;
    return x;
}
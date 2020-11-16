fn main() {
    let x = 5;

    // let x =, shadows by taking the original value and adding 1 so the value of x

    // Even if x is immutable, we can change its value/type by using the let keyword because we are effectively
    // creating a new variable
    // This helps in changing the value of immutable x, apply transformations and still be immutable
    let x = x + 1;

    println!("The value of x is: {}", x);

    // For example, say our program asks a user to show how many spaces they want between some
    // text by inputting space characters, but we really want to store that input as a number:
    // This helps in come up with different names, such as spaces_str and spaces_num; instead,
    // we can reuse the simpler spaces
    let spaces = "   ";
    let spaces = spaces.len();

    // But if we name a variable with mut and try to shadow, we get compile time error
    let mut spaces = "     ";
    spaces = spaces.len();

}
// io is a stl that can be used to handle user inputs

// Rust automatically imports several packages by default which is called prelude, this helps in
// keeping the code less verbose https://doc.rust-lang.org/std/prelude/index.html
use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game");
    println!("Enter the number");

    // let is used to crate a variable let foo = bar

    // The rand::thread_rng function will give us the particular random number generator that we’re
    // going to use: one that is local to the current thread of execution and seeded by the operating system.

    // gen_range takes 2 ints and returns a number between them, in this case 1, 100
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        // variables by default are immutable in rust, to make them mutable, we need to add mut

        // String is a type provided by stl that is growable and is utf-8 encoded
        // ::new line indicates that new is an associated function of the String type.
        // An associated function is implemented on a type, in this case String, rather than on a
        // particular instance of a String. Some languages call this a static method. The new() method
        // creates a new empty string without allocating memory
        let mut guess = String::new();

        // If we hadn't put the use std::io line at the beginning of the program, we could have written
        // this function call as std::io::stdin.

        // References are immutable by default, to make them mutable we need to add &mut

        // read_line puts what the user types into the string we’re passing it, but it also returns a
        // value—in this case, an io::Result.
        //
        //For Result, the variants are Ok or Err.
        // The Ok variant indicates the operation was successful, and inside Ok is the successfully generated value.
        // The Err variant means the operation failed, and Err contains information about how or why the
        // operation failed.

        // Result has an expect() method which takes in a string.
        // On error that method get triggered by a program crash with the message inside it
        // But on Ok, expect() will take the value in Ok and will return it

        // If we do not use expect the program will compile but will throw warning during `cargo build`
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }

}

The issue with the tuple code in Listing 4-5 is that we have to return the String to the calling function so we can still use the String after the call to calculate_length, because the String was moved into calculate_length.

Here is how you would define and use a calculate_length function that has a reference to an object as a parameter instead of taking ownership of the value:

```
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
    // it refers to, nothing happens.
```

This is called as `borrowing`.  As in real life, if a person owns something, you can borrow it from them. 
When you’re done, you have to give it back.

When you want to modify a value that you are borrowing, you want to make it mutable as references are immutable by 
default as well

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

## Mutable References
If we want to change the value that is borrowed, we need to work with mutable references

```rust
fn main() {
    let mut s = String::from("hello");
    
    change(&mut s); // Send a mutable reference
}

fn change(s1: &mut String) {  // Accept a mutable reference
    s1.push_str(", world");
}
```

You can only have once mutable reference at a time, the below code is not possible

```
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s;
```

This analogy helps preventing data races, where 2 or more pointers try to access / modify the data at the same time

```
let s = String::from("hello");

{
    let r1 = &mut s;
} // here r1 goes out of scope so we can use it in r2

let r2 = &mut s;
```

## Combination of mutable and immutable references
```
let mut s = String::from("hello");

let r1 = &s;
let r2 = &s;
let r3 = &mut s;

println!("{},{},{}", r1,r2,r3);
```
We cannot have the above code compile because in the view of r1 and r2 they are just using readonly references to s, but 
r3 can modify and if it does, r1 and r2 will not understand what is happening, so Rust will not allow this to compile

```
let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    let r3 = &mut s; // no problem
    println!("{}", r3);
```
But this is fine

## Dangling Pointers
```rust
fn dangle() -> &String {
    let s = String::from("hello");

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
// Danger!
```
Because s is created inside dangle, when the code of dangle is finished, s will be deallocated. But we tried to return 
a reference to it. That means this reference would be pointing to an invalid String. That’s no good! Rust won’t
let us do this.

The solution is to return string directly
```rust
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```
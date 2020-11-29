# Ownership

## Ownership Rules
- Each value in Rust has a variable called as `Owner`
- There can be only one owner at a time
- When the owner goes out of scope the value will be dropped

## Variable Scope
```
fn main() {
    {                      // s is not valid here, it’s not yet declared
        let s = "hello";   // s is valid from this point forward

        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
}
```

## The String Type
There are 2 ways to declare strings

`let s = "hello";`

The above statement is immutable and sometimes we might not know what the string is gonna be like in the code, for these cases there is second type `String`

`String` stores the value on the heap and is helpful when we are unable to know how much memory to store

```
let s = String::from("hello);

// This kind of string can is mutable

s.push_str(", world!");
```

The first way is faster because we know the amount of memory that we need to allocate and we will add to binary, but for the other one we might not know how much mem is required and adding all that info to binary can be cluttery

`String::from` helps us request us memory from runtime, but we need to give it back to runtime after using it. 

Doing this correctly has historically been a difficult programming problem. If we forget, we’ll waste memory. If we do it too early, we’ll have an invalid variable. If we do it twice, that’s a bug too. We need to pair exactly one allocate with exactly one free.

But in Rust this is handled for us when the variable goes out of scope. A function called `drop` is called when a variable goes out of scope

```
    {
        let s = String::from("hello"); // s is valid from this point forward

        // do stuff with s
    }                                  // this scope is now over, and s is no
```

## Ways variables and data interact
```
let x = 5;
let y = x;
```
In the above, y gets a copy of x and they will get two memory locations and pushed onto stack

```
let s1 = String::from("Hello");
let s2 = s1
```

Here a copy is not made for s2, `s1` has 3 fields, `ptr` holds a pointer to memory that holds the contents of the string and `length` and `capacity`, the right is the memory on heap that holds contents

`length` is the amount of memory in bytes the string is currently using
`capacity` is the amount of memory in bytes that the allocator has allocated

![alt text](https://doc.rust-lang.org/book/img/trpl04-02.svg)

When we assign s1 to s1 data is copied, meaning we copy the pointer, the length, and the capacity that are on the stack. We do not copy the data on the heap that the pointer refers to

If we copy and create a new pointer that will be vert expensive

As `s1` and `s2` point to the same pointer and when both go out of scope, both will try to call the `drop` function which might result in `double free` error. Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities

So what rust does is, when we assign s1 to s2, Rust considers s1 to be no longer valid and any operations of it will lead to compilation errors
```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2

    println!("{}, world!", s1);
}
```
```
error[E0382]: borrow of moved value: `s1`
```

### clone
Clone is a part where new hep allocation is done when we copy data to s2
```
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
```

## Function and ownership
```
fn main() {
    let s = String::from("hello");  // s comes into scope

    takes_ownership(s);             // s's value moves into the function...
                                    // ... and so is no longer valid here

    let x = 5;                      // x comes into scope

    makes_copy(x);                  // x would move into the function,
                                    // but i32 is Copy, so it’s okay to still
                                    // use x afterward

} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope. Nothing special happens.
```

If we try to use `s` after `take_ownership()` then we get compile error as s was moved

## Return Values and Scope
```
fn main() {
    let s1 = gives_ownership();         // gives_ownership moves its return
                                        // value into s1

    let s2 = String::from("hello");     // s2 comes into scope

    let s3 = takes_and_gives_back(s2);  // s2 is moved into
                                        // takes_and_gives_back, which also
                                        // moves its return value into s3
} // Here, s3 goes out of scope and is dropped. s2 goes out of scope but was
  // moved, so nothing happens. s1 goes out of scope and is dropped.

fn gives_ownership() -> String {             // gives_ownership will move its
                                             // return value into the function
                                             // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string                              // some_string is returned and
                                             // moves out to the calling
                                             // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String { // a_string comes into
                                                      // scope

    a_string  // a_string is returned and moves out to the calling function
}
```

If we want the function just to take value and not ownership, we need to return the value from the function back along with other data as well

```
fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    return (s, length);
}
```

This might be too much, so we have something called `references`
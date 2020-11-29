# Match keyword
- Allows you to compare a value against a series of patterns and then execute code based on which pattern matches
- Patterns can be made up of literal values, variable names, wildcards, and many other things
- Its similar to switch case
```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn check_value(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    println!("{}", check_value(Coin::Dime));
}
```

- On each arm we can execute custom code by opening a scope
```rust
    match coin {
        Coin::Penny => {
            println!("hello");
            return 1;
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
```

## Getting value from Option<T>
```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
```
- *Note*: If we forget the None case in match the rust compiler will throw an error saying all cases were not handled

## _ Operator
- Rust also has a pattern we can use when we don’t want to list all possible values. For example, a u8 can have valid values of 0 through 255.
- If we only care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2, 4, 6, 8, 9 all the way up to 255.

```rust
fn main() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

```
- In this way we need to handle all the exhaustive cases with match
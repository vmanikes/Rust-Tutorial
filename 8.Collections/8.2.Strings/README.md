# Strings

## Creating empty string
`let s = String::new();`

## Creating string with data
```rust
let data = "initial contents";
let s = data.to_string();

let s2 = String::from("HEllo");
```

## Updating String
```rust
let mut s = String::from("Hello");
s.push_str("hello");
```

## Indexing strings
- Rust does not support indexing on strings as it supports UTF-8, its very difficult to infer hwo data is gonna be 
represented in memory
  
## Iterating on strings
```rust

#![allow(unused)]
fn main() {
    for c in "नमस्ते".chars() {
        println!("{}", c);
    }
}

```
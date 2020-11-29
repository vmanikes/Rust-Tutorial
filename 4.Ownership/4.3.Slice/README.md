## String Slices

```rust
fn main() {
    let s = String:: from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
}
```

Essentially we are storing immutable references to `s` in `hello` and `world`

![alt_text](https://doc.rust-lang.org/book/img/trpl04-06.svg)

- `&s[..5]` -> 0 to 5
- `&s[0..]` -> 0 to end
- `&s[..]` -> Entire string
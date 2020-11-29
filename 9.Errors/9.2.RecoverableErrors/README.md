# Recoverable errors with Result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

We use Result to make our programs not panic

Here 
`T -> Type of value when we have success`
`E -> Type of value when we enounter error`

## Example
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}
```

## unwrap and expect
### unwrap
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```
If the Result value is the Ok variant, unwrap will return the value inside the Ok. If the Result is the Err variant, unwrap will call the panic!

### expect
```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}

```
We use expect in the same way as unwrap: to return the file handle or call the panic! macro. The error message used by expect in its call to panic! will be the parameter that we pass to expect, rather than the default panic! message that unwrap uses
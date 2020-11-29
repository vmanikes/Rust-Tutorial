# Data Types

- Rust is a statically typed language, meaning it should know the type of the variable at compile time
- There are two kinds of variables `Scalar` and `Compound`

## Scalar Types

### Types of Scalar:
- Integers
- Floating
- Characters
- Boolean

#### Integers

| Length        | Signed        | Unsigned  |
| ------------- |:-------------:| ---------:|
| 8 Bit         | i8            | u8        |
| 16 Bit        | i16           | u16       |
| 32 Bit        | i32           | u32       |
| 64 Bit        | i64           | u64       |
| 128 Bit       | i128          | u128      |
| arch          | isize         | usize     |

- Additionally, the isize and usize types depend on the kind of computer your program is running on: 64 bits if you’re 
on a 64-bit architecture and 32 bits if you’re on a 32-bit architecture.
- By default Rust int's are `i32`
- `let x: i32 = 0`

#### Floating
- There are 2 types of floating points in rust `f32` and `f64`
- The default type is `f64` because on modern CPUs it’s roughly the same speed as `f32` but is capable of more precision.
```rust
fn main() {
    let x = 2.0; // f64

    let y: f32 = 3.0; // f32
}
```
```rust
fn main() {
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;

    // remainder
    let remainder = 43 % 5;
}
```

#### Boolean
- Boolean type in Rust has two possible values: `true` and `false`. Booleans are one byte in size. 
The Boolean type in Rust is specified using `bool`

```rust
fn main() {
    let a = false;
    let b: bool = true;
}
```

#### Character
- Specifies a single type, specified using `char` type
- Usually a char is specified using single quotes
- Its a UTF-8 supported and is of 4 bytes in size
```rust
fn main() {
    let c = 'h';
}
```

## Compound Types
#### Tuple Types:
- Tuple is a grouping of multiple types into one
- They are of fixed size and once declared cannot be increased or shrinked in size
- Tuple is considered to be a single compound, to get individual values in a tuple we need to decouple them
```rust
fn main() {
    let a: (i16, f32, u64) = (1, 2.54, 6);
    let (x, y, z) = a;
    
    println!("The value of y is {}", y);
}
```
```rust
fn main() {
    let a: (i16, f32, u64) = (1, 2.54, 6);
    
    let first_value = a.0;
    let second_value = a.1;
    let third_value = a.2;

}
```

#### Arrays:
- Every element of array must be of the same type
- Arrays have a fixed length like tuples
- Arrays are stored on `stack` instead of `heap`
```rust
fn main() {
    let a = [1, 2, 3, 4, 5];
    
    let b: [i8; 5] = [1, 2, 3, 4, 5];
}
```
- If you want to create an array that contains the same value for each element, you can specify the initial value,
followed by a semicolon, and then the length of the array in square brackets
```rust
fn main() {
    let a = [3; 5]; // Will print [3,3,3,3,3]
}
```
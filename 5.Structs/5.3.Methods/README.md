# Methods
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
 
impl Rectangle {
    // We used & because we do not want to give the method ownership of self
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}
```

## Associated functions
- These are the functions that go into the `impl` block but do not take the `self` parameter
- They are still functions but do not have any reference with the struct
- `String::from` is an example of associated function
- They are often used as constructors, that return the instance of struct
```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn init(height: u32, width: u32) -> Rectangle {
        Rectangle {
            height,
            width
        }
    }
}

fn main() {
    let rect = Rectangle::init(4, 5);
}
```

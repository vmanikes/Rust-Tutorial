# Enums
- Say we need to work with IP addresses. Currently, two major standards are used for IP addresses: version four and 
  version six. These are the only possibilities for an IP address that our program will come across: we can enumerate 
  all possible variants, which is where enumeration gets its name.
  
```rust
enum IpAddrKind {
    v4,
    v6
}

enum UsState {
  Alabama,
  Alaska,
  // --snip--
}

enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}
```
- `IpAddrKind` is a enum that we can use anywhere in code
- `let v4 = IpAddrKind::v4`
- `let v6 = IpAddrKind::v6`
- The above both are of type `IpAddrKind`, so if we have a function signature `route(ip: IpAddrKind)`, we can pass 
either v4 or v6
  
- We can use them n structs to store data
```rust
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
```

## Storing data without having extra struct
```rust
enum IPAddr {
    v4(String),
    v6(String)
}

let home = IPAddr::v4(String::from("127.0.0.1"));
```

We know that the ip v4 only container 4 blocks of `u8` so we can change enum as
```rust
enum IPAddr {
    v4(u8, u8, u8, u8),
    v6(String)
}

let home = IpAddr::V4(127, 0, 0, 1);
```

This is not possible if we use structs, also in this way we can define our data more clearly

- Enum accepts strings, numeric types, or structs, for example. You can even include another enum!

## Methods on Enums
- Similar to how we declare methods on struct, we can define methods on enums as well
```rust
enum IPAddr {
    v4(u8, u8, u8, u8),
    v6(String)
}

impl IPAddr {
    fn call(&self) {
        // Do something
    }
}

fn main() {
    let a = IPAddr::v4(127, 0, 0, 1);
    a.call();
}
```

# Option Enum
- Rust does not have a null value, but it has a enum that says if a value is present or not
```rust
enum Option<T> {
    Some(T),
    None
}
```
- This is included in prelude, so no need to import like `Option::`
- `let some_number = Some(5);` or `let i_am_none: Option<i32> = None`
- If are planning to store `None`, we need to tell the type before because the compiler cannot infer the type 
as it does in `Some`
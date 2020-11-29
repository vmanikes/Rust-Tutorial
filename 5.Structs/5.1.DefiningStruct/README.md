# Structs
Structs are used to group data by giving name to them

```rust
struct User {
    user_name: String,
    email: String,
    sign_in_count: u64,
    active: bool,

}
```

## Initialization
If you can see below, order does not matter
```rust
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}
```

## Usage
To access a element, we can follow teh `.` notation, i.e., `user1.email`

## Modify value in struct
To have this possible the instance must be mutable
```rust
let mut user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
}

user1.active = false;
```

Rust does not allow for each indiviual parts of a struct to be mutable or not, entire struct must be mutable to change 
value

## Init shorthand
Instead of doing this
```rust
fn build_user(username: String, email: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}
```
```rust
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
```

## Updating struct
```
let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };
```
or
```rust
 let user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        ..user1
    };
```
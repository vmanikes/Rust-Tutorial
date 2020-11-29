# Use keyword
Rather than specifying the entire absolute or relative path, we can simplify using the `use` keyword
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```

- `use` is similarly used to bring the external crates to scope
```rust
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
}
```

## Using Nested Paths to Clean Up Large use Lists
Instead of doing this
```rust
use std::cmp::Ordering;
use std::io;
```

We could do
```rust
use std::{cmp::Ordering, io};
```
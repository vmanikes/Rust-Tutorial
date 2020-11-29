# Modules
- Modules let us organize code within a crate into groups for readability and easy reuse.
- Modules also control the privacy of items, which is whether an item can be used by outside code (public) or is an 
  internal implementation detail and not available for outside use (private).
  
## Creating library
- We can create new library using `cargo new --lib {}`
- This will create a new package with `cargo.toml` and `stc/lib.rs`
- Everything in a module is private by default
- Items in parent module cannot use the child module, but the child module can access the parent
- To make a child module accessible, we add the `pub` keyword
```rust
mod front_of_house {
    pub mod hosting {
        fn add_to_waitlist() {}
    }
}
```
Even with this we cannot access `add_to_waitlist` because the module is public, but the function inside is still private
```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
```

## Making struct public
If we use `pub` before a struct definition, we make the struct public, but the struct’s fields will still be private.
```rust

#![allow(unused)]
fn main() {
  mod back_of_house {
    pub struct Breakfast {
      pub toast: String,
      seasonal_fruit: String,
    }

    impl Breakfast {
      pub fn summer(toast: &str) -> Breakfast {
        Breakfast {
          toast: String::from(toast),
          seasonal_fruit: String::from("peaches"),
        }
      }
    }
  }

  pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // The next line won't compile if we uncomment it; we're not allowed
    // to see or modify the seasonal fruit that comes with the meal
    // meal.seasonal_fruit = String::from("blueberries");
  }
}

```

## Make enum public
If we make enum public, everything in it will be public
```rust
mod back_of_house {
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}
```

## Super keyword
We can also construct relative paths that begin in the parent module by using super at the start of the path. This is like starting a filesystem path with the .. syntax. Why would we want to do this?

Consider the situation in which a chef fixes an incorrect order and personally brings it out to the customer. The function fix_incorrect_order calls the function serve_order by specifying the path to serve_order starting with super
```rust
fn serve_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::serve_order();
    }

    fn cook_order() {}
}

fn main() {}

```

The `fix_incorrect_order` function is in the `back_of_house` module, so we can use `super` to go to the parent module of 
`back_of_house`, which in this case is `crate`, the root. From there, we look for `serve_order` and find it. 
Success! We think the `back_of_house` module and the `serve_order` function are likely to stay in the same relationship 
to each other and get moved together should we decide to reorganize the crate’s module tree. Therefore, we used `super` 
so we’ll have fewer places to update code in the future if this code gets moved to a different module.


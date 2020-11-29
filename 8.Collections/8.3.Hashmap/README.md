# Hashmap
The type HashMap<K, V> stores a mapping of keys of type K to values of type V. It does this via a hashing function,
which determines how it places these keys and values into memory

## Creating a new Hashmap
```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}
```
All the keys and values of a hashmap should be of same type

## Hashmap and ownership
```rust
fn main() {
    use std::collections::HashMap;

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!
}
```

## Accessing values in a Hashmap
```rust
use std::colections::Hashmap;

fn main() {
    let mut score = Hashmap::new();
    score.insert("team_a", 100);

    let team_name = "team_a";
    scores.get(&team_name)
}
```
The `.get()`  function returns `Option<&V>`

## Iterating a hashmap
```rust
for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
```

## Updating a hashmap
- If we insert a new key that is already in hashmap, that value will be replaced

### Inserting only if it not exists
- There is an `entry` API that will check if key exists or not and returns an enum called `Entry`
```rust
fn main() {
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);
}
```
he first call to entry will insert the key for the Yellow team with the value 50 because the Yellow team doesn’t have a 
value already. The second call to entry will not change the hash map because the Blue team already has the value 10.

### Updating based on old value
Here is a sample program that counts the occurrences of each word in string
```rust
fn main() {
    use std::collections::HashMap;

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

```
The or_insert method actually returns a mutable reference (&mut V) to the value for this key. Here we store that mutable 
reference in the count variable, so in order to assign to that value, we must first dereference count using the asterisk (*).
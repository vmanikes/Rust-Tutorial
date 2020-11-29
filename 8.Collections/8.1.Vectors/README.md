# Vectors
Vectors allow you to store more than one value in a single data structure that puts all the values next to each other in memory.

## Create a vector
`let v: Vec<i32> = Vec::new();`

Another way to declare it is by using
`let v = vec![1,2,3]`

If we do not want to give it a type, by default it assumes `i32`

## Updating a vector
```rust
fn main() {
    let v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
}
```

## Going out of scope
When a vector goes out of scope it and its values get dropped

## Accessing elements in vector 
```rust
fn main() {
    let v = vec![1, 2, 3, 4, 5];

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
}
```

The `get` method will give us `Option<&T>`, So we will have `Some` and `None` which will help use to handle the case where
a element out of bounds is requested. The first way will cause the program to panic

```rust
    let mut v = vec![1, 2, 3, 4, 5];

    let first = &v[0];

    v.push(6);

    println!("The first element is: {}", first);
```
this will not work as we have both mutable and immutable references, i.e., we give first a immutable reference
we change v and access first.  This error is due to the way vectors work: adding a new element onto the end of the 
vector might require allocating new memory and copying the old elements to the new space, if there isn’t enough room to 
put all the elements next to each other where the vector currently is. In that case, the reference to the first element 
would be pointing to deallocated memory. The borrowing rules prevent programs from ending up in that situation.

To fix this we can move the `println!` to before push

## Iterating on Vector
```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

If we want to modify a vector on iteration
```rust
let v = vec![1,2,3];
for i in &mut v {
    *v += 10;
}
```

## Using an Enum to Store Multiple Types
```rust
fn main() {
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
}

```
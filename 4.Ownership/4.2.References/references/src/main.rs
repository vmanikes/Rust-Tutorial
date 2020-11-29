fn main() {
    let s = String::from("hello");

    let len = calculate_length(&s);

    println!("The lenght of '{}' is {}", s, len);
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}
fn main() {
    let mut number = 3;

    if number > 4 {
        println!("Greater");
    } else if number == 4{
        println!("equal");
    } else {
        println!("Lower");
    }

    loop {
        // This is an infinite loop that will only stop if specify a breaking condition
    }

    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    while number != 0 {
        number -= 1;
    }

    // For loop
    let a = [1,2,3,4,5,6];
    for element in a.iter() {
        println!(element)
    }
}

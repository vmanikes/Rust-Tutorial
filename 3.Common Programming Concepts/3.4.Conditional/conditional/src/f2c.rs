fn main() {
    println!("{}", farenheit_to_celcius(68.00));

    println!("{}", fibonaaci(5))
}

fn farenheit_to_celcius(temprature: f64) -> f64 {
    (temprature - 32.0) * 5.0/9.0
}

fn fibonaaci(n: i64) -> i64 {
    if n == 0 {
        return 0;
    } else if n == 1 {
        return 1;
    } else {
        return fibonaaci(n - 1) + fibonaaci(n - 2);
    }
}
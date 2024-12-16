fn main() {
    println!("Hello, world!");

    println!("f_to_c(32): {}", fahrenheit_to_celsius(32.0));
    println!("f_to_c(212): {}", fahrenheit_to_celsius(212.0));
    println!("f_to_c(100): {}", fahrenheit_to_celsius(100.0));

    println!("3rd fib: {}", nth_fib(3));
    println!("9th fib: {}", nth_fib(9));
    println!("15th fib: {}", nth_fib(15));
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    (temp - 32f64) * { 5f64 / 9f64 }
}

fn nth_fib(n: u64) -> u64 {
    if n == 1 {
        0
    } else if n == 2 {
        1
    } else {
        let mut last: u64 = 0;
        let mut current: u64 = 1;
        for _ in 1..n {
            (last, current) = (current, current + last);
        }
        return current;
    }
}

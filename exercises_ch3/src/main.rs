fn main() {
    let degrees: f64 = 100.;
    let fib_index = 44;

    println!("{}°F to C is: {}",degrees,to_celcius(degrees));
    println!("And back to °F is: {}",to_fahrenheit(to_celcius(degrees)));

    println!("The #{} fibonnaci number is: {}", fib_index, fib(fib_index));
}

fn to_celcius(f: f64) -> f64 {
    (f - 32.0) * (5.0/9.0)
}

fn to_fahrenheit(c: f64) -> f64 {
    c * (9.0/5.0) + 32.0
}

fn fib(nth: i32) -> i32 {
    if nth == 0 {
        0
    } else if nth == 1 {
        1
    } else {
        fib(nth - 1) + fib(nth - 2)
    }
}


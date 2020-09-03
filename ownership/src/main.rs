fn main() {
    // Heap
    let s1 = String::from("hello");
    // Not shallow copy, move because s1 is now disabled
    let s2 = s1;
    println!("s1 was moved to s2 and it says: {}", s2);

    let s3 = String::from("hello");
    // Deep copy
    let s4 = s3.clone();
    println!("s3 = {}, s4= {}", s3, s4);

    // Stack 
    // Doesn't break because i32 is on the stack
    let x = 5;
    let y = x;

    println!("x = {}, y = {}", x, y);

    let s1 = String::from("hello");
    let (s2, len) = calculate_length(s1);
    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
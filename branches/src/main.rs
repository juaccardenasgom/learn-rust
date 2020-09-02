fn main() {
    let number = 7;

    if number < 5 {
        println!("CONDITION WAS T");
    } else {
        println!("CONDITION WAS F");
    }

    let condition = true;
    let number2 = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number2);
    
}

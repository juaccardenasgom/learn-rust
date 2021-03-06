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
    
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    let a = [10,20, 30, 40, 50];

    for element in a.iter() {
        println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

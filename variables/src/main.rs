fn main() {
    // Mutable var
    let mut mutable_variable = 5;
    println!("The value of mutable_variable is: {}",mutable_variable);
    mutable_variable = 6;
    println!("The value of mutable_variable is: {}",mutable_variable);

    // Shadowing a var
    let shadow_variable = 5;
    let shadow_variable = shadow_variable + 1;
    let shadow_variable = shadow_variable * 2;
    println!("The value of shadow_variable is: {}",shadow_variable);

    // Chaging type
    let spaces = "   ";
    println!("The value of spaces is: '{}'", spaces);
    let spaces = spaces.len();
    println!("The value of spaces is: {}", spaces);

    // Parsing & shadowing var
    let guess = "42";
    println!("The value of guess is: '{}'", guess);
    let guess: u32 = guess.parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    // Integer (Scalar) type
    let signed_integer: i32 = 123_456;
    println!("The value of signed_integer is: {}", signed_integer);
    let signed_integer = 0xff;
    println!("The value of signed_integer is: {}", signed_integer);
    let byte_integer: u8 = b'A';
    println!("The value of byte_integer is: {}", byte_integer);

    // Floating (Scalar) type
    let float_64 = 123.45;
    println!("The value of float_64 is: {}", float_64);

    // Boolean type
    let t = true;
    let f: bool = false;
    println!("The value of t is: {}, and f is: {}", t, f);

    // Character type
    let c = 'z';
    let z = 'z';
    let heart_eyed_cat = '😻';
    println!("The value of c is: {}, z is: {}, and emoji is: {}", c, z, heart_eyed_cat);

    // Tuple (compound) type
    let tup: (i32,f64,u8) = (500,6.4,1);
    let (x, y, z) = tup;
    println!("The value of x,y,z is: {},{},{}", x, y, z);

    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    println!("The value of 500: {}, 6.4: {} and one is: {}", five_hundred, six_point_four, one);

    // Array (compound) type
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    println!("The value of first value of a is: {}", a[0]);

    let a2 = [3; 5];
    println!("All the values of a2 are: {}", a2[0]);
}

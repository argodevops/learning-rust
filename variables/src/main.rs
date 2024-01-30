fn main() {

    // vars can be mut(able)
    let x = 5;
    println!("The first value of x is: {}", x);

    // shadow var
    let x = 6;
    println!("The second value of x is: {}", x);

    // const are not mutable
    const SUBSCRIBER_COUNT: u32 = 100_000;

    // Scaler Types - Integer, Float, Boolean, Character
    let float: u8 = 255;
    let quotient: f64 = 3.14279;

    // Compound Types
    // Tuples
    let tup = ("Let's Get Rusty!", 100_000);
    let (channel, sub_count) = tup;
    let sub_count = tup.1;

    // Arrays
    let error_codes = [200, 404, 500];
    let not_found = error_codes[1];
    //let x = error_codes[3]; not allowed
    let byte = [0; 8];
}

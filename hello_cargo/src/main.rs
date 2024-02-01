fn main() {
    println!("{}, {}!", "Hello", "cargo"); // Hello, cargo!
    println!("{0}, {1}!", "Hello", "cargo"); // Hello, cargo!
    println!("{greeting}, {name}!", greeting = "Hello", name = "cargo"); // Hello, cargo!

    let (greeting, name) = ("Hello", "cargo"); // 💡 Two Variable bindings declare & initialize in one line.
    println!("{greeting}, {name}!"); // Hello, cargo!

    println!("{:?}", [1, 2, 3]); // [1, 2, 3]
    println!("{:#?}", [1, 2, 3]);
    /*
        [
            1,
            2,
            3
        ]
    */

    // 🔎 The format! macro is used to store the formatted string.
    let x = format!("{}, {}!", "Hello", "cargo");
    println!("{}", x); // Hello, cargo!

    // 💡 Rust has a print!() macro as well
    print!("Hello, cargo!"); // Without new line
    println!(); // A new line

    print!("Hello, cargo!\n"); // With new line
}

fn main() {
    // ----- Ownership rules -----
    // 1. Each value in Rust has a variable that's called owner.
    // 2. There can only be one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    { // is not valid here, it's not yet declared
        let s1 = String::from("hello"); // s1 is valid from this point forward
        let s2 = s1; // s1 moved to s2 (not a shallow copy)
        let s3 = s2.clone(); // shallow copy
        // simple types on the stack are copied
    } // this scope is now over, and s1 is no longer valid
    

    // ----- Reference rules -----
    // 1. At any given time, you can either have one mutable reference or any number of immutable references
    // 2. References must always be valid (not reference value going out of scope)

    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
    // takes a reference string (borrowing) to avoid passing ownership of 's' variable
    let length = s.len();
    length
}

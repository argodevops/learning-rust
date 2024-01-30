fn main() {
    let sum = my_function(11, 22);
    println!("The sum of x and y: {}", sum);
    show_conditions();
    show_loop();
    show_while_loop();
    show_for_loop();
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The values are x: {}, y: {}", x, y);
    let sum = x + y;
    sum
}

fn show_conditions() {
    let number = 5;

    if number < 10 {
        println!("first condition was true");
    } else if number < 22 {
        println!("second condition was true")
    } else {
        println!("condition was false")
    }
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("Number is: {}", number); 
}

fn show_loop() {
    let mut counter = 0;

    loop {
        counter += 1;
        if counter == 10 {
            break counter;
        }
    };
    println!("loop counter: {}!", counter);
}

fn show_while_loop() {
    let mut counter = 3;

    while counter != 0 {
        println!("while: {}!", counter);
        counter -= 1;
    }
}

fn show_for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("for element: {}!", element);
    }

    for number in 1..4 {
        println!("for number: {}!", number)
    }
}
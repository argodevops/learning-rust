fn main() {
    let sum = my_function(11, 22);
    println!("The sum of x and y: {}", sum);
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("The values are x: {}, y: {}", x, y);
    let sum = x + y;
    sum
}
use std::io;

fn main() {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to readline");
    
    let num1: i32 = input.trim().parse().expect("Error in input1");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to readline");

    let num2: i32 = input.trim().parse().expect("Error in input2");

    println!("PROD = {}", num1 * num2);
}
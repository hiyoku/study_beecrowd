use std::io;

fn main() {
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num1: i32 = input.trim().parse().expect("Please type a number! 1");

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let num2: i32 = input2.trim().parse().expect("Please type a number! 2");

    println!("X = {}", num1 + num2);
}
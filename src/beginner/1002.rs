use std::io;

fn main() {
    let mut input = String::new();
    let pi = 3.14159f64;

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num1: f64 = input.trim().parse().expect("Please type a number! 1");

    println!("A={:.1$}", pi * (num1.powi(2)), 4);
}
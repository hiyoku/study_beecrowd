use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error in input1");

    let num1: f64 = input.trim().parse().expect("Error to parse1");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error in input1");

    let num2: f64 = input.trim().parse().expect("Error to parse1");

    println!("MEDIA = {:.5}", ((num1 * 3.5) + (num2 * 7.5)) / 11.0);
}
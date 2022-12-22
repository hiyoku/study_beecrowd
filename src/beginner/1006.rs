use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error in input1");

    let num1: f64 = input.trim().parse().expect("Error to parse1");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error in input1");

    let num2: f64 = input.trim().parse().expect("Error to parse1");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error in input1");

    let num3: f64 = input.trim().parse().expect("Error to parse1");

    println!("MEDIA = {:.1}", ((num1 * 2.0) + (num2 * 3.0) + (num3 * 5.0)) / 10.0);
}
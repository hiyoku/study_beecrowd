use std::io;

fn main() {
    // Escreva a sua solução aqui
    // Code your solution here
    // Escriba su solución aquí
    let mut input = String::new();
    
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num1: u32 = input.trim().parse().expect("Please type a number! 1");

    let mut input2 = String::new();
    io::stdin()
        .read_line(&mut input2)
        .expect("Failed to read line");

    let num2: u32 = input2.trim().parse().expect("Please type a number! 2");

    println!("X = {}", num1 + num2);
}
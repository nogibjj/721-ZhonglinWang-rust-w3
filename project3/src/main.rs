use std::io;

fn main() {
    println!("Enter first number:");
    let mut first = String::new();
    io::stdin().read_line(&mut first).unwrap();

    println!("Enter second number:");
    let mut second = String::new();
    io::stdin().read_line(&mut second).unwrap();

    let first = first.trim().parse::<i32>().unwrap();
    let second = second.trim().parse::<i32>().unwrap();

    let sum = first.pow(2) + second.pow(2);
    println!("Sum of squares: {}", sum);
}
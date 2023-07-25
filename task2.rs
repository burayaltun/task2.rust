use std::io;

enum Operation{
    Add(f64, f64);
    Subtract(f64, f64);
    Multiply(f64, f64);
    Divide(f64, f64);
}
fn calculate(op: Operation) -> f64 {
    match op {
        Operation::Add(a, b) => a + b,
        Operation::Subtract(a, b) => a - b,
        Operation::Multiply(a, b) => a * b,
        Operation::Divide(a, b) => a / b,
    }
}
fn main() {
    println!("Write the first number:");
    let mut first_num = String::new();
    io::stdin().read_line(&mut first_num).expect("Couldn't read the input.");
    let first_num: f64 = first_num.trim().parse().expect("Invalid input. Please write a valid number please.");

    println!("Write the operation that you want (Add, Subtract, Multiply, Divide):");
    let mut operation = String::new();
    io::stdin().read_line(&mut operation).expect("Couldn't read the input.");
    let operation = match operation.trim().to_lowercase().as_str() {
        "add" => Operation::Add,
        "subtract" => Operation::Subtract,
        "multiply" => Operation::Multiply,
        "divide" => Operation::Divide,
        _ => {
            println!("Invalid operation. Please enter a valid operation.");
            return;
        }
    };

    println!("Write the second number:");
    let mut second_num = String::new();
    io::stdin().read_line(&mut second_num).expect("Couldn't read the input.");
    let second_num: f64 = second_num.trim().parse().expect("Invalid input. Please write a valid number.");

    let result = calculate(operation(first_num, second_num));
    println!("Result: {}", result);
}








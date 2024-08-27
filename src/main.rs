use std::io;

fn main() {
    println!("Welcome to the Rust Calculator!");

    // Get the first number from the user
    println!("Enter the first number:");
    let mut num1 = String::new();
    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read input");
    let num1: f64 = num1.trim().parse().expect("Please enter a valid number");

    // Get the operator from the user
    println!("Enter an operator (+, -, *, /):");
    let mut operator = String::new();
    io::stdin()
        .read_line(&mut operator)
        .expect("Failed to read input");
    let operator = operator.trim();

    // Get the second number from the user
    println!("Enter the second number:");
    let mut num2 = String::new();
    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read input");
    let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

    // Perform the calculation
    let result = match operator {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => {
            if num2 == 0.0 {
                println!("Error: Division by zero is not allowed.");
                return;
            } else {
                num1 / num2
            }
        },
        _ => {
            println!("Error: Invalid operator.");
            return;
        }
    };

    // Print the result
    println!("The result of {} {} {} = {}", num1, operator, num2, result);
}


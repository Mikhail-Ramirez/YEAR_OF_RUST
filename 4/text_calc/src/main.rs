use std::io;
use std::io::prelude::*;

fn main() {
    let stdin = io::stdin();
    println!("Enter calculations (e.g., 10+57, 89-45). Press Ctrl+D to exit.");

    for line in stdin.lock().lines() {
        match line {
            Ok(input) => {
                let trimmed = input.trim();
                if trimmed.is_empty() {
                    continue; // Skip empty lines
                }

                match parse_and_compute(trimmed) {
                    Ok(result) => println!("{} = {}", trimmed, result),
                    Err(e) => eprintln!("Error processing '{}': {}", trimmed, e),
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
            }
        }
    }
}

/// Parses a calculation string and computes the result.
/// Supports addition (+) and subtraction (-).
fn parse_and_compute(expression: &str) -> Result<i32, String> {
    // Find the operator position
    let operator_pos = expression.find(|c| c == '+' || c == '-')
        .ok_or("No operator found (+ or -)".to_string())?;

    // Split the expression into two operands
    let left = &expression[..operator_pos];
    let operator = &expression[operator_pos..operator_pos + 1];
    let right = &expression[operator_pos + 1..];

    // Parse operands to integers
    let left_num: i32 = left.trim().parse()
        .map_err(|_| format!("Invalid number: '{}'", left))?;
    let right_num: i32 = right.trim().parse()
        .map_err(|_| format!("Invalid number: '{}'", right))?;

    // Perform the calculation based on the operator
    match operator {
        "+" => Ok(left_num + right_num),
        "-" => Ok(left_num - right_num),
        _ => Err(format!("Unsupported operator: '{}'", operator)),
    }
}


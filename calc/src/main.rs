use std::io::{self, Write};

mod calculator;

use calculator::Calculator;

fn main() {
    println!("========================================");
    println!("  Welcome to Rust Calculator");
    println!("========================================\n");
    
    let calculator = Calculator::new();  // Default language: Japanese
    
    loop {
        println!("\nOptions:");
        println!("1. Add (+)");
        println!("2. Subtract (-)");
        println!("3. Multiply (*)");
        println!("4. Divide (/)");
        println!("5. Nth Root (√)");
        println!("6. Power (^)");
        println!("7. Exit");
        
        print!("\nSelect operation (1-7): ");
        io::stdout().flush().unwrap();
        
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        
        match choice.trim() {
            "1" => {
                if let Ok(result) = calculator.add() {
                    println!("\n✓ Result: {}\n", result);
                    calculator.speak(&result.to_string());
                }
            }
            "2" => {
                if let Ok(result) = calculator.subtract() {
                    println!("\n✓ Result: {}\n", result);
                    calculator.speak(&result.to_string());
                }
            }
            "3" => {
                if let Ok(result) = calculator.multiply() {
                    println!("\n✓ Result: {}\n", result);
                    calculator.speak(&result.to_string());
                }
            }
            "4" => {
                match calculator.divide() {
                    Ok(result) => {
                        println!("\n✓ Result: {}\n", result);
                        calculator.speak(&result.to_string());
                    },
                    Err(e) => println!("\n✗ Error: {}\n", e),
                }
            }
            "5" => {
                match calculator.nth_root() {
                    Ok(result) => {
                        println!("\n✓ Result: {}\n", result);
                        calculator.speak(&result.to_string());
                    },
                    Err(e) => println!("\n✗ Error: {}\n", e),
                }
            }
            "6" => {
                match calculator.power() {
                    Ok(result) => {
                        println!("\n✓ Result: {}\n", result);
                        calculator.speak(&result.to_string());
                    },
                    Err(e) => println!("\n✗ Error: {}\n", e),
                }
            }
            "7" => {
                println!("\nThank you for using Rust Calculator!");
                break;
            }
            _ => println!("Invalid choice! Please select 1-7."),
        }
    }
}

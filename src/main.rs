use std::io;
mod infix;

use crate::infix::evaluate_infix_expression;
pub const EXIT_COMMAND: &str = "q";
pub const MEMORY_FLAG: &str = "MemBecauseMemory";

fn main() {
    let mut last_result: Option<f64> = None; // Variable to store the last result in memory

    loop {
        // Select mode if not already selected

        loop {
            println!("Enter an expression to calculate:");

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            let input = input.trim();

            // Check for output
            if input.eq_ignore_ascii_case(EXIT_COMMAND) {
                return;
            }

            // Handle the case when MEMORY_FLAG is entered to use the last result
            let expression = if input.contains(MEMORY_FLAG) {
                if let Some(result) = last_result {
                    // Replace "-R" with the last result
                    input.replace(MEMORY_FLAG, &result.to_string()) // Use the last result
                } else {
                    println!("No result available."); // If there is no result
                    continue;
                }
            } else {
                input.to_string() // Otherwise, use the entered value
            };


            // Evaluate the expression
            match evaluate_infix_expression(&expression) {
                Ok(result) => {
                    last_result = Some(result); // Save the result for future use
                    println!("Result: {}", result);
                }
                Err(e) => println!("Error: {}", e), // Print an error if it occurs
            }
        }
    }
}
use regex::Regex;
use std::io;

struct Calc;

impl Calc {
    fn evaluate_expression(expression: &str) -> Result<f64, &'static str> {
        // Remove all spaces
        let expression = expression.replace(" ", "");

        // Regex to match numbers and operators
        let re = Regex::new(r"(\d+\.?\d*|[-+*/%])").unwrap();
        let tokens: Vec<&str> = re.find_iter(&expression).map(|mat| mat.as_str()).collect();

        // Vectors for numbers and operators
        let mut numbers: Vec<f64> = Vec::new();
        let mut operators: Vec<char> = Vec::new();

        for token in tokens {
            if let Ok(num) = token.parse::<f64>() {
                numbers.push(num);
            } else if let Some(op) = token.chars().next() {
                operators.push(op);
            }
        }

        if numbers.len() == 0 {
            return Err("No numbers found");
        }

        let mut result = numbers[0];

        for (i, &op) in operators.iter().enumerate() {
            if i + 1 >= numbers.len() {
                return Err("Mismatched number of operators and operands");
            }

            match op {
                '+' => result += numbers[i + 1],
                '-' => result -= numbers[i + 1],
                '*' => result *= numbers[i + 1],
                '/' => {
                    if numbers[i + 1] == 0.0 {
                        return Err("Division by zero");
                    }
                    result /= numbers[i + 1];
                }
                '%' => result %= numbers[i + 1],
                _ => return Err("Invalid operator"),
            }
        }

        Ok(result)
    }
}

fn main() {
    println!("Rust calculator");
    let mut scanner = String::new();

    loop {
        io::stdin()
            .read_line(&mut scanner)
            .expect("Failed to read input");
        let input = scanner.trim();

        if input == "q" {
            println!("Exiting the calculator. Goodbye!");
            break;
        } else {
            match Calc::evaluate_expression(input) {
                Ok(result) => println!("Result: {}", result),
                Err(e) => println!("Error: {}", e),
            }
        }

        scanner.clear();
    }
}

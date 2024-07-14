use regex::Regex;
use std::collections::VecDeque;
use std::io;

struct Calc;

impl Calc {
    fn evaluate_expression(expression: &str) -> Result<f64, &'static str> {
        // Remove all spaces
        let expression = expression.replace(" ", "");

        // Regex to match numbers and operators
        let re = Regex::new(r"(\d+\.?\d*|[-+*/%()])").unwrap();
        let tokens: Vec<&str> = re.find_iter(&expression).map(|mat| mat.as_str()).collect();

        // Convert infix expression to postfix using the Shunting Yard algorithm
        let mut output_queue: VecDeque<String> = VecDeque::new();
        let mut operator_stack: Vec<char> = Vec::new();

        for token in tokens {
            if let Ok(_) = token.parse::<f64>() {
                output_queue.push_back(token.to_string());
            } else if let Some(op) = token.chars().next() {
                match op {
                    '+' | '-' => {
                        while let Some(&top_op) = operator_stack.last() {
                            if top_op == '(' {
                                break;
                            }
                            output_queue.push_back(operator_stack.pop().unwrap().to_string());
                        }
                        operator_stack.push(op);
                    }
                    '*' | '/' | '%' => {
                        while let Some(&top_op) = operator_stack.last() {
                            if top_op == '(' || top_op == '+' || top_op == '-' {
                                break;
                            }
                            output_queue.push_back(operator_stack.pop().unwrap().to_string());
                        }
                        operator_stack.push(op);
                    }
                    '(' => operator_stack.push(op),
                    ')' => {
                        while let Some(top_op) = operator_stack.pop() {
                            if top_op == '(' {
                                break;
                            }
                            output_queue.push_back(top_op.to_string());
                        }
                    }
                    _ => return Err("Invalid operator"),
                }
            }
        }

        while let Some(op) = operator_stack.pop() {
            output_queue.push_back(op.to_string());
        }

        // Evaluate the postfix expression
        let mut eval_stack: Vec<f64> = Vec::new();

        while let Some(token) = output_queue.pop_front() {
            if let Ok(num) = token.parse::<f64>() {
                eval_stack.push(num);
            } else if let Some(op) = token.chars().next() {
                let b = eval_stack.pop().ok_or("Invalid expression")?;
                let a = eval_stack.pop().ok_or("Invalid expression")?;
                let result = match op {
                    '+' => a + b,
                    '-' => a - b,
                    '*' => a * b,
                    '/' => {
                        if b == 0.0 {
                            return Err("Division by zero");
                        }
                        a / b
                    }
                    '%' => a % b,
                    _ => return Err("Invalid operator"),
                };
                eval_stack.push(result);
            }
        }

        eval_stack.pop().ok_or("Invalid expression")
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
                Ok(result) => println!("    {}", result),
                Err(e) => println!("Error: {}", e),
            }
        }

        scanner.clear();
    }
}

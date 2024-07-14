use regex::Regex;
use std::io::{self, Write};
fn calculator(input: &str) -> Result<f64, &'static str> {
    let expression = input.replace(" ", "");

    let re = Regex::new(r"(\d+\.?\d*|[-+*/%])").unwrap();

    let tokens: Vec<&str> = re.find_iter(&expression).map(|mat| mat.as_str()).collect();

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
fn main() {
    println!("Rust calculator");
    let mut scanner = String::new();
    loop {
        print!("; ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut scanner).unwrap();

        let input = scanner.trim();
        if input == "q" {
            break;
        } else {
            match calculator(input) {
                Ok(val) => println!(">    {val}"),
                Err(e) => eprintln!("{e}"),
            }
        }

        scanner.clear();
    }
}

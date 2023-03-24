use std::io;
//use std::num::ParseFloatError;
use std::str::FromStr;

#[derive(Debug)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Sqrt,
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Subtract),
            "*" => Ok(Operator::Multiply),
            "/" => Ok(Operator::Divide),
            "sqrt" => Ok(Operator::Sqrt),
            _ => Err(()),
        }
    }
}

fn main() {
    println!("Enter an expression:");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    match calculate(&input.trim()) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn calculate(input: &str) -> Result<f64, String> {
    let tokens: Vec<&str> = input.split_whitespace().collect();

    if tokens.len() != 3 && !(tokens.len() == 2 && tokens[0] == "sqrt") {
        return Err("Invalid input format".to_string());
    }

    let operator = tokens[0]
        .parse::<Operator>()
        .map_err(|_| "Invalid operator")?;

    let operand1 = tokens[1]
        .parse::<f64>()
        .map_err(|e| format!("Invalid operand: {}", e))?;

    let result = match operator {
        Operator::Add => {
            let operand2 = parse_operand(&tokens[2], "Invalid operand")?;
            operand1 + operand2
        }
        Operator::Subtract => {
            let operand2 = parse_operand(&tokens[2], "Invalid operand")?;
            operand1 - operand2
        }
        Operator::Multiply => {
            let operand2 = parse_operand(&tokens[2], "Invalid operand")?;
            operand1 * operand2
        }
        Operator::Divide => {
            let operand2 = parse_operand(&tokens[2], "Division by zero")?;
            operand1 / operand2
        }
        Operator::Sqrt => {
            if operand1 < 0.0 {
                return Err("Invalid input for sqrt: negative number".to_string());
            }
            operand1.sqrt()
        }
    };

    Ok(result)
}

fn parse_operand(s: &str, error_msg: &str) -> Result<f64, String> {
    s.parse::<f64>().map_err(|_| error_msg.to_string())
}

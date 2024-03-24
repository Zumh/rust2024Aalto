use std::io;
/*
// model solution
// If failed to parse then panic otherwise return original error
fn read_line() -> io::Result<String> {
    io::stdin()
        .lines()
        .next()
        .ok_or(io::Error::from(io::ErrorKind::UnexpectedEof))
        .unwrap()
}

pub fn parse_number(number: &str) -> f32 {
    match number.parse() {
        Ok(n) => n,
        Err(error) => panic!("Failed to parse {number} as a number. Error: {error}"),
    }
}
*/
fn read_line() -> io::Result<String> {
    io::stdin().lines().next().unwrap()
}

// Implement the parse_number function so that parses a number-like &str to f32

const ALLOWED_OPERATORS: [char; 4] = ['+', '-', '*', '/'];

fn parse_number(number: &str) -> f32 {
    //number.parse().unwrap()
    if let Err(e) = number.parse::<f32>() {
        panic!("Failed to parse {} as a number. Error: {}.", number, e);
    } else {
        number.parse().unwrap()
    }

}
fn calculate_from_string(line: &str) -> f32 {
    let mut numbers = Vec::new();
    let mut operators = Vec::new();

    // Remove spaces
    let line = line.replace(|c: char| c.is_whitespace(), "");

    // Split the string into numbers and operators
    let mut parts = line
        .split_inclusive(ALLOWED_OPERATORS)
        // Extract the operator from the string
        .map(|part| {
            if part.ends_with(ALLOWED_OPERATORS) {
                part.split_at(part.len() - 1)
            } else {
                (part, "")
            }
        })
        .flat_map(|(number, operator)| [number, operator])
        .filter(|part| !part.is_empty())
        .collect::<Vec<&str>>();
    if parts.len() % 2 == 0 {
        parts.push("");
    }

    // Parse the numbers and operators
    for (i, part) in parts.iter().enumerate() {
        if i % 2 == 0 {
            // Part is a number
            let number = parse_number(part);
            numbers.push(number);
        } else {
            // Part is an operator
            operators.push(part);
        }
    }

    // Calculate the result (naively without worrying about operator precedence)
    let mut result = numbers[0];
    for (i, operator) in operators.iter().enumerate() {
        match **operator {
            "+" => result += numbers[i + 1],
            "-" => result -= numbers[i + 1],
            "*" => result *= numbers[i + 1],
            "/" => result /= numbers[i + 1],
            _ => panic!("Unknown operator {operator}"),
        }
    }
    result
}

pub fn parsing_input_into_numbers() {
    let line = read_line().expect("Could not read line");
    let calculation_result = calculate_from_string(&line);
    println!("{calculation_result}");
}

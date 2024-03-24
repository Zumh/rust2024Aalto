// Implement the read_line function to read a line from the user

use std::io;

fn read_line() -> String {

    // model solution without handling any error
    // io::stdin().lines().next().unwrap().unwrap()
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            input = input.trim().to_string();
        }
        Err(error) => println!("error: {error}"),

    }
    input
}
pub fn read_line_from_stdin() {
    let accepted_answers = vec!["yes", "y", "no", "n"];

    println!("Do you like Rust? Yes or no (y/n)?");

    let mut answer = String::new();
    while !accepted_answers.contains(&answer.as_str()) {
        answer = read_line();
        match answer.as_str() {
            "yes" | "y" => println!("Awesome!"),
            "no" | "n" => println!("Oh :("),
            _ => println!("Unrecognized answer, please answer yes or no (y/n)"),
        }
    }
}

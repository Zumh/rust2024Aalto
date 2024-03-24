use std::io;

pub fn read_user_input() {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {error}"),
    }
}

pub fn no_panicking_read_user_input() {
    let mut input = String::new();
    // .read_line(&mut input) will panic if it cannot read user input or end with newline or EOF.
    // detect EOF or newline.
    let current_input = io::stdin().read_line(&mut input).unwrap();
    println!("{} bytes read", current_input);
    println!("{}", input);
}

pub fn trim_user_input() {
    let mut input = String::new();
    // we need only one unwrap. no trim and no need to difine mutable string to store the input.
    io::stdin().read_line(&mut input).unwrap();
    println!("Hi!, {}!", input.trim());
}

pub fn take_user_input() {
    let mut numbers = vec![];
    for line in io::stdin().lines().take(3) {
        numbers.push(line);
    }
    println!("The numbers were {numbers:?}");
}

pub fn next_user_input() {
    println!("Input one line");
    // first unwrap is on the option returned by next that needs to be handled for any iterator.
    // second unwrap is the REuslt for handling invalid input.
    let line = io::stdin().lines().next().unwrap().unwrap();

    println!("The line was {line:?}");
}

// error type is io::Error and ios::Result is an alias for Result<String, io::Error>
// collecting infinite stream of lines
pub fn infinite_user_input() {
    println!("Input 3 numbers");
    let lines: Vec<io::Result<String>> = io::stdin().lines().collect();
    println!("The numbers were {lines:?}");
}

// collecting with limit of 10 repeated ones
pub fn collect_with_limit_user_input(){
    let ones = std::iter::repeat(1).take(10).collect::<Vec<i32>>();
    println!("{ones:?}");
}


fn read_i32() -> i32 {
    let line = io::stdin().lines().next().unwrap().unwrap();
    line.parse().unwrap()
}
pub fn parsing_input_into_numbers() {
    println!("Input a number");
    let number1 = read_i32();
    println!("Input another number");
    let number2 = read_i32();
    println!("{number1} + {number2} = {}", number1 + number2);
}

pub fn error_kind_user_input() {
    use std::io;
    use std::num::IntErrorKind;
    let mut line = String::new();
    loop {
        println!("Please enter a number");
        io::stdin().read_line(&mut line).unwrap();
        // handling error kind that return from .parse
        if let Err(e) = line.trim().parse::<i32>() {
            match e.kind() {
                IntErrorKind::Empty => {
                    println!("Exiting...");
                    break;
                }
                IntErrorKind::InvalidDigit => {
                    println!("Invalid digit, try again")
                }
                error => {
                    panic!("Unexpected error {error:?}")
                }
            }
        }
        line.clear()
    }
}
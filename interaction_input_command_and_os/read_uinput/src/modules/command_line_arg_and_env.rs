use std::env;

// this will echo the command line arguments
// cargo run -- Hello World
// -- tell arguments after are not for cargo run but for the program that is run.
pub fn echo() {
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
}

// we have default number just in case we don't have two arguments
pub fn multiplying_two_args_values() {
    let args: Vec<String> = env::args().collect();
    let default = "1".to_string();
    let number1 = args.get(1).unwrap_or(&default).parse::<f64>().expect("Expected a number");
    let number2 = args.get(2).unwrap_or(&default).parse::<f64>().expect("Expected a number");
    let product = number1 * number2;
    println!("{product}");
}

pub fn product_of_two_args() {
    // skip the the first argument which is the program name or path.
    // if we provide 1 argument it will return 1.0
    let product = env::args()
    .skip(1)
    .map(|arg| arg.parse::<f64>().unwrap_or(1.0))
    .product::<f64>();
    println!("{product}");
}

use std::collections::HashMap;
// setting rust backtrace environment variable 'export RUST_BACKTRACE=1'
// for setting and overriding environment variable for just one cargo run 'export RUST_BACKTRACE=1 cargo run'
pub fn environment_variable() {
    // env::vars() return environment variables
    let vars: HashMap<String, String> = env::vars().collect();
    println!("{vars:#?}");
}





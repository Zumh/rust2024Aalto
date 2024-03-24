
/*
A Rust program may try to read non-UTF-8 data from input. 
Make the function read_line in the starter code safer by handling possible errors by returning a Result instead of a String so that it works as the main function expects.

Use the compiler hints to see the correct error type for the Result given by reading a line with stdin.

To test the error locally non-UTF-8 data needs to be passed as input to the Rust program. 
This can be done by running cargo run < filename in a terminal where filename is any file with content that is not valid UTF-8, an image file for instance. The < symbol redirects the contents of the file to the program's stdin. 
This works in both Windows and Unix-like systems.
*/
// Make the function read_line in the starter code safer by handling possible errors by returning a Result instead of a String so that it works as the main function expectsuse std::io;
use std::io;

// Modify read_line to work with the updated main function
fn read_line() -> io::Result<String> {
    let mut line = String::new();

    match io::stdin().read_line(&mut line) {
        Ok(_) => {
            Ok(line.trim().to_string())
        }
        Err(_) => {
            // Non-UTF-8 input, prompt the user again
            Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "stream did not contain valid UTF-8",
            ))
        }
    }
}

/*

//model solution
fn read_line() -> Result<String, io::Error> {
    io::stdin().lines().next().unwrap()
}
*/
pub fn bytes_arent_utf_8() {
    let accepted_answers = vec!["yes", "y", "no", "n"];

    println!("Do you like Rust? Yes or no (y/n)?");

    let mut answer = String::new();
    while !accepted_answers.contains(&answer.as_str()) {
        let maybe_answer = read_line();
        println!("{:?}", maybe_answer);

        // check if UTF-8
        

        answer = match maybe_answer {
            Ok(answer) => answer.to_lowercase(),
            Err(_) => {
                println!("What is this nonsense?");
                continue;
            }
        };


        match answer.as_str() {
            "yes" | "y" => println!("Awesome!"),
            "no" | "n" => println!("Oh :("),
            _ => println!("Unrecognized answer, please answer yes or no (y/n)"),
        }
    }
}

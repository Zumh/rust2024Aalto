// Returning a Result from a function requires both Ok and Err to be explicitly defined. Below is an example in a program that mimics (poorly) a general artificial intelligence.
fn agi9000(question: &str) -> Result<String, String> {
    match question {
        "what is the meaning of life?" => Ok("42".to_string()),
        _ if question.ends_with("?") => Ok("probably yes".to_string()),
        _ => Err("this is not a question!".to_string()),
    }
}
 // Ok and Err is the one that we are checking using answer variable
fn ask_computer(question: &str) {
    println!("Asking the computer: {question}");
    let answer = agi9000(question);
    match answer {
        Ok(answer) => println!("Computer's answer: {answer}"),
        Err(message) => println!("ERROR: {message}"),
    };
}

fn main() {
    ask_computer("what is the meaning of life?");
    ask_computer("can I have a pet dragon?");
    ask_computer("all your codebase are belong to us.");
}


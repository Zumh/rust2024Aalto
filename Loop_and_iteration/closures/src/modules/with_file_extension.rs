// Capturing the enclosing scope also means that we can use variables from the enclosing scope in the body of the closure.
pub fn capturing_enclosing_variables() {
    println!("\n\ncapturing_enclosing_variables!");
    let base = "main";
    let with_file_extension = |extension| {
        let mut s = base.to_string();
        s.push_str(extension);
        s
    };
    println!("{}", with_file_extension(".rs"));
    println!("{}", with_file_extension(".py"));
}

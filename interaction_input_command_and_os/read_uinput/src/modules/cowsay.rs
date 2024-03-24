use std::env;
const COWSAY: &str = r#"
\  ^__^
 \ (oo)\_______
   (__)\       )\/\
       ||----w |
       ||     ||
"#;

fn prepend_spaces(s: &str, n: usize) -> String {
    let mut s = s.to_string();
    s.insert_str(0, &" ".repeat(n));
    s
}

fn read_line() -> String {
    //let mut line = String::new();
    //std::io::stdin().read_line(&mut line).unwrap();
    
    // this will return the args without the program name and turn it into string
    env::args().skip(1).collect::<Vec<String>>().join(" ")
}

/*
// model solution
 let message = std::env::args()
        .skip(1)
        .next()
        .unwrap_or_else(|| "Moo!".to_string());
*/
pub fn cow_say() {
    let mut message = read_line();
    if message.is_empty() {
        message.replace_range(.., "Moo!");
    }

    println!("< {} >", message);
    let indent = |s| prepend_spaces(s, message.len() + 4);
    let cow = COWSAY
        .trim()
        .split("\n")
        .map(indent)
        .collect::<Vec<String>>()
        .join("\n");
    println!("{cow}");
}

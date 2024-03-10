fn append(s: String, prefix: String) -> String {
    // the life of prefix is end here
    format!("{}{}", prefix, s)
}

fn main() {
    let love = "💕".to_owned();
    let greeting = "Hello".to_owned();
    let lovely_greeting = append(greeting.clone(), love);
    println!("{}", greeting.clone());
    println!("{lovely_greeting}");
    // cannot use `love` anymore due to moving
}
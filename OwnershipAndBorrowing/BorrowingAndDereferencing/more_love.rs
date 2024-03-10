fn more_love(s: &mut String) -> String {
    s.push_str("💕"); // mutates s
    s.repeat(3)       // creates a new String
}

fn main() {
    let mut love = "💕".to_owned();
    println!("{love}");
    let lots_of_love = more_love(&mut love); // borrow ends at the end of the function call
    println!("{love}");
    println!("{lots_of_love}");
}
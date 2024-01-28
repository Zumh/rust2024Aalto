fn word_count(s: &String) -> usize {
    s.split_whitespace().count()
}

fn main() {
    let text = String::from("I am a robot");
    let wc = word_count(&text);
    println!("The text \"{text}\" contains {wc} words.");
}

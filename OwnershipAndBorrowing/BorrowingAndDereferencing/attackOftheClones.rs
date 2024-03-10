fn attack_of_the_clone(string: &String) -> String {
    let mut clone = string.clone();
    clone.push_str(" clone");
    clone
}

fn main() {
    let string = String::from("Hello");
    let clone = attack_of_the_clone(&string);
    println!("original={string}, clone={clone}");
}

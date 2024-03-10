fn greet(string: String) {
    println!("Hello {string}!");

}

fn main() {
    let x = String::from("🙀");
    greet(x); // value of 'x' is moved into the function
    println!("{x}"); // we shouldn't be able to print herer
}
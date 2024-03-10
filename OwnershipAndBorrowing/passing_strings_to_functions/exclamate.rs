fn exclamate(string: String) -> String {
    format!("{string}!")
    // `format!` macro works like `println!` but returns a formatted `String` instead of printing it
}
fn main() {
    let x = String::from("😼");
    // value of `x` is moved into the function, then a new value returned by the function (created using `x`) is moved into `y`
    let y = exclamate(x); 
    println!("{y}");
}
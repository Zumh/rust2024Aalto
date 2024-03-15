// Similar to the print macros is the format! macro that writes formatted text to a string
fn main() {
    let x = Some(1);
    let variable = "x";
    let s = format!("{} = {:?}", variable, x);
    println!("{}", s);
}

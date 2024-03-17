// here we slice a string like an array
fn main() {
    let pizza = "quattro formaggi";
    let slice = &pizza[7..];
    println!("{}", slice);
}

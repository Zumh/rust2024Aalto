fn main() {
    let mut x = 100;
    let y = &mut x; // y is a reference to x
    *y = 300; // change the value stored in x
    println!("{x}"); // 300
}
fn main() {
    let mut x = 100;
    let y = &mut x; // y is a reference to x
    *y = 300; // change the value stored in x
    //let z = x + *y; // error: cannot use `x` because it was mutably borrowed
    let z = *y + x; // scope of ref x end with *y before reading x
    println!("{z}");
}

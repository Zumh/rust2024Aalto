// coercing arrays to slices.
#![allow(unused)]
fn main() {
    let one = [1];
    let two = [1, 2];
    let three = [1, 2, 3];
    let slices: [&[i32]; 3] = [&one[..], &two.as_slice(), &three];
    println!("{:?}", slices);
}


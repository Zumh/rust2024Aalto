// here we can use copy feature from array because of primitive types integer are the same.
//fn to_zeros(array: mut [i32]) {
fn to_zeros(array: &mut [i32]) {
    array.fill(0);
}
fn main() {
    let mut array = [1, 2, 3];
    // here array is copy
    println!("{:?}", array);

    // here array is borrowing mutable
    to_zeros(&mut array);
    println!("{:?}", array);
}


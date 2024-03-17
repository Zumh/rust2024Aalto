// print array that slices
fn print_slice(slice: &[i32]) {
    for i in 0..slice.len() {
        println!("slice[{}]: {}", i, slice[i]);
    }
}

fn main() {
    let slice = [1, 2, 3, 4, 5];
    print_slice(&slice[1..3]);
}

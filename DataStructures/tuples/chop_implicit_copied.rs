// unlike strings, integers are always copied implicitly
// this is a deliberate design choice from the standard library
fn main() {
    let number = 5;
    let chopped = chop(number);
    println!("chpping {} gives {:?}", number, chopped);
}

fn chop(number: i32) -> (i32, i32) {
    let first = number / 2;
    let second = number - first;
    (first, second)
}

// how to slice them like a pizza
// destructuring them
#![allow(unused)]
fn main() {
    let arr = [1, 2, 3, 4, 5];
    let slice = &arr[1..4];
    let first_twos_sum = match slice {
        [first, second, ..] => first + second,
        _ => 0,
    };
    println!("first two's sum: {first_twos_sum}");

    if let [_, _, third, ..] = slice {
        println!("third: {third}");
    }
}


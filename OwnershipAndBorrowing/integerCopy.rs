fn add_one(mut some_integer: i32) -> i32 {
    some_integer += 1;
    return some_integer;
}

fn main() {
    let mut x = 5;
    x = add_one(x);
    println!("{x}"); // ??
}

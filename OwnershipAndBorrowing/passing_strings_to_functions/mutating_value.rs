fn add_one(some_integer: i32) -> i32 {
    some_integer + 1
}

fn add_one_ref(x: &mut i32) {
    *x += 1;
}
fn main() {
    // copy value
    let mut x = 5;
    x = add_one(x);
    println!("{x}"); // ??

    // ref mutate
    let mut y = 9;
    add_one_ref(&mut y);
    println!("{y}");
}
// tuple is useful for printing multiple values at once from a function
fn main() {
    // destructure tuple into individual variables
    let (f32_result, f64_result) = divide(10, 20);
    println!("{} {}", f32_result, f64_result);
}

fn divide(x: i32, y: i32) -> (f32, f64) {
    (x as f32 / y as f32, x as f64 / y as f64)
}

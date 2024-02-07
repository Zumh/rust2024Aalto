// Implement the function multiplied_by_2() that takes two arguments and returns a tuple of the arguments multiplied by 2.
fn multiplied_by_2(a: i32, b: f32)->(i32, f32){
    
    return (a * 2, b*2.0);
}
fn main() {
    let (a, b) = multiplied_by_2(1, 2.0);
    println!("a={a}, b={b}");
}

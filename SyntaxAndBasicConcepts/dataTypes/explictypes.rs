/*
fn not_working() {
    let f1: f64 = 3.5;
    let f2 = 3.49;
    println!("{}", f1.round());
    println!("{}", f2.round()); // error: ambiguous numeric type `{float}`
}*/

fn working_type() {
    let f1: f64 = 3.5;
    let f2 = 3.49f64;
    println!("{}", f1.round());
    println!("{}", f2.round()); // error: ambiguous numeric type `{float}`
}
fn main() {
    //not_working();
    working_type();
}
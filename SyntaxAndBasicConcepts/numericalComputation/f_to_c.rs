// Implement the function f_to_c() that converts a temperature in Fahrenheit to Celsius.
fn f_to_c(fahrenheit: f64)->f64{
    return (fahrenheit - 32.0)*(5.0/9.0);

}
fn main() {
    println!("{}", f_to_c(32.0));
    println!("{}", f_to_c(41.0));
    println!("{}", f_to_c(0.0));
    println!("{}", f_to_c(100.0));
}

const GLOBE:&str = "🌎";
/*
    Constants can only be set to constant expressions, i.e. values that can be evaluated at compile-time. Immutable variables can be set to any value, including values known only at runtime.
    Constants must be annotated with their data type. We can let the compiler infer the type of an immutable variable.
    Constants can be declared anywhere, including outside functions aka globally. Immutable variables can only be declared in functions.
    Static is not use a lot because constant is a better choice.
*/
fn main() {
    println!("{GLOBE}");
}

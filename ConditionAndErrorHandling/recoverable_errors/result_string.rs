// Working with Results is similar to working with Options. We can use multiple methods to work with Results, such as unwrap (may panic), except (may panic), unwrap_or, is_ok, is_err, etc. Here, as well as for other types, the exhaustive list can be found in Rust's documentation.
fn main() {
    let result: Result<&str, ()> = Ok("🦀"); // The error type is unit, needs explicit type for compiler
    let error = Err("💥"); // No explicit type, both ok and err variant types can inferred from usage
 
    println!("{}, {}", result.unwrap(), error.unwrap_or("🦀"));

    if result.is_ok() {
        println!("The result seems ok");
    }
    if error.is_err() {
        println!("The result seems not ok");
        error.expect("💥💥");
    }
}


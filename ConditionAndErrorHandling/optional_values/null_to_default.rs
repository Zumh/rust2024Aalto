// how to handle panic when a value is null
fn main(){
    let mut maybe_a_value = Some("value");
    println!("{}", maybe_a_value.unwrap_or("default value"));
    maybe_a_value = None;
    println!("{}", maybe_a_value.unwrap_or("null"));

    // Another useful method is expect, which causes panic like unwrap, but also allows us to specify a custom error message to be shown after the panic.
    let bob: Option<&str> = None;
    println!("{}", bob.expect("no value"));
}

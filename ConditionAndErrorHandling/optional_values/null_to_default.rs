// how to handle panic when a value is null
fn main(){
    let mut maybe_a_value = Some("value");
    println!("{}", maybe_a_value.unwrap_or("default value"));
    maybe_a_value = None;
    println!("{}", maybe_a_value.unwrap_or("null"));
}

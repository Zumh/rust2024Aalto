// if let syntax some option
// The if let syntax takes the form if let $pattern = $expression. If the lefthand pattern matches the righthand expression, Rust can safely unwrap the expression some_value into value. The same logic works for all enums in Rust.
fn main() {
    let some_text = Some("text");

    if let None = some_text {
        println!("The value is None"); // no match, no execution
    }

    if let Some("other") = some_text {
        println!("The value is other"); // no match, no execution
    }

    if let Some("text") = some_text {
        println!("The value is text"); // match, therefore execution
    }
    else if let Some(value) = some_text {
        println!("The value is {value}"); // match on previous branch, no execution
    }
}


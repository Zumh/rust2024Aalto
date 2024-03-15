// add dbg! feature here
// option is use debugging and error handling
fn main(){
    let number = 5;
    let roman = number_to_roman(number);
    println!("{:?}", roman);

    // if the string is empty
    let empty_string = String::new();
    println!("{empty_string:?}");

    // gdb!(number_to_roman(number)); is another way to debug
    let number = Some(5);
    // The macro works by using the Debug implementation of the type of 
    // the given expression to print the value to stderr along
    // with the source location of the macro invocation as well as the source code of the expression.
    dbg!(number);
}

fn number_to_roman(number: usize) -> Option<String> {
    match number {
        1..=3 => Some("I".repeat(number)),
        4 | 9 => Some("I".to_string() + &number_to_roman(number + 1).unwrap_or_default()),
        5..=8 => Some("V".to_string() + &number_to_roman(number - 5).unwrap_or_default()),
        10..=39 => Some("X".to_string() + &number_to_roman(number - 10).unwrap_or_default()),
        _ => None
            
        
    }
}   

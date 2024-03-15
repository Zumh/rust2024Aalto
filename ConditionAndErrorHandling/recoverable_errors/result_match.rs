// As with Option variants, we can use the match expression to pattern match the Ok and Err variants and destructure the values inside. Likewise, the if let expression can be used to handle the Ok and Err variants.
fn main() {
    let mut result: Result<(), String> = Ok(());
    match result {
        Ok(value) => println!("The value is {value:?}"),
        Err(error) => println!("The error is {error}"),
    }

    result = Err("💥".to_string());

    if let Ok(value) = result {
        println!("The value is {value:?}");
    }
    else if let Err(error) = result {
        println!("The error is {error}");
    }
}


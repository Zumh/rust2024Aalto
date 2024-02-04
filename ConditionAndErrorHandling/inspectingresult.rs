fn inspect(result: Result<(), String>) {
    match result {
        Ok(()) => println!("All ok!"),
        Err(e) => println!("Error: {e}"),
    }
}

fn main() {
    inspect(Ok(()));
    inspect(Err("Something went wrong!".to_owned()));
}

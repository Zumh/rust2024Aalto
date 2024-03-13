fn is_valid_password(password: &str) -> bool {
    /*1. the password contains at least one number
2. the password contains at least one lowercase letter
3. the password contains at least one uppercase letter
4. the length of the password is 10 or greater
Otherwise, the function should return false.*/
    password.contains(char::is_numeric)
    &&
    password.contains(char::is_uppercase)
    &&
    password.contains(char::is_lowercase)
    &&
    password.len() >= 10
    
    
    
    
}

fn main() {
    println!("{:?}", is_valid_password("no_uppercase_chars0"));
    println!("{:?}", is_valid_password("NO_LOWERCASE_CHARS0"));
    println!("{:?}", is_valid_password("noNumbersHere:("));
    println!("{:?}", is_valid_password("Short0"));
    println!("{:?}", is_valid_password("shouldBeS4fE"));
}


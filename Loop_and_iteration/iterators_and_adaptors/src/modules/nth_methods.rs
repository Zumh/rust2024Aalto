/*
The nth method does not consume the iterator fully, but only until the nth value. This means that we can use the iterator again after calling nth. 
We need to be careful though, as the iterator will continue from where we left off.
*/

pub fn nth_methods() {
    let philosopher = "Ἀριστοτέλης";
    let third = philosopher
        .chars()
        .nth(2)
        .unwrap();
    let number_of_chars = philosopher
        .chars()
        .count();
    println!("The length of {philosopher} is {}", philosopher.len()); // length of u8 bytes
    println!("The number of chars in {philosopher} is {number_of_chars}"); // number of chars
    println!("The third character of {philosopher} is {third}.");

}

// just like string, array must be clone before use
// for ownership of array, use slice
fn main() {
    let cardinals = [
        "blue grosbeak".to_string(),
        "blackbird".to_string(),
        "cricket".to_string(),
        "crane".to_string(),
    ];
    //let second_bird = &cardinals[1];
    let second_bird = cardinals[1].clone();
    println!("{}", second_bird);
}

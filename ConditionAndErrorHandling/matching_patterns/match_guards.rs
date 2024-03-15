// match guards with if keyword and condition after pattern in a match arm.
fn main() {
    let x = 3;
    let polarity = match x {
        0 => "neutra",
        _ if x > 0 => "positive",
        _ if x < 0 => "negative",
        _ => "zero"
    };
    println!("{}", polarity);
}


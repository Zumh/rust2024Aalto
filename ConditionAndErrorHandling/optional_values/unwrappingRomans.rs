fn roman_numeral(num: usize) -> Option<String> {
    match num {
        1..=3 => Some("I".repeat(num)),
        4 | 9 => Some("I".to_string() + &roman_numeral(num + 1).unwrap_or_default()),
        5..=8 => Some("V".to_string() + &roman_numeral(num - 5).unwrap_or_default()),
        10..=39 => Some("X".to_string() + &roman_numeral(num - 10).unwrap_or_default()),
        _ => None,
    }
}

fn main() {
    let roman10 = roman_numeral(10).unwrap();
    println!("{roman10}");
    let roman0 = roman_numeral(0).unwrap();
    println!("{roman0}");
}

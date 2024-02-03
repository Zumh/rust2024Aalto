pub fn roman_numeral(num: usize) -> String {
    match num {
        1..=3 => "I".repeat(num),
        4 | 9 => "I".to_string() + &roman_numeral(num + 1),
        5 => "V".to_string(),
        6..=8 => "V".to_string() + &roman_numeral(num - 5),
        10 => "X".to_string(),
        10..=39 => "X".to_string() + &roman_numeral(num - 10),
        _ => panic!("Invalid number '{num}', only values from 1 to 39 are supported!"),
    }
}

fn main() {
    println!("{}", roman_numeral(24));
    println!("{}", roman_numeral(0));
}

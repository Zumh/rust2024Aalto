
fn roman_numeral(num: usize) -> String{
    match num {
        1..=3 => "I".repeat(num),
        4 | 9 => "I".to_string() + &roman_numeral(num + 1),
        5..=8 => "V".to_string() + &roman_numeral(num - 5),
        10..=39 => "X".to_string() + &roman_numeral(num - 10),
        _=> String::new(), // underscore is like else with if 
    }
}

fn main(){
    let input_number: usize = 9;
    let number: String = roman_numeral(input_number);
    println!("{number}")
}

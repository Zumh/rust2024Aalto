// we can do range matching for number between or including
fn main() {
    let number = 2;

    let polarity = match number{
        
        ..=-1 => "negatively!",
        0 => "neutral!",
        1..=i32::MAX => "positive!",
    };

    println!("{polarity}");
}

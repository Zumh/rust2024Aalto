
fn main(){
    let polarity = if x > 0 {
        "positive";
    } else if x < 0 {
        "negative";
    } else {
        "neutral";
    };
    println!("{x} is {polarity}");
}
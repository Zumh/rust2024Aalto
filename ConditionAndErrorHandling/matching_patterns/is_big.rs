// key word match "big"
// if not match the length greater than 10
fn main() {
    println!("{}", is_big("big"));
    println!("{}", is_big("small"));
    println!("{}", is_big("a very long text"));
}

fn is_big(text: &str) -> bool {
   let len: usize =  match text {
        "big" => return true,
        text => {
            let trimmed = text.trim();
            trimmed.len()
        },
    };

    len > 10
}

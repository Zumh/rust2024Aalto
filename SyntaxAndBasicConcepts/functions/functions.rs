// the unit type function
fn explicit_nothing() -> () {
    "this &str goies nowhere";
}

fn main(){
    println!("{:?}", explicit_nothing())
}
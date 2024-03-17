fn main(){
    // convert bytes to string
    let hello = [104, 101, 108, 108, 111, 32, 240, 159, 140, 141, 33];
    println!("{:?}", std::str::from_utf8(&hello));
    // here we have invalid utf8 sequence
    let not_utf8 = [104, 101, 108, 108, 111, 32, 255, 33];
    println!("{:?}", std::str::from_utf8(&not_utf8));
}

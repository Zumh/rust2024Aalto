fn main() {
    // string types
    let s = "I hAte tHis sTyLe Of WrItInG";

    println!("{}",s.to_lowercase());
    //println!("{s.to_lowercase()}"); // doesn't work because of s.to... need to compile the string literal first

    // replace every t with j
    println!("{}", "testing".replace("t", "j"));

    // replace first n matches
    println!("{}", "testing".replacen("t", "j", 1)); 
}

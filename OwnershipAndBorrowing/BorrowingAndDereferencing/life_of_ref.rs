fn main() {
    let mut book = String::from("📕");
    let borrowed_book = &book; // borrows `book` 
    println!("{borrowed_book}");
    book = String::from("📘");
     // error: cannot assign to `book` because it is borrowed
     //   println!("{borrowed_book}");
    println!("{book}");
}
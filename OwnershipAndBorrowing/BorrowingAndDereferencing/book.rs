fn read_book(book: &String) {
    println!("reading {book} ... 📖 ... {book}");
}

fn main() {
    let book = String::from("📕");
    let borrowed_book = &book; // borrows `book` 
    read_book(borrowed_book);
    println!("{book}"); // All good!
}
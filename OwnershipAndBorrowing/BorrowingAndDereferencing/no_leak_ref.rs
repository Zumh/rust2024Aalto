fn main() {
    let book = {
        let green_book = String::from("📗");
        &green_book // return a reference to `green_book`
    }; // `green_book` drops here

    println!("{book}"); // does not live long enough
}
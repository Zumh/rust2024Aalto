pub fn only_borrow_no_take(string: &mut String){
    string.insert(0, '🦀');
    string.push('🦀');
    // borrowing here at the end of this function scope
    //string
}

fn main() {
    let mut string = String::from("testing");
    only_borrow_no_take(&mut string);
    println!("{string}");
}

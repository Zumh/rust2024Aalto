fn main() {
    println!("Copy\n");
    // copied 
    copy();

    println!("Moving\n");
    // move
    moving();
}
fn moving() {
    let t = ("1".to_string(), "2".to_string());
    let t2 = t.clone();
    println!("{:#?} {:#?}", t, t2);
}
fn copy() {
    // move
    let t = (1, 2);
    let t2 = t;
    println!("{:#?} {:#?}", t, t2);
}

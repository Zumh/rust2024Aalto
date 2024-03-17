// accessing an index out of bounds of an array return None if not exists.
fn main() {
    let a = [1, 2, 3];
    let x: Option<&i32> = a.get(3);
    println!("{:?}", x);
    let y: Option<&i32> = a.get(1);
    println!("{:?}", y);
}

fn sort_descending(a: &mut [i16; 5]) {
    a.sort();
    a.reverse();
}

fn main() {
    let mut a = [10i16, 30, -11, 20, 4];
    sort_descending(&mut a);
    println!("{:?}", a);
}

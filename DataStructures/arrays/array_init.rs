// arrays can be initialized with an array literal
// usize is the indexing type for arrays
fn main() {
    let a = [1, 2, 3, 4, 5];
    println!("{:?}", a);

    let b = [1; 5];
    println!("{:?}", b);

    // destructuring array
    let [c, d, e, f, g] = a;
    println!("{:?} {:?} {:?} {:?} {:?}", c, d, e, f, g);

    destructure_array();
    // matching destructuring
    
    // initializing array
    let array: [u32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);
}

fn destructure_array() {
    let strings = ["a", "b", "c"];
    let [a, b, c] = strings;
    println!("[{a}, {b}, {c}]"); // [a, b, c]
    match strings {
        ["a", _, _] => println!("starts with a!"),
        _ => println!("no match"),
    }

}

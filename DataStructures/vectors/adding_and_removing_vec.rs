// push and append method will be explore here
// get, contains, join, sort work for vectors.

fn main() {
    println!("Adding vectors!");
    let mut v: Vec<i32> = Vec::new(); 
    println!("{}", v.len());

    v.push(1);
    v.push(2);
    println!("{}", v.len()); // 2
    println!("{v:?}"); // [1, 2]
    

    let mut v2 = vec![1, 2, 3];

    v2.append(&mut v);
    println!("{}", v2.len()); // 3
    println!("{v:?}");
    println!("{v2:?}"); // [1, 2, 3]
    println!("\n\nremoving"); 
    removing();
    // vector doesn't have remove method directly
}

fn removing() {
    let mut v = vec![1, 2, 3, 4, 5];
    println!("{v:?}"); // [1, 2, 3, 4, 5]
    v.remove(0);
    println!("{v:?}"); // [2, 3, 4, 5]
    let popped = v.pop();
    println!("pooped: {popped:?}, remaning: {v:?}"); // pooped: 5 {v:?}"); // [2, 3, 4]
}

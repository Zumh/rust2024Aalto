pub fn difference_in_iter_into_iter() {
    let vec = vec![1, 2, 3, 4, 5];

    println!("Using iter() borrowing");
    // Using iter() borrowing
    for &x in vec.iter() {
        println!("{}", x);
    }

    println!("\nUsing into_iter() moving or copying");
    // Using into_iter()
    for x in vec.into_iter() {
        println!("{}", x);
    }
    println!("Trying to use vec here would cause a compilation error because it has been moved into the iterator.");
}


  /*
  The nth method does not consume the iterator fully, 
  but only until the nth value. This means that we can use the iterator again after calling nth. We need to be careful though, 
  as the iterator will continue from where we left off.
   */
pub fn nth_number() {
    let numbers = vec![10, 20, 30, 40, 50];
    println!("numbers: {:#?}", numbers);
    // Retrieve the element at index 2
    let third_element = numbers.iter().nth(2);

    match third_element {
        Some(&value) => println!("The element at index 2 is: {}", value),
        None => println!("Index is out of bounds."),
    }
}

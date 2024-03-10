fn add_one(value: i32) -> i32 { // Use i32 for integer type
    value + 1
  }
  
  fn add_to_list(list: &mut Vec<i32>, value: i32) {
    list.push(value); // Use push method to add element
  }
  
  fn main() {
    let x = 5; // Use mut keyword for mutable variable
    let _new_x = add_one(x); // Assign returned value
    println!("Value of x is {}", x); // Use println! for formatted output (5)
  
    let mut list = vec![1, 2, 3]; // Use vec! macro for creating vectors
    add_to_list(&mut list, 4);
    println!("List is {:?}", list); // Use {:?} for debug formatting ([1, 2, 3, 4])
  }
  
/*

A closure can capture variables in three ways: immutable borrow, mutable borrow, or by move (take ownership). 
The compiler infers which one to use based on the closure body. 
Imagine a function that decides automatically whether it should take a mutable or 
immutable reference to a variable based on the body of the function.
*/

pub fn capture_by_borrwing() {
    println!("\n\ncapture_by_borrwing!");
    let mut i = 2i32;
    let double_i = |j: &mut i32| *j = *j * 2; // capture i by borrowing mutably
    double_i(&mut i); // use mutable borrow 
    println!("i = {}", i); // borrow immutably
    double_i(&mut i); // use mutable borrow again (not allowed)

}

pub fn just_variables() {
    println!("\n\njust_variables! same as capture_by_borrwing");
 
    let mut i = 2;
    let j = &mut i;  // borrow i mutably
    *j = *j * 2;     // use mutable borrow
    println!("{i}");   // borrow i immutably
    let j = &mut i;  // borrow i mutably
    *j = *j * 2;     // use a different mutable borrow (completely fine)
}

pub fn moved_ownership() {
    println!("\n\nmoved_ownership!");
    let data = vec![1, 2, 3];
    // capture data by moving ownership to a closure variable
    let closure = move || println!("captured {data:?} by value");
    closure();
}
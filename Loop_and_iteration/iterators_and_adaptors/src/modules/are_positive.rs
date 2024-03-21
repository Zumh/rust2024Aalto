pub fn check_positive(){
    let numbers = [1, -2, 3, -4, 5, -6, 7, -8, 9, -10];
    let positive_numbers = are_positive(&numbers);
    println!("positive numbers: {:?}", positive_numbers);
}


/*
In case we need owned values, but into_iter returns references, 
we can use the copied or cloned method (depending on the type) to get owned copies of the values 
— we could of course alternatively use map with a function that calls to_owned() on its parameter.
 */
fn are_positive(numbers: &[i32]) -> Vec<bool> {
    numbers
    .into_iter()
    .cloned()
    .map(i32::is_positive)
    .collect()
}
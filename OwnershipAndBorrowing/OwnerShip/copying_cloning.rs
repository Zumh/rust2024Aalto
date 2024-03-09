
// copying
fn main(){
    let mut variable = 5;
    let new_variable = variable; // assign the copy of the value to `variable`
    
    variable = 6;
    println!("old: {variable}, new: {new_variable}"); // this is fine because of the implicit copy

    let x = String::from("Each value can have only one owner at a time");
    let _y = x.clone(); // assign the cloned value to y
    println!("{x}"); // this is ok because the value of 'x' was cloned and not moved
}
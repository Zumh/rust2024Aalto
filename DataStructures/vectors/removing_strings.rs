/*
 *Write a function remove_string that takes a mutable reference to a vector of Strings and an index. If the string at a given index is safe to remove, the function should remove the string from the vector and return it in an Option. Otherwise, the function should return None
 * */
// Write the function `removing_string`
fn main() {
    let mut bears = vec![
           "🐻".to_string(),
        "🐨".to_string(),
        "🐼".to_string(),
        "🐾".to_string(),
    ];
    // if we want a string  or clean return use this technqiue
    let not_bears = removing_string(&mut bears, 1).unwrap_or_default();

    // if you want Some("") use this
    //let not_bears = removing_string(&mut bears, 1);
    // not_bears should return String if not None
   
    // if you want Some("") use this
    //println!("{:?}", not_bears);
    println!("{not_bears:?}");

    println!("{:?}", bears);
}

fn removing_string(vec: &mut Vec<String>, index: usize) -> Option<String> {
    // return None if index is out of bounds
    if index >= vec.len(){
        return None;
    }
    //Option::from(vec.remove(index))
    Some(vec.remove(index))
}

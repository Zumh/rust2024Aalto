
fn main() {
    // cardinals 
    let cardinals = ( "one", 2, 3, 4, 5, 6, 7, 8, 9, 10);
    //let num_elements = std::mem::size_of_val(&cardinals) / std::mem::size_of::<&str>();
    for i in cardinals.0..cardinals.8 {
        // check if a string type exists
        if std::mem::size_of::<&str>() == 0 {
            // Print the value of `i` in the current iteration
            println!("{} is an invalid type", i);
        } else {
            // Print the value of `i` in the current iteration
            println!("{} is a valid type", i);
        }
    }
    // Define a nested tuple named `tuple_ception`

    let tuple_ception = (
        "one",        // String literal "one"
        2,              // Integer literal 2
        ("Yo", "dawg",  // Inner tuple with strings
         "I", "am",
         "the", "one",
         "and", "only",
         "in", "the",
         "tuple",
         ("and", "I", "am", "the", "one", "and", "only", "in", "the", "tuple"),        
         ),

   );

    // Print the contents of `tuple_ception`
    println!("{:?}", tuple_ception);
}


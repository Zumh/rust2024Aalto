
fn main() {
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


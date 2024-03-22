pub fn additions() {
    println!("\n\n## additions");
    let add = |a, b| a + b;
    println!("1 + 1 = {}", add(1, 1));
}

pub fn additions2() {
    println!("\n\n## Different types of closures");
    // take two integers and return their sum
    fn  add(a: i32, b: i32) -> i32 { a + b } // Obligatory type annotations and braces
    println!("fn  add(a: i32, b: i32) -> i32 {{ a + b }} > 1 + 2 = {}", add(1, 2));
    
    // let add = |a: i32, b: i32| -> i32 { a + b }; // Optional type annotations
    println!("let add = |a: i32, b: i32| -> i32 {{ a + b }}; > 1 + 3 = {}", add(1, 3));
    
    // let add = |a: i32, b: i32| { a + b };
    println!("let add = |a, b| -> i32 {{ a + b }}; > 1 + 4 = {}", add(1, 4)); 
    
    // let add = |a: i32, b: i32| a + b  ;
    println!("let add = |a, b| a + b > 1 + 5 = {}", add(1, 5));
}   

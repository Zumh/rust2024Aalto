fn main() {
    let mut person1 = "Estelle";
    let mut person2 = "Joshua";
    {
        let person3 = "Lloyd";
        println!("{}, {} and {}", person1, person2, person3);
    }
    person3 = "Elie";
    println!("{}, {} and {}", person1, person2, person3);
}

/*
error[E0425]: cannot find value `person3` in this scope
 --> src/main.rs:8:5
  |
8 |     person3 = "Elie";
  |     ^^^^^^^ help: a local variable with a similar name exists: `person1`"
*/

fn main() {
    let mut person1 = "Estelle";
    let mut person2 = "Joshua";
    {
        person1 = "Kloe";
        println!("{} and {}", person1, person2);
    }
    person2 = "Kloe";
    println!("{} and {}", person1, person2);
}

/*
Kloe and Joshua
Kloe and Kloe
*/

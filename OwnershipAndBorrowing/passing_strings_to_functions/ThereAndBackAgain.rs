fn visit_lonely_mountain(hobbit: String)->String {
    println!("{} reached the lonely mountain!", hobbit);
    return hobbit;
}

fn main() {
    let hobbit = String::from("Bilbo");
    println!("{hobbit} is going on an adventure.");
    let hobbit = visit_lonely_mountain(hobbit);
    println!("{hobbit} returned safely.");
}

fn print_speed(mph: u32) {
    if mph == 88 {
        println!("Ah! What did I tell you? 88 miles per hour!")
    } else {
        println!("{mph} miles per hour!");

    }
}

fn main() {
    let mph = 88;
    print_speed(mph);
}


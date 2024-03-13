fn second(t: (u8, u16, u32)) -> u16 {
    // Return the second element of the tuple

    return t.1

}

fn main() {
    let second = second((1, 2, 3));
    println!("{second}");
}

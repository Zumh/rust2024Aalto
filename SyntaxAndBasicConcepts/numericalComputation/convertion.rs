fn main() {
    println!("{}", 5i32 as f32); // 5.0
    println!("{}", 1.5f32 as u8); // truncated to 1
    println!("{}", 1.5f32.round() as u8); // 2
    println!("{}", 65u8 as char); // 'A'
    println!("{}", 128u8 as i8); // -128 since 127 is the largest value of i8
    println!("{}", 'a' as i32); // 97


}

fn main() {
    let bowstring: String = "🏹".to_string();
    let mut string_materials: String = String::from("🧶");
    string_materials.push_str("🧵"); // push_str appends a string literal to a String
    println!("{string_materials}{bowstring}");
}
// check if the parent start iwth the start in string slice
// In the first call to starts_with, we pass a &str to it, but in the second it looks like we pass in a &String. Strings coerce to str slices, so &start coerces from a &String to a &str.
fn starts_with(start: &str, parent: &str) -> bool {
    if start.len() > parent.len() {
        false
    } else {
        &parent[0..start.len()] == start
    }
}

fn main() {
    let s = "Rust";
    let start = "R";

    println!("{s} starts with '{}': {}", start, starts_with(start, s));

    let start: String = "Rust is iron oxide".to_string();
    println!("{s} starts with '{}': {}", start, starts_with(&start, s));
}

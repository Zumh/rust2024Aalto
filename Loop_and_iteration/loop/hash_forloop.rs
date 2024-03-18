use std::collections::HashMap;

fn main() {
    let map = HashMap::from([
          ("bear", "🐻"),
        ("wolf", "🐺"),
        ("fox", "🦊"),
    ]);

    for (k, v) in map {
        println!(":{}: -> {}", k, v);
    }
}

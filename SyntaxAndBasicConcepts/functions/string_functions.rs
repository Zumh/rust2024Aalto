/*
 * passing variable to a function need explicit data type
 */
fn simon_says(command: &str) {
    println!("Simon says {}", command);
}

fn main() {
    let cmd = "learn Rust";
    simon_says(cmd);
}

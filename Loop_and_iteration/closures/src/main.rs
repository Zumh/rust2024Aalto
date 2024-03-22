
mod modules;
use modules::{
    additions::{additions, additions2},
    with_file_extension::{capturing_enclosing_variables},
    closures_ownership::{capture_by_borrwing, just_variables, moved_ownership},
};

// celsius to fahrenheit
fn main() {
    println!("Closures!");
    additions();
    additions2();

    capturing_enclosing_variables();
    capture_by_borrwing();
    just_variables();
    moved_ownership();
}

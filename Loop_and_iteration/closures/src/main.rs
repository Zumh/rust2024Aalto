
mod modules;
use modules::{
    additions::{additions, additions2},
    with_file_extension::{capturing_enclosing_variables},
    closures_ownership::{capture_by_borrwing, just_variables, moved_ownership},
    iter_mut_c_to_fahren::{iter_mut_c_to_fahren, wrap_print_macro},
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

    // Iterator methods and closures
    iter_mut_c_to_fahren();
    wrap_print_macro();
}

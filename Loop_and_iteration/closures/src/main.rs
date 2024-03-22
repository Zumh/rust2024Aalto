
mod modules;
use modules::{
    additions::{additions, additions2},
    with_file_extension::{capturing_enclosing_variables},
    closures_ownership::{capture_by_borrwing, just_variables, moved_ownership},
    iter_mut_c_to_fahren::{iter_mut_c_to_fahren, wrap_print_macro, zip_miles_to_kms},
    filiter_and_finding::{filter_temperature, 
        filter_country_populations, 
        find_first_match,
        iterator_zoo
    },
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
    zip_miles_to_kms();

    // filter and finding
    filter_temperature();
    filter_country_populations();
    // find first match
    find_first_match();

    // iterator zoo
    iterator_zoo();
}


mod modules;
use modules::{
    for_each_celsius_to_fahrenheit::c_to_f_temp,
    into_collection_celsius_to_fahrenheit::c_to_f_map,
    consuming_iterators_miles_to_km::miles_to_kilometers_map,
    generic_function::power_up_generic,
    generic_absolute_numbers::absolute_numbers,
    are_positive::check_positive,
    iter_m_to_k_count_sum::iter_m_to_k_count_sum,
    difference_in_iter_into_iter::difference_in_iter_into_iter,
    nth_number_array::nth_number,
    nth_methods::nth_methods,
    iterate_one_at_a_time::iterate_one_at_a_time
};

// celsius to fahrenheit
fn main() {
    println!("## Iterator and Adaptors!");
    println!("## iter_mut Celsius to Fahrenheit");
    c_to_f_temp();

    println!("\n\n## map Celsius to Fahrenheit");
    c_to_f_map();

    println!("\n\n## map Miles to Kilometers");
    miles_to_kilometers_map();
 
    println!("\n\n## power up");
    power_up_generic();

    println!("\n\n## absolute numbers");
    absolute_numbers();

    println!("\n\n## are positive");
    check_positive();

    println!("\n\n## miles to kilometers");
    iter_m_to_k_count_sum();

    println!("\n\n## difference in iter and into iter");
    difference_in_iter_into_iter();

    println!("\n\n## nth number");
    nth_number();

    println!("\n\n## nth methods");
    nth_methods();

    println!("\n\n## iterate one at a time");
    iterate_one_at_a_time();
}

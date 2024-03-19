//mod modules;
mod modules;
use modules::for_each_celsius_to_fahrenheit::c_to_f_temp;
use modules::into_collection_celsius_to_fahrenheit::c_to_f_map;
use modules::consuming_iterators_miles_to_km::miles_to_kilometers_map;

// celsius to fahrenheit
fn main() {
    println!("## Iterator and Adaptors!");
    println!("## iter_mut Celsius to Fahrenheit");
    c_to_f_temp();

    println!("\n\n## map Celsius to Fahrenheit");
    c_to_f_map();

    println!("\n\n## map Miles to Kilometers");
    miles_to_kilometers_map();
 
}

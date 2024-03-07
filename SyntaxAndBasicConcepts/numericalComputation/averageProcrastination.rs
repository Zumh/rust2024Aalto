fn hours_spent_procrastinating(years: u32) -> u32 {
    let days_in_a_year: f64 = 365.25; //days in a year
    let hours_of_procastination_per_day: f64 = 1.62; // hours of online procrastination per day on average
    
    return (years as f64 * days_in_a_year * hours_of_procastination_per_day).round() as u32;
    
}

fn main() {
    println!("{}", hours_spent_procrastinating(5));
}

// Convert the miles_to_kilometers function to modify the given slice in place.
fn miles_to_kilometers(miles: &mut [f64]) {
    for mile in miles.iter_mut() {
        *mile = (*mile) * 1.609344;
    }
}

// Modify the the main function to call the new miles_to_kilometers function with a mutable reference.
fn main() {
    let mut distances = [1.0f64, 3.0, 10.0, 100.0]; // in miles
    println!("Distances in miles: {distances:?}");
    // convert to kilometers
    miles_to_kilometers(&mut distances);
    println!("Distances in kilometers: {distances:?}");
}


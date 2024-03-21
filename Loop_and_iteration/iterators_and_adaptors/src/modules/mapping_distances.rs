// convert miles to kilometers
// takes an immutable iterator miles and maps each element to its value multiplied by
// 1.609344. The ressult is collected into a new collection and returned.
// In other words, it converts miles to kilometers.
fn miles_to_kilometers(distances: &[f64]) -> Vec<f64> {

    // model solution
    // distances.into_iter().copied().map(miles_as_kilometers).collect()
    distances.iter().map(|distance: &f64| miles_as_kilometers(*distance)).collect::<Vec<f64>>()
    
}

fn miles_as_kilometers(miles: f64) -> f64 {
    miles * 1.609344
}


pub fn mapping_distances() {
    let distances = [1.0f64, 3.0, 10.0, 100.0]; // in miles
    println!("Distances in miles: {:?}", distances);
    // convert to kilometers
    let distances: Vec<f64> = miles_to_kilometers(&distances);
    //let distances: Vec<f64>    = distances.iter().map(|miles: &f64| miles_as_kilometers(*miles)).collect::<Vec<f64>>();
    println!("Distances in kilometers: {:?}", distances);
}

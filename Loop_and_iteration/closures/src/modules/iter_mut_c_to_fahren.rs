fn celsius_to_fahrenheit(f: f64) -> f64 {
    f * 9.0 / 5.0 + 32.0
}

pub fn iter_mut_c_to_fahren() {
    println!("\n\niter_mut_c_to_fahren");
    let mut temps = vec![0.0, 10.0, 20.0, 30.0];
    temps.iter_mut().for_each(|temp| {
        *temp = celsius_to_fahrenheit(*temp);
    });
    // the original vector is mutated using iter_mut and closure
    println!("{:?}", temps);
}


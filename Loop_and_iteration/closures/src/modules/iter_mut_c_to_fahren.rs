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

pub fn wrap_print_macro() {
    println!("\n\nwrap_pring_macro and we are using borrowing for conversion");
    let temps = vec![0.0, 10.0, 20.0, 30.0];
    println!("{:?}", temps);
    temps.iter().for_each(|temp| {
        println!("{}°C is {}°F", temp, celsius_to_fahrenheit(*temp));
    }); 

}

pub fn zip_miles_to_kms() {
        println!("\n\nzip_miles_to_kms");
        // iterating over two iterators and print them converting miles to kilometers
        let miles = [1.0f64, 3.0, 10.0, 100.0];
        let kilometers = miles.iter().map(|m| m * 1.609).collect::<Vec<_>>();
        miles
            .iter()
            .zip(kilometers.iter())
            .for_each(|(m, k)| println!("{m} miles equals {k} kilometers"));

}    


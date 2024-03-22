pub fn filter_temperature(){
        println!("\n\nfilter_temperature");
        let temps = [1.0f64, 3.0, 10.0, 100.0];
        // closure takes a double reference: &&f64 so we need to dereference it twice with ** to get the actual value.
        // filter adaptor does not need ownership of the values it is filtering, in this case the values have type &f64.
        let low_temps = temps.iter().filter(|temp| **temp < 15.0);
        low_temps.for_each(|low_temp| {
            println!("It's {low_temp}°C. So cold!");
        });
    
}

pub fn filter_country_populations() {
    use std::collections::HashMap;
    println!("\n\nfilter_country_populations");
    let country_populations = HashMap::from([
        ("Finland", 5_500_000),
        ("Estonia", 1_300_000),
        ("Sweden", 10_200_000),
        ("Norway", 5_300_000),
        ("Denmark", 5_800_000),
        ("Iceland", 400_000),
    ]);

    // we can destructure the key and value in the closure because Hash map have tuples as keys
    let over_5mil = country_populations.iter()
        .filter(|(_, population)| **population >= 5_000_000);
    for (country, population) in over_5mil {
        println!("{country} has population {population}");
    }
}



pub fn find_first_match(){
    // this code removes an element from a vector based on a condition, in this case if the element is less than 15.0
    // first we retain the elements that match the condition, retain only keeps the elements that the closure returns true for
    // then we use the position method to find the index of the first element that matches the condition, if no element matches the condition, position will return None
    // if the index is found, we remove the element at that index from the vector, if the index is not found, nothing happens
    // the final println! shows the result of the filtering, it should be [1.0, 10.0]
    println!("\n\nfind_first_match");

    let mut temps = vec![1.0f64, 3.0, 10.0, 100.0];
    temps.retain(|temp| *temp < 15.0);
    println!("{temps:?}"); // [1.0, 3.0, 10.0]

    if let Some(index) = temps
        .iter()
        .position(|temp| (*temp - 3.0).abs() < f64::EPSILON)
    {
        temps.remove(index);
    }
    println!("{temps:?}"); // [1.0, 10.0]
}

pub fn iterator_zoo() {
    //  reduce, fold, take_while, flatten, max_by_key.
    println!("\n\niterator_zoo");

        let a = [1, 5, 3, 2];
    
        // Compute max value using reduce
        let max = a.iter().reduce(|a, b| if a >= b { a } else { b });
        assert_eq!(max, Some(&5));
        println!("{max:?}");
    
        // Compute product using fold
        // calculates the product of all elements in the array a using the fold method
        let product = a.iter().fold(1, |accumulator, a| accumulator * a);
        assert_eq!(product, 1 * 5 * 3 * 2);
        println!("product: {product}");
    
        // Take all values until the first non-increasing value using take_while
        let mut prev = i32::MIN;
        // take_while takes a closure that returns a boolean
        // create a new vector call increasing by taking elements from the original vector a until the elements stop increasing
        // increasing in order
        let increasing = a
            .iter()
            .take_while(|next| {
                if **next > prev {
                    prev = **next;
                    true // keep taking
                } else {
                    false // stop taking
                }
            })
            .collect::<Vec<&i32>>();
        assert_eq!(&increasing, &[&1, &5]);
        println!("increasing: {increasing:?}");
    
        // Flatten a vector of tuples using flatten
        let flat = a.iter().map(|num| [*num, 0])
            .flatten().collect::<Vec<i32>>();
        assert_eq!(flat, &[1, 0, 5, 0, 3, 0, 2, 0]);
        println!("flat: {flat:?}");

        // Find the longest word using max_by_key
        let numbers = [(1, "one"), (2, "two"), (3, "three"), (4, "four")];
        let longest_number = numbers.iter().max_by_key(|(_num, word)| word.len());
        assert_eq!(longest_number, Some(&(3, "three")));
        println!("longest_number: {longest_number:?}");
    }
    

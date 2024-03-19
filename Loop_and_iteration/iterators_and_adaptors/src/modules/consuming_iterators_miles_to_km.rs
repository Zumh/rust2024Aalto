    // convert miles to kilometers
    fn miles_to_kilometers(miles: &f64) -> f64 {
        miles * 1.609344 
    }
    pub fn miles_to_kilometers_map(){

        let miles = [1.0f64, 5.0, 10.0, 15.0, 20.0];
        // iterate over miles and convert to kilometers
        // .iter() returns an Iterator over immutable references
        //  into_iter() returns an Iterator over owned values
        //   collect() collects the Iterator into a collection
        //   map consumes the iterator and applies a function
        //   we create a new collection that converts miles to kilometers
        let kilometers = miles.iter().map(miles_to_kilometers).collect::<Vec<f64>>();
        println!("miles: {:?}", miles);
        println!("kilometers: {:?}", kilometers);
    }   

fn miles_to_kilometers(miles: f32)-> f32{
    return miles * 1.609344;
}

fn main() {
    println!("1 mile is equal to {} km", miles_to_kilometers(1.0));

    println!("15 mile is equal to {} km", miles_to_kilometers(15.0));


    println!("50 mile is equal to {} km", miles_to_kilometers(50.0));
}
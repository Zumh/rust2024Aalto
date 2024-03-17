
use std::collections::HashMap;

fn main(){
    println!("hash map new initialization");
    translation();
    println!("\n\n hash map from function");
    country_pollutions();
    println!("\n\n remove entry");
    remove_entry();

    println!("\n\n Modifying hashmap");
    modifying();
}

fn modifying(){
    // get_mut which get a mutalb erference to the value inside an Option
    let mut food_prices = HashMap::from([("beetroot", 1.2), ("cabbage", 1.1)]);
    println!("before inflation {food_prices:#?}");

    println!("some inflation happens...");

    // unwrap when we aren't afraid of panicking
    let beetroot_price: &mut f32 = food_prices.get_mut("beetroot").unwrap();
    *beetroot_price *= 1.2;

    // let us not forget `if let`
    if let Some(cabbage_price) = food_prices.get_mut("cabbage") {
        *cabbage_price *= 1.2;
    }

    println!("after inflation {food_prices:#?}");

}


// remove entry 
fn remove_entry(){
    let mut country_pollutions = HashMap::from([
        ("US", 10),
        ("UK", 5),
        ("Germany", 2),
        ("France", 1),
        ("Russia", 4),
        ("China", 3),
        ("Japan", 5),
        ("India", 1),
        ("Egypt", 6),
        ("Brazil", 7),
    ]);
    println!("{:#?}", country_pollutions);
    // simple remove 
    country_pollutions.remove("UK");
    println!("\n remove {}", "UK");
    println!("{:#?}", country_pollutions);
    
    // return value
    let removed = country_pollutions.remove_entry("US");
    println!("\n remove {}", removed.unwrap().0);
    println!("{:#?}", country_pollutions);
}


fn country_pollutions(){
    // HashMap new initiailization 
    let country_pollutions = HashMap::from([
        ("US", 10),
        ("UK", 5),
        ("Germany", 2),
        ("France", 1),
        ("Russia", 4),
        ("China", 3),
        ("Japan", 5),
        ("India", 1),
        ("Egypt", 6),
        ("Brazil", 7),
    ]);
    println!("{:#?}", country_pollutions);
}
fn translation(){
     let mut english_to_chinese = HashMap::new();
    println!("{:?}", english_to_chinese);

    english_to_chinese.insert("hello".to_string(), "你好".to_string());
    english_to_chinese.insert("world".to_string(), "世界".to_string());

    println!("{:#?}", english_to_chinese);

}

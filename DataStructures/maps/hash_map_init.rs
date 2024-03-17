use std::collections::HashMap;

fn main(){
    println!("hash map new initialization");
    translation();
    println!("\n\n hash map from function");
    country_pollutions();
    println!("\n\n remove entry");
    remove_entry();
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
    country_pollutions.remove_entry("US");
    println!("\n remove {}", "US");
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

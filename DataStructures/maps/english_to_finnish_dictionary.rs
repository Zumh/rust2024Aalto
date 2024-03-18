use std::collections::HashMap;

fn main() {
    let dictionary: HashMap<String, String> = eng_fin_dictionary();
    println!("{:?}", dictionary);
    
}

fn eng_fin_dictionary() -> HashMap<String, String> {
    let dictionary = HashMap::from([
        ("bear".to_string(), "karhu".to_string()),
        ("paw".to_string(), "tassu".to_string()),
        ("tail".to_string(), "häntä".to_string()),
        ("ear".to_string(), "korva".to_string()),]);
    dictionary
}

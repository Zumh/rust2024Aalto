use std::collections::HashMap;
// Implement the translate_fin function here

fn eng_fin_dictionary() -> HashMap<String, String> {
    let dictionary = HashMap::from([
        ("bear".to_string(), "karhu".to_string()),
        ("paw".to_string(), "tassu".to_string()),
        ("tail".to_string(), "häntä".to_string()),
        ("ear".to_string(), "korva".to_string()),
    ]);

    return dictionary;
}

fn translate_fin(dictionary: &HashMap<String, String>, word: &str) -> String {
    match dictionary.get(word) {
        Some(translation) => format!("{word} in Finnish is {translation}"),
        None => format!("sorry, no translation for {word} available"),
    }
    /*    
    if !dictionary.contains_key(word) {
        return format!("sorry, no translation for {} available", word);
    }
    let fin: String = dictionary.get(word).unwrap_or(&word.to_string()).to_string();
    
    // format the word and fin
    return format!("{} in Finnish is {}", word, fin);
  */  
}
fn main() {
    let dictionary: HashMap<String, String> = eng_fin_dictionary();
    println!("{}", translate_fin(&dictionary, "bear"));
    println!("{}", translate_fin(&dictionary, "paw"));
    println!("{}", translate_fin(&dictionary, "tail"));
    println!("{}", translate_fin(&dictionary, "🐻"));
}


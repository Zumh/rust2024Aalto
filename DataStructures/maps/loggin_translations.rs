use std::collections::HashMap;

fn translate_fin(dictionary: &HashMap<String, String>, word: &str, usage_log: &mut HashMap<String, i32>) -> String {
    // access usage with entry and if not exist then insert 1 otherwise add 1 to count mutable ref
    usage_log.entry(word.to_string()).and_modify(|count| *count += 1).or_insert(1);

    match dictionary.get(word) {
        Some(translation) => format!("{word} in Finnish is {translation}"),
        None => format!("sorry, no translation for {word} available"),
    }

}

fn eng_fin_dictionary() -> HashMap<String, String> {
    let dictionary = HashMap::from([
        ("bear".to_string(), "karhu".to_string()),
        ("paw".to_string(), "tassu".to_string()),
        ("tail".to_string(), "häntä".to_string()),
        ("ear".to_string(), "korva".to_string()),
    ]);

    return dictionary;
}

    
fn main() {
    let mut usage_log = HashMap::new();
    let dictionary = eng_fin_dictionary();
    let _ = translate_fin(&dictionary, "bear", &mut usage_log);
    let _ = translate_fin(&dictionary, "paw", &mut usage_log);
    let _ = translate_fin(&dictionary, "bear", &mut usage_log);
    let _ = translate_fin(&dictionary, "🐻", &mut usage_log);
    println!("{usage_log:?}"); // {"bear": 2, "paw": 1, "🐻": 1} (may be in different order)
}


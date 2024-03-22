use std::collections::HashMap;
pub fn calling_enumerate_on_iterators() {
    println!("\n\nenumerating iterators");

    let a = ["a", "b", "c", "d"];
    let mut e = a.iter().enumerate();
    println!("{:?}", e.next()); // Some((0, "a"))
    println!("{:?}", e.next()); // Some((1, "b"))

    println!("{:?}", e.next()); // Some((2, "c"))
    println!("{:?}", e.next()); // Some((3, "d"))
    println!("{:?}", e.next()); // None
    
}


pub fn enumerate_forloop() {
    println!("\n\nenumerate_forloop");

    let a = ["a", "b", "c", "d"];
    for (index, element) in a.iter().enumerate() {
        println!("{}: {}", index, element);
    }

}

pub fn push_everything_together(){
    println!("\n\npush_everything_together");
    let letters = vec!["a", "b", "c", "d"];
    let (evensn , odds) = split_adjacent(&letters);
    println!("evens: {:?}\n odds: {:?}", evensn, odds);

}

fn split_adjacent(items: &Vec<&str>) -> (Vec<String>, Vec<String>) {
    let mut evens = vec![];
    let mut odds = vec![];
    // we are push an object of String not string literal &str
    // how to push &str need generic lifetime parameter
    for (i, item) in items.iter().enumerate() {
        if i % 2 == 0 {
            evens.push(item.to_string());
        } else {
            odds.push(item.to_string());
        }
    }
    (evens, odds)
}

/*
    When collecting the results of enumerated string references, we cannot use the cloned method to get owned copies from the references. 
    This is because cloned works only on iterators that yield references but the iterator yields tuples (usize, &String), which are not references but owned values (only the second value of the tuple is a reference). 
    Instead, we can clone the &Strings inside the tuple with the map method.
*/
pub fn clone_enumerated() {
    println!("\n\nclone_enumerated");
    let strings = ["a".to_string(), "b".to_string(), "c".to_string(), "d".to_string()];

    let enumerate: Vec<(usize, String)> = strings
        .iter()
        .enumerate()
        .map(|(i, s)| (i, s.clone()))
        .collect();

    println!("{:?}", enumerate);

}
/*
In this example, we could also opt to clone the string references using cloned prior to enumerating with enumerate, or use into_iter to iterate over owned values instead of references. However, such options are not always available, e.g. when working with collections or iterators as function parameters.
*/

pub fn reverse_fruits(){
    

    // Implement the reverse_index function here that takes a slice of strings and returns a HashMap of the strings and their indices

    let fruits = [
        "🥭".to_string(),
        "🍑".to_string(),
        "🍐".to_string(),
    ];
    let fruit_indices = reverse_index(&fruits);
    println!("{:#?}", fruit_indices);
    println!("value at index 2 = {}", fruits[2]);
    println!("index of 🍐 = {}", fruit_indices[&"🍐".to_string()]);
}

// model solution
fn reverse_index(input: &[String]) -> HashMap<String, usize> {
    input
        .iter()
        .enumerate()
        .map(|(i, s)| (s.to_owned(), i))
        .collect()
}
/* 
fn reverse_index(fruits: &[String]) -> HashMap<String, usize> {
    let mut fruit_indices = HashMap::new();
    for (i, fruit) in fruits.iter().enumerate() {
        fruit_indices.insert(fruit.clone(), i);
    }
    fruit_indices
}
*/


pub fn string_stream_processing() {
    println!("\n\nstring_stream_processing");
    let strings = vec![
        "This is ASCII",
        "This is not -> ä",
        "Valid ASCII again!",
        "And then 🚫",
    ];
    let strings = strings
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    println!("{:?}", longest_string_length(&strings)); // Some(18)

    println!("{:?}", ascii_strings_only(&strings)); // ["This is ASCII", "Valid ASCII again!"]
}

/*
longest_string_length should return the length of the longest string in the given vector in an Option. 
If the vector is empty, the function should return None.
 (hint: Iterator has some convenient methods for finding the largest or smallest element in the iterator)
*/
fn longest_string_length(input: &[String]) -> Option<usize> {
    // iterate through the vector and find the max length
    input
        .iter()
        .map(|s| s.len())
        .max()
        
}

/**
 ascii_strings_only should filter out all non-ASCII strings from the given vector and return all the ASCII-only strings in a vector. 
 (hint: there is a function for str that can tell you if a string consists only of ASCII characters, and Strings coerce to strs so you can use it on Strings too)

 */
fn ascii_strings_only(input: &[String]) -> Vec<String> {
    input
    .iter()
    .filter(|s| s.chars().all(|c| c.is_ascii()))    
    .cloned()
    .collect()
}



/*
Continuing with adapters and closures, implement the following two functions that mutate strings in vectors.
lowercase should convert all strings in the given vector to lowercase.
ellipse should cut off strings that are longer than max_len and add an ellipsis ... (three dots). The ellipsis must fit into the length limit, so with max_len = 5 the string "short" should stay as "short", but the string "longest" should become "lo...". (hint: String::replace_range might come in handy)
See the main function for examples on how the functions should behave.
*/
pub fn mutating_strings() {
    println!("\n\nmutating_strings");
    let strings = vec![
        "This is ASCII",
        "This is not -> ä",
        "Valid ASCII again!",
        "And then not 🐈",
    ];
    let mut strings = strings
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();

    lowercase(&mut strings);
    println!("{:?}", strings); // ["this is ascii", "this is not -> ä", "valid ascii again!", "and then not 🐈"]

    ellipse(&mut strings, 10);
    println!("{:?}", strings); // ["this is...", "this is...", "valid a...", "and the..."]
}

fn lowercase(input: &mut [String]) {
    // TODO
    for s in input.iter_mut() {
        *s = s.to_lowercase();
    }

    // input.iter_mut().for_each(|s| *s = s.to_lowercase());
}
fn ellipse(sentences: &mut [String], max_len: usize) {
    // TODO
    for s in sentences.iter_mut() {
        if s.len() > max_len {
            s.replace_range((max_len - 3).., "...");
        }
    }
    /*
        sentences
        .iter_mut()
        .filter(|sentence| sentence.len() > max_len)
        .for_each(|sentence| sentence.replace_range(max_len - 3.., "..."));
     */
}


/*

This exercise is a continuation to a previous exercise where the task was to implement the is_finnish_domain function. 
The task now is to implement functions for filtering a vector of domains.

The template already includes the is_finnish_domain for checking if a domain is finnish. 
There are two more functions that are used in the main function, which are yet to be implemented.
remove_by_value_first should take a vector of strings and a string as parameters, and remove the first occurrence of the string from the vector.
remove_by_value_all should take a vector of strings and a string as parameters, and remove all occurrences of the string from the vector.
See the main function again for examples of how the functions should behave.
*/

pub fn pruning_domains() {
    println!("\n\n## pruning_domains");
    let mut domains: Vec<String> = vec![
        "test.fi",
        "svenska.se",
        "aalto.fi",
        "commercial.com",
        "aalto.fi",
        "suomi.fi",
        "suomi.fi",
    ]
    .into_iter()
    .map(|x| x.to_string())
    .collect();

    // keep only Finnish domains
    retain_finnish_domains(&mut domains);
    println!("{:#?}", domains);
    // remove the first suomi.fi domain
    remove_by_value_first(&mut domains, "suomi.fi");
    println!("{:#?}", domains);
    // remove all aalto.fi domains
    remove_by_value_all(&mut domains, "aalto.fi");
    println!("{:#?}", &domains);
}

fn is_finnish_domain(domain: &str) -> bool {
    domain.ends_with(".fi")
}

fn retain_finnish_domains(domains: &mut Vec<String>) {
    domains.retain(|d| is_finnish_domain(d));
}

// Implement the remove_by_value_first function
// remove_by_value_first should take a vector of strings and a string as parameters, and remove the first occurrence of the string from the vector.
fn remove_by_value_first(domains: &mut Vec<String>, value: &str) {
    if domains.iter().position(|d| d == value).is_some() {
        domains.remove(domains.iter().position(|d| d == value).unwrap());
    } 
}

// model solution 
/*
fn remove_by_value_first(values: &mut Vec<String>, value: &str) {
    // If let solution
    if let Some(index) = values.iter().position(|x| x == value) {
        values.remove(index);
    }
    // Option map solution
    let index = values.iter().position(|x| x == value);
    index.map(|index| values.remove(index));
}
*/

// Implement the remove_by_value_all function
fn remove_by_value_all(domains: &mut Vec<String>, value: &str) {
    domains.retain(|d| d != value);
}

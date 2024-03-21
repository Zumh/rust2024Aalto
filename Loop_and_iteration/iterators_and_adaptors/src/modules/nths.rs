// Implement the function nth_chars
fn nth_chars(s: &str, nths: &[usize]) -> String {
    let mut result = String::new();
    // model solution 
    //  let mut result = String::with_capacity(ns.len());
    for nth in nths {
        result.push(s.chars().nth(*nth).unwrap());
    }
    result
}
pub fn nths() {
    let emojis =
        "🐀🐁🐂🐃🐄🐅🐆🐇🐈🐉🐊🐋🐌🐍🐎🐏🐐🐑🐒🐓🐕🐖🐘🐙🐚🐛🐜🐝🐞🐟🐠🐡🐢🐣🐤🐥🐦🐧🐩🐪🐫🐳";
    let common_pets = nth_chars(emojis, &[7, 8, 20]);
    println!("Common pets: {common_pets}");
}

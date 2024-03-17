// here we have to clone the tree because of ownership rules
fn main() {
    let tree = "🌴".to_owned();
    let chopped = chop(tree.clone());
    println!("chopping a {tree} gives {chopped}"); 
}

fn chop(tree: String) -> String {
    match tree.as_str() {
        "🌴" => "🪵🪵🪵".to_owned(),
        "🌳" => "🪴🪴🪴".to_owned(),
        _ => tree.to_owned(),
    }
}

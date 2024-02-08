fn main() {
    println!("{}", is_finnish_domain("stackoverflow.com"));
    println!("{}", is_finnish_domain("aalto.fi"));
}

// Slice the last three characters of the domain and compare them to ".fi".
fn is_finnish_domain(domain: &str) -> bool {
    &domain[domain.len() - 3..] == ".fi"
}

// Use ends_with to check if the domain ends with ".fi".
fn is_finnish_domain(domain: &str) -> bool {
    domain.ends_with(".fi")
}

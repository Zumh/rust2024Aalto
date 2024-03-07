fn main() {
    println!("{}€/kg", price_per_kg(2.99, 450.0).round());
    println!("{}€/kg", price_per_kg(29.99, 1514.0).round());
}

fn price_per_kg(price: f64, weight_in_grams: f64) -> f64 {
    //let price = 14.99;
    //let weight_in_grams = 100.0;
    return price / weight_in_grams * 1000.0;
}

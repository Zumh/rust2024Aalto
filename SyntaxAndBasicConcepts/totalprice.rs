// The function total_price() should return the item price multiplied by the quantity.
fn total_price(item_price: f64, quantity: u32) -> f64 {
    
    return item_price * (quantity as f64);
}

fn main() {
    println!("{}", total_price(1.0, 10));
    println!("{}", total_price(1.5, 30));
    println!("{}", total_price(2.99, 450));
    println!("{}", total_price(1.49, 100));
}

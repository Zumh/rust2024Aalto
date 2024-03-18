use std::collections::HashMap;
const INFLATION_RATE: f32 = 1.07;

fn inflate_prices(map: &mut HashMap<&str, f32>, years: i32) {
    for price in map.values_mut() {
        *price *= INFLATION_RATE.powi(years);
    }
}

fn main() {
     let mut food_prices = HashMap::from([("beetroot", 1.2), ("cabbage", 1.1), ("carrot", 1.0)]);
    println!("before inflation {food_prices:#?}");
    inflate_prices(&mut food_prices, 10);
    println!("after 10 years of inflation {food_prices:#?}");

}

// parse and Iterator's collect, have generic output types
fn power_up(values: &[i32], power: u32) -> Vec<i32> {
    values.iter().map(|x| x.pow(power)).collect()
}

pub fn power_up_generic() {
    let values = [1, 2, 3, 4];
    let power = 2;
    let result = power_up(&values, power);
    println!("initial values {:?}", values);
    println!("power up by {}values: {:?}", power, result);
}
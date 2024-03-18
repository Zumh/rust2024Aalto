fn celcius_to_fahrenheit(celcius: f32) -> f32 {
    (celcius * 9.0 / 5.0) + 32.0
}

fn main(){

    let celcius = [0.0, 37.5, 100.0];
    let mut fahrenheit = Vec::new();
    for c in celcius {
        fahrenheit.push(celcius_to_fahrenheit(c));
    }

    println!("C: {:?}\nF: {:?}", celcius, fahrenheit);
}

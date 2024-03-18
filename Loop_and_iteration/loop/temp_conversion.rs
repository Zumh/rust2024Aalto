fn celcius_to_fahrenheit(celcius: f32) -> f32 {
    (celcius * 9.0 / 5.0) + 32.0
}

fn normal_conversion(){
     let celcius = [0.0, 37.5, 100.0];
    let mut fahrenheit = Vec::new();
    for c in celcius {
        fahrenheit.push(celcius_to_fahrenheit(c));
    }

    println!("C: {:?}\nF: {:?}", celcius, fahrenheit);
   
}


fn main(){
    // normal conversion
    println!("Normal Conversion");
    normal_conversion();

    // replace f to c
    println!("\n\nReplace F to C conversion");
    replace_f_to_c();

}


fn fahrenheit_to_celcius(fahrenheit: f32) -> f32 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}
fn replace_f_to_c() {
    let mut fahrenheit = [0.0, 37.5, 100.0];
    for f in &mut fahrenheit {
        *f = fahrenheit_to_celcius(*f);
    }
    println!("celcius: {:?}", fahrenheit);
}

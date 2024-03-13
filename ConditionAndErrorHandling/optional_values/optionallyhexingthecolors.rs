fn get_color_hex(color: &str) -> Option<String> {
    let hex_value = match color {
        "black" => "#000000",
        "white" => "#FFFFFF",
        "red" => "#FF0000",
        "green" => "#00FF00",
        "blue" => "#0000FF",
        _ => return None
    };
    Some(hex_value.to_owned())
}

fn main() {
    println!("{:?}", get_color_hex("red"));
    println!("{:?}", get_color_hex("green"));
    println!("{:?}", get_color_hex("blue"));
    println!("{:?}", get_color_hex("pink"));
}

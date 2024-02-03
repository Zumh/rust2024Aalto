fn get_color_hex(color: &str) -> String {
    String::from(match color {
        _ if color == "red" => "#FF0000",
        _ if color == "green" => "#00FF00",
        _ if color == "blue" => "#0000FF",
        _ if color == "white" => "#FFFFFF",
        _ if color == "black" => "#000000",
        _ => ""
        
    })
}

fn main() {
    println!("{}", get_color_hex("red"));
    println!("{}", get_color_hex("green"));
    println!("{}", get_color_hex("blue"));
}

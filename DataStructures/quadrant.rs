// The first value of the point tuple is the x coordinate and the second value is the y coordinate.
fn quadrant(point: (i32, i32)) -> Option<String> {
    let quadrant = match point {
        (0, _) | (_, 0) => return None,
        (1.., 1..) => "north-east",
        (..=-1, 1..) => "north-west",
        (..=-1, _) => "south-west",
        _ => "south-east",
    }
    .to_string();
    Some(quadrant)

    // solution with guards
    // let quadrant = match point {
    //     (x, y) if x > 0 && y > 0 => "north-east",
    //     (x, y) if x < 0 && y > 0 => "north-west",
    //     (x, y) if x < 0 && y < 0 => "south-west",
    //     (x, y) if x > 0 && y < 0 => "south-east",
    //     _ => return None,
    // }
    // .to_string();
    // Some(quadrant)
}

fn main() {
    let point = (1, 1);
    println!("The point {:?} is in {} quadrant", point, quadrant(point).unwrap_or("no".to_string()));
    let point = (0, -1);
    println!("The point {:?} is in {} quadrant", point, quadrant(point).unwrap_or("no".to_string()));
}

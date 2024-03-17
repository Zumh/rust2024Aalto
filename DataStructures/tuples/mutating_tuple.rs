fn main() {
    let mut point: (f32, f32) = (0.0, 0.0);
    move_point(&mut point, (1.0, 0.0), 5.0);
    //move_point(&point, (0.28734789, 0.95782629), 10.0);
    println!("{:?}", point);

}

fn move_point((x, y): &mut (f32, f32), direction: (f32, f32), velocity: f32) {
    *x += direction.0 * velocity;
    *y += direction.1 * velocity;

}

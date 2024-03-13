fn points_to_grade(points: u32) -> u32 {
    match points {
    0..=39 => 0,
    40..=54 => 1,
    55..=69 => 2,
    70..=79 => 3,
    80..=89 => 4,
    _ => 5
        
    }
    
 
}

fn main() {
    println!("{}", points_to_grade(0));
    println!("{}", points_to_grade(50));
    println!("{}", points_to_grade(60));
    println!("{}", points_to_grade(70));
    println!("{}", points_to_grade(80));
    println!("{}", points_to_grade(100));
}


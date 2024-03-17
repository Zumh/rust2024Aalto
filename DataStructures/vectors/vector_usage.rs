// vector initialization and usage
fn main() {
    println!("new, !vec initialization");
    let v: Vec<i32> = Vec::new();
    let v2: Vec<i32> = vec![1, 2, 3];
    println!("{v:?} {v2:?}");

    println!("\n\nmacro initialization");
    let v = vec!["air", "water", "fire"];
    println!("{v:?}");


    println!("\n\ncloning vectors");
    let v1 = [1, 2, 3][1..].to_vec();
    println!("{v1:?}");

    // indexing vectors 
    let v2 = vec![1, 2, 3];
    println!("{}", v2[2]);
    println!("{:?}", v2.get(2));
}

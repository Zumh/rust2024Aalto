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


    // vectors moved data type like String
    // we can pass it as a reference or pass an explicity cloned value
    println!("\n\nborrowing vectors");
    let v3 = vec![1, 2, 3];
    print_length(&v3);
    // here we can borrow or explicitly clone
    let v4 = &v3;
    println!("{v3:?} {v4:?}");
}

fn print_length(v: &Vec<i32>) {
    println!("size: {}", v.len());
}

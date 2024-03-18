fn main() {
    let mut i = 0;
    let mut p = 1;
    println!("The power of 2 is:");
    while i < 10 {
        println!("2^{} = {}", i, p);
        i += 1;
        p *= 2;
    }

    println!("\n\n while do to do while");
    while_do_to_do_while();
}

fn while_do_to_do_while(){
      let mut i = 0;
    let mut p = 1;
    println!("The powers of 3:");
    while {
        // do
        println!("3^{}={}", i, p);
        i += 1;
        p *= 3;
        // while
        p < 2000
    } {
        // the actual body of the loop
        // do nothing here
    }
}


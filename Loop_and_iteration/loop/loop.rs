// loop, while and for 

fn main() {
    let mut i = 0;
    loop {
        if i > 10 {
            break;
        }
        println!("{}", i);
        i += 1;
    }

    println!("Done!");
    println!("\n\n leap year");

    // jupm to the beginnning
    let mut year = 2023;
    loop {
        year += 1;
        if year % 4 == 0 {
            continue;
        }

        if year % 100 == 0 && year % 400 != 0 {
            continue;
        }

        println!("The next leap year is {}", year);
        break;
    };
}

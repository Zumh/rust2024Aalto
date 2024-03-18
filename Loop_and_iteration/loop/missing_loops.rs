fn sum_of_range_inclusive(a: i32, b: i32) -> i32 {
    let mut sum = 0;
    let mut i = a;
    // loop until i > b
    loop {
        if i > b {
            break;
        }
        sum += i;
        i += 1;
    }
    sum
}

fn sum_of_range_exclusive(a: i32, b: i32) -> i32 {
    let mut sum = 0;
    let mut i = a + 1;
    // loop as long as i < b
    while i < b {
        sum += i;
        i += 1;
    }
    sum
}

fn factorial(n: u8) -> u128 {
    let mut product = 1;
    // loop i from 2 to n
    for i in 2..=n {
        product *= i as u128;
    }
    product
}

fn main() {
    println!("sum of range 5..=10: {}", sum_of_range_inclusive(5, 10));
    println!("sum of range 6..10: {}", sum_of_range_exclusive(5, 10));
    println!("factorial 11: {}", factorial(11));
}


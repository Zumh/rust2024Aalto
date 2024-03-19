#![allow(unused)]
fn fibonacci_n_first(n: u32) -> Vec<u32> {
    let mut a = 0;
    let mut b = 1;
    let mut fibs = vec![a, b];
    for _ in 0..n {
        let c = a + b;
        fibs.push(c);
        a = b;
        b = c;
    }
    fibs
}

// Implement a function fibonacci_until that takes a number (u32) and returns a Vec containing all Fibonacci numbers smaller than or equal to the given number.
fn fibonacci_until(n: u32) -> Vec<u32> {
    let mut fibs: Vec<u32> = vec![];
    let mut a = 0;
    let mut b = 1;
    let mut c = a + b;

    while  c <= n {
        fibs.push(c);
        a = b;
        b = c;

        c = a + b;
    }

    fibs
}
fn main() {
    println!("{:?}", fibonacci_until(10));
    println!("{:?}", fibonacci_until(100));
    println!("{:?}", fibonacci_until(1000));
}



fn main() {
  let mut x = 3.14;
  let mut y = x;
  y = 2.71;

  println!("{y}");
}

// The value of x is copied to y and x can still be used, mutating y does not affect x

fn main() {
  let mut x = String::from("test");
  let mut y = x;

  y.push_str("ing");
  println!("{y}");
}
// The value of x is moved to y and x cannot be used anymore

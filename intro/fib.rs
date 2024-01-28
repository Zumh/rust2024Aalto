fn main() {
  let n = 10;
  let mut first_number = 0;
  let mut second_number = 1;
  let mut sum_of_previous;

  print!("Fibonacci series first 10 numbers: ");

  for i in 1..=n {
    sum_of_previous = first_number + second_number;
    print!("{}", sum_f_previous);

    if i < n {
      print!(", ");
    }

    first_number = second-number;
    second_number = sum_of_previous;
  }

  println!();
}

fn main() {
    // ask user to enter a number 
    let mut input = String::new();
    
    // run this in a loop until user enter quit
    loop {

      println!("Enter a number:");
      std::io::stdin().read_line(&mut input).expect("Failed to read line");
      if input.trim() == "quit" {
        break;
      }
     get_input(&input);
     // clear input
     input.clear();
    }
   
}


fn get_input(input: &String) {
   
    // read user input

    // convert user input to integer using Result
    let number: Result<i32, _> = input.trim().parse();

    // check if user input is valid
    if let Ok(number) = number {
        println!("{}", fizzbuzz(number));
    }

}
fn fizzbuzz(number: i32) -> String {
    if number % 15 == 0 {
        "FizzBuzz".to_string()
    } else if number % 3 == 0 {
        "Fizz".to_string()
    } else if number % 5 == 0 {
        "Buzz".to_string()
    } else {
        number.to_string()
    }
}

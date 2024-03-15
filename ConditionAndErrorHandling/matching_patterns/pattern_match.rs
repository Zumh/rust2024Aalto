fn main(){
   let number: i32 = 4;

   match number {
       1 => println!("One!"),

       2 => println!("One!"),
       3 => println!("Two!"),
       4 => println!("Three!"),
       n => println!("{n} is not one, two or three!"),
   }
}

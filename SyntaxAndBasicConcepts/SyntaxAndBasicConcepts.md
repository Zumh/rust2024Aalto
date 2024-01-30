## Main function and program output
- main function
- ! invoke macro
- println! and print is macro
- println!() print content in a new line
- println!("") multiple line of string literal
### Excaping and raw strings
- backslash `\` for escaping special chracters
- `\\` will allow us to print one backslash
- raw string literals `println!(r#"Backslash" \! Not a newline: \n"#);
- `#` sign is optional but allow us to use double quote to print
### Comments
- single line `//`
- multiple line `/**/`
- documentation comments `///`
- same as `#[doc="..."]`
- document comment can be convert to website using
  - we can use doc comment for markdown format 
  - rustdoc tool
  - cargo doc
  - cargo doc --open
## Variables
- `let` is use for defining variables
- print
  - not allow to print vriable directly
```rust
let bac = "abc"
println!(abc); // error 
```
### print string
- `println!("x is {first} and y is {second}", second=y, first=x); // x is 3 and y is 5.4`
- `println!("x is {} and y is {}", x, y);`
- `println!("x is {x} and y is {y}");`

### Mutability
- mutable `let mut x = 3;`
- imutable variable is good because mutable variable can cause bug because of it change in some other program
### Constants
- let vs constant
- constant can be global and immutable
- constant must be in capital
  - `const RUST: &str = "is awesome";`
- it can have expressions
  - `const HOURS_IN_WEEK: i32 = DAYS_IN_WEEK * HOURS_IN_DAY;`
- predefined constants are available like `let big = i32::MAX;`

## Data types
- statically typed "which means every variable must be known at compile-time"
- type inference allow us to define same variable only once.
- multiple data type have `&str` have to_lowercase, replace
- f32 has round, abs and max.
- https://doc.rust-lang.org/reference/types.html
### Explicit data types
- explicitly define variable for floating
- `let f1: f64 = 3.5;`
- so that round method can round floating point number base on explicit type
- `println!("{}", f1.round());`
### Primitive and custom types
- Primitive 64 bit float f64 or 32 bit integer i32
- strucs and enums (capitalized name by convention) are custom data types built on top of primitive
- https://doc.rust-lang.org/rust-by-example/primitives.html
- Custom types
  - String > `let a_string: String = String::from("Hello, world!");`
- scalar (integers, floats, booleans and characters) lower case
  - single value and cannot seperate into multiple
- compound type
  - tuples
  - arrays
  - slices
- flavors
  - signed and unsigned
  - 8 bit to 128 bit
  - i8, i16, i32, i64, i128, isize
  - u8, u16, u32, u64, u128, usize
- Donald knuth"premature optimization is the root of all evil".
  - stick with i32 for most cases. 
- floting type
  - 32 bit f32 smaller mem print and default
  - 64 bit f64 better precision 

## Functions

### Defining and calling functions
- snake case `snake_case` is recommended in Rust for function name
- calling and defining function is same as other programming languages.
### Function parameters
- func declaration and all parameter types must be defined explicitly
- `fn simon_says(command: &str) {}`
### Returning from functions
- explicit return `fn miles_to_kilometers(miles: f32) -> f32 {}`
- implicit return `fn miles_to_kilometers(miles: f32){}`
- main doesn't need explicit return because it return unit type ().

## Numerical computation
- quotient truncated.
```rust
fn main() {
    println!("{}", 4 / 5);  // 0
    println!("{}", 14 / 5);  // 2
}

```
- number/0 will becaught in compile time
- This will result in errror when program execute because compiler didn't catch empty string length is 0
```rust
fn main() {
    let zero = "".len();       // 0
    println!("{}", 1 / zero);  // thread 'main' panicked at 'attempt to divide by zero'
}

```
- divide by float can be safer because of it can be infinite
```rust
fn main() {
    println!("{}", 4.0 / 0.0);  // inf
    println!("{}", -1.0 / 0.0);  // -inf
    println!("{}", 0.0 / 0.0);  // NaN
}
```
- different types of number operation can cause error
  - like adding float and integer
  - like adding unsigned int and int value
### Casting values as other types
- as keyword can help us convert number to same type
```rust
fn main() {
    println!("{}", 5i32 as f32); // 5.0
    println!("{}", 1.5f32 as u8); // truncated to 1
    println!("{}", 1.5f32.round() as u8); // 2
    println!("{}", 65u8 as char); // 'A'
    println!("{}", 128u8 as i8); // -128 since 127 is the largest value of i8
    println!("{}", 'a' as i32); // 97
}
```
- u8 to char converfsion is allow
- 128 to i8 is not allow because i8 end in 127 so the result is -128. It wrap around.
- only primitive data types can be cast.
- custom types need to implement meethods to convert. like &str to_string result to String.
https://doc.rust-lang.org/reference/type-coercions.html
```rust
fn main() {
    let a = "String or &str?";
    let b = a.to_string();
    println!("{b}"); // String or &str?
}

```
- Different way to casts
https://doc.rust-lang.org/reference/expressions/operator-expr.html#type-cast-expressions
## Summary of symbols

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
### Function parameters
### Returning from functions

## Numerical computation
### Casting values as other types

## Summary of symbols

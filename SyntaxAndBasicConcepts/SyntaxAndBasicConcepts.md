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
### Explicit data types
### Primitive and custom types

## Functions
### Defining and calling functions
### Function parameters
### Returning from functions

## Numerical computation
### Casting values as other types

## Summary of symbols

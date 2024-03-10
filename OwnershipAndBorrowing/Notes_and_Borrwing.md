## Intro
- basic understing of Ownership is require.
- no need to understand memory layer
- all Rust libraries and community can help us.
## Scope
- lexical or aka static scoping is when variable can accessible in the scope they are defined in.
- accessible only in inside a function.
- outer scope cannot see inner scope variables.
- Shadowing is allow > resuing same variables in the same scope.
- Related to scopes, Rust supports shadowing.
  - Shadowing allows reusing the same variable name for another variable.
  - Shadowing a variable from the outer scope inside a subscope does not change the variable.

## Ownership
- Rules
  - Each value in Rust is owned by a variable.
  - Each value can have only one owner at a time.
  - When the owner goes out of scope, the value will be dropped.
- Copying and clonging
  - cheap to copy like primitive scalar data type are copy (integers and others)
  - expensive one are moved  like String
- What about functions?
  -  passing to a function copy/moved value and destroy variable in that function scope.
  -  If we want to maintain the value that we pass then we must recieve value from that function return.
  -  Variables like integers, float cheap variables won't change their value if we pass by value.

## Borrowing and dereferencing
- referencing is borrwing using &symbol
  - meaning we don't change the owner of a vriable
-  `fn read_book(book: &String) {}`
## The scope of references
- while we are borrowing reference from variablek, we cannot modified the value right a way.
- we have to do something with variable that we just borrowed using reference.
- Then we can change value from the owner.
```rust
fn main() {
    let mut book = String::from("📕");
    let borrowed_book = &book; // borrows `book` 
    book = String::from("📘");
    println!("{borrowed_book}"); // error: cannot assign to `book` because it is borrowed
    println!("{book}");
}
```
- this way we can prevent invalid reference.
- When we want to use a value from an expression or a function that depends on borrowed data, we can use the to_owned method of a borrow to create an owned copy of the borrowed data.
- On a &str, to_owned returns a new String, just like to_string().
## pass by value vs pass by reference
- pass by value make sense if we want to own the data passsed to it.
- Otherwise we clone the data using .to_string() for String, for &str or literal string to_owned
```rust
fn append(s: String, prefix: String) -> String {
    format!("{}{}", prefix, s)
}

fn main() {
    let love = "💕".to_string();
    let greeting = "Hello".to_string();
    let lovely_greeting = append(greeting.clone(), love);
    println!("{}", greeting.clone());
    println!("{lovely_greeting}");
    // cannot use `love` anymore due to moving
}

```
## Mutable borrows
- Mutable dereferencing
```rust
fn main() {
    let mut x = 100;
    let y = &mut x; // y is a reference to x
    *y = 300; // change the value stored in x
    println!("{x}"); // 300
}

```
- cannot use borrow variable
```rust
fn main() {
    let mut x = 100;
    let y = &mut x; // y is a reference to x
    *y = 300; // change the value stored in x
    let z = x + *y; // error: cannot use `x` because it was mutably borrowed
    println!("{z}");
}

```
- The scope of a reference ends when it is used for the last time and the arithmetic expression is read from left to right, 
- so the code can be fixed by simply changing the order of *y and x in *y + x.
- The scope of reference y (the mutable reference to x) ends in the expression *y, which happens before reading x in + x, and thus the rules of references are satisfied.
## Borrowing and dereferencing
- &x, &mut x
- 
## Compiles?
```rust
fn main() {
    let mut x = 10;
    let y = &x;
    x = x * x;
    println!("{x}, {y}");
}
```
- x = x * x; causes the errror
## It compiles
- The program compiles
```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    println!("{}", x);
}
```
## compiles ?
```rust
fn main() {
    let mut x = 5;
    let y = &mut x;
    *y += 1;
    let z = &x;
    println!("x={x}, y={y}, z={z}");
}
```
- The program does not compile, the line let z = &x; causes the error

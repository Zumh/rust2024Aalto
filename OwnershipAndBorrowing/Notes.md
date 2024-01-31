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

## Borrowing and dereferencing

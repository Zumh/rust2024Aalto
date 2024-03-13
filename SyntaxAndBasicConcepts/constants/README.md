## constant vs immutable
The important technical differences are:
- Constants can only be set to constant expressions, i.e. values that can be evaluated at compile-time. Immutable variables can be set to any value, including values known only at runtime.
- Constants must be annotated with their data type. We can let the compiler infer the type of an immutable variable.
- Constants can be declared anywhere, including outside functions aka globally. Immutable variables can only be declared in functions.
- There is one more type of variable in Rust, static variables, which are similar to constants. When we try to define an immutable variable in globally, the compiler will suggest using not only const but also static.

## static
- Static variables can be used globally like global constants, but they are rarely used because constants are most often the better choice. 
- We don't need to worry about them for now. We will come back to them later in the course when discussing memory in more detail.
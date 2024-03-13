## Rules of Owner Ship
### The rules of ownership are threefold:

- Each value in Rust is owned by a variable.
- Each value can have only one owner at a time.
- When the owner goes out of scope, the value will be dropped.

## Copying And Cloning
- The first rule lets the compiler keep track of each value through their owner. 
- The second rule is to ensure that a value within a variable won't be changed without explicitly moving or mutating the value.
- The second rule ensures also that values can be safely freed once the owner goes out of scope (the memory will be freed only once).
- The third rule guarantees that memory reserved for a variable is freed when the variable is no longer accessible. 
- Since variables are dropped automatically after the owner goes out of scope, we don't need to remember to free the reserved memory.
https://doc.rust-lang.org/reference/types.html

## Rust String useful methods
-  the string literal &str has to_lowercase and replace, 
- and f32 has round, abs and max that can make some programming tasks become a breeze.

## Important Data Types 
### Scalar Types
- Signed integers: i8, i16, i32, i64, i128 and isize (pointer size)
- Unsigned integers: u8, u16, u32, u64, u128 and usize (pointer size)
- Floating point: f32, f64
- char Unicode scalar values like 'a', 'α' and '∞' (4 bytes each)
- bool either true or false
- The unit type (), whose only possible value is an empty tuple: ()
Despite the value of a unit type being a tuple, it is not considered a compound type because it does not contain multiple values.

### Compound Types
- Arrays like [1, 2, 3]
- Tuples like (1, true)

### Static Typing 
- Rust is a strictly statically typed language. This means that 
- Ans: every variable type needs to be known at compile time.

### Valid variable types
- Select the option that does NOT result in an error in a Rust program.
- let i32 = 42;

### Valid variable types 2
- Select the option that does NOT result in an error in a Rust program.
- ans: let money: f64 = 2.718;

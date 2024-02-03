## if, else if and else
- Ternary
```rust
fn main() {
    let age = 18;
    let can_vote = if age >= 18 { true } else { false };
    println!("Can vote: {can_vote}");
}

```
- Assign if condition value
```rust
fn is_small(text: &str) -> bool {
  let length = if text == "small" {
    return true
  } else {
    text.len() > 0
  };
  length
}

fn main() {
  println!("{}", is_small("small"));
  println!("{}", is_small("smol"));
  println!("{}", is_small("smoll"));
}

```
- We can have early return but if we early return more than twice, do not use return. It auto return
- All return type must be the same.
- Floating point can use error margin check like EPSILON
```rust
fn main() {
    let a = 3.0 * 0.15; // type gets inferred as f64 from b
    let b = 0.45f64;
    println!("{a} == {b} ± {}", f64::EPSILON);
    println!("{}", (a - b).abs() < f64::EPSILON); // (a - b).abs() evaluates to the absolute difference of the two values
}

```

## Matching pattern
1. **Pattern Matching in Rust:** Rust provides a match expression for conditional control flow based on pattern matching. This is similar to a switch statement in other languages but more functional and powerful.

2. **Exhaustive Matches:** Matches in Rust are exhaustive, meaning that all possible values of the type must be covered by the patterns. This ensures comprehensive handling of different scenarios.

3. **Syntax of Match Expression:** The match syntax includes the match keyword, a scrutinee expression, and match arms enclosed in braces {}. The matching is performed from top to bottom, and the first matching arm is evaluated.

4. **Functional and Powerful:** The match expression is more functional and powerful compared to traditional switch statements. It is an expression itself and supports pattern matching.

5. **Range Patterns:** Rust allows the use of range patterns using the .. operator. For example, ..=-1 matches all numbers less than or equal to -1. This is useful for covering the full range of values, especially for integers.

6. **Variable Patterns:** Using a variable name as a pattern allows matching any value and binding it to the variable. This is helpful when working with types that can take an arbitrary number of values.

7. **Wildcard Pattern (_):** The underscore _ wildcard pattern matches any value but ignores it. This helps eliminate warnings about unused variables.

8. **Combining Patterns with |:** Multiple patterns can be combined into one using the | operator. This provides flexibility in specifying different cases in a concise manner.

9. **Match Target Expressions:** The expressions after the => in match arms work similarly to branches in if expressions. They can be any expression as long as they evaluate to a value of the correct type or return early from the function.

10. **Redundant Variables:** The use of variable patterns may introduce redundant variables. The underscore wildcard pattern can be used to match any value while ignoring it, addressing the issue of unused variable warnings.
- Match Guard
- do not guard the the last arm because compiler cannot determine a collection of guards cover all possible cases of a pattern.
```rust
fn main() {
    let x = -10;
    let polarity = match x {
        0 => "neutral",
        _ if x > 0 => "positive",
        _  => "negative", // problem?
    };
    println!("{polarity}");
}
```

## Handling errors by panicking
Errors in Rust can be handled using mechanisms such as panicking. Panicking occurs when something goes wrong in a Rust program, either due to actions leading to panic (e.g., integer division by zero) or explicitly invoking the `panic!` macro.

In the context of error handling, the `panic!` macro is used to signal a critical error where the program cannot continue. It takes a format string as an argument, similar to the `print` and `println` macros, and the specified message is displayed when the program terminates.

The provided practical example illustrates how panicking can be applied. In a program managing a drink machine with a maximum capacity of 50 drinks, if the program encounters a situation where the machine is stated to have more drinks than it can physically hold, it should use the `panic!` macro to halt the program and display an appropriate error message. This demonstrates the use of panicking as a means to handle exceptional situations and communicate errors effectively in Rust programs.
## Optional values
1. **Null Values in Programming Languages:**
   - Many programming languages use a special value called `null` to represent the absence of a value.
   - Null is often employed when a function or variable doesn't have a valid or meaningful value to return.

2. **Handling Unsupported Input:**
   - When dealing with unsupported input or invalid cases, functions may return a special value (such as an empty string or null) to indicate the absence of a meaningful result.

3. **Clarity in Function Behavior:**
   - Choosing an appropriate way to handle unsupported cases is essential for clarity in function behavior.
   - Returning a designated value for unsupported input helps users understand the outcome and prevents unexpected behavior or errors.

4. **Rust's Approach to Null Values:**
   - Rust does not allow variables to have null values.
   - Instead, Rust uses the `Option` type to represent the possible absence of a value.
   - The `Option` type provides a clear and explicit way to handle situations where a value may or may not be present.

5. **Avoiding Panics:**
   - In some languages, encountering unsupported input might lead to a program panic. Rust prefers providing a structured way to handle such cases, avoiding abrupt program termination.

6. **Emphasis on Safety:**
   - Rust's design choices, like disallowing null values, contribute to the language's focus on safety and preventing common programming errors.

7. **Explicit Handling with Option Type:**
   - The use of the `Option` type in Rust makes it explicit when a value may be absent, promoting safer and more predictable code.

8. **Understanding Ownership and Borrowing:**
   - Rust's ownership and borrowing system plays a role in ensuring safe handling of values, contributing to the language's emphasis on memory safety.

9. **Preventing Null-Related Bugs:**
   - The absence of null values in Rust can help prevent certain types of bugs associated with null references in other languages.

10. **Functional Programming Principles:**
    - Rust's use of the `Option` type aligns with functional programming principles, where explicit handling of absence or optional values is preferred over null references.

- Option type
    - Some and None
```rust
fn roman_numeral(num: usize) -> Option<String> {
    match num {
        1..=3 => Some("I".repeat(num)),
        4 | 9 => Some("I".to_string() + &roman_numeral(num + 1).unwrap_or_default()),
        5..=8 => Some("V".to_string() + &roman_numeral(num - 5).unwrap_or_default()),
        10..=39 => Some("X".to_string() + &roman_numeral(num - 10).unwrap_or_default()),
        _ => None, 
    }
}
fn main() {
    println!(
        "{} + {} = {}",
        roman_numeral(14).unwrap(),
        roman_numeral(19).unwrap(),
        roman_numeral(14 + 19).unwrap()
    );
}
```


## Recoverable errors

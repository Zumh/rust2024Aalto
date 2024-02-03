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

## Optional values

## Recoverable errors

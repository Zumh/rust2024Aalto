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

## Matching pattern

## Handling errors by panicking

## Optional values

## Recoverable errors
